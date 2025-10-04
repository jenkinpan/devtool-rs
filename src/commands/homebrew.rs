// Homebrew 相关命令实现
// 包含 brew update, brew upgrade, brew cleanup

use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::runner::Runner;
use crate::ui::progress::Bar;

/// 分析 Homebrew 升级情况
///
/// 比较升级前后的版本信息，返回升级的软件包列表
fn analyze_brew_upgrades(versions_before: &str, versions_after: &str) -> Vec<String> {
    let mut before_map: HashMap<String, String> = HashMap::new();
    let mut after_map: HashMap<String, String> = HashMap::new();

    // 解析升级前的版本
    for line in versions_before.lines() {
        if let Some((name, version)) = parse_brew_version_line(line) {
            before_map.insert(name, version);
        }
    }

    // 解析升级后的版本
    for line in versions_after.lines() {
        if let Some((name, version)) = parse_brew_version_line(line) {
            after_map.insert(name, version);
        }
    }

    // 找出升级的软件包
    let mut upgrades = Vec::new();
    for (name, after_version) in &after_map {
        if let Some(before_version) = before_map.get(name) {
            if before_version != after_version {
                upgrades.push(format!("{}: {} → {}", name, before_version, after_version));
            }
        }
    }

    upgrades.sort();
    upgrades
}

/// 解析 Homebrew 版本行
///
/// 从类似 "nginx 1.21.0" 的行中提取包名和版本号
fn parse_brew_version_line(line: &str) -> Option<(String, String)> {
    let mut parts = line.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(name), Some(version)) => Some((name.to_string(), version.to_string())),
        _ => None,
    }
}

/// 解析 Homebrew 升级输出
///
/// 从 brew upgrade 的输出中提取升级信息
fn parse_brew_upgrade_output(output: &str) -> Vec<String> {
    let mut upgrades = Vec::new();
    let lines: Vec<&str> = output.lines().collect();

    for line in lines {
        let line = line.trim();
        // 匹配 "package old_version -> new_version" 格式
        if line.contains(" -> ") {
            if let Some(upgrade) = parse_upgrade_line(line) {
                upgrades.push(upgrade);
            }
        }
    }

    upgrades
}

/// 解析升级行
///
/// 从类似 "nginx 1.21.0 -> 1.22.0" 的行中提取升级信息
fn parse_upgrade_line(line: &str) -> Option<String> {
    if let Some(arrow_pos) = line.find(" -> ") {
        let before_arrow = &line[..arrow_pos];
        let after_arrow = &line[arrow_pos + 4..];

        let parts: Vec<&str> = before_arrow.split_whitespace().collect();
        if parts.len() >= 2 {
            let package_name = parts[0];
            let old_version = parts[1];
            let new_version = after_arrow.split_whitespace().next().unwrap_or("");

            if !new_version.is_empty() {
                return Some(format!(
                    "{}: {} → {}",
                    package_name, old_version, new_version
                ));
            }
        }
    }
    None
}

/// Homebrew 更新索引
///
/// 执行 `brew update` 更新 Homebrew 的软件包索引
///
/// # 返回值
/// 返回元组 (状态, 退出码, 日志文件路径)
/// - 状态: "changed", "unchanged", 或 "failed"
/// - 退出码: 命令的退出码
/// - 日志文件路径: 命令输出的日志文件
pub fn brew_update(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
    _pbar: &mut Option<Bar>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("brew_update.log");

    // 获取更新前的 git commit hash
    let (_, commit_before) = runner.run(
        "cd $(brew --repository) && git log -1 --format='%H' 2>/dev/null || echo 'unknown'",
        &logfile,
        verbose,
    )?;

    // 执行更新
    let (rc_update, out_update) = runner.run("brew update --quiet", &logfile, verbose)?;

    if rc_update != 0 {
        return Ok(("failed".to_string(), rc_update, logfile));
    }

    // 获取更新后的 git commit hash
    let (_, commit_after) = runner.run(
        "cd $(brew --repository) && git log -1 --format='%H' 2>/dev/null || echo 'unknown'",
        &logfile,
        verbose,
    )?;

    let state = if (commit_before.trim() == commit_after.trim()
        && commit_before.trim() != "unknown")
        || out_update.contains("Already up-to-date.")
    {
        "unchanged"
    } else {
        "changed"
    };

    Ok((state.to_string(), rc_update, logfile))
}

