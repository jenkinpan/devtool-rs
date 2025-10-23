// Rustup 相关命令实现
// 包含 rustup update 命令

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::commands::upgrade_details::{UpgradeDetail, UpgradeDetails, UpgradeDetailsManager};
use crate::runner::Runner;

/// Rustup 工具链版本信息
#[derive(Debug, Deserialize, Serialize)]
struct ToolchainVersion {
    name: String,
    version: String,
}

/// 获取并保存工具链版本信息
///
/// 获取所有已安装工具链的版本信息并保存到临时文件
/// 包含错误处理和备用机制
fn get_toolchain_versions_json(
    runner: &dyn Runner,
    tmpdir: &Path,
) -> Result<Vec<ToolchainVersion>> {
    // 尝试主要方法：使用 rustup show
    match get_toolchain_versions_from_show(runner, tmpdir) {
        Ok(versions) => {
            if !versions.is_empty() {
                return Ok(versions);
            }
        }
        Err(e) => {
            // 记录错误但不立即失败，尝试备用方法
            if let Ok(mut file) = File::create(tmpdir.join("rustup_errors.log")) {
                let _ = writeln!(file, "rustup show method failed: {}", e);
            }
        }
    }

    // 备用方法：使用 rustup toolchain list
    match get_toolchain_versions_from_list(runner, tmpdir) {
        Ok(versions) => Ok(versions),
        Err(e) => {
            // 如果所有方法都失败，返回空列表而不是错误
            if let Ok(mut file) = File::create(tmpdir.join("rustup_errors.log")) {
                let _ = writeln!(file, "All methods failed: {}", e);
            }
            Ok(Vec::new())
        }
    }
}

