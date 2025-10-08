// devtool - 开发工具统一更新管理器
// 统一管理 Homebrew、Rustup、Mise 等开发工具的更新

use anyhow::Result;
use clap::{Parser, CommandFactory};
use clap_complete::Shell;
use clap_complete_nushell::Nushell;
use std::fs;
use tempfile::tempdir;
use which::which;

// 模块声明
mod cli;
mod commands;
mod i18n;
mod runner;
mod ui;
mod utils;

// 导入需要使用的项
use cli::{Args, Commands, ShellType};
use commands::{brew_cleanup, brew_update, brew_upgrade, mise_up, rustup_update, Step, StepFn};
use i18n::LocalizedStrings;
use runner::ShellRunner;
use ui::colors::{print_banner, print_error, print_info, print_success, print_warning};
use ui::progress::{progress_finish, progress_start, progress_status_cmd, progress_update, Bar};
use utils::get_cache_dir;

fn main() -> Result<()> {
    let args = Args::parse();

    // 处理补全生成命令
    if let Some(Commands::Completion { shell }) = &args.command {
        let mut cmd = Args::command();
        match shell {
            ShellType::Bash => clap_complete::generate(Shell::Bash, &mut cmd, "devtool", &mut std::io::stdout()),
            ShellType::Zsh => clap_complete::generate(Shell::Zsh, &mut cmd, "devtool", &mut std::io::stdout()),
            ShellType::Fish => clap_complete::generate(Shell::Fish, &mut cmd, "devtool", &mut std::io::stdout()),
            ShellType::Powershell => clap_complete::generate(Shell::PowerShell, &mut cmd, "devtool", &mut std::io::stdout()),
            ShellType::Elvish => clap_complete::generate(Shell::Elvish, &mut cmd, "devtool", &mut std::io::stdout()),
            ShellType::Nushell => clap_complete::generate(Nushell, &mut cmd, "devtool", &mut std::io::stdout()),
        }
        return Ok(());
    }

    // 处理 progress-status 子命令
    if let Some(Commands::ProgressStatus) = &args.command {
        return progress_status_cmd();
    }

    // 获取 update 命令的参数，如果没有指定命令则使用默认值
    let (dry_run, verbose, no_color, keep_logs, _parallel, no_banner, _compact) = match &args.command {
        Some(Commands::Update {
            dry_run,
            verbose,
            no_color,
            keep_logs,
            parallel,
            no_banner,
            compact,
        }) => (*dry_run, *verbose, *no_color, *keep_logs, *parallel, *no_banner, *compact),
        None => (false, false, false, false, false, false, false), // 默认值
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

    // 构建步骤列表
    let mut steps: Vec<Step> = Vec::new();
    let mut skipped: Vec<&str> = Vec::new();

    // 检查并添加 Homebrew 相关步骤
    if which("brew").is_ok() {
        steps.push(Step {
            desc: localized.step_homebrew_update.clone(),
            fn_name: brew_update as StepFn,
        });
        steps.push(Step {
            desc: localized.step_homebrew_upgrade.clone(),
            fn_name: brew_upgrade as StepFn,
        });
        steps.push(Step {
            desc: localized.step_cleanup.clone(),
            fn_name: brew_cleanup as StepFn,
        });
    } else {
        skipped.push("Homebrew");
    }

    // 检查并添加 Rustup 相关步骤
    if which("rustup").is_ok() {
        steps.push(Step {
            desc: localized.step_rust_update.clone(),
            fn_name: rustup_update as StepFn,
        });
    } else {
        skipped.push("Rust (rustup)");
    }

    // 检查并添加 Mise 相关步骤
    if which("mise").is_ok() {
        steps.push(Step {
            desc: localized.step_mise_update.clone(),
            fn_name: mise_up as StepFn,
        });
    } else {
        skipped.push("Mise");
    }

    let total = steps.len();
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
    let run_tmp = tmp.path().to_path_buf();

    // 创建进度条
    let mut pb_opt = Some(Bar::new(total, "devtool"));

    // 打印步骤列表
    let steps_msg = format!(
        "📋 {}",
        localized.steps_count.replace("{}", &total.to_string())
    );
    if ui::colors::supports_color() && !no_color {
        print_info(&steps_msg);
    } else {
        println!("{}", steps_msg);
    }
    for (i, s) in steps.iter().enumerate() {
        println!("  {}) {}", i + 1, s.desc);
    }

    // 开始外部进度跟踪
    progress_start(total as u64, "devtool", &mut pb_opt);

    // 初始化进度条显示
    if let Some(pb) = pb_opt.as_mut() {
        pb.update_to(0, &localized.progress_preparing);
    }

    // 执行步骤
    let mut succ: Vec<&str> = Vec::new();
    let mut fail: Vec<&str> = Vec::new();
    let mut updated: Vec<&str> = Vec::new();
    let mut unchanged: Vec<String> = Vec::new();
    let mut actions: Vec<&str> = Vec::new();
    let mut short_updates: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    let runner = ShellRunner;

    for (idx, step) in steps.iter().enumerate() {
        let (state, _rc, logfile) = if dry_run {
            (
                "unchanged".to_string(),
                0,
                run_tmp.join(format!("step.{}.log", idx)),
            )
        } else {
            match (step.fn_name)(&runner, &run_tmp, verbose, &mut pb_opt) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("step failed: {}", e);
                    (
                        "failed".to_string(),
                        1,
                        run_tmp.join(format!("step.{}.log", idx)),
                    )
                }
            }
        };

        if state == "changed" {
            // 根据步骤描述分类
            if step.desc.contains("清理") || step.desc.starts_with("Action：") {
                actions.push(&step.desc);
            } else {
                updated.push(&step.desc);
            }
            succ.push(&step.desc);
        } else if state == "unchanged" {
            if step.desc.contains("清理") || step.desc.starts_with("Action：") {
                actions.push(&step.desc);
            } else {
                let mut name = step.desc.to_string();
                name = name
                    .replace("更新", "")
                    .replace("升级", "")
                    .replace("  ", " ")
                    .trim()
                    .to_string();
                unchanged.push(name);
            }
            succ.push(&step.desc);
        } else {
            fail.push(&step.desc);
        }

        // 可选保留日志
        if keep_logs {
            let devcache = get_cache_dir();
            fs::create_dir_all(&devcache)?;
            let dest = devcache.join(
                logfile
                    .file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new("log")),
            );
            fs::copy(&logfile, &dest).ok();
        } else {
            fs::remove_file(&logfile).ok();
        }

        // 读取步骤输出的详情文件
        let mise_short = run_tmp.join("mise_short_updates.txt");
        if mise_short.exists() {
            if let Ok(s) = fs::read_to_string(&mise_short) {
                let details: Vec<String> = s.lines().map(|line| line.trim().to_string()).collect();
                if !details.is_empty() {
                    short_updates.insert(step.desc.to_string(), details);
                }
            }
            let _ = fs::remove_file(&mise_short);
        }

        let brew_details = run_tmp.join("brew_upgrade_details.txt");
        if brew_details.exists() {
            if let Ok(s) = fs::read_to_string(&brew_details) {
                let details: Vec<String> = s.lines().map(|line| line.trim().to_string()).collect();
                if !details.is_empty() {
                    short_updates.insert(step.desc.to_string(), details);
                }
            }
            let _ = fs::remove_file(&brew_details);
        }

        let rustup_details = run_tmp.join("rustup_upgrade_details.txt");
        if rustup_details.exists() {
            if let Ok(s) = fs::read_to_string(&rustup_details) {
                let details: Vec<String> = s.lines().map(|line| line.trim().to_string()).collect();
                if !details.is_empty() {
                    short_updates.insert(step.desc.to_string(), details);
                }
            }
            let _ = fs::remove_file(&rustup_details);
        }

        // 更新外部进度跟踪
        let done_count = (idx + 1) as u64;
        let percent = (100 * (idx + 1) / total) as i32;
        progress_update(percent, done_count, total as u64, &step.desc, &mut pb_opt);

        // 直接更新进度条显示
        if let Some(pb) = pb_opt.as_mut() {
            pb.update_to(done_count as usize, &step.desc);
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
