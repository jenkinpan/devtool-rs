// Homebrew 相关命令实现
// 包含 brew update, brew upgrade, brew cleanup

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::runner::Runner;

/// Homebrew 过时软件包信息
#[derive(Debug, Deserialize, Serialize)]
struct OutdatedPackage {
    name: String,
    installed_version: String,
    current_version: String,
}

/// Homebrew 过时软件包 JSON 输出
#[derive(Debug, Deserialize, Serialize)]
struct OutdatedPackages {
    formulae: Vec<OutdatedPackage>,
    casks: Vec<OutdatedPackage>,
}

/// 获取并保存过时软件包信息
///
/// 使用 `brew outdated --json` 获取过时软件包信息并保存到临时文件
fn get_outdated_packages(runner: &dyn Runner, tmpdir: &Path) -> Result<Vec<OutdatedPackage>> {
    let logfile = tmpdir.join("brew_outdated.log");
    let (_, out) = runner.run("brew outdated --json", &logfile, false)?;

    let outdated: OutdatedPackages = serde_json::from_str(&out)?;

    // 合并 formulae 和 casks
    let mut all_outdated = Vec::new();
    all_outdated.extend(outdated.formulae);
    all_outdated.extend(outdated.casks);

    // 保存到临时文件
    let json_file = tmpdir.join("outdated_packages.json");
    if let Ok(mut file) = File::create(&json_file) {
        let _ = writeln!(file, "{}", serde_json::to_string_pretty(&all_outdated)?);
    }

    Ok(all_outdated)
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
    _pbar: &mut Option<()>,
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
    _pbar: &mut Option<()>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("brew_upgrade.log");

    // 获取升级前的过时软件包信息
    let outdated_packages = get_outdated_packages(runner, tmpdir)?;
    if outdated_packages.is_empty() {
        return Ok(("unchanged".to_string(), 0, logfile));
    }

    // 执行升级
    let (rc_upgrade, _out_upgrade) = runner.run("brew upgrade", &logfile, verbose)?;

    if rc_upgrade != 0 {
        return Ok(("failed".to_string(), rc_upgrade, logfile));
    }

    // 检查升级后的过时软件包信息
    let updated_outdated = get_outdated_packages(runner, tmpdir)?;

    // 比较升级前后的过时软件包，生成升级详情
    let mut upgrade_details = Vec::new();

    for outdated in &outdated_packages {
        // 检查这个软件包是否还在过时列表中
        let still_outdated = updated_outdated.iter().any(|p| p.name == outdated.name);

        if !still_outdated {
            // 如果不再过时，说明已经升级了
            upgrade_details.push(format!(
                "{}: {} → {}",
                outdated.name, outdated.installed_version, outdated.current_version
            ));
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
    pbar: &mut Option<()>,
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
