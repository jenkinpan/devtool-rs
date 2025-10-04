use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::tempdir;
use which::which;

fn get_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("devtool")
}

// 颜色输出函数 - 只对关键信息使用颜色进行区分
fn print_success(msg: &str) {
    if supports_color() {
        println!("{}", msg.green().bold());
    } else {
        println!("{}", msg);
    }
}

fn print_info(msg: &str) {
    if supports_color() {
        println!("{}", msg.blue());
    } else {
        println!("{}", msg);
    }
}

fn print_warning(msg: &str) {
    if supports_color() {
        println!("{}", msg.yellow());
    } else {
        println!("{}", msg);
    }
}

fn print_error(msg: &str) {
    if supports_color() {
        println!("{}", msg.red().bold());
    } else {
        println!("{}", msg);
    }
}

fn print_banner(msg: &str) {
    if supports_color() {
        println!("{}", msg.magenta().bold());
    } else {
        println!("{}", msg);
    }
}

// 检查终端是否支持颜色
fn supports_color() -> bool {
    atty::is(atty::Stream::Stdout) && std::env::var("NO_COLOR").is_err()
}

struct Bar {
    last_done: usize,
    total: usize,
}

impl Bar {
    fn new(total: usize, _desc: &str) -> Self {
        // 隐藏光标
        print!("\x1b[?25l");
        io::stdout().flush().ok();

        Bar {
            last_done: 0,
            total,
        }
    }


    fn update_to(&mut self, done: usize, current_step: &str) {
        self.last_done = done;

        // 显示自定义格式的进度条
        let percent = if self.total > 0 {
            (done * 100) / self.total
        } else {
            0
        };
        let bar_width = 40;
        let filled = (done * bar_width) / self.total.max(1);

        // 根据进度选择颜色（暂时未使用，保留用于未来扩展）
        let _bar_color = if percent >= 100 {
            "=".green()
        } else if percent >= 50 {
            "=".yellow()
        } else {
            "=".blue()
        };

        let filled_bar = "=".repeat(filled);
        let empty_bar = " ".repeat(bar_width - filled);
        let bar = if supports_color() {
            format!("{}{}", filled_bar.green(), empty_bar)
        } else {
            format!("{}{}", filled_bar, empty_bar)
        };

        // 构建进度条字符串，确保长度一致以覆盖之前的内容
        let progress_line = if supports_color() {
            // 只对进度条本身使用颜色，数字和文字保持原色
            format!(
                "[{}] {}/{} ({}%) | {}",
                bar, done, self.total, percent, current_step
            )
        } else {
            format!(
                "[{}] {}/{} ({}%) | {}",
                bar, done, self.total, percent, current_step
            )
        };

        // 使用回车符回到行首，然后输出新内容，用空格填充到足够长度
        print!("\r{:<100}", progress_line);
        io::stdout().flush().ok();
    }
}

impl Drop for Bar {
    fn drop(&mut self) {
        // 显示光标
        print!("\x1b[?25h");
        io::stdout().flush().ok();
    }
}
fn progress_start(total: u64, desc: &str, _pbar: &mut Option<Bar>) {
    // write a structured status file to cache dir
    let cache_dir = get_cache_dir();
    let _ = fs::create_dir_all(&cache_dir);
    let status_file = cache_dir.join("progress.status");
    let ps = ProgressStatus {
        state: "start".to_string(),
        percent: Some(0),
        done: Some(0),
        total: Some(total),
        desc: Some(desc.to_string()),
        ts: Some(chrono::Local::now().to_rfc3339()),
    };
    let _ = fs::write(
        &status_file,
        serde_json::to_string(&ps).unwrap_or_else(|_| "{}".to_string()),
    );
    // 进度条描述由 update_to 方法统一管理
}

