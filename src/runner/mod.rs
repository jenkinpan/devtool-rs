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
        run_command(cmd, logfile, verbose)
    }
}

/// 执行 shell 命令
///
/// 此函数执行 shell 命令并捕获其输出到日志文件。
/// 输出到终端的行为由 `DEVTOOL_SUPPRESS_OUTPUT` 环境变量控制。
///
/// # 参数
/// * `cmd` - 要执行的命令字符串
/// * `logfile` - 日志文件路径
/// * `verbose` - 是否打印详细输出（当未设置输出抑制时）
///
/// # 环境变量
/// * `DEVTOOL_SUPPRESS_OUTPUT` - 设置为 "1" 或 "true" 时抑制终端输出
///
/// # 返回
/// * `Ok((exit_code, output))` - 成功时返回退出码和输出
/// * `Err(error)` - 失败时返回错误
pub fn run_command(cmd: &str, logfile: &Path, verbose: bool) -> Result<(i32, String)> {
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

    // 检查环境变量以确定是否应该抑制输出
    let suppress_output = std::env::var("DEVTOOL_SUPPRESS_OUTPUT")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);

    // 为 stdout 创建读取线程
    if let Some(stdout_rd) = child.stdout.take() {
        let f = Arc::clone(&shared_file);
        let verbose_flag = verbose;
        let suppress_flag = suppress_output;
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
                        // 当详细模式开启且未设置输出抑制时，输出到终端
                        if verbose_flag && !suppress_flag {
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
        let suppress_flag = suppress_output;
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
                        // 当详细模式开启且未设置输出抑制时，输出到终端
                        if verbose_flag && !suppress_flag {
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

/// 启用输出抑制
///
/// 设置环境变量以抑制命令输出到终端。
/// 这在显示进度条时特别有用，可以防止命令输出干扰进度条显示。
///
/// # 注意
/// 所有命令输出仍然会被写入日志文件，只是不会显示在终端上。
///
/// # 示例
/// ```no_run
/// use devtool::runner::enable_output_suppression;
///
/// enable_output_suppression();
/// // 执行命令...
/// // 命令输出不会显示在终端上
/// ```
pub fn enable_output_suppression() {
    std::env::set_var("DEVTOOL_SUPPRESS_OUTPUT", "1");
}

/// 禁用输出抑制
///
/// 移除输出抑制环境变量，允许命令输出显示到终端。
///
/// # 示例
/// ```no_run
/// use devtool::runner::{enable_output_suppression, disable_output_suppression};
///
/// enable_output_suppression();
/// // 执行命令...
/// disable_output_suppression();
/// // 后续命令的输出会正常显示
/// ```
pub fn disable_output_suppression() {
    std::env::remove_var("DEVTOOL_SUPPRESS_OUTPUT");
}

/// 检查输出抑制是否已启用
///
/// # 返回
/// 如果输出抑制已启用则返回 `true`，否则返回 `false`
///
/// # 示例
/// ```no_run
/// use devtool::runner::is_output_suppressed;
///
/// if is_output_suppressed() {
///     println!("输出当前被抑制");
/// }
/// ```
#[allow(dead_code)]
pub fn is_output_suppressed() -> bool {
    std::env::var("DEVTOOL_SUPPRESS_OUTPUT")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_run_command_success() {
        let tmp = tempdir().unwrap();
        let logfile = tmp.path().join("test.log");

        let result = run_command("echo 'test'", &logfile, false);
        assert!(result.is_ok());

        let (rc, output) = result.unwrap();
        assert_eq!(rc, 0);
        assert!(output.contains("test"));
    }

    #[test]
    fn test_run_command_failure() {
        let tmp = tempdir().unwrap();
        let logfile = tmp.path().join("test.log");

        let result = run_command("exit 1", &logfile, false);
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