/// 使用 rustup show 获取工具链版本信息
fn get_toolchain_versions_from_show(
    runner: &dyn Runner,
    tmpdir: &Path,
) -> Result<Vec<ToolchainVersion>> {
    let mut versions = Vec::new();

    // 获取所有已安装的工具链
    let (_, toolchains_output) =
        runner.run("rustup show", &tmpdir.join("rustup_show.log"), false)?;

    // 验证输出
    if toolchains_output.trim().is_empty() {
        return Err(anyhow::anyhow!("Empty output from rustup show"));
    }

    // 解析工具链信息
    for line in toolchains_output.lines() {
        let line = line.trim();
        if line.contains("stable-") || line.contains("nightly-") || line.contains("beta-") {
            // 提取工具链名称和版本
            if let Some(toolchain) = line.split_whitespace().next() {
                // 验证工具链名称
                if !toolchain.is_empty() && toolchain.len() > 3 {
                    // 获取该工具链的 rustc 版本
                    let cmd = format!("rustup run {} rustc --version", toolchain);
                    match runner.run(&cmd, &tmpdir.join("toolchain_version.log"), false) {
                        Ok((_, version_output)) => {
                            if let Some(version) = extract_rust_version(&version_output) {
                                // 验证版本号
                                if !version.is_empty() && version.contains('.') {
                                    versions.push(ToolchainVersion {
                                        name: toolchain.to_string(),
                                        version,
                                    });
                                }
                            }
                        }
                        Err(e) => {
                            // 记录单个工具链的错误，但继续处理其他工具链
                            if let Ok(mut file) = File::create(tmpdir.join("rustup_errors.log")) {
                                let _ = writeln!(
                                    file,
                                    "Failed to get version for {}: {}",
                                    toolchain, e
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    // 保存到临时文件
    let json_file = tmpdir.join("toolchain_versions.json");
    if let Ok(mut file) = File::create(&json_file) {
        let _ = writeln!(file, "{}", serde_json::to_string_pretty(&versions)?);
    }

    Ok(versions)
}

/// 使用 rustup toolchain list 获取工具链版本信息（备用方法）
fn get_toolchain_versions_from_list(
    runner: &dyn Runner,
    tmpdir: &Path,
) -> Result<Vec<ToolchainVersion>> {
    let mut versions = Vec::new();

    // 获取工具链列表
    let (_, toolchains_output) = runner.run(
        "rustup toolchain list",
        &tmpdir.join("rustup_toolchain_list.log"),
        false,
    )?;

    // 验证输出
    if toolchains_output.trim().is_empty() {
        return Err(anyhow::anyhow!("Empty output from rustup toolchain list"));
    }

    // 解析工具链信息
    for line in toolchains_output.lines() {
        let line = line.trim();
        if line.contains("stable-") || line.contains("nightly-") || line.contains("beta-") {
            // 提取工具链名称（移除默认标记）
            let toolchain = line.split_whitespace().next().unwrap_or("").to_string();
            if !toolchain.is_empty() {
                // 获取该工具链的 rustc 版本
                let cmd = format!("rustup run {} rustc --version", toolchain);
                match runner.run(&cmd, &tmpdir.join("toolchain_version.log"), false) {
                    Ok((_, version_output)) => {
                        if let Some(version) = extract_rust_version(&version_output) {
                            // 验证版本号
                            if !version.is_empty() && version.contains('.') {
                                versions.push(ToolchainVersion {
                                    name: toolchain,
                                    version,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        // 记录单个工具链的错误，但继续处理其他工具链
                        if let Ok(mut file) = File::create(tmpdir.join("rustup_errors.log")) {
                            let _ =
                                writeln!(file, "Failed to get version for {}: {}", toolchain, e);
                        }
                    }
                }
            }
        }
    }

    // 保存到临时文件
    let json_file = tmpdir.join("toolchain_versions_list.json");
    if let Ok(mut file) = File::create(&json_file) {
        let _ = writeln!(file, "{}", serde_json::to_string_pretty(&versions)?);
    }

    Ok(versions)
}

/// 提取 Rust 版本号
///
/// 从 rustc 的版本输出中提取版本号
/// 例如: "rustc 1.70.0 (90c541806 2023-05-31)" -> "1.70.0"
fn extract_rust_version(version_output: &str) -> Option<String> {
    version_output
        .split_whitespace()
        .nth(1)
        .filter(|_| version_output.starts_with("rustc"))
        .map(|s| s.to_string())
}

/// 检测版本变化
///
/// 比较升级前后的工具链版本信息，检测是否有版本变化或新安装的工具链
///
/// # 参数
/// * `before` - 升级前的工具链版本列表
/// * `after` - 升级后的工具链版本列表
///
/// # 返回值
/// 返回 `true` 如果检测到版本变化或新安装的工具链，否则返回 `false`
fn detect_version_changes(before: &[ToolchainVersion], after: &[ToolchainVersion]) -> bool {
    // 检查现有工具链版本变化
    for before_tc in before {
        if let Some(after_tc) = after.iter().find(|tc| tc.name == before_tc.name) {
            if before_tc.version != after_tc.version {
                return true;
            }
        }
    }

    // 检查新安装的工具链
    for after_tc in after {
        if !before.iter().any(|tc| tc.name == after_tc.name) {
            return true;
        }
    }

    false
}

/// 检测输出文本中的更新指示
///
/// 分析 rustup update 命令的输出文本，检测是否包含更新指示关键词
///
/// # 参数
/// * `output` - rustup update 命令的输出文本
///
/// # 返回值
/// 返回 `true` 如果检测到更新指示，否则返回 `false`
fn detect_output_indicators(output: &str) -> bool {
    let out_text = output.to_lowercase();

    // 首先检查是否明确表示无更新
    if out_text.contains("unchanged") || out_text.contains("up to date") {
        return false;
    }

    let update_indicators = [
        "updated",
        "upgraded",
        "installed",
        "downloaded",
        "installing",
        "downloading",
    ];

    // 检查更新指示关键词
    if update_indicators
        .iter()
        .any(|indicator| out_text.contains(indicator))
    {
        return true;
    }

    // 检查箭头符号（但排除"up to date"中的"to"）
    if out_text.contains("->") || out_text.contains("→") {
        return true;
    }

    // 检查"from"和"to"的组合（版本变化指示）
    if out_text.contains("from") && out_text.contains("to") && !out_text.contains("up to date") {
        return true;
    }

    false
}

/// 综合判断升级状态
///
/// 结合版本比较和输出文本检测，提供可靠的升级状态判断
///
/// # 参数
/// * `versions_before` - 升级前版本信息
/// * `versions_after` - 升级后版本信息
/// * `output` - 命令输出文本
///
/// # 返回值
/// 返回 `true` 如果检测到升级，否则返回 `false`
fn determine_upgrade_status(
    versions_before: &[ToolchainVersion],
    versions_after: &[ToolchainVersion],
    output: &str,
) -> bool {
    let has_version_changes = detect_version_changes(versions_before, versions_after);
    let has_output_indicators = detect_output_indicators(output);

    // 版本变化优先，输出指示作为辅助验证
    has_version_changes || has_output_indicators
}

/// Rustup 更新所有工具链
///
/// 执行 `rustup update` 更新所有已安装的 Rust 工具链
///
/// 此函数负责执行 Rustup 的工具链更新命令，不涉及进度条管理。
/// 进度条管理在应用程序的编排层（main.rs）统一处理。
///
/// # 参数
/// * `runner` - 命令执行器
/// * `tmpdir` - 临时目录，用于存储日志和版本信息
/// * `verbose` - 是否输出详细信息
///
/// # 返回值
/// 返回元组 (状态, 退出码, 日志文件路径)
/// - 状态: "changed", "unchanged", 或 "failed"
/// - 退出码: 命令的退出码
/// - 日志文件路径: 命令输出的日志文件
pub fn rustup_update(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("rustup_update.log");

    // 获取更新前的工具链版本信息
    let versions_before = get_toolchain_versions_json(runner, tmpdir)?;

    // 执行更新 - 更新所有已安装的工具链
    let (rc, out) = runner.run("rustup update", &logfile, verbose)?;

    if rc != 0 {
        return Ok(("failed".to_string(), rc, logfile));
    }

    // 始终获取升级后的版本信息，不依赖输出文本检测
    let versions_after = get_toolchain_versions_json(runner, tmpdir)?;

    // 使用新的综合检测逻辑判断是否有升级
    let has_upgrade = determine_upgrade_status(&versions_before, &versions_after, &out);

    let mut upgrade_details = Vec::new();

    if has_upgrade {
        // 比较版本变化，生成升级详情
        for before_tc in &versions_before {
            if let Some(after_tc) = versions_after.iter().find(|tc| tc.name == before_tc.name) {
                if before_tc.version != after_tc.version {
                    upgrade_details.push(UpgradeDetail::version_upgrade(
                        before_tc.name.clone(),
                        before_tc.version.clone(),
                        after_tc.version.clone(),
                    ));
                }
            }
        }

        // 检查是否有新安装的工具链
        for after_tc in &versions_after {
            if !versions_before.iter().any(|tc| tc.name == after_tc.name) {
                upgrade_details.push(UpgradeDetail::new_installation(
                    after_tc.name.clone(),
                    after_tc.version.clone(),
                ));
            }
        }
    }

    // 创建标准化的升级详情
    let mut details = UpgradeDetails::new("Rustup".to_string());
    details.add_details(upgrade_details);

    // 保存升级详情到标准文件（只有在有升级时才保存）
    if details.has_upgrades() {
        let _ = UpgradeDetailsManager::save_upgrade_details(&details, tmpdir, "rustup");
    }

    let state = if has_upgrade { "changed" } else { "unchanged" };

    Ok((state.to_string(), rc, logfile))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rust_version() {
        let output = "rustc 1.70.0 (90c541806 2023-05-31)";
        let result = extract_rust_version(output);
        assert_eq!(result, Some("1.70.0".to_string()));
    }

    #[test]
    fn test_extract_rust_version_invalid() {
        let output = "1.70.0 (90c541806 2023-05-31)";
        let result = extract_rust_version(output);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_rust_version_with_beta() {
        let output = "rustc 1.71.0-beta.1 (a2b1646c 2023-06-03)";
        let result = extract_rust_version(output);
        assert_eq!(result, Some("1.71.0-beta.1".to_string()));
    }

    #[test]
    fn test_extract_rust_version_empty() {
        let output = "";
        let result = extract_rust_version(output);
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_version_changes_with_version_upgrade() {
        let before = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.70.0".to_string(),
        }];
        let after = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.71.0".to_string(),
        }];
        assert!(detect_version_changes(&before, &after));
    }

    #[test]
    fn test_detect_version_changes_with_new_installation() {
        let before = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.70.0".to_string(),
        }];
        let after = vec![
            ToolchainVersion {
                name: "stable".to_string(),
                version: "1.70.0".to_string(),
            },
            ToolchainVersion {
                name: "nightly".to_string(),
                version: "1.72.0".to_string(),
            },
        ];
        assert!(detect_version_changes(&before, &after));
    }

    #[test]
    fn test_detect_version_changes_no_changes() {
        let before = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.70.0".to_string(),
        }];
        let after = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.70.0".to_string(),
        }];
        assert!(!detect_version_changes(&before, &after));
    }

    #[test]
    fn test_detect_output_indicators_with_updates() {
        let output = "info: downloading component 'rustc' for 'stable-x86_64-apple-darwin'\ninfo: installing component 'rustc' for 'stable-x86_64-apple-darwin'";
        assert!(detect_output_indicators(output));
    }

    #[test]
    fn test_detect_output_indicators_with_arrows() {
        let output = "stable-x86_64-apple-darwin updated -> 1.71.0";
        assert!(detect_output_indicators(output));
    }

    #[test]
    fn test_detect_output_indicators_no_updates() {
        let output = "info: all toolchains are up to date";
        assert!(!detect_output_indicators(output));
    }

    #[test]
    fn test_determine_upgrade_status_with_version_changes() {
        let before = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.70.0".to_string(),
        }];
        let after = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.71.0".to_string(),
        }];
        let output = "info: all toolchains are up to date";
        assert!(determine_upgrade_status(&before, &after, output));
    }

    #[test]
    fn test_determine_upgrade_status_with_output_indicators() {
        let before = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.70.0".to_string(),
        }];
        let after = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.70.0".to_string(),
        }];
        let output = "info: downloading component 'rustc'";
        assert!(determine_upgrade_status(&before, &after, output));
    }

    #[test]
    fn test_determine_upgrade_status_no_changes() {
        let before = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.70.0".to_string(),
        }];
        let after = vec![ToolchainVersion {
            name: "stable".to_string(),
            version: "1.70.0".to_string(),
        }];
        let output = "info: all toolchains are up to date";
        assert!(!determine_upgrade_status(&before, &after, output));
    }
}