fn progress_update(percent: i32, done: u64, total: u64, desc: &str, _pbar: &mut Option<Bar>) {
    let cache_dir = get_cache_dir();
    let _ = fs::create_dir_all(&cache_dir);
    let status_file = cache_dir.join("progress.status");
    let ps = ProgressStatus {
        state: "update".to_string(),
        percent: Some(percent),
        done: Some(done),
        total: Some(total),
        desc: Some(desc.to_string()),
        ts: Some(chrono::Local::now().to_rfc3339()),
    };
    let _ = fs::write(
        &status_file,
        serde_json::to_string(&ps).unwrap_or_else(|_| "{}".to_string()),
    );
    // 进度条描述由 update_to 方法统一管理
}

#[derive(Serialize, Deserialize, Debug)]
struct ProgressStatus {
    state: String,
    percent: Option<i32>,
    done: Option<u64>,
    total: Option<u64>,
    desc: Option<String>,
    ts: Option<String>,
}

fn progress_finish() {
    let cache_dir = get_cache_dir();
    let _ = fs::create_dir_all(&cache_dir);
    let status_file = cache_dir.join("progress.status");
    let ps = ProgressStatus {
        state: "finish".to_string(),
        percent: None,
        done: None,
        total: None,
        desc: None,
        ts: Some(chrono::Local::now().to_rfc3339()),
    };
    let _ = fs::write(
        &status_file,
        serde_json::to_string(&ps).unwrap_or_else(|_| "{}".to_string()),
    );
    println!(); // 为下一行输出准备
}

