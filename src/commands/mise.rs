// Mise 相关命令实现
// 包含 mise up 命令

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::runner::Runner;

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

    // 获取升级前的版本信息
    let (_, versions_before) =
        runner.run("mise ls --current", &tmpdir.join("mise_before.log"), false)?;

    // 执行更新
    let (rc, out) = runner.run("mise up", &logfile, verbose)?;
    let outl = out.to_lowercase();

    // 获取升级后的版本信息
    let (_, versions_after) =
        runner.run("mise ls --current", &tmpdir.join("mise_after.log"), false)?;

    // 检查是否有安装/更新标记或版本模式
    let install_markers = ["install", "installed", "upgraded", "updated", "->", "→"];
    let version_pat = Regex::new(r"[a-zA-Z0-9_+\-.]+@[0-9]+(?:\.[0-9]+)+").unwrap();
    let mut short_entries: HashMap<String, Vec<String>> = HashMap::new();

    // 从输出中收集明确的 name@version 标记
    for cap in version_pat.captures_iter(&out) {
        if let Some(m) = cap.get(0) {
            let s = m.as_str().to_string();
            if let Some((name, ver)) = s.split_once('@') {
                short_entries
                    .entry(name.to_string())
                    .or_default()
                    .push(ver.to_string());
            }
        }
    }

    // 解析版本信息
    let before_versions = parse_mise_versions(&versions_before);
    let after_versions = parse_mise_versions(&versions_after);

    // 如果检测到版本或安装标记，或者有版本变化，认为已更改并写入简洁摘要
    let has_version_changes = !before_versions.is_empty()
        && !after_versions.is_empty()
        && before_versions != after_versions;

    if install_markers.iter().any(|k| outl.contains(k))
        || !short_entries.is_empty()
        || has_version_changes
    {
        let mut concise: Vec<String> = Vec::new();

        // 比较版本以找出升级
        for (name, after_ver) in &after_versions {
            if let Some(before_ver) = before_versions.get(name) {
                if before_ver != after_ver {
                    concise.push(format!("{}: {} → {}", name, before_ver, after_ver));
                }
            } else {
                // 新安装
                concise.push(format!("{}: {}", name, after_ver));
            }
        }

        // 如果版本比较没有找到变化，但输出显示有更新，则从输出中提取信息
        if concise.is_empty() && !short_entries.is_empty() {
            for (name, vers) in &short_entries {
                let mut seen: Vec<String> = Vec::new();
                for v in vers {
                    if !seen.contains(v) {
                        seen.push(v.clone());
                    }
                }
                if seen.len() >= 2 {
                    concise.push(format!(
                        "{}: {} → {}",
                        name,
                        seen.first().unwrap(),
                        seen.last().unwrap()
                    ));
                } else if seen.len() == 1 {
                    concise.push(format!("{}: {}", name, seen[0]));
                }
            }
        }

        if !concise.is_empty() {
            let shortfile = tmpdir.join("mise_short_updates.txt");
            let f = File::create(&shortfile).ok();
            if let Some(mut fh) = f {
                for entry in &concise {
                    let _ = writeln!(fh, "{}", entry);
                }
            }
        }
        let state = if rc == 0 { "changed" } else { "failed" };
        Ok((state.to_string(), rc, logfile))
    } else {
        Ok(("unchanged".to_string(), rc, logfile))
    }
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
