// devtool - 开发工具统一更新管理器
// 统一管理 Homebrew、Rustup、Mise 等开发工具的更新

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use clap_complete_nushell::Nushell;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;
use tempfile::tempdir;
use ui::progress::{ProgressBarManager, ProgressState};
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
use cli::{Args, Commands, FeedbackType, ShellType};
use commands::{brew_cleanup, brew_update, brew_upgrade, mise_up, rustup_update};
use i18n::LocalizedStrings;
use parallel::{ParallelScheduler, TaskResult, Tool};
use runner::ShellRunner;
use ui::colors::{print_banner, print_error, print_info, print_success, print_warning};
use ui::progress::progress_status_cmd;

/// Get detailed description of what a tool will do
fn get_tool_description(tool: &Tool) -> String {
    match tool {
        Tool::Homebrew => "Homebrew update & upgrade & cleanup".to_string(),
        Tool::Rustup => "Rustup all toolchains update".to_string(),
        Tool::Mise => "Mise tools update".to_string(),
    }
}

/// 读取升级详情文件
fn read_upgrade_details(tmpdir: &std::path::Path, tool: &Tool) -> Vec<String> {
    let details_file = match tool {
        Tool::Homebrew => tmpdir.join("brew_upgrade_details.txt"),
        Tool::Rustup => tmpdir.join("rustup_upgrade_details.txt"),
        Tool::Mise => tmpdir.join("mise_upgrade_details.txt"),
    };

    if let Ok(content) = std::fs::read_to_string(&details_file) {
        content.lines().map(|s| s.to_string()).collect()
    } else {
        Vec::new()
    }
}

/// Execute tool updates in parallel
async fn execute_parallel_updates(
    tools: Vec<Tool>,
    jobs: usize,
    dry_run: bool,
    verbose: bool,
    keep_logs: bool,
    tmpdir: std::path::PathBuf,
    _localized: &LocalizedStrings,
) -> Result<Vec<TaskResult>> {
    let scheduler = ParallelScheduler::new(jobs);

    // 创建进度条管理器
    let mut progress_manager = ProgressBarManager::new();
    progress_manager.create_progress_bars(&tools);
    let multi_progress = progress_manager.get_multi_progress();

    let update_fn = move |tool: Tool| {
        let tool_clone = tool.clone();
        let _multi_progress = multi_progress.clone();
        let tmpdir_path = tmpdir.clone();

        tokio::spawn(async move {
            // 创建临时进度条管理器来处理单个工具
            let mut temp_manager = ProgressBarManager::new();
            temp_manager.create_progress_bars(std::slice::from_ref(&tool_clone));

            // 更新状态为执行中
            temp_manager.update_state(&tool_clone, ProgressState::Executing);

            // 执行工具更新
            let result = execute_tool_update(
                tool_clone.clone(),
                dry_run,
                verbose,
                keep_logs,
                &tmpdir_path,
            )
            .await;

            // 根据结果更新状态
            match &result {
                Ok(task_result) => {
                    if task_result.success {
                        temp_manager.update_state(&tool_clone, ProgressState::Completed);
                    } else {
                        temp_manager.update_state(&tool_clone, ProgressState::Failed);
                    }
                }
                Err(_) => {
                    temp_manager.update_state(&tool_clone, ProgressState::Failed);
                }
            }

            // 延迟显示完成状态，确保用户能看到结果
            tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

            // 完成进度条
            temp_manager.finalize_all();

            result
        })
    };

    scheduler.execute_parallel(tools, update_fn).await
}

