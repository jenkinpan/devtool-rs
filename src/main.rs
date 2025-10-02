use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::tempdir;
use which::which;

struct Bar {
    last_done: usize,
    total: usize,
}

impl Bar {
    fn new(total: usize, _desc: &str) -> Self {
        Bar {
            last_done: 0,
            total,
        }
    }

    fn set_description(&mut self, _d: String) {
        // 不再需要 kdam 的 set_description
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
        let bar = "=".repeat(filled) + &" ".repeat(bar_width - filled);

        // 构建进度条字符串，确保长度一致以覆盖之前的内容
        let progress_line = format!(
            "[{}] {}/{} ({}%) | {}",
            bar, done, self.total, percent, current_step
        );

        // 使用回车符回到行首，然后输出新内容，用空格填充到足够长度
        print!("\r{:<100}", progress_line);
        io::stdout().flush().ok();
    }
}
fn progress_start(total: u64, desc: &str, pbar: &mut Option<Bar>) {
    // write a structured status file to cache dir
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("devtool");
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
    if let Some(pb) = pbar.as_mut() {
        pb.set_description(desc.to_string());
        // 不在这里调用 update_to，避免重复输出
    }
}

fn progress_update(percent: i32, done: u64, total: u64, desc: &str, pbar: &mut Option<Bar>) {
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("devtool");
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
    if let Some(pb) = pbar.as_mut() {
        let mut desc_short = desc.to_string();
        if desc_short.len() > 40 {
            desc_short.truncate(37);
            desc_short.push_str("...");
        }
        pb.set_description(desc_short.clone());
        // 不在这里更新进度条，避免重复输出，由主循环统一管理
    }
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
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("devtool");
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
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("devtool");
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

    let state = if commit_before.trim() == commit_after.trim() && commit_before.trim() != "unknown"
    {
        "unchanged"
    } else {
        match out_update.contains("Already up-to-date.") {
            true => "unchanged",
            false => "changed",
        }
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
    let (rc_before, out_before) = runner.run("brew outdated --quiet", &logfile, verbose)?;
    if out_before.trim().is_empty() {
        return Ok(("unchanged".to_string(), rc_before, logfile));
    }
    let (rc2, _out_upgrade) = runner.run("brew upgrade --quiet", &logfile, verbose)?;
    Ok(("changed".to_string(), rc2, logfile))
}

fn brew_cleanup(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
    pbar: &mut Option<Bar>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("brew_cleanup.log");
    let (_rc, to_remove) = run_command("brew cleanup -n --prune=7", &logfile, verbose, pbar)?;
    if to_remove.trim().is_empty() {
        let (rc_real, _) = runner.run("brew cleanup -n --prune=7", &logfile, verbose)?;
        Ok(("unchanged".to_string(), rc_real, logfile))
    } else {
        let (rc2, _) = runner.run("brew cleanup --prune=7 --quiet", &logfile, verbose)?;
        Ok(("changed".to_string(), rc2, logfile))
    }
}

fn rustup_update(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
    pbar: &mut Option<Bar>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("rustup_update.log");
    let (_rc_b, _out_b) = runner.run("rustc --version", &tmpdir.join("rustc_before.log"), false)?;

    let (rc, out) = runner.run("rustup update stable", &logfile, verbose)?;
    let (_rc_a, _out_a) = run_command(
        "rustc --version",
        &tmpdir.join("rustc_after.log"),
        false,
        pbar,
    )
    .ok()
    .unwrap_or((1, String::new()));
    let out_text = out.to_lowercase();
    if out_text.contains("unchanged") || out_text.contains("up to date") || rc != 0 {
        Ok(("unchanged".to_string(), rc, logfile))
    } else {
        Ok(("changed".to_string(), rc, logfile))
    }
}

fn mise_up(
    runner: &dyn Runner,
    tmpdir: &Path,
    verbose: bool,
    _pbar: &mut Option<Bar>,
) -> Result<(String, i32, PathBuf)> {
    let logfile = tmpdir.join("mise_up.log");
    let (rc, out) = runner.run("mise up", &logfile, verbose)?;
    let outl = out.to_lowercase();
    // Consider 'changed' only when we see explicit install/update markers or version patterns
    let install_markers = ["install", "installed", "upgraded", "updated", "->", "→"];
    let version_pat = Regex::new(r"[a-zA-Z0-9_+\-.]+@[0-9]+(?:\.[0-9]+)+").unwrap();
    let mut short_entries: HashMap<String, Vec<String>> = HashMap::new();
    // collect explicit name@version tokens
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
        // write concise short updates to a temp file for main to read
        let mut concise: Vec<String> = Vec::new();
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
        if !concise.is_empty() {
            let shortfile = tmpdir.join("mise_short_updates.txt");
            let f = File::create(&shortfile).ok();
            if let Some(mut fh) = f {
                let _ = writeln!(fh, "{}", concise.join(", "));
            }
        }
        if rc == 0 {
            Ok(("changed".to_string(), rc, logfile))
        } else {
            Ok(("failed".to_string(), rc, logfile))
        }
    } else {
        Ok(("unchanged".to_string(), rc, logfile))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    // support an administrative subcommand to read the progress status
    if args.command == "progress-status" {
        return progress_status_cmd();
    }

    // 记录开始时间
    let start_time = chrono::Local::now();

    if !args.no_banner {
        println!(
            "🚀 开始 devtool 更新：{}",
            start_time.format("%Y-%m-%d %H:%M:%S")
        );
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
        println!("⚠️ 未检测到可执行步骤。跳过： {}", skipped.join(", "));
        return Ok(());
    }

    let tmp = tempdir()?;
    let run_tmp = tmp.path().to_path_buf();

    // progress bar (simple single-line Bar)
    let mut pb_opt = Some(Bar::new(total, "devtool"));

    // Always print the numbered steps so the user sees what's going to run.
    println!("📋 将执行 {} 个步骤：", total);
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
    let mut short_updates: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

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
            updated.push(step.desc);
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
            let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
            let devcache = cache_dir.join("devtool");
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
                let val = s.trim().to_string();
                if !val.is_empty() {
                    short_updates.insert(step.desc.to_string(), vec![val.clone()]);
                }
            }
            // remove after reading
            let _ = fs::remove_file(&mise_short);
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
    let duration_str = if duration.num_seconds() < 60 {
        format!("{}秒", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        format!(
            "{}分{}秒",
            duration.num_minutes(),
            duration.num_seconds() % 60
        )
    } else {
        format!(
            "{}小时{}分{}秒",
            duration.num_hours(),
            duration.num_minutes() % 60,
            duration.num_seconds() % 60
        )
    };

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

    // Print concise mise short-updates if present
    if let Some(vals) = short_updates.get("Mise：更新托管工具") {
        if !vals.is_empty() {
            println!("🔎 Mise 简要更新：{}", vals.join(", "));
        }
    }

    if !fail.is_empty() {
        println!("❌ 失败：{}", fail.join(", "));
        std::process::exit(1);
    }

    Ok(())
}
