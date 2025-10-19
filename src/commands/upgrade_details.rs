// 升级详情标准化模块
// 提供统一的升级详情格式和文件处理

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// 升级详情条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeDetail {
    /// 工具/软件包名称
    pub name: String,
    /// 旧版本
    pub old_version: String,
    /// 新版本
    pub new_version: String,
    /// 升级类型
    pub upgrade_type: UpgradeType,
}

/// 升级类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpgradeType {
    /// 版本升级
    VersionUpgrade,
    /// 新安装
    NewInstallation,
    /// 降级
    Downgrade,
}

/// 升级详情集合
#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeDetails {
    /// 工具名称
    pub tool_name: String,
    /// 升级详情列表
    pub details: Vec<UpgradeDetail>,
    /// 升级时间戳
    pub timestamp: String,
    /// 升级总数
    pub total_count: usize,
}

impl UpgradeDetail {
    /// 创建版本升级详情
    pub fn version_upgrade(name: String, old_version: String, new_version: String) -> Self {
        Self {
            name,
            old_version,
            new_version,
            upgrade_type: UpgradeType::VersionUpgrade,
        }
    }

    /// 创建新安装详情
    pub fn new_installation(name: String, version: String) -> Self {
        Self {
            name,
            old_version: "未安装".to_string(),
            new_version: version,
            upgrade_type: UpgradeType::NewInstallation,
        }
    }

    /// 创建降级详情
    #[allow(dead_code)]
    pub fn downgrade(name: String, old_version: String, new_version: String) -> Self {
        Self {
            name,
            old_version,
            new_version,
            upgrade_type: UpgradeType::Downgrade,
        }
    }

    /// 格式化为显示字符串
    #[allow(dead_code)]
    pub fn to_display_string(&self) -> String {
        match self.upgrade_type {
            UpgradeType::VersionUpgrade => {
                format!("{}: {} → {}", self.name, self.old_version, self.new_version)
            }
            UpgradeType::NewInstallation => {
                format!("{}: new installation → {}", self.name, self.new_version)
            }
            UpgradeType::Downgrade => {
                format!(
                    "{}: {} → {} (降级)",
                    self.name, self.old_version, self.new_version
                )
            }
        }
    }

    /// 格式化为简单字符串（兼容现有格式）
    pub fn to_legacy_string(&self) -> String {
        match self.upgrade_type {
            UpgradeType::VersionUpgrade => {
                format!("{}: {} → {}", self.name, self.old_version, self.new_version)
            }
            UpgradeType::NewInstallation => {
                format!("{}: new installation → {}", self.name, self.new_version)
            }
            UpgradeType::Downgrade => {
                format!("{}: {} → {}", self.name, self.old_version, self.new_version)
            }
        }
    }

    /// 格式化为增强的显示字符串（支持工具链类型标识）
    pub fn to_enhanced_string(&self) -> String {
        let toolchain_type = self.get_toolchain_type();
        let type_indicator = if !toolchain_type.is_empty() {
            format!("[{}] ", toolchain_type)
        } else {
            String::new()
        };

        match self.upgrade_type {
            UpgradeType::VersionUpgrade => {
                format!(
                    "{}{}: {} → {}",
                    type_indicator, self.name, self.old_version, self.new_version
                )
            }
            UpgradeType::NewInstallation => {
                format!(
                    "{}{}: new installation → {}",
                    type_indicator, self.name, self.new_version
                )
            }
            UpgradeType::Downgrade => {
                format!(
                    "{}{}: {} → {} (降级)",
                    type_indicator, self.name, self.old_version, self.new_version
                )
            }
        }
    }

    /// 获取工具链类型标识
    fn get_toolchain_type(&self) -> String {
        if self.name.contains("stable") {
            "stable".to_string()
        } else if self.name.contains("beta") {
            "beta".to_string()
        } else if self.name.contains("nightly") {
            "nightly".to_string()
        } else {
            String::new()
        }
    }
}

impl UpgradeDetails {
    /// 创建新的升级详情集合
    pub fn new(tool_name: String) -> Self {
        Self {
            tool_name,
            details: Vec::new(),
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            total_count: 0,
        }
    }

    /// 添加升级详情
    #[allow(dead_code)]
    pub fn add_detail(&mut self, detail: UpgradeDetail) {
        self.details.push(detail);
        self.total_count = self.details.len();
    }

    /// 批量添加升级详情
    pub fn add_details(&mut self, details: Vec<UpgradeDetail>) {
        self.details.extend(details);
        self.total_count = self.details.len();
    }

    /// 检查是否有升级
    pub fn has_upgrades(&self) -> bool {
        !self.details.is_empty()
    }

