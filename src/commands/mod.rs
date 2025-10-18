// Commands 模块 - 包含所有工具的更新命令实现
// 包括 Homebrew、Rustup 和 Mise 的更新逻辑

pub mod homebrew;
pub mod mise;
pub mod rustup;
pub mod upgrade_details;

// 重新导出各个模块的公共函数
pub use homebrew::{brew_cleanup, brew_update, brew_upgrade};
pub use mise::mise_up;
pub use rustup::rustup_update;
pub use upgrade_details::{UpgradeDetail, UpgradeDetails, UpgradeDetailsManager};
