// Mise 相关命令实现
// 包含 mise up 命令

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::commands::upgrade_details::{UpgradeDetail, UpgradeDetails, UpgradeDetailsManager};
use crate::runner::Runner;

/// Mise 工具版本信息
#[derive(Debug, Deserialize, Serialize)]
struct MiseToolVersion {
    name: String,
    version: String,
}

/// 获取并保存 Mise 工具版本信息
///
/// 获取所有已安装工具的版本信息并保存到临时文件
/// 包含错误处理和备用机制
fn get_mise_versions_json(runner: &dyn Runner, tmpdir: &Path) -> Result<Vec<MiseToolVersion>> {
    // 尝试主要方法：使用 mise ls --current
    match get_mise_versions_from_ls(runner, tmpdir) {
        Ok(versions) => {
            if !versions.is_empty() {
                return Ok(versions);
            }
        }
        Err(e) => {
            // 记录错误但不立即失败，尝试备用方法
            if let Ok(mut file) = File::create(tmpdir.join("mise_errors.log")) {
                let _ = writeln!(file, "mise ls --current method failed: {}", e);
            }
        }
    }

    // 备用方法：使用 mise ls
    match get_mise_versions_from_ls_simple(runner, tmpdir) {
        Ok(versions) => Ok(versions),
        Err(e) => {
            // 如果所有方法都失败，返回空列表而不是错误
            if let Ok(mut file) = File::create(tmpdir.join("mise_errors.log")) {
                let _ = writeln!(file, "All methods failed: {}", e);
            }
            Ok(Vec::new())
        }
    }
}

/// 使用 mise ls --current 获取工具版本信息
fn get_mise_versions_from_ls(runner: &dyn Runner, tmpdir: &Path) -> Result<Vec<MiseToolVersion>> {
    let (_, versions_output) = runner.run(
        "mise ls --current",
        &tmpdir.join("mise_versions.log"),
        false,
    )?;

    // 验证输出
    if versions_output.trim().is_empty() {
        return Err(anyhow::anyhow!("Empty output from mise ls --current"));
    }

    let versions = parse_mise_versions(&versions_output);
    let mut tool_versions = Vec::new();

    for (name, version) in versions {
        // 验证工具名称和版本
        if !name.is_empty() && !version.is_empty() && version.contains('.') {
            tool_versions.push(MiseToolVersion { name, version });
        }
    }

    // 保存到临时文件
    let json_file = tmpdir.join("mise_versions.json");
    if let Ok(mut file) = File::create(&json_file) {
        let _ = writeln!(file, "{}", serde_json::to_string_pretty(&tool_versions)?);
    }

    Ok(tool_versions)
}

/// 使用 mise ls 获取工具版本信息（备用方法）
fn get_mise_versions_from_ls_simple(
    runner: &dyn Runner,
    tmpdir: &Path,
) -> Result<Vec<MiseToolVersion>> {
    let (_, versions_output) =
        runner.run("mise ls", &tmpdir.join("mise_versions_simple.log"), false)?;

    // 验证输出
    if versions_output.trim().is_empty() {
        return Err(anyhow::anyhow!("Empty output from mise ls"));
    }

    let versions = parse_mise_versions_simple(&versions_output);
    let mut tool_versions = Vec::new();

    for (name, version) in versions {
        // 验证工具名称和版本
        if !name.is_empty() && !version.is_empty() && version.contains('.') {
            tool_versions.push(MiseToolVersion { name, version });
        }
    }

    // 保存到临时文件
    let json_file = tmpdir.join("mise_versions_simple.json");
    if let Ok(mut file) = File::create(&json_file) {
        let _ = writeln!(file, "{}", serde_json::to_string_pretty(&tool_versions)?);
    }

    Ok(tool_versions)
}

/// 解析 Mise 版本信息
///
/// 从 mise ls 的输出中解析工具名称和版本号
///
/// # 参数
/// * `output` - mise ls 命令的输出
///
/// # 返回值
/// 返回一个 HashMap，键为工具名称，值为版本号
fn parse_mise_versions(output: &str) -> HashMap<String, String> {
    let mut versions = HashMap::new();

    // 跳过 JSON 格式，只使用文本格式（更简单可靠）
    if output.trim().starts_with('{') {
        return versions;
    }

    // 解析文本格式
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty()
            || line.starts_with('{')
            || line.starts_with('}')
            || line.starts_with('"')
        {
            continue;
        }

        // 尝试解析 "tool@version" 格式
        if let Some((name, version)) = line.split_once('@') {
            let name = name.trim().to_string();
            let version = version
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            if !version.is_empty() {
                versions.insert(name, version);
            }
            continue;
        }

        // 尝试解析空格分隔的格式
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let version = parts[1].to_string();
            // 确保版本看起来像版本号（包含数字和点）
            if version.contains(|c: char| c.is_numeric()) {
                versions.insert(name, version);
            }
        }
    }

    versions
}