/// Homebrew 升级软件包
///
/// 执行 `brew upgrade` 升级所有过时的软件包
///
/// # 返回值
/// 返回元组 (状态, 退出码, 日志文件路径)
pub fn brew_upgrade(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
    _pbar: &mut Option<Bar>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("brew_upgrade.log");

    // 首先检查是否有过时的软件包
    let (rc_outdated, out_outdated) = runner.run("brew outdated", &logfile, verbose)?;
    if rc_outdated != 0 || out_outdated.trim().is_empty() {
        return Ok(("unchanged".to_string(), rc_outdated, logfile));
    }

    // 检查输出是否包含过时软件包的信息
    let has_outdated = !out_outdated.trim().is_empty()
        && !out_outdated.contains("No outdated packages")
        && !out_outdated.contains("No outdated formulae");

    if !has_outdated {
        return Ok(("unchanged".to_string(), 0, logfile));
    }

    // 记录升级前的版本信息
    let (_, versions_before) = runner.run("brew list --formula --versions", &logfile, verbose)?;

    // 执行升级
    let (rc_upgrade, out_upgrade) = runner.run("brew upgrade", &logfile, verbose)?;

    if rc_upgrade != 0 {
        return Ok(("failed".to_string(), rc_upgrade, logfile));
    }

    // 记录升级后的版本信息
    let (_, versions_after) = runner.run("brew list --formula --versions", &logfile, verbose)?;

    // 分析升级的软件包
    let upgrade_details = analyze_brew_upgrades(&versions_before, &versions_after);

    // 如果版本比较没有找到变化，但从输出中可以看到升级信息，则解析输出
    if upgrade_details.is_empty() && out_upgrade.contains("==> Upgrading") {
        let parsed_upgrades = parse_brew_upgrade_output(&out_upgrade);
        if !parsed_upgrades.is_empty() {
            let details_file = tmpdir.join("brew_upgrade_details.txt");
            if let Ok(mut file) = File::create(&details_file) {
                for detail in &parsed_upgrades {
                    let _ = writeln!(file, "{}", detail);
                }
            }
            return Ok(("changed".to_string(), rc_upgrade, logfile));
        }
    }

    // 将升级详情写入文件供主程序读取
    if !upgrade_details.is_empty() {
        let details_file = tmpdir.join("brew_upgrade_details.txt");
        if let Ok(mut file) = File::create(&details_file) {
            for detail in &upgrade_details {
                let _ = writeln!(file, "{}", detail);
            }
        }
    }

    let state = if upgrade_details.is_empty() {
        "unchanged"
    } else {
        "changed"
    };

    Ok((state.to_string(), rc_upgrade, logfile))
}

/// Homebrew 清理旧版本
///
/// 执行 `brew cleanup` 清理旧版本的软件包
///
/// # 返回值
/// 返回元组 (状态, 退出码, 日志文件路径)
pub fn brew_cleanup(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
    pbar: &mut Option<Bar>,
) -> Result<(String, i32, PathBuf)> {
    use crate::runner::run_command;

    let logfile = tmpdir.join("brew_cleanup.log");
    let (_rc, to_remove) = run_command("brew cleanup -n --prune=7", &logfile, verbose, pbar)?;
    let (rc, state) = if to_remove.trim().is_empty() {
        let (rc_real, _) = runner.run("brew cleanup -n --prune=7", &logfile, verbose)?;
        (rc_real, "unchanged")
    } else {
        let (rc2, _) = runner.run("brew cleanup --prune=7 --quiet", &logfile, verbose)?;
        (rc2, "changed")
    };
    Ok((state.to_string(), rc, logfile))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_brew_version_line() {
        let result = parse_brew_version_line("nginx 1.21.0");
        assert_eq!(result, Some(("nginx".to_string(), "1.21.0".to_string())));
    }

    #[test]
    fn test_parse_brew_version_line_invalid() {
        let result = parse_brew_version_line("nginx");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_upgrade_line() {
        let line = "nginx 1.21.0 -> 1.22.0";
        let result = parse_upgrade_line(line);
        assert_eq!(result, Some("nginx: 1.21.0 → 1.22.0".to_string()));
    }

    #[test]
    fn test_parse_upgrade_line_invalid() {
        let line = "nginx 1.21.0";
        let result = parse_upgrade_line(line);
        assert_eq!(result, None);
    }

    #[test]
    fn test_analyze_brew_upgrades() {
        let before = "nginx 1.21.0\nredis 6.2.0";
        let after = "nginx 1.22.0\nredis 6.2.0";
        let upgrades = analyze_brew_upgrades(before, after);
        assert_eq!(upgrades.len(), 1);
        assert!(upgrades[0].contains("nginx"));
        assert!(upgrades[0].contains("1.21.0"));
        assert!(upgrades[0].contains("1.22.0"));
    }

    #[test]
    fn test_analyze_brew_upgrades_no_changes() {
        let before = "nginx 1.21.0\nredis 6.2.0";
        let after = "nginx 1.21.0\nredis 6.2.0";
        let upgrades = analyze_brew_upgrades(before, after);
        assert_eq!(upgrades.len(), 0);
    }

    #[test]
    fn test_parse_brew_upgrade_output() {
        let output = "==> Upgrading nginx 1.21.0 -> 1.22.0\n==> Upgrading redis 6.2.0 -> 6.2.5";
        let upgrades = parse_brew_upgrade_output(output);
        assert_eq!(upgrades.len(), 2);
    }
}