fn progress_status_cmd() -> Result<()> {
    let cache_dir = get_cache_dir();
    let status_file = cache_dir.join("progress.status");
    if !status_file.exists() {
        println!("No progress.status file: {:?}", status_file);
        return Ok(());
    }
    let s = fs::read_to_string(&status_file).with_context(|| "read progress.status")?;
    match serde_json::from_str::<ProgressStatus>(&s) {
        Ok(ps) => {
            println!("Progress status: {:?}", ps);
        }
        Err(e) => {
            println!("Raw content: {}", s);
            eprintln!("failed to parse JSON: {}", e);
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "devtool")]
struct Args {
    #[arg(default_value_t = String::from("update"))]
    command: String,
    #[arg(short = 'n', long = "dry-run")]
    dry_run: bool,
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
    #[arg(long = "no-color")]
    no_color: bool,
    #[arg(long = "keep-logs")]
    keep_logs: bool,
    #[arg(long = "parallel")]
    parallel: bool,
    #[arg(long = "no-banner")]
    no_banner: bool,
    #[arg(long = "compact")]
    compact: bool,
}
type StepFn = fn(&dyn Runner, &Path, bool, &mut Option<Bar>) -> Result<(String, i32, PathBuf)>;

struct Step {
    desc: &'static str,
    fn_name: StepFn,
}

/// A trait for running commands, allowing for mocking in tests.
trait Runner {
    fn run(&self, cmd: &str, logfile: &Path, verbose: bool) -> Result<(i32, String)>;
}

/// The production implementation of the `Runner` trait.
struct ShellRunner;

impl Runner for ShellRunner {
    fn run(&self, cmd: &str, logfile: &Path, verbose: bool) -> Result<(i32, String)> {
        run_command(cmd, logfile, verbose, &mut None) // pbar is not available here
    }
}

fn run_command(
    cmd: &str,
    logfile: &Path,
    verbose: bool,
    pbar: &mut Option<Bar>,
) -> Result<(i32, String)> {
    // Create logfile; we'll share it across reader threads using Arc<Mutex<..>>
    let file = File::create(logfile).with_context(|| format!("create logfile {:?}", logfile))?;
    if verbose {
        writeln!(&file, "Running: {}", cmd)?;
    }

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawn command: {}", cmd))?;

    use std::sync::{Arc, Mutex};
    use std::thread;

    let shared_file = Arc::new(Mutex::new(file));
    let mut handles = Vec::new();
    // determine whether a progress bar is active now; capture the value for threads
    let has_bar = pbar.as_ref().is_some();

    // Helper to spawn a reader thread for a given stream
    if let Some(stdout_rd) = child.stdout.take() {
        let f = Arc::clone(&shared_file);
        let verbose_flag = verbose;
        let has_bar_flag = has_bar;
        let h = thread::spawn(move || {
            let mut rd = stdout_rd;
            let mut buf = [0u8; 4096];
            loop {
                match rd.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        // write to logfile
                        if let Ok(mut fh) = f.lock() {
                            let _ = fh.write_all(&buf[..n]);
                            let _ = fh.flush();
                        }
                        // optionally forward to terminal when verbose and no progress bar
                        if verbose_flag && !has_bar_flag {
                            let _ = io::stdout().write_all(&buf[..n]);
                            let _ = io::stdout().flush();
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        handles.push(h);
    }

    if let Some(stderr_rd) = child.stderr.take() {
        let f = Arc::clone(&shared_file);
        let verbose_flag = verbose;
        let has_bar_flag = has_bar;
        let h = thread::spawn(move || {
            let mut rd = stderr_rd;
            let mut buf = [0u8; 4096];
            loop {
                match rd.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Ok(mut fh) = f.lock() {
                            let _ = fh.write_all(&buf[..n]);
                            let _ = fh.flush();
                        }
                        if verbose_flag && !has_bar_flag {
                            let _ = io::stdout().write_all(&buf[..n]);
                            let _ = io::stdout().flush();
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        handles.push(h);
    }

    // Wait for process to exit, then join reader threads
    let status = child.wait()?;
    for h in handles {
        let _ = h.join();
    }

    let rc = status.code().unwrap_or(1);
    // read tail for short output by reopening the logfile
    let mut short = String::new();
    if let Ok(mut f2) = File::open(logfile) {
        let mut s = String::new();
        f2.read_to_string(&mut s).ok();
        let lines: Vec<&str> = s.lines().rev().take(40).collect();
        short = lines.into_iter().rev().collect::<Vec<&str>>().join("\n");
    }
    Ok((rc, short))
}

fn analyze_brew_upgrades(versions_before: &str, versions_after: &str) -> Vec<String> {
    // 解析版本信息到 HashMap
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

fn parse_brew_version_line(line: &str) -> Option<(String, String)> {
    let mut parts = line.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(name), Some(version)) => Some((name.to_string(), version.to_string())),
        _ => None,
    }
}

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

fn parse_upgrade_line(line: &str) -> Option<String> {
    // 匹配类似 "mise 2025.10.0 -> 2025.10.1" 的格式
    if let Some(arrow_pos) = line.find(" -> ") {
        let before_arrow = &line[..arrow_pos];
        let after_arrow = &line[arrow_pos + 4..];

        // 提取包名和版本
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

fn brew_update(
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

fn brew_upgrade(
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

    // 记录升级前的版本信息（使用更准确的方法）
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

fn brew_cleanup(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
    pbar: &mut Option<Bar>,
) -> Result<(String, i32, PathBuf)> {
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

fn rustup_update(
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

fn extract_rust_version(version_output: &str) -> Option<String> {
    // 从 "rustc 1.90.0 (1159e78c4 2025-09-14)" 提取 "1.90.0"
    version_output
        .split_whitespace()
        .nth(1)
        .filter(|_| version_output.starts_with("rustc"))
        .map(|s| s.to_string())
}

fn parse_mise_versions(output: &str) -> HashMap<String, String> {
    let mut versions = HashMap::new();

    // 跳过 JSON 格式，只使用文本格式（更简单可靠）
    if output.trim().starts_with('{') {
        // JSON 解析较复杂，当前使用文本格式命令
        return versions;
    }

    // 解析文本格式: "node    22.20.0  ~/.tool-versions 22.20.0"
    // 或: "node@22.20.0"
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

        // 尝试解析空格分隔的格式: "node    24.9.0  ~/.config/mise/config.toml  latest"
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

fn mise_up(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
    _pbar: &mut Option<Bar>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("mise_up.log");

    // 获取升级前的版本信息（使用文本格式，更容易解析）
    let (_, versions_before) =
        runner.run("mise ls --current", &tmpdir.join("mise_before.log"), false)?;

    // 执行更新
    let (rc, out) = runner.run("mise up", &logfile, verbose)?;
    let outl = out.to_lowercase();

    // 获取升级后的版本信息
    let (_, versions_after) =
        runner.run("mise ls --current", &tmpdir.join("mise_after.log"), false)?;

    // Consider 'changed' only when we see explicit install/update markers or version patterns
    let install_markers = ["install", "installed", "upgraded", "updated", "->", "→"];
    let version_pat = Regex::new(r"[a-zA-Z0-9_+\-.]+@[0-9]+(?:\.[0-9]+)+").unwrap();
    let mut short_entries: HashMap<String, Vec<String>> = HashMap::new();

    // collect explicit name@version tokens from output
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

    // If we detected versions or install markers, consider changed and write concise summary
    if install_markers.iter().any(|k| outl.contains(k)) || !short_entries.is_empty() {
        // Parse before and after versions for accurate comparison
        let before_versions = parse_mise_versions(&versions_before);
        let after_versions = parse_mise_versions(&versions_after);

        // write concise short updates to a temp file for main to read
        let mut concise: Vec<String> = Vec::new();

        // Compare versions to find upgrades
        for (name, after_ver) in &after_versions {
            if let Some(before_ver) = before_versions.get(name) {
                if before_ver != after_ver {
                    concise.push(format!("{}: {} → {}", name, before_ver, after_ver));
                }
            } else {
                // New installation
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
                    // 假设第一个是旧版本，最后一个是新版本
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
                // 每个工具一行，与 Homebrew 和 Rustup 保持一致
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

fn main() -> Result<()> {
    let args = Args::parse();

    // 初始化颜色支持
    if args.no_color {
        colored::control::set_override(false);
    } else if supports_color() {
        colored::control::set_override(true);
    }

    // support an administrative subcommand to read the progress status
    if args.command == "progress-status" {
        return progress_status_cmd();
    }

    // 记录开始时间
    let start_time = chrono::Local::now();

    if !args.no_banner {
        if supports_color() && !args.no_color {
            print_banner(&format!(
                "🚀 开始 devtool 更新：{}",
                start_time.format("%Y-%m-%d %H:%M:%S")
            ));
        } else {
            println!(
                "🚀 开始 devtool 更新：{}",
                start_time.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    let mut steps: Vec<Step> = Vec::new();
    let mut skipped: Vec<&str> = Vec::new();

    if which("brew").is_ok() {
        steps.push(Step {
            desc: "Homebrew：更新索引",
            fn_name: brew_update,
        });
        steps.push(Step {
            desc: "Homebrew：升级软件包",
            fn_name: brew_upgrade,
        });
        steps.push(Step {
            desc: "Action：清理旧版本",
            fn_name: brew_cleanup,
        });
    } else {
        skipped.push("Homebrew");
    }

    if which("rustup").is_ok() {
        steps.push(Step {
            desc: "Rust：更新 stable 工具链",
            fn_name: rustup_update,
        });
    } else {
        skipped.push("Rust (rustup)");
    }

    if which("mise").is_ok() {
        steps.push(Step {
            desc: "Mise：更新托管工具",
            fn_name: mise_up,
        });
    } else {
        skipped.push("Mise");
    }

    let total = steps.len();
    if total == 0 {
        if supports_color() && !args.no_color {
            print_warning(&format!(
                "⚠️ 未检测到可执行步骤。跳过： {}",
                skipped.join(", ")
            ));
        } else {
            println!("⚠️ 未检测到可执行步骤。跳过： {}", skipped.join(", "));
        }
        return Ok(());
    }

    let tmp = tempdir()?;
    let run_tmp = tmp.path().to_path_buf();

    // progress bar (simple single-line Bar)
    let mut pb_opt = Some(Bar::new(total, "devtool"));

    // Always print the numbered steps so the user sees what's going to run.
    if supports_color() && !args.no_color {
        print_info(&format!("📋 将执行 {} 个步骤：", total));
    } else {
        println!("📋 将执行 {} 个步骤：", total);
    }
    for (i, s) in steps.iter().enumerate() {
        println!("  {}) {}", i + 1, s.desc);
    }

    // Start external progress helper
    progress_start(total as u64, "devtool", &mut pb_opt);

    // 初始化进度条显示
    if let Some(pb) = pb_opt.as_mut() {
        pb.update_to(0, "准备开始");
    }

    let mut succ: Vec<&str> = Vec::new();
    let mut fail: Vec<&str> = Vec::new();
    let mut updated: Vec<&str> = Vec::new();
    let mut unchanged: Vec<String> = Vec::new();
    let mut actions: Vec<&str> = Vec::new();
    // collect short updates (step desc -> concise list)
    let mut short_updates: HashMap<String, Vec<String>> = HashMap::new();

    for (idx, step) in steps.iter().enumerate() {
        // progress display handled centrally by `progress_update`; do not draw here.

        let (state, _rc, logfile) = if args.dry_run {
            (
                "unchanged".to_string(),
                0,
                run_tmp.join(format!("step.{}.log", idx)),
            )
        } else {
            match (step.fn_name)(&ShellRunner, &run_tmp, args.verbose, &mut pb_opt) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("step failed: {}", e);
                    (
                        "failed".to_string(),
                        1,
                        run_tmp.join(format!("step.{}.log", idx)),
                    )
                }
            }
        };

        if state == "changed" {
            // 根据步骤描述分类：清理操作归类为 actions，其他归类为 updated
            if step.desc.contains("清理") || step.desc.starts_with("Action：") {
                actions.push(step.desc);
            } else {
                updated.push(step.desc);
            }
            succ.push(step.desc);
        } else if state == "unchanged" {
            // classify actions (contain '清理' or start with 'Action：') separately
            if step.desc.contains("清理") || step.desc.starts_with("Action：") {
                actions.push(step.desc);
            } else {
                // remove words like '更新'/'升级' from the displayed name
                let mut name = step.desc.to_string();
                name = name
                    .replace("更新", "")
                    .replace("升级", "")
                    .replace("  ", " ")
                    .trim()
                    .to_string();
                unchanged.push(name);
            }
            succ.push(step.desc);
        } else {
            fail.push(step.desc);
        }

        // Optionally keep logs
        if args.keep_logs {
            let devcache = get_cache_dir();
            fs::create_dir_all(&devcache)?;
            let dest = devcache.join(
                logfile
                    .file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new("log")),
            );
            fs::copy(&logfile, &dest).ok();
        } else {
            fs::remove_file(&logfile).ok();
        }

        // If the step wrote a mise short updates file into tmpdir, read and record it
        let mise_short = run_tmp.join("mise_short_updates.txt");
        if mise_short.exists() {
            if let Ok(s) = fs::read_to_string(&mise_short) {
                let details: Vec<String> = s.lines().map(|line| line.trim().to_string()).collect();
                if !details.is_empty() {
                    short_updates.insert(step.desc.to_string(), details);
                }
            }
            // remove after reading
            let _ = fs::remove_file(&mise_short);
        }

        // If the step wrote a brew upgrade details file into tmpdir, read and record it
        let brew_details = run_tmp.join("brew_upgrade_details.txt");
        if brew_details.exists() {
            if let Ok(s) = fs::read_to_string(&brew_details) {
                let details: Vec<String> = s.lines().map(|line| line.trim().to_string()).collect();
                if !details.is_empty() {
                    short_updates.insert(step.desc.to_string(), details);
                }
            }
            // remove after reading
            let _ = fs::remove_file(&brew_details);
        }

        // If the step wrote a rustup upgrade details file into tmpdir, read and record it
        let rustup_details = run_tmp.join("rustup_upgrade_details.txt");
        if rustup_details.exists() {
            if let Ok(s) = fs::read_to_string(&rustup_details) {
                let details: Vec<String> = s.lines().map(|line| line.trim().to_string()).collect();
                if !details.is_empty() {
                    short_updates.insert(step.desc.to_string(), details);
                }
            }
            // remove after reading
            let _ = fs::remove_file(&rustup_details);
        }

        // update external progress helper (this also updates the local bar)
        let done_count = (idx + 1) as u64;
        let percent = (100 * (idx + 1) / total) as i32;
        progress_update(percent, done_count, total as u64, step.desc, &mut pb_opt);

        // 直接更新进度条显示
        if let Some(pb) = pb_opt.as_mut() {
            pb.update_to(done_count as usize, step.desc);
        }
    }

    // finish progress helper
    // 显示最终的完成进度条
    if let Some(pb) = pb_opt.as_mut() {
        pb.update_to(total, "完成");
    }
    println!(); // 换行
    if !args.dry_run {
        progress_finish();
    }

    // 计算总耗时
    let end_time = chrono::Local::now();
    let duration = end_time.signed_duration_since(start_time);
    let duration_str = match (
        duration.num_hours(),
        duration.num_minutes(),
        duration.num_seconds(),
    ) {
        (h, _, _) if h > 0 => format!(
            "{}小时{}分{}秒",
            h,
            duration.num_minutes() % 60,
            duration.num_seconds() % 60
        ),
        (_, m, _) if m > 0 => format!("{}分{}秒", m, duration.num_seconds() % 60),
        (_, _, s) => format!("{}秒", s),
    };

    if supports_color() && !args.no_color {
        print_success(&format!(
            "\n🎉 更新完成：{} (耗时: {})",
            end_time.format("%Y-%m-%d %H:%M:%S"),
            duration_str
        ));
        if !updated.is_empty() {
            print_success(&format!("✅ 已更新：{}", updated.join(", ")));
        } else {
            print_info("ℹ️ 无更新应用。");
        }
        if !actions.is_empty() {
            print_info(&format!("🛠️ 已执行动作：{}", actions.join(", ")));
        }
        if !unchanged.is_empty() {
            print_warning(&format!("⚠️ 已是最新：{}", unchanged.join(", ")));
        }
    } else {
        println!(
            "\n🎉 更新完成：{} (耗时: {})",
            end_time.format("%Y-%m-%d %H:%M:%S"),
            duration_str
        );
        if !updated.is_empty() {
            println!("✅ 已更新：{}", updated.join(", "));
        } else {
            println!("ℹ️ 无更新应用。");
        }
        if !actions.is_empty() {
            println!("🛠️ 已执行动作：{}", actions.join(", "));
        }
        if !unchanged.is_empty() {
            println!("⚠️ 已是最新：{}", unchanged.join(", "));
        }
    }

    // Print Homebrew upgrade details if present
    if let Some(vals) = short_updates.get("Homebrew：升级软件包") {
        if !vals.is_empty() {
            if supports_color() && !args.no_color {
                print_info("📦 Homebrew 升级详情：");
            } else {
                println!("📦 Homebrew 升级详情：");
            }
            for detail in vals {
                println!("   {}", detail);
            }
        }
    }

    // Print Rustup upgrade details if present
    if let Some(vals) = short_updates.get("Rust：更新 stable 工具链") {
        if !vals.is_empty() {
            if supports_color() && !args.no_color {
                print_info("🦀 Rust 升级详情：");
            } else {
                println!("🦀 Rust 升级详情：");
            }
            for detail in vals {
                println!("   {}", detail);
            }
        }
    }

    // Print Mise upgrade details if present
    if let Some(vals) = short_updates.get("Mise：更新托管工具") {
        if !vals.is_empty() {
            if supports_color() && !args.no_color {
                print_info("🔧 Mise 升级详情：");
            } else {
                println!("🔧 Mise 升级详情：");
            }
            for detail in vals {
                println!("   {}", detail);
            }
        }
    }

    if !fail.is_empty() {
        if supports_color() && !args.no_color {
            print_error(&format!("❌ 失败：{}", fail.join(", ")));
        } else {
            println!("❌ 失败：{}", fail.join(", "));
        }
        std::process::exit(1);
    }

    Ok(())
}
