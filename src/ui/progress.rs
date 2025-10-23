use super::icons::IconManager;
use crate::parallel::Tool;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::time::Duration;

/// 简化的进度条状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleProgressState {
    Preparing, // 准备中 (0%)
    Executing, // 执行中 (50%)
    Completed, // 已完成 (100%)
    Failed,    // 失败 (100%)
}

impl SimpleProgressState {
    /// 获取状态对应的进度百分比
    pub fn progress_percentage(&self) -> u64 {
        match self {
            SimpleProgressState::Preparing => 0,
            SimpleProgressState::Executing => 50,
            SimpleProgressState::Completed => 100,
            SimpleProgressState::Failed => 100,
        }
    }

    /// 获取状态显示消息
    pub fn display_message(&self, tool_name: &str) -> String {
        let icons = IconManager::new();
        match self {
            SimpleProgressState::Preparing => format!("{} 准备中...", tool_name),
            SimpleProgressState::Executing => format!("{} 执行中...", tool_name),
            SimpleProgressState::Completed => format!("{} {} 完成", icons.success(), tool_name),
            SimpleProgressState::Failed => format!("{} {} 失败", icons.failure(), tool_name),
        }
    }
}

/// 简化的进度条管理器
///
/// 此管理器负责在应用程序编排层统一管理所有工具的进度条显示。
/// 这是进度条管理的唯一入口点，命令执行函数（如 brew_update, rustup_update 等）
/// 不应该直接处理进度条，而是由此管理器根据执行结果更新状态。
///
/// # 架构说明
///
/// 进度条管理遵循以下分层架构：
/// - **编排层（main.rs）**：创建 SimpleProgressManager，在工具执行前后更新进度条状态
/// - **命令层（commands/*）**：专注于执行命令并返回结果，不涉及进度条
/// - **UI层（ui/progress.rs）**：提供进度条的创建、更新和显示功能
///
/// # 示例
///
/// ```rust
/// use devtool::ui::progress::{SimpleProgressManager, SimpleProgressState};
/// use devtool::parallel::Tool;
///
/// let mut manager = SimpleProgressManager::new();
/// let tools = vec![Tool::Homebrew, Tool::Rustup];
///
/// // 创建进度条
/// manager.create_progress_bars(&tools);
///
/// // 更新状态
/// manager.update_state(&Tool::Homebrew, SimpleProgressState::Executing);
/// manager.update_state(&Tool::Homebrew, SimpleProgressState::Completed);
///
/// // 完成所有进度条
/// manager.finalize_all();
/// ```
pub struct SimpleProgressManager {
    multi_progress: MultiProgress,
    progress_bars: HashMap<Tool, ProgressBar>,
    states: HashMap<Tool, SimpleProgressState>,
}

impl SimpleProgressManager {
    /// 创建新的简化进度条管理器
    ///
    /// 初始化一个空的进度条管理器，准备管理多个工具的进度显示。
    pub fn new() -> Self {
        Self {
            multi_progress: MultiProgress::new(),
            progress_bars: HashMap::new(),
            states: HashMap::new(),
        }
    }

