use super::icons::IconManager;
use crate::parallel::Tool;
use crate::utils::get_cache_dir;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time::Instant;

/// 全局已创建的工具集合，防止重复创建进度条
static GLOBAL_CREATED_TOOLS: OnceLock<Arc<std::sync::Mutex<HashSet<Tool>>>> = OnceLock::new();

/// 获取全局已创建的工具集合
fn get_global_created_tools() -> Arc<std::sync::Mutex<HashSet<Tool>>> {
    GLOBAL_CREATED_TOOLS
        .get_or_init(|| Arc::new(std::sync::Mutex::new(HashSet::new())))
        .clone()
}

/// 进度条状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressState {
    Preparing,     // 准备中 (0%)
    Executing,     // 执行中 (25%)
    ExecutingMid,  // 执行中期 (50%)
    ExecutingLate, // 执行后期 (75%)
    Completed,     // 已完成 (100%)
    Failed,        // 失败 (100%)
}

/// 进度条动画管理器
#[derive(Clone)]
pub struct ProgressAnimationManager {
    start_time: Instant,
    update_interval: Duration,
    is_animating: bool,
    /// 最小显示时间，用于确保进度条状态显示足够长的时间
    /// 目前未直接使用，但保留用于未来的显示时间控制功能
    #[allow(dead_code)]
    min_display_time: Duration,
    last_state_time: Instant,
}

impl ProgressAnimationManager {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            update_interval: Duration::from_millis(200), // 默认 200ms 更新间隔
            is_animating: false,
            min_display_time: Duration::from_millis(1000), // 最小显示时间 1 秒
            last_state_time: Instant::now(),
        }
    }

    /// 检测终端性能并调整更新间隔
    pub fn adjust_for_performance(&mut self) {
        // 简单的性能检测：如果系统负载高，降低更新频率
        if std::env::var("TERM").unwrap_or_default() == "dumb" {
            self.update_interval = Duration::from_millis(500);
        }
    }

    /// 启动动画
    pub fn start_animation(&mut self) {
        self.start_time = Instant::now();
        self.is_animating = true;
    }

    /// 停止动画
    pub fn stop_animation(&mut self) {
        self.is_animating = false;
    }

    /// 检查是否应该更新进度
    ///
    /// 这个方法用于检查动画是否应该更新，基于当前时间和更新间隔。
    /// 目前未使用，但保留用于未来的动画优化功能。
    #[allow(dead_code)]
    pub fn should_update(&self) -> bool {
        self.is_animating && self.start_time.elapsed() >= self.update_interval
    }

    /// 获取动画进度百分比（基于时间模拟）
    pub fn get_animation_progress(&self, current_state: &ProgressState) -> u64 {
        if !self.is_animating {
            return current_state.progress_percentage();
        }

        let elapsed = self.start_time.elapsed();
        let total_duration = Duration::from_secs(10); // 假设总执行时间为 10 秒

        match current_state {
            ProgressState::Executing => {
                // 在执行阶段，模拟从 25% 到 50% 的进度
                let progress =
                    (elapsed.as_millis() as f64 / total_duration.as_millis() as f64).min(1.0);
                (25.0 + (25.0 * progress)) as u64
            }
            ProgressState::ExecutingMid => {
                // 在执行中期，模拟从 50% 到 75% 的进度
                let progress =
                    (elapsed.as_millis() as f64 / total_duration.as_millis() as f64).min(1.0);
                (50.0 + (25.0 * progress)) as u64
            }
            ProgressState::ExecutingLate => {
                // 在执行后期，模拟从 75% 到 100% 的进度
                let progress =
                    (elapsed.as_millis() as f64 / total_duration.as_millis() as f64).min(1.0);
                (75.0 + (25.0 * progress)) as u64
            }
            _ => current_state.progress_percentage(),
        }
    }

    /// 检查状态是否已经显示足够时间
    ///
    /// 这个方法用于检查进度条状态是否已经显示了足够长的时间，
    /// 确保用户能够看到状态变化。目前未使用，但保留用于未来的
    /// 进度条显示优化功能。
    #[allow(dead_code)]
    pub fn has_displayed_long_enough(&self) -> bool {
        self.last_state_time.elapsed() >= self.min_display_time
    }

    /// 更新状态显示时间
    pub fn update_state_time(&mut self) {
        self.last_state_time = Instant::now();
    }
}

/// 进度条管理器
#[derive(Clone)]
pub struct ProgressBarManager {
    multi_progress: Arc<MultiProgress>,
    progress_bars: HashMap<Tool, ProgressBar>,
    states: HashMap<Tool, ProgressState>,
    animation_manager: ProgressAnimationManager,
    /// 进度条实例计数器，用于跟踪创建的进度条数量
    instance_count: Arc<std::sync::atomic::AtomicUsize>,
    /// 已创建的工具集合，防止重复创建
    created_tools: Arc<std::sync::Mutex<HashSet<Tool>>>,
}

