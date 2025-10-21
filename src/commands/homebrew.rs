// Homebrew 相关命令实现
// 包含 brew update, brew upgrade, brew cleanup

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::commands::upgrade_details::{UpgradeDetail, UpgradeDetails, UpgradeDetailsManager};
use crate::runner::Runner;

/// 创建调试日志文件的辅助函数
///
/// 统一调试日志文件的创建和写入，避免重复代码
fn write_debug_log(tmpdir: &Path, message: &str) {
    if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
        let _ = writeln!(debug_file, "{}", message);
    }
}

/// Homebrew 过时软件包信息
#[derive(Debug, Deserialize, Serialize)]
struct OutdatedPackage {
    name: String,
    installed_versions: Vec<String>,
    current_version: String,
    pinned: bool,
    pinned_version: Option<String>,
}

/// Homebrew 过时软件包 JSON 输出
#[derive(Debug, Deserialize, Serialize)]
struct OutdatedPackages {
    formulae: Vec<OutdatedPackage>,
    casks: Vec<OutdatedPackage>,
}

/// 简化的过时软件包信息（用于升级详情）
#[derive(Debug, Deserialize, Serialize)]
struct SimpleOutdatedPackage {
    name: String,
    installed_version: String,
    current_version: String,
}

/// 获取并保存过时软件包信息
///
/// 使用 `brew outdated --json` 获取过时软件包信息并保存到临时文件
/// 包含错误处理和备用机制
fn get_outdated_packages(runner: &dyn Runner, tmpdir: &Path) -> Result<Vec<SimpleOutdatedPackage>> {
    let logfile = tmpdir.join("brew_outdated.log");

    // 添加主函数调试信息
    write_debug_log(tmpdir, "=== get_outdated_packages 主函数开始 ===");
    write_debug_log(
        tmpdir,
        &format!("时间: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")),
    );
    write_debug_log(tmpdir, &format!("临时目录: {}", tmpdir.display()));

    // 尝试主要方法：brew outdated --json
    write_debug_log(tmpdir, "=== 尝试 JSON 方法 ===");

    match get_outdated_packages_json(runner, tmpdir, &logfile) {
        Ok(packages) => {
            if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
                let _ = writeln!(
                    debug_file,
                    "JSON 方法成功，发现 {} 个过时软件包",
                    packages.len()
                );
            }
            if !packages.is_empty() {
                if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
                    let _ = writeln!(debug_file, "返回 JSON 方法结果");
                }
                return Ok(packages);
            } else if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log"))
            {
                let _ = writeln!(debug_file, "JSON 方法返回空列表，尝试备用方法");
            }
        }
        Err(e) => {
            // 记录错误但不立即失败，尝试备用方法
            if let Ok(mut file) = File::create(tmpdir.join("brew_errors.log")) {
                let _ = writeln!(file, "JSON method failed: {}", e);
            }
            if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
                let _ = writeln!(debug_file, "JSON 方法失败: {}，尝试备用方法", e);
            }
        }
    }

    // 备用方法：使用文本格式解析
    if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
        let _ = writeln!(debug_file, "=== 尝试文本方法 ===");
    }

    match get_outdated_packages_text(runner, tmpdir, &logfile) {
        Ok(packages) => {
            if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
                let _ = writeln!(
                    debug_file,
                    "文本方法成功，发现 {} 个过时软件包",
                    packages.len()
                );
            }
            Ok(packages)
        }
        Err(e) => {
            // 如果所有方法都失败，返回空列表而不是错误
            if let Ok(mut file) = File::create(tmpdir.join("brew_errors.log")) {
                let _ = writeln!(file, "All methods failed: {}", e);
            }
            if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
                let _ = writeln!(debug_file, "所有方法都失败: {}，返回空列表", e);
            }
            Ok(Vec::new())
        }
    }
}

