use crate::utils::get_cache_dir;
use serde::{Deserialize, Serialize};
use std::fs;

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