    /// 为工具创建进度条
    ///
    /// 根据工具列表创建对应的进度条。此方法会自动检测是否在交互式终端中运行，
    /// 在非交互式环境中只记录状态而不显示进度条。
    ///
    /// # 参数
    /// * `tools` - 需要创建进度条的工具列表
    ///
    /// # 注意
    /// - 会自动避免为同一工具重复创建进度条
    /// - 在非交互式终端（如 CI 环境）中不会显示进度条
    pub fn create_progress_bars(&mut self, tools: &[Tool]) {
        // 检查是否在交互式终端中
        let is_interactive =
            std::env::var("TERM").unwrap_or_default() != "dumb" && atty::is(atty::Stream::Stdout);

        if !is_interactive {
            // 在非交互式终端中，只记录状态而不显示进度条
            for tool in tools {
                self.states
                    .insert(tool.clone(), SimpleProgressState::Preparing);
            }
            return;
        }

        for tool in tools {
            // 检查是否已存在该工具的进度条，避免重复创建
            if self.progress_bars.contains_key(tool) {
                continue;
            }

            let pb = self.multi_progress.add(ProgressBar::new(100));

            // 设置进度条样式 - 使用无边框现代化设计
            if let Ok(style) = ProgressStyle::default_bar()
                .template("{spinner:.green} {bar:20.cyan/blue} {pos}% {msg}")
            {
                pb.set_style(style.progress_chars("▰▱▰▱"));
            }

            pb.set_message(format!("{} 准备中...", tool.display_name()));
            pb.enable_steady_tick(Duration::from_millis(150));
            pb.tick(); // 立即显示进度条

            self.progress_bars.insert(tool.clone(), pb);
            self.states
                .insert(tool.clone(), SimpleProgressState::Preparing);
        }
    }

    /// 更新进度条状态
    ///
    /// 更新指定工具的进度条状态和显示消息。
    ///
    /// # 参数
    /// * `tool` - 要更新的工具
    /// * `new_state` - 新的进度状态
    pub fn update_state(&mut self, tool: &Tool, new_state: SimpleProgressState) {
        if let Some(pb) = self.progress_bars.get(tool) {
            let progress = new_state.progress_percentage();
            let message = new_state.display_message(tool.display_name());

            pb.set_position(progress);
            pb.set_message(message);
            pb.tick(); // 强制更新显示
        }
        self.states.insert(tool.clone(), new_state);
    }

    /// 完成所有进度条
    ///
    /// 根据每个工具的最终状态设置完成消息，并结束进度条显示。
    /// 此方法应在所有工具执行完成后调用。
    pub fn finalize_all(&mut self) {
        for (tool, pb) in &self.progress_bars {
            match self.states.get(tool) {
                Some(SimpleProgressState::Completed) => {
                    pb.set_message(format!(
                        "{} {} 完成",
                        IconManager::new().success(),
                        tool.display_name()
                    ));
                }
                Some(SimpleProgressState::Failed) => {
                    pb.set_message(format!(
                        "{} {} 失败",
                        IconManager::new().failure(),
                        tool.display_name()
                    ));
                }
                _ => {
                    pb.set_message(format!(
                        "{} {} 中断",
                        IconManager::new().pause(),
                        tool.display_name()
                    ));
                }
            }
            pb.finish();
        }
    }

    /// 检查工具是否已有进度条
    ///
    /// # 参数
    /// * `tool` - 要检查的工具
    ///
    /// # 返回值
    /// 如果该工具已有进度条则返回 true，否则返回 false
    pub fn has_progress_bar(&self, tool: &Tool) -> bool {
        self.progress_bars.contains_key(tool)
    }

    /// 获取当前活跃的进度条数量
    ///
    /// 这个方法用于调试和监控进度条状态，返回当前管理的进度条数量。
    /// 主要用于测试和调试目的，帮助开发者了解进度条管理器的状态。
    ///
    /// # 返回值
    /// 返回当前活跃的进度条数量
    ///
    /// # 示例
    /// ```rust
    /// let manager = SimpleProgressManager::new();
    /// assert_eq!(manager.progress_bar_count(), 0);
    /// ```
    #[allow(dead_code)]
    pub fn progress_bar_count(&self) -> usize {
        self.progress_bars.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_progress_state() {
        assert_eq!(SimpleProgressState::Preparing.progress_percentage(), 0);
        assert_eq!(SimpleProgressState::Executing.progress_percentage(), 50);
        assert_eq!(SimpleProgressState::Completed.progress_percentage(), 100);
        assert_eq!(SimpleProgressState::Failed.progress_percentage(), 100);
    }

    #[test]
    fn test_simple_progress_manager_creation() {
        let manager = SimpleProgressManager::new();
        assert_eq!(manager.progress_bar_count(), 0);
    }
}
