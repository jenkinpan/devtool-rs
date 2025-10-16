// devtool - 开发工具统一更新管理器
// 统一管理 Homebrew、Rustup、Mise 等开发工具的更新

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use clap_complete_nushell::Nushell;
use tempfile::tempdir;
use which::which;

// 模块声明
mod cli;
mod commands;
mod i18n;
mod parallel;
mod runner;
mod ui;
mod utils;

// 导入需要使用的项
use cli::{Args, Commands, ShellType};
use commands::{brew_cleanup, brew_update, brew_upgrade, mise_up, rustup_update};
use i18n::LocalizedStrings;
use parallel::{ParallelScheduler, TaskResult, Tool};
use runner::ShellRunner;
use ui::colors::{print_banner, print_error, print_info, print_success, print_warning};
use ui::progress::{progress_finish, progress_start, progress_status_cmd, Bar};

/// Execute tool updates in parallel
async fn execute_parallel_updates(
    tools: Vec<Tool>,
    jobs: usize,
    dry_run: bool,
    verbose: bool,
    keep_logs: bool,
    _localized: &LocalizedStrings,
) -> Result<Vec<TaskResult>> {
    let scheduler = ParallelScheduler::new(jobs);

    let update_fn = move |tool: Tool| {
        let tool_clone = tool.clone();
        tokio::spawn(
            async move { execute_tool_update(tool_clone, dry_run, verbose, keep_logs).await },
        )
    };

    scheduler.execute_parallel(tools, update_fn).await
}