    /// 获取升级数量
    #[allow(dead_code)]
    pub fn upgrade_count(&self) -> usize {
        self.details.len()
    }

    /// 获取新安装数量
    #[allow(dead_code)]
    pub fn new_installation_count(&self) -> usize {
        self.details
            .iter()
            .filter(|d| matches!(d.upgrade_type, UpgradeType::NewInstallation))
            .count()
    }

    /// 获取版本升级数量
    #[allow(dead_code)]
    pub fn version_upgrade_count(&self) -> usize {
        self.details
            .iter()
            .filter(|d| matches!(d.upgrade_type, UpgradeType::VersionUpgrade))
            .count()
    }

    /// 获取降级数量
    #[allow(dead_code)]
    pub fn downgrade_count(&self) -> usize {
        self.details
            .iter()
            .filter(|d| matches!(d.upgrade_type, UpgradeType::Downgrade))
            .count()
    }

    /// 格式化为显示字符串列表
    #[allow(dead_code)]
    pub fn to_display_strings(&self) -> Vec<String> {
        self.details.iter().map(|d| d.to_display_string()).collect()
    }

    /// 格式化为传统字符串列表（兼容现有格式）
    #[allow(dead_code)]
    pub fn to_legacy_strings(&self) -> Vec<String> {
        self.details.iter().map(|d| d.to_legacy_string()).collect()
    }

    /// 保存到文件（JSON 格式）
    pub fn save_to_json_file(&self, file_path: &Path) -> Result<()> {
        let json_content = serde_json::to_string_pretty(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json_content.as_bytes())?;
        Ok(())
    }

    /// 保存到文件（传统文本格式）
    pub fn save_to_text_file(&self, file_path: &Path) -> Result<()> {
        let mut file = File::create(file_path)?;
        for detail in &self.details {
            writeln!(file, "{}", detail.to_legacy_string())?;
        }
        Ok(())
    }

    /// 保存到增强文本文件（支持工具链类型分组）
    pub fn save_to_enhanced_text_file(&self, file_path: &Path) -> Result<()> {
        let mut file = File::create(file_path)?;

        // 按工具链类型分组
        let mut stable_upgrades = Vec::new();
        let mut beta_upgrades = Vec::new();
        let mut nightly_upgrades = Vec::new();
        let mut other_upgrades = Vec::new();

        for detail in &self.details {
            let toolchain_type = detail.get_toolchain_type();
            match toolchain_type.as_str() {
                "stable" => stable_upgrades.push(detail),
                "beta" => beta_upgrades.push(detail),
                "nightly" => nightly_upgrades.push(detail),
                _ => other_upgrades.push(detail),
            }
        }

        // 写入分组后的内容
        if !stable_upgrades.is_empty() {
            writeln!(file, "Stable 工具链:")?;
            for detail in stable_upgrades {
                writeln!(file, "  {}", detail.to_enhanced_string())?;
            }
            writeln!(file)?;
        }

        if !beta_upgrades.is_empty() {
            writeln!(file, "Beta 工具链:")?;
            for detail in beta_upgrades {
                writeln!(file, "  {}", detail.to_enhanced_string())?;
            }
            writeln!(file)?;
        }

        if !nightly_upgrades.is_empty() {
            writeln!(file, "Nightly 工具链:")?;
            for detail in nightly_upgrades {
                writeln!(file, "  {}", detail.to_enhanced_string())?;
            }
            writeln!(file)?;
        }

        if !other_upgrades.is_empty() {
            writeln!(file, "其他工具链:")?;
            for detail in other_upgrades {
                writeln!(file, "  {}", detail.to_enhanced_string())?;
            }
        }

        Ok(())
    }

    /// 从 JSON 文件加载
    pub fn load_from_json_file(file_path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(file_path)?;
        let details: Self = serde_json::from_str(&content)?;
        Ok(details)
    }

    /// 从文本文件加载（传统格式）
    pub fn load_from_text_file(file_path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(file_path)?;
        let mut details = Self::new("unknown".to_string());

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // 解析传统格式: "name: old → new" 或 "name: new installation → new"
            if let Some((name_part, version_part)) = line.split_once(':') {
                let name = name_part.trim().to_string();

                if version_part.contains("new installation") {
                    // 新安装格式: "name: new installation → version"
                    if let Some((_, version)) = version_part.split_once("→") {
                        let version = version.trim().to_string();
                        details.add_detail(UpgradeDetail::new_installation(name, version));
                    }
                } else if let Some((old, new)) = version_part.split_once("→") {
                    // 版本升级格式: "name: old → new"
                    let old_version = old.trim().to_string();
                    let new_version = new.trim().to_string();
                    details.add_detail(UpgradeDetail::version_upgrade(
                        name,
                        old_version,
                        new_version,
                    ));
                }
            }
        }

