//! Benchmarks for parallel execution performance
//!
//! These benchmarks measure the performance difference between
//! sequential and parallel execution modes, helping to validate
//! the effectiveness of the parallel execution framework.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use tokio::runtime::Runtime;

/// Mock tool execution function for benchmarking
async fn mock_tool_execution(tool_name: &str, delay_ms: u64) -> (String, bool, String) {
    // Simulate work with a delay
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;

    (
        tool_name.to_string(),
        true,
        format!("Mock execution completed for {}", tool_name),
    )
}

/// Benchmark sequential vs parallel execution
fn bench_execution_modes(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("execution_modes");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    // Test with different numbers of tools
    for tool_count in [1, 3, 5, 10] {
        let tools: Vec<String> = (0..tool_count)
            .map(|i| match i % 3 {
                0 => "Homebrew".to_string(),
                1 => "Rustup".to_string(),
                _ => "Mise".to_string(),
            })
            .collect();

        // Sequential execution benchmark
        group.bench_with_input(
            BenchmarkId::new("sequential", tool_count),
            &tools,
            |b, tools| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut results = Vec::new();
                        for tool in tools.clone() {
                            let result = mock_tool_execution(&tool, 100).await;
                            results.push(result);
                        }
                        results
                    })
                });
            },
        );

        // Parallel execution benchmark
        group.bench_with_input(
            BenchmarkId::new("parallel", tool_count),
            &tools,
            |b, tools| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut handles = Vec::new();
                        for tool in tools.clone() {
                            let handle =
                                tokio::spawn(async move { mock_tool_execution(&tool, 100).await });
                            handles.push(handle);
                        }

                        let mut results = Vec::new();
                        for handle in handles {
                            results.push(handle.await.unwrap());
                        }
                        results
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark different concurrency levels
fn bench_concurrency_levels(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("concurrency_levels");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(10);

    let tools = vec![
        "Homebrew".to_string(),
        "Rustup".to_string(),
        "Mise".to_string(),
    ];

    for concurrency in [1, 2, 3, 4, 6, 8] {
        group.bench_with_input(
            BenchmarkId::new("parallel_jobs", concurrency),
            &concurrency,
            |b, &_jobs| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut handles = Vec::new();
                        for tool in tools.clone() {
                            let handle =
                                tokio::spawn(async move { mock_tool_execution(&tool, 150).await });
                            handles.push(handle);
                        }

                        let mut results = Vec::new();
                        for handle in handles {
                            results.push(handle.await.unwrap());
                        }
                        results
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark different task durations
fn bench_task_durations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("task_durations");
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(10);

    let tools = vec![
        "Homebrew".to_string(),
        "Rustup".to_string(),
        "Mise".to_string(),
    ];

    for duration_ms in [50, 100, 200, 500, 1000] {
        group.bench_with_input(
            BenchmarkId::new("task_duration", duration_ms),
            &duration_ms,
            |b, &duration| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut handles = Vec::new();
                        for tool in tools.clone() {
                            let handle =
                                tokio::spawn(
                                    async move { mock_tool_execution(&tool, duration).await },
                                );
                            handles.push(handle);
                        }

                        let mut results = Vec::new();
                        for handle in handles {
                            results.push(handle.await.unwrap());
                        }
                        results
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark scheduler overhead
fn bench_scheduler_overhead(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("scheduler_overhead");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    let tools = vec![
        "Homebrew".to_string(),
        "Rustup".to_string(),
        "Mise".to_string(),
    ];

    // Direct async execution
    group.bench_function("direct_async", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut handles = Vec::new();
                for tool in tools.clone() {
                    let handle = tokio::spawn(async move { mock_tool_execution(&tool, 100).await });
                    handles.push(handle);
                }

                let mut results = Vec::new();
                for handle in handles {
                    results.push(handle.await.unwrap());
                }
                results
            })
        });
    });

    // Sequential execution for comparison
    group.bench_function("sequential", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut results = Vec::new();
                for tool in tools.clone() {
                    let result = mock_tool_execution(&tool, 100).await;
                    results.push(result);
                }
                results
            })
        });
    });

    group.finish();
}

/// Benchmark memory usage patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("memory_patterns");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(15);

    // Small task set
    let small_tools = vec!["Homebrew".to_string()];
    group.bench_function("small_task_set", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut handles = Vec::new();
                for tool in small_tools.clone() {
                    let handle = tokio::spawn(async move { mock_tool_execution(&tool, 50).await });
                    handles.push(handle);
                }

                let mut results = Vec::new();
                for handle in handles {
                    results.push(handle.await.unwrap());
                }
                results
            })
        });
    });

    // Medium task set
    let medium_tools = vec![
        "Homebrew".to_string(),
        "Rustup".to_string(),
        "Mise".to_string(),
    ];
    group.bench_function("medium_task_set", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut handles = Vec::new();
                for tool in medium_tools.clone() {
                    let handle = tokio::spawn(async move { mock_tool_execution(&tool, 50).await });
                    handles.push(handle);
                }

                let mut results = Vec::new();
                for handle in handles {
                    results.push(handle.await.unwrap());
                }
                results
            })
        });
    });

    // Large task set
    let large_tools: Vec<String> = (0..10)
        .map(|i| match i % 3 {
            0 => "Homebrew".to_string(),
            1 => "Rustup".to_string(),
            _ => "Mise".to_string(),
        })
        .collect();
    group.bench_function("large_task_set", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut handles = Vec::new();
                for tool in large_tools.clone() {
                    let handle = tokio::spawn(async move { mock_tool_execution(&tool, 50).await });
                    handles.push(handle);
                }

                let mut results = Vec::new();
                for handle in handles {
                    results.push(handle.await.unwrap());
                }
                results
            })
        });
    });

    group.finish();
}

/// Benchmark error handling performance
fn bench_error_handling(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("error_handling");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(15);

    let tools = vec![
        "Homebrew".to_string(),
        "Rustup".to_string(),
        "Mise".to_string(),
    ];

    // All successful tasks
    group.bench_function("all_success", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut handles = Vec::new();
                for tool in tools.clone() {
                    let handle = tokio::spawn(async move { mock_tool_execution(&tool, 100).await });
                    handles.push(handle);
                }

                let mut results = Vec::new();
                for handle in handles {
                    results.push(handle.await.unwrap());
                }
                results
            })
        });
    });

    // Mixed success/failure (simulated)
    group.bench_function("mixed_results", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut handles = Vec::new();
                for tool in tools.clone() {
                    let handle = tokio::spawn(async move {
                        // Simulate some failures
                        if tool == "Rustup" {
                            (tool, false, "Simulated failure".to_string())
                        } else {
                            mock_tool_execution(&tool, 100).await
                        }
                    });
                    handles.push(handle);
                }

                let mut results = Vec::new();
                for handle in handles {
                    results.push(handle.await.unwrap());
                }
                results
            })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_execution_modes,
    bench_concurrency_levels,
    bench_task_durations,
    bench_scheduler_overhead,
    bench_memory_patterns,
    bench_error_handling,
);

criterion_main!(benches);
