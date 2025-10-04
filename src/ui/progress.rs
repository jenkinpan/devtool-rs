use crate::utils::get_cache_dir;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

/// 进度条结构体
pub struct Bar {
    last_done: usize,
    total: usize,
}

impl Bar {
    /// 创建新的进度条
    pub fn new(total: usize, _desc: &str) -> Self {
        // 隐藏光标 / Hide cursor
        print!("\x1b[?25l");
        io::stdout().flush().ok();

        Bar {
            last_done: 0,
            total,
        }
    }

    /// 更新进度条到指定进度
    pub fn update_to(&mut self, done: usize, current_step: &str) {
        self.last_done = done;

        // 显示自定义格式的进度条 / Display custom formatted progress bar
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
        let bar = if super::colors::supports_color() {
            format!("{}{}", filled_bar.green(), empty_bar)
        } else {
            format!("{}{}", filled_bar, empty_bar)
        };

        // 构建进度条字符串，确保长度一致以覆盖之前的内容
        let progress_line = if super::colors::supports_color() {
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
        // 显示光标 / Show cursor
        print!("\x1b[?25h");
        io::stdout().flush().ok();
    }
}

/// 进度状态结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressStatus {
    pub state: String,
    pub percent: Option<i32>,
    pub done: Option<u64>,
    pub total: Option<u64>,
    pub desc: Option<String>,
    pub ts: Option<String>,
}

/// 开始进度跟踪
pub fn progress_start(total: u64, desc: &str, _pbar: &mut Option<Bar>) {
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
}

/// 更新进度
pub fn progress_update(percent: i32, done: u64, total: u64, desc: &str, _pbar: &mut Option<Bar>) {
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
}

/// 完成进度跟踪
pub fn progress_finish() {
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

/// 查询进度状态命令
pub fn progress_status_cmd() -> anyhow::Result<()> {
    use anyhow::Context;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_creation() {
        let bar = Bar::new(100, "Test");
        assert_eq!(bar.total, 100);
        assert_eq!(bar.last_done, 0);
    }

    #[test]
    fn test_progress_status_serialization() {
        let ps = ProgressStatus {
            state: "test".to_string(),
            percent: Some(50),
            done: Some(5),
            total: Some(10),
            desc: Some("Testing".to_string()),
            ts: Some("2024-01-01T00:00:00+00:00".to_string()),
        };
        let json = serde_json::to_string(&ps).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("50"));
    }
}
