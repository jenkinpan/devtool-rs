use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

/// 命令执行器 trait
pub trait Runner {
    fn run(&self, cmd: &str, logfile: &Path, verbose: bool) -> Result<(i32, String)>;
}

/// Shell 命令执行器实现
pub struct ShellRunner;

impl Runner for ShellRunner {
    fn run(&self, cmd: &str, logfile: &Path, verbose: bool) -> Result<(i32, String)> {
        run_command(cmd, logfile, verbose, &mut None)
    }
}

/// 执行 shell 命令
///
/// # 参数
/// * `cmd` - 要执行的命令
/// * `logfile` - 日志文件路径
/// * `verbose` - 是否输出详细信息
/// * `pbar` - 可选的进度条
///
/// # 返回
/// * `Ok((exit_code, output))` - 成功时返回退出码和输出
/// * `Err(error)` - 失败时返回错误
pub fn run_command(
    cmd: &str,
    logfile: &Path,
    verbose: bool,
    pbar: &mut Option<()>,
) -> Result<(i32, String)> {
    // 创建日志文件，使用 Arc<Mutex<..>> 在多线程间共享
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

    let shared_file = Arc::new(Mutex::new(file));
    let mut handles = Vec::new();

    // 判断当前是否有活动的进度条
    let has_bar = pbar.as_ref().is_some();

    // 为 stdout 创建读取线程
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
                        // 写入日志文件
                        if let Ok(mut fh) = f.lock() {
                            let _ = fh.write_all(&buf[..n]);
                            let _ = fh.flush();
                        }
                        // 当详细模式开启且没有进度条时，输出到终端
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

    // 为 stderr 创建读取线程
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

    // 等待进程退出，然后等待所有读取线程完成
    let status = child.wait()?;
    for h in handles {
        let _ = h.join();
    }

    let rc = status.code().unwrap_or(1);

    // 重新打开日志文件读取最后 40 行输出
    let mut short = String::new();
    if let Ok(mut f2) = File::open(logfile) {
        let mut s = String::new();
        f2.read_to_string(&mut s).ok();
        let lines: Vec<&str> = s.lines().rev().take(40).collect();
        short = lines.into_iter().rev().collect::<Vec<&str>>().join("\n");
    }

    Ok((rc, short))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_run_command_success() {
        let tmp = tempdir().unwrap();
        let logfile = tmp.path().join("test.log");

        let result = run_command("echo 'test'", &logfile, false, &mut None);
        assert!(result.is_ok());

        let (rc, output) = result.unwrap();
        assert_eq!(rc, 0);
        assert!(output.contains("test"));
    }

    #[test]
    fn test_run_command_failure() {
        let tmp = tempdir().unwrap();
        let logfile = tmp.path().join("test.log");

        let result = run_command("exit 1", &logfile, false, &mut None);
        assert!(result.is_ok());

        let (rc, _) = result.unwrap();
        assert_eq!(rc, 1);
    }

    #[test]
    fn test_shell_runner() {
        let runner = ShellRunner;
        let tmp = tempdir().unwrap();
        let logfile = tmp.path().join("test.log");

        let result = runner.run("echo 'runner test'", &logfile, false);
        assert!(result.is_ok());
    }
}