        Ok(details)
    }
}

/// 工具升级详情管理器
pub struct UpgradeDetailsManager;

impl UpgradeDetailsManager {
    /// 为 Homebrew 创建升级详情
    #[allow(dead_code)]
    pub fn create_homebrew_details(
        tool_name: String,
        details: Vec<UpgradeDetail>,
    ) -> UpgradeDetails {
        let mut upgrade_details = UpgradeDetails::new(tool_name);
        upgrade_details.add_details(details);
        upgrade_details
    }

    /// 为 Rustup 创建升级详情
    #[allow(dead_code)]
    pub fn create_rustup_details(tool_name: String, details: Vec<UpgradeDetail>) -> UpgradeDetails {
        let mut upgrade_details = UpgradeDetails::new(tool_name);
        upgrade_details.add_details(details);
        upgrade_details
    }

    /// 为 Mise 创建升级详情
    #[allow(dead_code)]
    pub fn create_mise_details(tool_name: String, details: Vec<UpgradeDetail>) -> UpgradeDetails {
        let mut upgrade_details = UpgradeDetails::new(tool_name);
        upgrade_details.add_details(details);
        upgrade_details
    }

    /// 保存升级详情到标准文件
    pub fn save_upgrade_details(
        details: &UpgradeDetails,
        tmpdir: &Path,
        tool_name: &str,
    ) -> Result<()> {
        // 保存 JSON 格式（用于程序处理）
        let json_file = tmpdir.join(format!("{}_upgrade_details.json", tool_name));
        details.save_to_json_file(&json_file)?;

        // 保存文本格式（用于显示）
        let text_file = tmpdir.join(format!("{}_upgrade_details.txt", tool_name));
        details.save_to_text_file(&text_file)?;

        // 为 Rustup 保存增强格式（支持工具链类型分组）
        if tool_name == "rustup" {
            let enhanced_file = tmpdir.join(format!("{}_upgrade_details_enhanced.txt", tool_name));
            details.save_to_enhanced_text_file(&enhanced_file)?;
        }

        Ok(())
    }

    /// 从标准文件加载升级详情
    #[allow(dead_code)]
    pub fn load_upgrade_details(tmpdir: &Path, tool_name: &str) -> Result<Option<UpgradeDetails>> {
        let json_file = tmpdir.join(format!("{}_upgrade_details.json", tool_name));

        if json_file.exists() {
            Ok(Some(UpgradeDetails::load_from_json_file(&json_file)?))
        } else {
            // 尝试加载传统文本格式
            let text_file = tmpdir.join(format!("{}_upgrade_details.txt", tool_name));
            if text_file.exists() {
                Ok(Some(UpgradeDetails::load_from_text_file(&text_file)?))
            } else {
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_detail_version_upgrade() {
        let detail = UpgradeDetail::version_upgrade(
            "test-package".to_string(),
            "1.0.0".to_string(),
            "1.1.0".to_string(),
        );

        assert_eq!(detail.name, "test-package");
        assert_eq!(detail.old_version, "1.0.0");
        assert_eq!(detail.new_version, "1.1.0");
        assert!(matches!(detail.upgrade_type, UpgradeType::VersionUpgrade));
        assert_eq!(detail.to_display_string(), "test-package: 1.0.0 → 1.1.0");
    }

    #[test]
    fn test_upgrade_detail_new_installation() {
        let detail =
            UpgradeDetail::new_installation("new-package".to_string(), "2.0.0".to_string());

        assert_eq!(detail.name, "new-package");
        assert_eq!(detail.old_version, "未安装");
        assert_eq!(detail.new_version, "2.0.0");
        assert!(matches!(detail.upgrade_type, UpgradeType::NewInstallation));
        assert_eq!(
            detail.to_display_string(),
            "new-package: new installation → 2.0.0"
        );
    }

    #[test]
    fn test_upgrade_details_collection() {
        let mut details = UpgradeDetails::new("test-tool".to_string());

        details.add_detail(UpgradeDetail::version_upgrade(
            "package1".to_string(),
            "1.0.0".to_string(),
            "1.1.0".to_string(),
        ));

        details.add_detail(UpgradeDetail::new_installation(
            "package2".to_string(),
            "2.0.0".to_string(),
        ));

        assert!(details.has_upgrades());
        assert_eq!(details.upgrade_count(), 2);
        assert_eq!(details.version_upgrade_count(), 1);
        assert_eq!(details.new_installation_count(), 1);
    }
}