/// Execute a single tool update
async fn execute_tool_update(
    tool: Tool,
    dry_run: bool,
    verbose: bool,
    _keep_logs: bool,
) -> Result<TaskResult> {
    let runner = ShellRunner;
    let run_tmp = std::env::temp_dir();

    let result = if dry_run {
        TaskResult {
            tool: tool.clone(),
            success: true,
            output: format!("{} (dry run)", tool.display_name()),
        }
    } else {
        match tool {
            Tool::Homebrew => {
                // Execute homebrew update sequence
                let update_result = brew_update(&runner, &run_tmp, verbose, &mut None)?;
                let upgrade_result = brew_upgrade(&runner, &run_tmp, verbose, &mut None)?;
                let cleanup_result = brew_cleanup(&runner, &run_tmp, verbose, &mut None)?;

                // Combine results - consider both "changed" and "unchanged" as success
                let success = (update_result.0 == "changed" || update_result.0 == "unchanged")
                    && (upgrade_result.0 == "changed" || upgrade_result.0 == "unchanged")
                    && (cleanup_result.0 == "changed" || cleanup_result.0 == "unchanged");
                let output = format!("Homebrew update completed");

                TaskResult {
                    tool,
                    success,
                    output,
                }
            }
            Tool::Rustup => {
                let result = rustup_update(&runner, &run_tmp, verbose, &mut None)?;
                TaskResult {
                    tool,
                    success: result.0 == "changed" || result.0 == "unchanged",
                    output: "Rustup update completed".to_string(),
                }
            }
            Tool::Mise => {
                let result = mise_up(&runner, &run_tmp, verbose, &mut None)?;
                TaskResult {
                    tool,
                    success: result.0 == "changed" || result.0 == "unchanged",
                    output: "Mise update completed".to_string(),
                }
            }
        }
    };

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 处理补全生成命令
    if let Some(Commands::Completion { shell }) = &args.command {
        let mut cmd = Args::command();
        match shell {
            ShellType::Bash => {
                clap_complete::generate(Shell::Bash, &mut cmd, "devtool", &mut std::io::stdout())
            }
            ShellType::Zsh => {
                clap_complete::generate(Shell::Zsh, &mut cmd, "devtool", &mut std::io::stdout())
            }
            ShellType::Fish => {
                clap_complete::generate(Shell::Fish, &mut cmd, "devtool", &mut std::io::stdout())
            }
            ShellType::Powershell => clap_complete::generate(
                Shell::PowerShell,
                &mut cmd,
                "devtool",
                &mut std::io::stdout(),
            ),
            ShellType::Elvish => {
                clap_complete::generate(Shell::Elvish, &mut cmd, "devtool", &mut std::io::stdout())
            }
            ShellType::Nushell => {
                clap_complete::generate(Nushell, &mut cmd, "devtool", &mut std::io::stdout())
            }
        }
        return Ok(());
    }

    // 处理 progress-status 子命令
    if let Some(Commands::ProgressStatus) = &args.command {
        return progress_status_cmd();
    }

    // 获取 update 命令的参数，如果没有指定命令则使用默认值
    let (dry_run, verbose, no_color, keep_logs, parallel, jobs, no_banner, _compact) =
        match &args.command {
            Some(Commands::Update {
                dry_run,
                verbose,
                no_color,
                keep_logs,
                parallel,
                jobs,
                no_banner,
                compact,
            }) => (
                *dry_run, *verbose, *no_color, *keep_logs, *parallel, *jobs, *no_banner, *compact,
            ),
            None => (false, false, false, false, false, 4, false, false), // 默认值
            _ => return Ok(()),
        };

    // 检测系统语言并初始化本地化
    let system_lang = i18n::detect_system_language();
    if verbose {
        println!("Debug: Detected language: {}", system_lang);
    }
    let localized = LocalizedStrings::new(&system_lang);

    // 初始化颜色支持
    if no_color {
        colored::control::set_override(false);
    } else if ui::colors::supports_color() {
        colored::control::set_override(true);
    }

    // 处理版本信息输出
    // 记录开始时间
    let start_time = chrono::Local::now();

    if !no_banner {
        if ui::colors::supports_color() && !no_color {
            print_banner(&format!(
                "{}{}",
                localized.banner,
                start_time.format("%Y-%m-%d %H:%M:%S")
            ));
        } else {
            println!(
                "{}{}",
                localized.banner,
                start_time.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    // 构建可用工具列表
    let mut available_tools: Vec<Tool> = Vec::new();
    let mut skipped: Vec<&str> = Vec::new();

    // 检查并添加 Homebrew
    if which("brew").is_ok() {
        available_tools.push(Tool::Homebrew);
    } else {
        skipped.push("Homebrew");
    }

    // 检查并添加 Rustup
    if which("rustup").is_ok() {
        available_tools.push(Tool::Rustup);
    } else {
        skipped.push("Rust (rustup)");
    }

    // 检查并添加 Mise
    if which("mise").is_ok() {
        available_tools.push(Tool::Mise);
    } else {
        skipped.push("Mise");
    }

    let total = available_tools.len();
    if total == 0 {
        let warning_msg = if system_lang == "zh" {
            format!("⚠️ 未检测到可执行步骤。跳过： {}", skipped.join(", "))
        } else {
            format!(
                "⚠️ No executable steps detected. Skipped: {}",
                skipped.join(", ")
            )
        };

        if ui::colors::supports_color() && !no_color {
            print_warning(&warning_msg);
        } else {
            println!("{}", warning_msg);
        }
        return Ok(());
    }

    // 创建临时目录用于日志
    let tmp = tempdir()?;
    let _run_tmp = tmp.path().to_path_buf();

    // 创建进度条
    let mut pb_opt = Some(Bar::new(total, "devtool"));

    // 打印工具列表
    let tools_msg = format!(
        "📋 {}",
        localized.steps_count.replace("{}", &total.to_string())
    );
    if ui::colors::supports_color() && !no_color {
        print_info(&tools_msg);
    } else {
        println!("{}", tools_msg);
    }
    for (i, tool) in available_tools.iter().enumerate() {
        println!("  {}) {}", i + 1, tool.display_name());
    }

    // 开始外部进度跟踪
    progress_start(total as u64, "devtool", &mut pb_opt);

    // 初始化进度条显示
    if let Some(pb) = pb_opt.as_mut() {
        pb.update_to(0, &localized.progress_preparing);
    }

    // 执行工具更新
    let mut results: Vec<TaskResult> = Vec::new();
    let short_updates: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    if parallel {
        // 并行执行
        if verbose {
            println!("🚀 并行执行模式 (最大并发数: {})", jobs);
        }
        results = execute_parallel_updates(
            available_tools,
            jobs,
            dry_run,
            verbose,
            keep_logs,
            &localized,
        )
        .await?;
    } else {
        // 顺序执行
        for (_idx, tool) in available_tools.iter().enumerate() {
            let result = if dry_run {
                TaskResult {
                    tool: tool.clone(),
                    success: true,
                    output: format!("{} (dry run)", tool.display_name()),
                }
            } else {
                match execute_tool_update(tool.clone(), dry_run, verbose, keep_logs).await {
                    Ok(result) => result,
                    Err(e) => {
                        if verbose {
                            eprintln!("Error executing {}: {}", tool.display_name(), e);
                        }
                        TaskResult {
                            tool: tool.clone(),
                            success: false,
                            output: format!("{} failed: {}", tool.display_name(), e),
                        }
                    },
                }
            };
            results.push(result);
        }
    }

    // 处理执行结果
    let mut succ: Vec<String> = Vec::new();
    let mut fail: Vec<String> = Vec::new();
    let mut updated: Vec<String> = Vec::new();
    let mut unchanged: Vec<String> = Vec::new();
    let actions: Vec<String> = Vec::new();

    for result in &results {
        if result.success {
            succ.push(result.tool.display_name().to_string());
            if result.output.contains("completed") {
                updated.push(result.tool.display_name().to_string());
            } else {
                unchanged.push(result.tool.display_name().to_string());
            }
        } else {
            fail.push(result.tool.display_name().to_string());
        }
    }

    // 完成进度跟踪
    if let Some(pb) = pb_opt.as_mut() {
        pb.update_to(total, &localized.progress_complete);
    }
    println!(); // 换行
    if !dry_run {
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

    let update_complete_msg = format!(
        "\n{}{} ({}: {})",
        localized.update_complete,
        end_time.format("%Y-%m-%d %H:%M:%S"),
        localized.time_taken,
        duration_str
    );

    if ui::colors::supports_color() && !no_color {
        print_success(&update_complete_msg);
        if !updated.is_empty() {
            let updated_msg = if system_lang == "zh" {
                format!("✅ 已更新：{}", updated.join(", "))
            } else {
                format!("✅ Updated: {}", updated.join(", "))
            };
            print_success(&updated_msg);
        } else {
            print_info(&localized.no_updates);
        }
        if !actions.is_empty() {
            let actions_msg = format!("{}{}", localized.actions_executed, actions.join(", "));
            print_info(&actions_msg);
        }
        if !unchanged.is_empty() {
            let unchanged_msg = format!("{}{}", localized.already_latest, unchanged.join(", "));
            print_warning(&unchanged_msg);
        }
    } else {
        println!("{}", update_complete_msg);
        if !updated.is_empty() {
            let updated_msg = if system_lang == "zh" {
                format!("✅ 已更新：{}", updated.join(", "))
            } else {
                format!("✅ Updated: {}", updated.join(", "))
            };
            println!("{}", updated_msg);
        } else {
            println!("{}", localized.no_updates);
        }
        if !actions.is_empty() {
            let actions_msg = format!("{}{}", localized.actions_executed, actions.join(", "));
            println!("{}", actions_msg);
        }
        if !unchanged.is_empty() {
            let unchanged_msg = format!("{}{}", localized.already_latest, unchanged.join(", "));
            println!("{}", unchanged_msg);
        }
    }

    // 打印详细更新信息
    if let Some(vals) = short_updates.get("Homebrew：升级软件包") {
        if !vals.is_empty() {
            if ui::colors::supports_color() && !no_color {
                print_info("📦 Homebrew 升级详情：");
            } else {
                println!("📦 Homebrew 升级详情：");
            }
            for detail in vals {
                println!("   {}", detail);
            }
        }
    }

    if let Some(vals) = short_updates.get("Rust：更新 stable 工具链") {
        if !vals.is_empty() {
            if ui::colors::supports_color() && !no_color {
                print_info("🦀 Rust 升级详情：");
            } else {
                println!("🦀 Rust 升级详情：");
            }
            for detail in vals {
                println!("   {}", detail);
            }
        }
    }

    if let Some(vals) = short_updates.get("Mise：更新托管工具") {
        if !vals.is_empty() {
            if ui::colors::supports_color() && !no_color {
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
        if ui::colors::supports_color() && !no_color {
            print_error(&format!("❌ 失败：{}", fail.join(", ")));
        } else {
            println!("❌ 失败：{}", fail.join(", "));
        }
        std::process::exit(1);
    }

    Ok(())
}