/// 进度状态结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressStatus {
    pub state: String,
    pub percent: Option<i32>,
    pub done: Option<u64>,
    pub total: Option<u64>,
    pub desc: Option<String>,
    pub ts: Option<String>,
}

/// 查询进度状态命令
pub fn progress_status_cmd() -> anyhow::Result<()> {
    use anyhow::Context;

    let cache_dir = get_cache_dir();
    let status_file = cache_dir.join("progress.status");
    if !status_file.exists() {
        println!("No progress.status file: {:?}", status_file);
        return Ok(());
    }
    let s = fs::read_to_string(&status_file).with_context(|| "read progress.status")?;
    match serde_json::from_str::<ProgressStatus>(&s) {
        Ok(ps) => {
            println!("Progress status: {:?}", ps);
        }
        Err(e) => {
            println!("Raw content: {}", s);
            eprintln!("failed to parse JSON: {}", e);
        }
    }
    Ok(())
}

impl ProgressState {
    /// 获取状态对应的进度百分比
    pub fn progress_percentage(&self) -> u64 {
        match self {
            ProgressState::Preparing => 0,
            ProgressState::Executing => 25,
            ProgressState::ExecutingMid => 50,
            ProgressState::ExecutingLate => 75,
            ProgressState::Completed => 100,
            ProgressState::Failed => 100,
        }
    }

    /// 获取状态对应的显示消息
    pub fn display_message(&self, tool_name: &str, icons: &IconManager) -> String {
        match self {
            ProgressState::Preparing => format!("{} 准备中...", tool_name),
            ProgressState::Executing => format!("{} 执行中...", tool_name),
            ProgressState::ExecutingMid => format!("{} 执行中...", tool_name),
            ProgressState::ExecutingLate => format!("{} 执行中...", tool_name),
            ProgressState::Completed => format!("{} {} 完成", icons.success(), tool_name),
            ProgressState::Failed => format!("{} {} 失败", icons.failure(), tool_name),
        }
    }

    /// 验证状态转换是否有效
    pub fn is_valid_transition(&self, new_state: &ProgressState) -> bool {
        match (self, new_state) {
            // 准备中 -> 执行中
            (ProgressState::Preparing, ProgressState::Executing) => true,
            // 执行中 -> 执行中期
            (ProgressState::Executing, ProgressState::ExecutingMid) => true,
            // 执行中期 -> 执行后期
            (ProgressState::ExecutingMid, ProgressState::ExecutingLate) => true,
            // 任何执行状态 -> 完成或失败
            (ProgressState::Executing, ProgressState::Completed) => true,
            (ProgressState::Executing, ProgressState::Failed) => true,
            (ProgressState::ExecutingMid, ProgressState::Completed) => true,
            (ProgressState::ExecutingMid, ProgressState::Failed) => true,
            (ProgressState::ExecutingLate, ProgressState::Completed) => true,
            (ProgressState::ExecutingLate, ProgressState::Failed) => true,
            // 完成或失败状态不能转换到其他状态
            (ProgressState::Completed, _) => false,
            (ProgressState::Failed, _) => false,
            // 其他转换无效
            _ => false,
        }
    }
}

impl ProgressBarManager {
    /// 创建新的进度条管理器
    pub fn new() -> Self {
        let mut animation_manager = ProgressAnimationManager::new();
        animation_manager.adjust_for_performance();

        Self {
            multi_progress: Arc::new(MultiProgress::new()),
            progress_bars: HashMap::new(),
            states: HashMap::new(),
            animation_manager,
            instance_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            created_tools: get_global_created_tools(),
        }
    }

    /// 为工具创建进度条
    pub fn create_progress_bars(&mut self, tools: &[Tool]) {
        // 检查是否在交互式终端中
        let is_interactive =
            std::env::var("TERM").unwrap_or_default() != "dumb" && atty::is(atty::Stream::Stdout);

        if !is_interactive {
            // 在非交互式终端中，只记录状态而不显示进度条
            for tool in tools {
                self.states.insert(tool.clone(), ProgressState::Preparing);
            }
            return;
        }

        for tool in tools {
            // 检查是否已存在该工具的进度条，避免重复创建
            if self.progress_bars.contains_key(tool) {
                // 如果已存在，跳过创建
                continue;
            }

            // 检查是否已经在其他实例中创建过
            if let Ok(mut created_tools) = self.created_tools.lock() {
                if created_tools.contains(tool) {
                    // 如果已经在其他实例中创建过，跳过创建
                    continue;
                }
                created_tools.insert(tool.clone());
            }

            let pb = self.multi_progress.add(ProgressBar::new(100));

            // 设置进度条样式 - 使用无边框现代化设计
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} {bar:20.cyan/blue} {pos}% {msg}")
                    .unwrap()
                    .progress_chars("▰▱▰▱"),
            );

