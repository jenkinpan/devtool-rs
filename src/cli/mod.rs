use clap::{Parser, Subcommand, ValueEnum};

/// 支持的 Shell 类型
#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum ShellType {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
    /// PowerShell
    Powershell,
    /// Elvish shell
    Elvish,
    /// Nushell
    Nushell,
}

/// devtool - 开发工具统一更新管理器
#[derive(Parser, Debug)]
#[command(name = "devtool")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 更新开发工具（默认命令）
    Update {
        /// 模拟执行，不实际运行命令
        #[arg(short = 'n', long = "dry-run")]
        dry_run: bool,

        /// 详细输出模式
        #[arg(short = 'v', long = "verbose")]
        verbose: bool,

        /// 禁用彩色输出
        #[arg(long = "no-color")]
        no_color: bool,

        /// 保留日志文件到 ~/.cache/devtool/
        #[arg(long = "keep-logs")]
        keep_logs: bool,

        /// 并行执行更新步骤 (默认启用)
        #[arg(long = "parallel", default_value_t = true)]
        parallel: bool,

        /// 顺序执行更新步骤 (覆盖并行模式)
        #[arg(long = "sequential")]
        sequential: bool,

        /// 并行任务数量限制
        #[arg(long = "jobs", default_value_t = 3)]
        jobs: usize,

        /// 不显示启动横幅
        #[arg(long = "no-banner")]
        no_banner: bool,

        /// 使用紧凑输出格式（适用于非交互环境）
        #[arg(long = "compact")]
        compact: bool,
    },
    /// 生成 shell 补全脚本
    Completion {
        /// Shell 类型
        #[arg(value_enum)]
        shell: ShellType,
    },
    /// 显示进度状态
    ProgressStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_defaults() {
        let args = Args::parse_from(["devtool"]);
        assert!(args.command.is_none());
    }

    #[test]
    fn test_args_update() {
        let args = Args::parse_from(["devtool", "update"]);
        match args.command {
            Some(Commands::Update {
                dry_run,
                verbose,
                no_color,
                ..
            }) => {
                assert!(!dry_run);
                assert!(!verbose);
                assert!(!no_color);
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_args_update_dry_run() {
        let args = Args::parse_from(["devtool", "update", "--dry-run"]);
        match args.command {
            Some(Commands::Update { dry_run, .. }) => {
                assert!(dry_run);
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_args_completion() {
        let args = Args::parse_from(["devtool", "completion", "bash"]);
        match args.command {
            Some(Commands::Completion { shell }) => {
                assert_eq!(shell, ShellType::Bash);
            }
            _ => panic!("Expected Completion command"),
        }
    }

    #[test]
    fn test_args_completion_nushell() {
        let args = Args::parse_from(["devtool", "completion", "nushell"]);
        match args.command {
            Some(Commands::Completion { shell }) => {
                assert_eq!(shell, ShellType::Nushell);
            }
            _ => panic!("Expected Completion command with nushell"),
        }
    }
}
