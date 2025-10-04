use std::process::Command;

/// 检测系统语言
/// 返回 "zh" 或 "en"
pub fn detect_system_language() -> String {
    // 检查环境变量（优先级最高） / Check environment variables (highest priority)
    if let Ok(lang) = std::env::var("LANG") {
        if lang.starts_with("zh") {
            return "zh".to_string();
        } else if lang.starts_with("en") {
            return "en".to_string();
        }
    }

    if let Ok(lang) = std::env::var("LC_ALL") {
        if lang.starts_with("zh") {
            return "zh".to_string();
        } else if lang.starts_with("en") {
            return "en".to_string();
        }
    }

    if let Ok(lang) = std::env::var("LC_CTYPE") {
        if lang.starts_with("zh") {
            return "zh".to_string();
        } else if lang.starts_with("en") {
            return "en".to_string();
        }
    }

    // 检查 LANGUAGE 环境变量 / Check LANGUAGE environment variable
    if let Ok(lang) = std::env::var("LANGUAGE") {
        // LANGUAGE 格式通常是 "zh_CN:en_US"，取第一个语言
        if let Some(first_lang) = lang.split(':').next() {
            if first_lang.starts_with("zh") {
                return "zh".to_string();
            } else if first_lang.starts_with("en") {
                return "en".to_string();
            }
        }
    }

    // 检查系统语言设置（macOS） / Check system language settings (macOS)
    if let Ok(output) = Command::new("defaults")
        .args(&["read", "-g", "AppleLanguages"])
        .output()
    {
        if let Ok(lang_str) = String::from_utf8(output.stdout) {
            if lang_str.contains("zh") {
                return "zh".to_string();
            }
        }
    }

    // 默认返回英语 / Default to English
    "en".to_string()
}

/// 本地化字符串结构 / Localized string structure
#[derive(Debug, Clone)]
pub struct LocalizedStrings {
    pub banner: String,
    pub steps_count: String,
    pub progress_preparing: String,
    pub progress_complete: String,
    pub update_complete: String,
    pub time_taken: String,
    pub no_updates: String,
    pub actions_executed: String,
    pub already_latest: String,
    pub step_homebrew_update: String,
    pub step_homebrew_upgrade: String,
    pub step_cleanup: String,
    pub step_rust_update: String,
    pub step_mise_update: String,
}

impl LocalizedStrings {
    /// 根据语言代码创建本地化字符串
    pub fn new(lang: &str) -> Self {
        match lang {
            "zh" => Self {
                banner: "🚀 开始 devtool 更新：".to_string(),
                steps_count: "将执行 {} 个步骤：".to_string(),
                progress_preparing: "准备开始".to_string(),
                progress_complete: "完成".to_string(),
                update_complete: "🎉 更新完成：".to_string(),
                time_taken: "耗时".to_string(),
                no_updates: "ℹ️ 无更新应用。".to_string(),
                actions_executed: "🛠️ 已执行动作：".to_string(),
                already_latest: "⚠️ 已是最新：".to_string(),
                step_homebrew_update: "Homebrew：更新索引".to_string(),
                step_homebrew_upgrade: "Homebrew：升级软件包".to_string(),
                step_cleanup: "Action：清理旧版本".to_string(),
                step_rust_update: "Rust：更新 stable 工具链".to_string(),
                step_mise_update: "Mise：更新托管工具".to_string(),
            },
            _ => Self {
                banner: "🚀 Starting devtool update: ".to_string(),
                steps_count: "Will execute {} steps:".to_string(),
                progress_preparing: "Preparing to start".to_string(),
                progress_complete: "Complete".to_string(),
                update_complete: "🎉 Update completed: ".to_string(),
                time_taken: "Time taken".to_string(),
                no_updates: "ℹ️ No updates applied.".to_string(),
                actions_executed: "🛠️ Actions executed: ".to_string(),
                already_latest: "⚠️ Already latest: ".to_string(),
                step_homebrew_update: "Homebrew: Update index".to_string(),
                step_homebrew_upgrade: "Homebrew: Upgrade packages".to_string(),
                step_cleanup: "Action: Cleanup old versions".to_string(),
                step_rust_update: "Rust: Update stable toolchain".to_string(),
                step_mise_update: "Mise: Update managed tools".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_system_language() {
        let lang = detect_system_language();
        assert!(lang == "zh" || lang == "en");
    }

    #[test]
    fn test_localized_strings_zh() {
        let strings = LocalizedStrings::new("zh");
        assert!(strings.banner.contains("开始"));
        assert!(strings.update_complete.contains("完成"));
    }

    #[test]
    fn test_localized_strings_en() {
        let strings = LocalizedStrings::new("en");
        assert!(strings.banner.contains("Starting"));
        assert!(strings.update_complete.contains("completed"));
    }
}
