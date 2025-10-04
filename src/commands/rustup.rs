// Rustup 相关命令实现
// 包含 rustup update 命令

use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::runner::{run_command, Runner};
use crate::ui::progress::Bar;

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

/// Rustup 更新 stable 工具链
///
/// 执行 `rustup update stable` 更新 Rust 的 stable 工具链
///
/// # 参数
/// * `runner` - 命令执行器
/// * `tmpdir` - 临时目录，用于存储日志
/// * `verbose` - 是否输出详细信息
/// * `pbar` - 可选的进度条
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
    pbar: &mut Option<Bar>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("rustup_update.log");

    // 获取更新前的版本信息
    let (_, version_before) =
        runner.run("rustc --version", &tmpdir.join("rustc_before.log"), false)?;
    let version_before = version_before.trim().to_string();

    // 执行更新
    let (rc, out) = runner.run("rustup update stable", &logfile, verbose)?;

    if rc != 0 {
        return Ok(("failed".to_string(), rc, logfile));
    }

    // 获取更新后的版本信息
    let (_, version_after) = run_command(
        "rustc --version",
        &tmpdir.join("rustc_after.log"),
        false,
        pbar,
    )
    .ok()
    .unwrap_or((1, String::new()));
    let version_after = version_after.trim().to_string();

    // 检查版本是否真的有变化
    let out_text = out.to_lowercase();
    let is_unchanged = out_text.contains("unchanged")
        || out_text.contains("up to date")
        || version_before == version_after;

    let state = if is_unchanged {
        "unchanged"
    } else {
        // 解析版本信息并保存升级详情
        if let (Some(before), Some(after)) = (
            extract_rust_version(&version_before),
            extract_rust_version(&version_after),
        ) {
            let details_file = tmpdir.join("rustup_upgrade_details.txt");
            if let Ok(mut file) = File::create(&details_file) {
                let _ = writeln!(file, "rustc: {} → {}", before, after);
            }
        }
        "changed"
    };

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
}