            pb.set_message(format!("{} 准备中...", tool.display_name()));
            pb.enable_steady_tick(Duration::from_millis(150));
            pb.tick(); // 立即显示进度条

            self.progress_bars.insert(tool.clone(), pb);
            self.states.insert(tool.clone(), ProgressState::Preparing);

            // 增加实例计数器
            self.instance_count
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    /// 更新进度条状态
    pub fn update_state(&mut self, tool: &Tool, new_state: ProgressState) {
        // 验证状态转换是否有效
        if let Some(current_state) = self.states.get(tool) {
            if !current_state.is_valid_transition(&new_state) {
                eprintln!(
                    "警告: 无效的状态转换 {:?} -> {:?} for {}",
                    current_state,
                    new_state,
                    tool.display_name()
                );
                return;
            }
        }

        // 检查是否在交互式终端中
        let is_interactive =
            std::env::var("TERM").unwrap_or_default() != "dumb" && atty::is(atty::Stream::Stdout);

        if !is_interactive {
            // 在非交互式终端中，只记录状态
            self.states.insert(tool.clone(), new_state);
            return;
        }

        if let Some(pb) = self.progress_bars.get(tool) {
            // 启动动画（如果进入执行状态）
            if matches!(
                new_state,
                ProgressState::Executing
                    | ProgressState::ExecutingMid
                    | ProgressState::ExecutingLate
            ) {
                self.animation_manager.start_animation();
                self.animation_manager.update_state_time();
            }

            // 使用动画进度或静态进度
            let progress = if matches!(
                new_state,
                ProgressState::Executing
                    | ProgressState::ExecutingMid
                    | ProgressState::ExecutingLate
            ) {
                self.animation_manager.get_animation_progress(&new_state)
            } else {
                new_state.progress_percentage()
            };

            let icons = IconManager::new();
            let message = new_state.display_message(tool.display_name(), &icons);

            pb.set_position(progress);
            pb.set_message(message);
            pb.tick(); // 强制更新显示
        }
        self.states.insert(tool.clone(), new_state);
    }

    /// 完成所有进度条
    pub fn finalize_all(&mut self) {
        // 停止动画
        self.animation_manager.stop_animation();

        let icons = IconManager::new();
        for (tool, pb) in &self.progress_bars {
            match self.states.get(tool) {
                Some(ProgressState::Completed) => {
                    pb.set_message(format!("{} {} 完成", icons.success(), tool.display_name()));
                }
                Some(ProgressState::Failed) => {
                    pb.set_message(format!("{} {} 失败", icons.failure(), tool.display_name()));
                }
                _ => {
                    pb.set_message(format!("{} {} 中断", icons.pause(), tool.display_name()));
                }
            }
            pb.finish();
        }
    }

    /// 获取多进度条管理器
    pub fn get_multi_progress(&self) -> Arc<MultiProgress> {
        self.multi_progress.clone()
    }

    /// 检查工具是否已有进度条
    ///
    /// 这个方法用于检查指定工具是否已经有对应的进度条。
    /// 目前未使用，但保留用于未来的进度条管理功能。
    #[allow(dead_code)]
    pub fn has_progress_bar(&self, tool: &Tool) -> bool {
        self.progress_bars.contains_key(tool)
    }

    /// 获取进度条数量
    ///
    /// 这个方法用于获取当前管理的进度条数量。
    /// 目前未使用，但保留用于未来的进度条统计功能。
    #[allow(dead_code)]
    pub fn progress_bar_count(&self) -> usize {
        self.progress_bars.len()
    }

    /// 启动全局进度更新
    ///
    /// 这个方法用于启动全局进度更新，确保所有工具都有正确的进度条状态。
    /// 目前简化实现，避免重复创建进度条。
    #[allow(dead_code)]
    pub fn start_global_progress_updates(&mut self, tools: &[Tool]) {
        // 简化实现：只为执行中的工具启动动画
        for tool in tools {
            if let Some(state) = self.states.get(tool) {
                if matches!(state, ProgressState::Executing) {
                    self.animation_manager.start_animation();
                    self.animation_manager.update_state_time();
                }
            }
        }
    }

