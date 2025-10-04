use clap::Parser;

/// devtool - 开发工具统一更新管理器
#[derive(Parser, Debug)]
#[command(name = "devtool")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 要执行的命令（默认为 update）
    #[arg(default_value_t = String::from("update"))]
    pub command: String,

    /// 模拟执行，不实际运行命令
    #[arg(short = 'n', long = "dry-run")]
    pub dry_run: bool,

    /// 详细输出模式
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// 禁用彩色输出
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// 保留日志文件到 ~/.cache/devtool/
    #[arg(long = "keep-logs")]
    pub keep_logs: bool,

    /// 并行执行更新步骤（尚未实现）
    #[arg(long = "parallel")]
    pub parallel: bool,

    /// 不显示启动横幅
    #[arg(long = "no-banner")]
    pub no_banner: bool,

    /// 使用紧凑输出格式（适用于非交互环境）
    #[arg(long = "compact")]
    pub compact: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_defaults() {
        let args = Args::parse_from(&["devtool"]);
        assert_eq!(args.command, "update");
        assert!(!args.dry_run);
        assert!(!args.verbose);
        assert!(!args.no_color);
    }

    #[test]
    fn test_args_dry_run() {
        let args = Args::parse_from(&["devtool", "--dry-run"]);
        assert!(args.dry_run);
    }

    #[test]
    fn test_args_verbose() {
        let args = Args::parse_from(&["devtool", "-v"]);
        assert!(args.verbose);
    }

    #[test]
    fn test_args_no_color() {
        let args = Args::parse_from(&["devtool", "--no-color"]);
        assert!(args.no_color);
    }
}
