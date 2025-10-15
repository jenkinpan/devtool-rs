//! Benchmarks for command execution performance
//!
//! These benchmarks help identify performance bottlenecks and measure
//! the impact of optimizations, especially for parallel execution.

use criterion::{criterion_group, criterion_main, Criterion};
use std::process::Command;
use std::time::Duration;

/// Benchmark sequential command execution
fn bench_sequential_commands(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_execution");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("echo_commands_seq", |b| {
        b.iter(|| {
            for i in 0..5 {
                let output = Command::new("echo")
                    .arg(format!("test {}", i))
                    .output()
                    .expect("Failed to execute command");
                std::hint::black_box(output);
            }
        });
    });

    group.bench_function("true_commands_seq", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let output = Command::new("true")
                    .output()
                    .expect("Failed to execute command");
                std::hint::black_box(output);
            }
        });
    });

    group.finish();
}

/// Benchmark command execution with output parsing
fn bench_command_output_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("output_parsing");

    group.bench_function("parse_version_output", |b| {
        b.iter(|| {
            let output = Command::new("echo")
                .arg("version 1.2.3")
                .output()
                .expect("Failed to execute command");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let version = stdout.split_whitespace().nth(1).unwrap_or("unknown");
            std::hint::black_box(version);
        });
    });

    group.bench_function("parse_multiline_output", |b| {
        b.iter(|| {
            let output = Command::new("printf")
                .arg("line1\\nline2\\nline3\\nline4\\nline5")
                .output()
                .expect("Failed to execute command");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            std::hint::black_box(lines);
        });
    });

    group.finish();
}

/// Benchmark different command invocation patterns
fn bench_command_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("command_patterns");

    group.bench_function("direct_command", |b| {
        b.iter(|| {
            let output = Command::new("echo")
                .arg("test")
                .output()
                .expect("Failed to execute command");
            std::hint::black_box(output);
        });
    });

    group.bench_function("shell_command", |b| {
        b.iter(|| {
            let output = Command::new("sh")
                .arg("-c")
                .arg("echo test")
                .output()
                .expect("Failed to execute command");
            std::hint::black_box(output);
        });
    });

    group.finish();
}

/// Benchmark I/O operations
fn bench_io_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("io_operations");

    group.bench_function("write_read_temp_file", |b| {
        use std::fs;
        use std::io::Write;

        b.iter(|| {
            let temp_dir = std::env::temp_dir();
            let temp_file = temp_dir.join("devtool_bench.txt");

            // Write
            let mut file = fs::File::create(&temp_file).unwrap();
            file.write_all(b"test data").unwrap();
            drop(file);

            // Read
            let content = fs::read_to_string(&temp_file).unwrap();
            std::hint::black_box(content);

            // Cleanup
            let _ = fs::remove_file(&temp_file);
        });
    });

    group.finish();
}

/// Benchmark string operations commonly used in the tool
fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    let sample_output = "Updating homebrew...\n\
                         ==> Updated Formulae\n\
                         pkg1 pkg2 pkg3 pkg4 pkg5\n\
                         ==> Updated Casks\n\
                         app1 app2 app3";

    group.bench_function("split_lines", |b| {
        b.iter(|| {
            let lines: Vec<&str> = sample_output.lines().collect();
            std::hint::black_box(lines);
        });
    });

    group.bench_function("filter_lines", |b| {
        b.iter(|| {
            let filtered: Vec<&str> = sample_output
                .lines()
                .filter(|line| line.starts_with("==>"))
                .collect();
            std::hint::black_box(filtered);
        });
    });

    group.bench_function("regex_search", |b| {
        use regex::Regex;
        let re = Regex::new(r"pkg\d+").unwrap();

        b.iter(|| {
            let matches: Vec<_> = re.find_iter(sample_output).collect();
            std::hint::black_box(matches);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_sequential_commands,
    bench_command_output_parsing,
    bench_command_patterns,
    bench_io_operations,
    bench_string_operations,
);

criterion_main!(benches);