/// 解析 Mise 版本信息（简化版本）
///
/// 从 mise ls 的输出中解析工具名称和版本号（备用解析方法）
///
/// # 参数
/// * `output` - mise ls 命令的输出
///
/// # 返回值
/// 返回一个 HashMap，键为工具名称，值为版本号
fn parse_mise_versions_simple(output: &str) -> HashMap<String, String> {
    let mut versions = HashMap::new();

    // 解析文本格式
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('{') || line.starts_with('}') {
            continue;
        }

        // 尝试解析 "tool@version" 格式
        if let Some((name, version)) = line.split_once('@') {
            let name = name.trim().to_string();
            let version = version
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            if !version.is_empty() && version.contains('.') {
                versions.insert(name, version);
            }
            continue;
        }

        // 尝试解析空格分隔的格式
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let version = parts[1].to_string();
            // 确保版本看起来像版本号（包含数字和点）
            if version.contains(|c: char| c.is_numeric()) && version.contains('.') {
                versions.insert(name, version);
            }
        }
    }

    versions
}

/// Mise 更新托管工具
///
/// 执行 `mise up` 更新 Mise 管理的所有工具
///
/// # 参数
/// * `runner` - 命令执行器
/// * `tmpdir` - 临时目录，用于存储日志
/// * `verbose` - 是否输出详细信息
/// * `_pbar` - 可选的进度条（当前未使用）
///
/// # 返回值
/// 返回元组 (状态, 退出码, 日志文件路径)
/// - 状态: "changed", "unchanged", 或 "failed"
/// - 退出码: 命令的退出码
/// - 日志文件路径: 命令输出的日志文件
pub fn mise_up(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
    _pbar: &mut Option<()>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("mise_up.log");

    // 获取升级前的工具版本信息
    let versions_before = get_mise_versions_json(runner, tmpdir)?;

    // 执行更新
    let (rc, out) = runner.run("mise up", &logfile, verbose)?;

    // 检查输出中是否包含更新标记
    let outl = out.to_lowercase();
    let install_markers = ["install", "installed", "upgraded", "updated", "->", "→"];
    let has_updates = install_markers.iter().any(|k| outl.contains(k));

    let mut upgrade_details = Vec::new();

    if has_updates {
        // 只有在有实际更新时才检查更新后的版本
        let versions_after = get_mise_versions_json(runner, tmpdir)?;

        // 比较版本变化，生成升级详情
        for before_tool in &versions_before {
            if let Some(after_tool) = versions_after
                .iter()
                .find(|tool| tool.name == before_tool.name)
            {
                if before_tool.version != after_tool.version {
                    upgrade_details.push(UpgradeDetail::version_upgrade(
                        before_tool.name.clone(),
                        before_tool.version.clone(),
                        after_tool.version.clone(),
                    ));
                }
            }
        }

        // 检查是否有新安装的工具
        for after_tool in &versions_after {
            if !versions_before
                .iter()
                .any(|tool| tool.name == after_tool.name)
            {
                upgrade_details.push(UpgradeDetail::new_installation(
                    after_tool.name.clone(),
                    after_tool.version.clone(),
                ));
            }
        }
    }

    // 创建标准化的升级详情
    let mut details = UpgradeDetails::new("Mise".to_string());
    details.add_details(upgrade_details);

    // 保存升级详情到标准文件（只有在有升级时才保存）
    if details.has_upgrades() {
        let _ = UpgradeDetailsManager::save_upgrade_details(&details, tmpdir, "mise");
    }

    let state = if has_updates {
        if rc == 0 {
            "changed"
        } else {
            "failed"
        }
    } else {
        "unchanged"
    };

    Ok((state.to_string(), rc, logfile))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mise_versions_tool_at_version() {
        let output = "node@20.11.0\npython@3.11.5";
        let versions = parse_mise_versions(output);
        assert_eq!(versions.get("node"), Some(&"20.11.0".to_string()));
        assert_eq!(versions.get("python"), Some(&"3.11.5".to_string()));
    }

    #[test]
    fn test_parse_mise_versions_space_separated() {
        let output = "node    20.11.0  ~/.tool-versions\npython  3.11.5  ~/.tool-versions";
        let versions = parse_mise_versions(output);
        assert_eq!(versions.get("node"), Some(&"20.11.0".to_string()));
        assert_eq!(versions.get("python"), Some(&"3.11.5".to_string()));
    }

    #[test]
    fn test_parse_mise_versions_empty() {
        let output = "";
        let versions = parse_mise_versions(output);
        assert!(versions.is_empty());
    }

    #[test]
    fn test_parse_mise_versions_json_skipped() {
        let output = r#"{"node": "20.11.0"}"#;
        let versions = parse_mise_versions(output);
        assert!(versions.is_empty());
    }

    #[test]
    fn test_parse_mise_versions_mixed_format() {
        let output = "node@20.11.0\npython  3.11.5  ~/.tool-versions";
        let versions = parse_mise_versions(output);
        assert_eq!(versions.len(), 2);
        assert_eq!(versions.get("node"), Some(&"20.11.0".to_string()));
        assert_eq!(versions.get("python"), Some(&"3.11.5".to_string()));
    }

    #[test]
    fn test_parse_mise_versions_invalid_lines() {
        let output = "\n   \n{}\n\"\"\nnodejs 20.11.0";
        let versions = parse_mise_versions(output);
        // 空行、{}、"" 都会被跳过，只解析 "nodejs 20.11.0"
        assert!(versions.len() <= 1);
        if !versions.is_empty() {
            assert_eq!(versions.get("nodejs"), Some(&"20.11.0".to_string()));
        }
    }
}