/// 使用 JSON 格式获取过时软件包信息
fn get_outdated_packages_json(
    runner: &dyn Runner,
    tmpdir: &Path,
    logfile: &Path,
) -> Result<Vec<SimpleOutdatedPackage>> {
    // 添加详细的调试信息
    if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
        let _ = writeln!(debug_file, "=== Homebrew 过时软件包检测调试信息 ===");
        let _ = writeln!(
            debug_file,
            "时间: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
        );
        let _ = writeln!(debug_file, "执行命令: brew outdated --json");
    }

    let (rc, out) = runner.run("brew outdated --json", logfile, false)?;

    // 记录命令执行结果
    if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
        let _ = writeln!(debug_file, "命令退出码: {}", rc);
        let _ = writeln!(debug_file, "输出长度: {} 字符", out.len());
        let _ = writeln!(debug_file, "原始输出:");
        let _ = writeln!(debug_file, "{}", out);
        let _ = writeln!(debug_file, "=== JSON 解析开始 ===");
    }

    // 检查输出是否为空
    if out.trim().is_empty() {
        if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
            let _ = writeln!(debug_file, "警告: brew outdated --json 输出为空");
        }
        return Ok(Vec::new());
    }

    // 尝试解析 JSON
    let outdated: OutdatedPackages = match serde_json::from_str::<OutdatedPackages>(&out) {
        Ok(parsed) => {
            if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
                let _ = writeln!(debug_file, "JSON 解析成功");
                let _ = writeln!(debug_file, "Formulae 数量: {}", parsed.formulae.len());
                let _ = writeln!(debug_file, "Casks 数量: {}", parsed.casks.len());
            }
            parsed
        }
        Err(e) => {
            if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
                let _ = writeln!(debug_file, "JSON 解析失败: {}", e);
                let _ = writeln!(debug_file, "原始输出内容:");
                let _ = writeln!(debug_file, "{}", out);
            }
            return Err(e.into());
        }
    };

    // 转换格式并合并 formulae 和 casks
    let mut all_outdated = Vec::new();

    // 处理 formulae
    if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
        let _ = writeln!(debug_file, "=== 处理 Formulae ===");
    }

    for (index, package) in outdated.formulae.iter().enumerate() {
        if let Some(installed_version) = package.installed_versions.first() {
            let simple_package = SimpleOutdatedPackage {
                name: package.name.clone(),
                installed_version: installed_version.clone(),
                current_version: package.current_version.clone(),
            };
            all_outdated.push(simple_package);

            if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
                let _ = writeln!(
                    debug_file,
                    "Formulae[{}]: {} {} -> {}",
                    index, package.name, installed_version, package.current_version
                );
            }
        } else if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
            let _ = writeln!(
                debug_file,
                "警告: Formulae[{}] {} 没有安装版本信息",
                index, package.name
            );
        }
    }

    // 处理 casks
    if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
        let _ = writeln!(debug_file, "=== 处理 Casks ===");
    }

    for (index, package) in outdated.casks.iter().enumerate() {
        if let Some(installed_version) = package.installed_versions.first() {
            let simple_package = SimpleOutdatedPackage {
                name: package.name.clone(),
                installed_version: installed_version.clone(),
                current_version: package.current_version.clone(),
            };
            all_outdated.push(simple_package);

            if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
                let _ = writeln!(
                    debug_file,
                    "Cask[{}]: {} {} -> {}",
                    index, package.name, installed_version, package.current_version
                );
            }
        } else if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
            let _ = writeln!(
                debug_file,
                "警告: Cask[{}] {} 没有安装版本信息",
                index, package.name
            );
        }
    }

    // 记录最终结果
    if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
        let _ = writeln!(debug_file, "=== 检测结果汇总 ===");
        let _ = writeln!(debug_file, "总共发现 {} 个过时软件包", all_outdated.len());
        if all_outdated.is_empty() {
            let _ = writeln!(debug_file, "所有软件包都是最新版本");
        } else {
            for (index, package) in all_outdated.iter().enumerate() {
                let _ = writeln!(
                    debug_file,
                    "  [{}] {}: {} -> {}",
                    index, package.name, package.installed_version, package.current_version
                );
            }
        }
    }

    // 保存到临时文件
    let json_file = tmpdir.join("outdated_packages.json");
    if let Ok(mut file) = File::create(&json_file) {
        let _ = writeln!(file, "{}", serde_json::to_string_pretty(&all_outdated)?);
    }

    Ok(all_outdated)
}

/// 使用文本格式获取过时软件包信息（备用方法）
fn get_outdated_packages_text(
    runner: &dyn Runner,
    _tmpdir: &Path,
    logfile: &Path,
) -> Result<Vec<SimpleOutdatedPackage>> {
    let (_, out) = runner.run("brew outdated", logfile, false)?;

    let mut packages = Vec::new();
    for line in out.lines() {
        if let Some((name, version_info)) = line.split_once(' ') {
            if let Some((installed, current)) = version_info.split_once(" -> ") {
                packages.push(SimpleOutdatedPackage {
                    name: name.to_string(),
                    installed_version: installed.to_string(),
                    current_version: current.to_string(),
                });
            }
        }
    }

    Ok(packages)
}

