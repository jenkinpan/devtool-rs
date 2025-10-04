// Commands 模块 - 包含所有工具的更新命令实现
// 包括 Homebrew、Rustup 和 Mise 的更新逻辑

pub mod homebrew;
pub mod mise;
pub mod rustup;

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::runner::Runner;
use crate::ui::progress::Bar;

// 重新导出各个模块的公共函数
pub use homebrew::{brew_cleanup, brew_update, brew_upgrade};
pub use mise::mise_up;
pub use rustup::rustup_update;

/// 步骤函数类型定义
///
/// 每个更新步骤都是一个接受以下参数的函数：
/// - `runner`: 命令执行器
/// - `tmpdir`: 临时目录路径
/// - `verbose`: 是否显示详细输出
/// - `pbar`: 可选的进度条
///
/// 返回值是一个 Result，包含：
/// - 状态字符串 ("changed", "unchanged", "failed")
/// - 退出码
/// - 日志文件路径
pub type StepFn = fn(&dyn Runner, &Path, bool, &mut Option<Bar>) -> Result<(String, i32, PathBuf)>;

/// 步骤结构体
///
/// 表示一个更新步骤，包含描述和要执行的函数
#[derive(Clone)]
pub struct Step {
    /// 步骤的描述文本
    pub desc: String,
    /// 要执行的函数
    pub fn_name: StepFn,
}

impl Step {
    /// 创建一个新的步骤
    ///
    /// # 参数
    /// * `desc` - 步骤的描述文本
    /// * `fn_name` - 要执行的函数
    ///
    /// # 示例
    /// ```
    /// let step = Step::new("Homebrew: Update index".to_string(), brew_update);
    /// ```
    #[allow(dead_code)]
    pub fn new(desc: String, fn_name: StepFn) -> Self {
        Step { desc, fn_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_creation() {
        let step = Step::new("Test step".to_string(), brew_update as StepFn);
        assert_eq!(step.desc, "Test step");
    }
}
