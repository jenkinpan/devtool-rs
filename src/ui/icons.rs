// 统一图标风格系统
// 提供本地化的图标资源管理，支持颜色和样式效果

use colored::*;
use std::env;

/// 图标类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum IconType {
    /// 本地化图标（支持颜色和样式）
    Local,
    /// ASCII 字符降级
    Ascii,
}

/// 图标样式枚举
#[derive(Debug, Clone, PartialEq)]
pub enum IconStyle {
    /// 默认样式
    Default,
    /// 成功样式（绿色加粗）
    Success,
    /// 失败样式（红色）
    Failure,
    /// 警告样式（黄色）
    Warning,
    /// 信息样式（蓝色）
    Info,
}

/// 图标管理器
pub struct IconManager {
    icon_type: IconType,
    supports_color: bool,
}

impl IconManager {
    /// 创建新的图标管理器
    pub fn new() -> Self {
        let icon_type = Self::detect_icon_support();
        let supports_color = Self::detect_color_support();
        Self {
            icon_type,
            supports_color,
        }
    }

    /// 检测终端对图标的支持情况
    fn detect_icon_support() -> IconType {
        // 检查是否明确禁用了图标
        if env::var("DEVMODE_NO_ICONS").is_ok() {
            return IconType::Ascii;
        }

        // 检查是否明确启用了本地化图标
        if env::var("DEVMODE_FORCE_LOCAL_ICONS").is_ok() {
            return IconType::Local;
        }

        // 默认使用本地化图标
        IconType::Local
    }

    /// 检测终端对颜色的支持情况
    fn detect_color_support() -> bool {
        // 检查是否明确禁用了颜色
        if env::var("NO_COLOR").is_ok() || env::var("DEVMODE_NO_COLOR").is_ok() {
            return false;
        }

        // 检查是否明确启用了颜色
        if env::var("FORCE_COLOR").is_ok() || env::var("DEVMODE_FORCE_COLOR").is_ok() {
            return true;
        }

        // 检查终端类型
        let term = env::var("TERM").unwrap_or_default();
        let term_program = env::var("TERM_PROGRAM").unwrap_or_default();

        // 大多数现代终端都支持颜色
        term.contains("xterm")
            || term.contains("screen")
            || term.contains("tmux")
            || term_program.contains("iTerm")
            || term_program.contains("Terminal")
            || term_program.contains("vscode")
            || term_program.contains("Alacritty")
            || term_program.contains("Kitty")
            || term_program.contains("WezTerm")
            || term_program.contains("Hyper")
            || term_program.contains("Terminus")
            || term_program.contains("Terminator")
            || term_program.contains("Gnome")
            || term_program.contains("Konsole")
            || term_program.contains("Xfce")
    }

    /// 获取当前图标类型
    #[cfg(test)]
    pub fn icon_type(&self) -> &IconType {
        &self.icon_type
    }

    /// 应用图标样式
    fn apply_style(&self, icon: &str, style: IconStyle) -> String {
        if !self.supports_color {
            return icon.to_string();
        }

        match style {
            IconStyle::Success => icon.green().bold().to_string(),
            IconStyle::Failure => icon.red().to_string(),
            IconStyle::Warning => icon.yellow().to_string(),
            IconStyle::Info => icon.blue().to_string(),
            IconStyle::Default => icon.to_string(),
        }
    }

    /// 获取火箭图标 (🚀)
    pub fn rocket(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "🚀";
                self.apply_style(icon, IconStyle::Default)
            }
            IconType::Ascii => ">".to_string(),
        }
    }

    /// 获取剪贴板图标 (📋)
    pub fn clipboard(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "📋";
                self.apply_style(icon, IconStyle::Default)
            }
            IconType::Ascii => "[*]".to_string(),
        }
    }

    /// 获取成功图标 (✅) - 绿色加粗
    pub fn success(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "✓";
                self.apply_style(icon, IconStyle::Success)
            }
            IconType::Ascii => "✓".to_string(),
        }
    }

    /// 获取失败图标 (❌) - 红色
    pub fn failure(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "✗";
                self.apply_style(icon, IconStyle::Failure)
            }
            IconType::Ascii => "✗".to_string(),
        }
    }

    /// 获取警告图标 (⚠️) - 黄色
    pub fn warning(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "⚠";
                self.apply_style(icon, IconStyle::Warning)
            }
            IconType::Ascii => "⚠".to_string(),
        }
    }

    /// 获取信息图标 (ℹ️) - 蓝色
    pub fn info(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "ℹ";
                self.apply_style(icon, IconStyle::Info)
            }
            IconType::Ascii => "ℹ".to_string(),
        }
    }

    /// 获取包裹图标 (📦) - Homebrew
    pub fn package(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "📦";
                self.apply_style(icon, IconStyle::Default)
            }
            IconType::Ascii => "📦".to_string(),
        }
    }

    /// 获取 Rust 图标 (🦀)
    pub fn rust(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "🦀";
                self.apply_style(icon, IconStyle::Default)
            }
            IconType::Ascii => "🦀".to_string(),
        }
    }

    /// 获取扳手图标 (🔧) - Mise
    pub fn wrench(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "🔧";
                self.apply_style(icon, IconStyle::Default)
            }
            IconType::Ascii => "🔧".to_string(),
        }
    }

    /// 获取暂停图标 (⏸️)
    pub fn pause(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "⏸";
                self.apply_style(icon, IconStyle::Default)
            }
            IconType::Ascii => "⏸".to_string(),
        }
    }

    /// 获取工具图标 (🛠️)
    pub fn tools(&self) -> String {
        match self.icon_type {
            IconType::Local => {
                let icon = "🛠";
                self.apply_style(icon, IconStyle::Default)
            }
            IconType::Ascii => "🛠".to_string(),
        }
    }
}

impl Default for IconManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_manager_creation() {
        let manager = IconManager::new();
        assert!(matches!(
            manager.icon_type(),
            IconType::Local | IconType::Ascii
        ));
    }

    #[test]
    fn test_icon_consistency() {
        let manager = IconManager::new();

        // 测试所有图标都能正确返回
        let _ = manager.rocket();
        let _ = manager.clipboard();
        let _ = manager.success();
        let _ = manager.failure();
        let _ = manager.warning();
        let _ = manager.info();
        let _ = manager.package();
        let _ = manager.rust();
        let _ = manager.wrench();
        let _ = manager.pause();
        let _ = manager.tools();
    }

    #[test]
    fn test_ascii_fallback() {
        // 设置环境变量强制使用 ASCII 模式
        env::set_var("DEVMODE_NO_ICONS", "1");
        let manager = IconManager::new();
        assert_eq!(manager.icon_type(), &IconType::Ascii);
        env::remove_var("DEVMODE_NO_ICONS");
    }
}