/// Execute a single tool update
async fn execute_tool_update(
    tool: Tool,
    dry_run: bool,
    verbose: bool,
    _keep_logs: bool,
    tmpdir: &std::path::Path,
) -> Result<TaskResult> {
    let runner = ShellRunner;

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
                let update_result = brew_update(&runner, tmpdir, verbose, &mut None)?;
                let upgrade_result = brew_upgrade(&runner, tmpdir, verbose, &mut None)?;
                let cleanup_result = brew_cleanup(&runner, tmpdir, verbose, &mut None)?;

                // Check if any step had changes
                let has_changes = update_result.0 == "changed"
                    || upgrade_result.0 == "changed"
                    || cleanup_result.0 == "changed";

                let success = (update_result.0 == "changed" || update_result.0 == "unchanged")
                    && (upgrade_result.0 == "changed" || upgrade_result.0 == "unchanged")
                    && (cleanup_result.0 == "changed" || cleanup_result.0 == "unchanged");

                let output = if has_changes {
                    "Homebrew updated".to_string()
                } else {
                    "Homebrew already latest".to_string()
                };

                TaskResult {
                    tool,
                    success,
                    output,
                }
            }
            Tool::Rustup => {
                let result = rustup_update(&runner, tmpdir, verbose, &mut None)?;
                let has_changes = result.0 == "changed";
                let output = if has_changes {
                    "Rustup updated".to_string()
                } else {
                    "Rustup already latest".to_string()
                };

                TaskResult {
                    tool,
                    success: result.0 == "changed" || result.0 == "unchanged",
                    output,
                }
            }
            Tool::Mise => {
                let result = mise_up(&runner, tmpdir, verbose, &mut None)?;
                let has_changes = result.0 == "changed";
                let output = if has_changes {
                    "Mise updated".to_string()
                } else {
                    "Mise already latest".to_string()
                };

                TaskResult {
                    tool,
                    success: result.0 == "changed" || result.0 == "unchanged",
                    output,
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

    // 处理 feedback 子命令
    if let Some(Commands::Feedback {
        feedback_type,
        message,
        verbose,
    }) = &args.command
    {
        return handle_feedback_command(feedback_type, message, *verbose);
    }

    // 获取 update 命令的参数，如果没有指定命令则使用默认值
    let (dry_run, verbose, no_color, keep_logs, parallel, sequential, jobs, no_banner, _compact) =
        match &args.command {
            Some(Commands::Update {
                dry_run,
                verbose,
                no_color,
                keep_logs,
                parallel,
                sequential,
                jobs,
                no_banner,
                compact,
            }) => (
                *dry_run,
                *verbose,
                *no_color,
                *keep_logs,
                *parallel,
                *sequential,
                *jobs,
                *no_banner,
                *compact,
            ),
            None => (false, false, false, false, true, false, 3, false, false), // 默认值：并行执行，3个任务
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

    // 不再使用自建进度条，完全使用 indicatif
    // let mut pb_opt = Some(Bar::new(total, "devtool"));

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
        let tool_description = get_tool_description(tool);
        println!("  {}) {}", i + 1, tool_description);
    }

    // 完全使用 indicatif 进度条，不再使用自建进度条

    // 执行工具更新
    let mut results: Vec<TaskResult> = Vec::new();
    let mut short_updates: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    // 确定执行模式：如果指定了 sequential，则顺序执行；否则并行执行
    let use_parallel = parallel && !sequential;

    if use_parallel {
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
            _run_tmp.clone(),
            &localized,
        )
        .await?;

        // 收集升级详情
        for result in &results {
            if result.success {
                // 检查是否有升级详情文件存在
                let details = read_upgrade_details(&_run_tmp, &result.tool);
                if !details.is_empty() {
                    let key = match result.tool {
                        Tool::Homebrew => "Homebrew：升级软件包".to_string(),
                        Tool::Rustup => "Rust：更新工具链".to_string(),
                        Tool::Mise => "Mise：更新托管工具".to_string(),
                    };
                    short_updates.insert(key, details);
                }
            }
        }
    } else {
        // 顺序执行 - 使用 indicatif 进度条
        if verbose {
            println!("🔄 顺序执行模式");
        }
        let multi_progress = Arc::new(MultiProgress::new());
        let mut progress_bars: Vec<(Tool, ProgressBar)> = Vec::new();

        // 为每个工具创建进度条
        for tool in &available_tools {
            let pb = multi_progress.add(ProgressBar::new(100));

            // 设置进度条样式 - 使用更简洁的样式减少显示冲突
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:20.cyan/blue}] {pos}% {msg}",
                    )
                    .unwrap()
                    .progress_chars("#>-"),
            );

            pb.set_message(format!("{} 准备中...", tool.display_name()));
            // 减少刷新频率，避免显示冲突
            pb.enable_steady_tick(std::time::Duration::from_millis(2000));
            progress_bars.push((tool.clone(), pb));
        }

        // 顺序执行每个工具
        for tool in available_tools.iter() {
            // 找到对应的进度条
            if let Some((_, pb)) = progress_bars.iter().find(|(t, _)| *t == *tool) {
                // 更新进度条状态
                pb.set_message(format!("{} 执行中...", tool.display_name()));
                pb.set_position(25);
            }

            let result = if dry_run {
                TaskResult {
                    tool: tool.clone(),
                    success: true,
                    output: format!("{} (dry run)", tool.display_name()),
                }
            } else {
                match execute_tool_update(tool.clone(), dry_run, verbose, keep_logs, &_run_tmp)
                    .await
                {
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
                    }
                }
            };

            // 更新进度条到完成状态
            if let Some((_, pb)) = progress_bars.iter().find(|(t, _)| *t == *tool) {
                pb.set_position(100);
                if result.success {
                    pb.set_message(format!("✅ {} 完成", tool.display_name()));
                } else {
                    pb.set_message(format!("❌ {} 失败", tool.display_name()));
                }
                // 添加延迟确保状态更新完成，避免显示冲突
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                pb.finish();
            }

            // 收集升级详情
            if result.success {
                let details = read_upgrade_details(&_run_tmp, tool);
                if !details.is_empty() {
                    let key = match tool {
                        Tool::Homebrew => "Homebrew：升级软件包".to_string(),
                        Tool::Rustup => "Rust：更新工具链".to_string(),
                        Tool::Mise => "Mise：更新托管工具".to_string(),
                    };
                    short_updates.insert(key, details);
                }
            }

            results.push(result);

            // 使用 indicatif 进度条，不需要更新旧进度条
            // if let Some(pb) = pb_opt.as_mut() {
            //     pb.update_to(idx + 1, &format!("{} 完成", tool.display_name()));
            // }
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

            // 检查是否有升级详情文件来判断是否有真正的升级
            let has_upgrade_details = !read_upgrade_details(&_run_tmp, &result.tool).is_empty();

            if result.output.contains("updated") && has_upgrade_details {
                updated.push(result.tool.display_name().to_string());
            } else if result.output.contains("updated") && !has_upgrade_details {
                // 有更新但没有升级详情，说明只是索引更新
                unchanged.push(result.tool.display_name().to_string());
            } else if result.output.contains("already latest") {
                unchanged.push(result.tool.display_name().to_string());
            } else {
                // Fallback for other successful cases
                unchanged.push(result.tool.display_name().to_string());
            }
        } else {
            fail.push(result.tool.display_name().to_string());
        }
    }

    // 使用 indicatif 进度条，不需要旧进度条
    println!(); // 换行
                // 使用 indicatif 进度条，不需要 progress_finish
                // if !dry_run {
                //     progress_finish();
                // }

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

    if let Some(vals) = short_updates.get("Rust：更新工具链") {
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

/// 处理反馈命令
fn handle_feedback_command(
    feedback_type: &Option<FeedbackType>,
    message: &Option<String>,
    verbose: bool,
) -> Result<()> {
    use std::io::{self, Write};
    use std::time::{SystemTime, UNIX_EPOCH};

    // 显示反馈收集界面
    if ui::colors::supports_color() {
        print_info("📝 devtool User Feedback Collection");
    } else {
        println!("📝 devtool User Feedback Collection");
    }

    // 收集系统信息
    let system_info = collect_system_info();

    // 获取反馈类型
    let feedback_type = match feedback_type {
        Some(ft) => ft.clone(),
        None => {
            println!("\nPlease select feedback type:");
            println!("1. Bug Report");
            println!("2. Feature Request");
            println!("3. User Experience Issue");
            println!("4. Performance Issue");
            println!("5. Documentation Issue");
            println!("6. Other");
            print!("Please enter your choice (1-6): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
                "1" => FeedbackType::Bug,
                "2" => FeedbackType::Feature,
                "3" => FeedbackType::Ux,
                "4" => FeedbackType::Performance,
                "5" => FeedbackType::Documentation,
                "6" => FeedbackType::Other,
                _ => FeedbackType::Other,
            }
        }
    };

    // 获取反馈内容
    let feedback_message = match message {
        Some(msg) => msg.clone(),
        None => {
            println!("\nPlease describe your feedback:");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    if feedback_message.is_empty() {
        println!("Feedback content cannot be empty!");
        return Ok(());
    }

    // 生成反馈报告
    let feedback_report =
        generate_feedback_report(&feedback_type, &feedback_message, &system_info, verbose);

    // 保存反馈到文件
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("devtool_feedback_{}.md", timestamp);
    let feedback_dir = dirs::home_dir().unwrap().join(".cache").join("devtool");
    std::fs::create_dir_all(&feedback_dir)?;

    let feedback_file = feedback_dir.join(&filename);
    std::fs::write(&feedback_file, &feedback_report)?;

    // 显示反馈信息
    if ui::colors::supports_color() {
        print_success(&format!(
            "✅ Feedback saved to: {}",
            feedback_file.display()
        ));
    } else {
        println!("✅ Feedback saved to: {}", feedback_file.display());
    }

    println!("\n📋 Feedback Summary:");
    println!("Type: {:?}", feedback_type);
    println!("Content: {}", feedback_message);

    if verbose {
        println!("\n🔧 System Information:");
        println!("{}", system_info);
    }

    println!("\n💡 You can also submit feedback through:");
    println!("- GitHub Issues: https://github.com/jenkinpan/devtool-rs/issues");
    println!("- GitHub Discussions: https://github.com/jenkinpan/devtool-rs/discussions");

    Ok(())
}

/// 收集系统信息
fn collect_system_info() -> String {
    let mut info = String::new();

    // 操作系统信息
    if let Ok(os) = std::env::var("OS") {
        info.push_str(&format!("操作系统: {}\n", os));
    } else if cfg!(target_os = "macos") {
        info.push_str("操作系统: macOS\n");
    } else if cfg!(target_os = "linux") {
        info.push_str("操作系统: Linux\n");
    } else if cfg!(target_os = "windows") {
        info.push_str("操作系统: Windows\n");
    }

    // devtool 版本
    info.push_str(&format!("devtool 版本: {}\n", env!("CARGO_PKG_VERSION")));

    // Rust 版本
    if let Ok(rustc_version) = std::process::Command::new("rustc")
        .arg("--version")
        .output()
    {
        if let Ok(version) = String::from_utf8(rustc_version.stdout) {
            info.push_str(&format!("Rust 版本: {}", version.trim()));
        }
    }

    info
}

/// 生成反馈报告
fn generate_feedback_report(
    feedback_type: &FeedbackType,
    message: &str,
    system_info: &str,
    _verbose: bool,
) -> String {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

    format!(
        "# devtool User Feedback Report

## Basic Information
- **Submission Time**: {}
- **Feedback Type**: {:?}
- **devtool Version**: {}

## Feedback Content
{}

## System Information
```
{}
```

## Feedback Processing
- [ ] Received
- [ ] Analyzed
- [ ] Processed
- [ ] Replied

## Notes
_This feedback was automatically generated by devtool's built-in feedback system_
",
        timestamp,
        feedback_type,
        env!("CARGO_PKG_VERSION"),
        message,
        system_info
    )
}
