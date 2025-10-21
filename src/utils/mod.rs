use std::fs;
use std::path::PathBuf;

/// 获取缓存目录路径
/// 返回 ~/.cache/devtool 或 /tmp/devtool（如果无法确定主目录）
pub fn get_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("devtool")
}

/// 确保缓存目录存在
/// 创建缓存目录及其子目录结构
pub fn ensure_cache_dir() -> Result<PathBuf, std::io::Error> {
    let cache_dir = get_cache_dir();
    fs::create_dir_all(&cache_dir)?;

    // 创建子目录结构
    let subdirs = ["homebrew", "rustup", "mise", "feedback"];
    for subdir in &subdirs {
        let subdir_path = cache_dir.join(subdir);
        fs::create_dir_all(&subdir_path)?;
    }

    Ok(cache_dir)
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
