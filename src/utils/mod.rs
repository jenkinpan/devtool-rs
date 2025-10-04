use std::path::PathBuf;

/// 获取缓存目录路径
/// 返回 ~/.cache/devtool 或 /tmp/devtool（如果无法确定主目录）
pub fn get_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("devtool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cache_dir() {
        let cache_dir = get_cache_dir();
        assert!(cache_dir.to_string_lossy().contains("devtool"));
    }
}