/// Homebrew 更新软件包索引
///
/// 执行 `brew update` 更新 Homebrew 的软件包索引
///
/// # 返回值
/// 返回元组 (状态, 退出码, 日志文件路径)
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

    // 执行更新 - 完全禁用 Homebrew 的进度条显示
    let (rc_update, out_update) = runner.run(
        "HOMEBREW_NO_PROGRESS=1 HOMEBREW_NO_ANALYTICS=1 HOMEBREW_NO_INSECURE_REDIRECT=1 brew update --quiet",
        &logfile,
        verbose,
    )?;

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

    // 添加调试信息
    if let Ok(mut file) = File::create(tmpdir.join("brew_upgrade_debug.log")) {
        let _ = writeln!(
            file,
            "Debug: 升级前过时软件包数量: {}",
            outdated_packages.len()
        );
        for pkg in &outdated_packages {
            let _ = writeln!(
                file,
                "  - {}: {} -> {}",
                pkg.name, pkg.installed_version, pkg.current_version
            );
        }
    }

    // 即使没有过时软件包，也执行 brew upgrade 命令
    // 因为 brew upgrade 可能会执行其他操作（如依赖检查、缓存清理等）
    if outdated_packages.is_empty() {
        if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
            let _ = writeln!(debug_file, "没有过时软件包，但仍执行 brew upgrade 命令");
        }
    }

    // 执行升级
    // 执行升级 - 完全禁用 Homebrew 的进度条显示
    let (rc_upgrade, _out_upgrade) = runner.run(
        "HOMEBREW_NO_PROGRESS=1 HOMEBREW_NO_ANALYTICS=1 HOMEBREW_NO_INSECURE_REDIRECT=1 brew upgrade --quiet",
        &logfile,
        verbose,
    )?;

    // 读取日志文件获取真正的命令输出
    let actual_output = std::fs::read_to_string(&logfile).unwrap_or_default();

    if rc_upgrade != 0 {
        return Ok(("failed".to_string(), rc_upgrade, logfile));
    }

    // 检查升级输出，如果没有实际升级，直接返回
    let has_actual_upgrades =
        actual_output.contains("==> Upgrading") || actual_output.contains("==> Installing");

    let mut upgrade_details = Vec::new();

    // 即使没有明显的升级输出，也要检查升级前后的状态变化
    if has_actual_upgrades || !outdated_packages.is_empty() {
        // 等待一下让 Homebrew 更新缓存
        std::thread::sleep(std::time::Duration::from_millis(1000));

        // 检查升级后的状态
        let updated_outdated = get_outdated_packages(runner, tmpdir)?;

        // 比较升级前后的过时软件包，生成升级详情
        for outdated in &outdated_packages {
            // 检查这个软件包是否还在过时列表中
            let still_outdated = updated_outdated.iter().any(|p| p.name == outdated.name);

            if !still_outdated {
                // 如果不再过时，说明已经升级了
                upgrade_details.push(UpgradeDetail::version_upgrade(
                    outdated.name.clone(),
                    outdated.installed_version.clone(),
                    outdated.current_version.clone(),
                ));
            }
        }
    }

    // 创建标准化的升级详情
    let mut details = UpgradeDetails::new("Homebrew".to_string());
    details.add_details(upgrade_details);

    // 保存升级详情到标准文件（只有在有升级时才保存）
    if details.has_upgrades() {
        let _ = UpgradeDetailsManager::save_upgrade_details(&details, tmpdir, "brew");
    }

    // 改进状态判断逻辑
    let state = if details.has_upgrades() {
        "changed"
    } else if has_actual_upgrades {
        // 如果有升级输出但没有检测到详情，仍然认为有变化
        "changed"
    } else if !outdated_packages.is_empty() {
        // 如果之前有过时软件包，即使没有检测到升级详情，也可能有变化
        "changed"
    } else {
        // 即使没有过时软件包，也要根据 brew upgrade 的实际输出来判断
        // 如果输出包含 "All formulae and casks are up to date"，说明检查完成
        if actual_output.contains("All formulae and casks are up to date")
            || actual_output.contains("Already up-to-date")
        {
            "unchanged"
        } else {
            // 如果有其他输出，可能执行了其他操作（如依赖检查等）
            "unchanged"
        }
    };

    // 添加状态判断的调试信息
    if let Ok(mut debug_file) = File::create(tmpdir.join("brew_detailed_debug.log")) {
        let _ = writeln!(debug_file, "=== 状态判断调试 ===");
        let _ = writeln!(
            debug_file,
            "details.has_upgrades(): {}",
            details.has_upgrades()
        );
        let _ = writeln!(debug_file, "has_actual_upgrades: {}", has_actual_upgrades);
        let _ = writeln!(
            debug_file,
            "outdated_packages.len(): {}",
            outdated_packages.len()
        );
        let _ = writeln!(
            debug_file,
            "actual_output 长度: {} 字符",
            actual_output.len()
        );
        let _ = writeln!(debug_file, "actual_output 内容: '{}'", actual_output);
        let _ = writeln!(
            debug_file,
            "actual_output 包含 'All formulae and casks are up to date': {}",
            actual_output.contains("All formulae and casks are up to date")
        );
        let _ = writeln!(
            debug_file,
            "actual_output 包含 'Already up-to-date': {}",
            actual_output.contains("Already up-to-date")
        );
        let _ = writeln!(
            debug_file,
            "actual_output 包含 'up to date': {}",
            actual_output.contains("up to date")
        );
        let _ = writeln!(debug_file, "最终状态: {}", state);
    }

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
    _pbar: &mut Option<()>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("brew_cleanup.log");

    // 执行清理
    // 执行清理 - 完全禁用 Homebrew 的进度条显示
    let (rc_cleanup, out_cleanup) = runner.run(
        "HOMEBREW_NO_PROGRESS=1 HOMEBREW_NO_ANALYTICS=1 HOMEBREW_NO_INSECURE_REDIRECT=1 brew cleanup --quiet",
        &logfile,
        verbose,
    )?;

    if rc_cleanup != 0 {
        return Ok(("failed".to_string(), rc_cleanup, logfile));
    }

    // 检查是否有清理内容
    let state = if out_cleanup.contains("Nothing to clean up") {
        "unchanged"
    } else {
        "changed"
    };

    Ok((state.to_string(), rc_cleanup, logfile))
}