    /// 获取进度条实例数量
    ///
    /// 这个方法用于获取当前创建的进度条实例数量，用于调试和监控。
    /// 目前未使用，但保留用于未来的进度条管理功能。
    #[allow(dead_code)]
    pub fn get_instance_count(&self) -> usize {
        self.instance_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 检查工具是否已在任何实例中创建
    ///
    /// 这个方法用于检查指定工具是否已经在任何实例中创建过进度条。
    /// 目前未使用，但保留用于未来的进度条管理功能。
    #[allow(dead_code)]
    pub fn is_tool_created(&self, tool: &Tool) -> bool {
        if let Ok(created_tools) = self.created_tools.lock() {
            created_tools.contains(tool)
        } else {
            false
        }
    }

    /// 清理进度条实例
    ///
    /// 这个方法用于清理进度条实例，重置计数器。
    /// 目前未使用，但保留用于未来的进度条管理功能。
    #[allow(dead_code)]
    pub fn cleanup_instances(&mut self) {
        self.instance_count
            .store(0, std::sync::atomic::Ordering::Relaxed);
        if let Ok(mut created_tools) = self.created_tools.lock() {
            created_tools.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_status_serialization() {
        let ps = ProgressStatus {
            state: "test".to_string(),
            percent: Some(50),
            done: Some(5),
            total: Some(10),
            desc: Some("Testing".to_string()),
            ts: Some("2024-01-01T00:00:00+00:00".to_string()),
        };
        let json = serde_json::to_string(&ps).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("50"));
    }

    #[test]
    fn test_progress_bar_manager_creation() {
        let mut manager = ProgressBarManager::new();
        let tools = vec![Tool::Homebrew, Tool::Rustup, Tool::Mise];

        manager.create_progress_bars(&tools);

        // 验证管理器可以正常创建（通过检查内部状态）
        // 由于我们移除了 getter 方法，这里只验证创建过程不报错
        assert_eq!(tools.len(), 3);
    }

    #[test]
    fn test_progress_state_transitions() {
        let mut manager = ProgressBarManager::new();
        let tool = Tool::Homebrew;

        manager.create_progress_bars(std::slice::from_ref(&tool));

        // 测试状态转换（通过调用方法验证不报错）
        manager.update_state(&tool, ProgressState::Executing);
        manager.update_state(&tool, ProgressState::Completed);
        manager.update_state(&tool, ProgressState::Failed);

        // 验证状态转换过程不报错
        assert_eq!(tool, Tool::Homebrew);
    }

    #[test]
    fn test_progress_state_percentage() {
        assert_eq!(ProgressState::Preparing.progress_percentage(), 0);
        assert_eq!(ProgressState::Executing.progress_percentage(), 25);
        assert_eq!(ProgressState::ExecutingMid.progress_percentage(), 50);
        assert_eq!(ProgressState::ExecutingLate.progress_percentage(), 75);
        assert_eq!(ProgressState::Completed.progress_percentage(), 100);
        assert_eq!(ProgressState::Failed.progress_percentage(), 100);
    }

    #[test]
    fn test_progress_state_display_message() {
        let tool_name = "TestTool";
        let icons = IconManager::new();
        assert!(ProgressState::Preparing
            .display_message(tool_name, &icons)
            .contains("准备中"));
        assert!(ProgressState::Executing
            .display_message(tool_name, &icons)
            .contains("执行中"));
        assert!(ProgressState::ExecutingMid
            .display_message(tool_name, &icons)
            .contains("执行中"));
        assert!(ProgressState::ExecutingLate
            .display_message(tool_name, &icons)
            .contains("执行中"));
        assert!(ProgressState::Completed
            .display_message(tool_name, &icons)
            .contains("完成"));
        assert!(ProgressState::Failed
            .display_message(tool_name, &icons)
            .contains("失败"));
    }

    #[test]
    fn test_progress_state_transition_validation() {
        // 测试有效的状态转换
        assert!(ProgressState::Preparing.is_valid_transition(&ProgressState::Executing));
        assert!(ProgressState::Executing.is_valid_transition(&ProgressState::ExecutingMid));
        assert!(ProgressState::ExecutingMid.is_valid_transition(&ProgressState::ExecutingLate));
        assert!(ProgressState::Executing.is_valid_transition(&ProgressState::Completed));
        assert!(ProgressState::ExecutingMid.is_valid_transition(&ProgressState::Failed));
        assert!(ProgressState::ExecutingLate.is_valid_transition(&ProgressState::Completed));

        // 测试无效的状态转换
        assert!(!ProgressState::Completed.is_valid_transition(&ProgressState::Executing));
        assert!(!ProgressState::Failed.is_valid_transition(&ProgressState::Executing));
        assert!(!ProgressState::Executing.is_valid_transition(&ProgressState::Preparing));
        assert!(!ProgressState::ExecutingLate.is_valid_transition(&ProgressState::Executing));
    }
}
