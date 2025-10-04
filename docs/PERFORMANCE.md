# Performance Optimization Guide

This document outlines the performance characteristics of devtool-rs and planned optimizations.

## Current Performance Baseline

### Sequential Execution (v0.4.0)

The current implementation executes tool updates sequentially:

```
Total Time = Homebrew Time + Rustup Time + Mise Time + Overhead
```

**Typical execution times** (approximate):
- Homebrew update: 15-60 seconds
- Rustup update: 5-30 seconds  
- Mise update: 5-20 seconds
- Total: 25-110 seconds

**Performance characteristics**:
- Memory usage: ~10-15MB base
- CPU usage: Low (waiting for external commands)
- I/O: Dominated by package manager operations
- Network: Depends on package managers

### Bottlenecks

1. **Sequential execution**: No parallelism, tools wait unnecessarily
2. **Blocking I/O**: Main thread blocks on command execution
3. **Log file writes**: Synchronous writes during execution

## Planned Optimizations

### Phase 1: Parallel Execution (v0.5.0)

**Goal**: Execute independent tool updates concurrently

#### Expected Improvements

```
With 3 tools running in parallel:
Total Time ≈ max(Homebrew, Rustup, Mise) + Overhead
Speedup: 2-3x faster (50-70% time reduction)
```

**Example scenario**:
```
Sequential:  [Homebrew: 30s] -> [Rustup: 15s] -> [Mise: 10s] = 55s
Parallel:    [Homebrew: 30s]
             [Rustup:  15s]
             [Mise:    10s]
             Total: ~30s (45% faster)
```

#### Implementation Strategy

1. **Dependency Graph**
   - Most tools are independent → can run in parallel
   - Future: Handle dependencies (e.g., mise after rustup for Rust tools)

2. **Async Runtime** 
   - Use Tokio for async/await
   - Convert Runner trait to async
   - Non-blocking I/O for command execution

3. **Concurrency Control**
   - Configurable max parallel jobs (default: CPU cores)
   - Resource-aware scheduling
   - Graceful degradation on errors

#### Technical Approach

```rust
// Before (v0.4.0)
fn update_all() -> Result<()> {
    update_homebrew()?;
    update_rustup()?;
    update_mise()?;
    Ok(())
}

// After (v0.5.0)
async fn update_all() -> Result<()> {
    let tasks = vec![
        tokio::spawn(update_homebrew()),
        tokio::spawn(update_rustup()),
        tokio::spawn(update_mise()),
    ];
    
    for task in tasks {
        task.await??;
    }
    Ok(())
}
```

#### Challenges

- **Progress bars**: Multiple progress bars for concurrent tasks
- **Log interleaving**: Prevent output from mixing
- **Error handling**: One task failure shouldn't stop others
- **Testing**: Mock async operations in tests

### Phase 2: Incremental Updates (v0.6.0)

**Goal**: Only check/update what's necessary

#### Optimizations

1. **Cache version checks**
   - Store last known versions
   - Skip checks if recently updated
   - TTL-based cache invalidation

2. **Smart update detection**
   - Check if updates available before starting
   - Skip tools with no updates
   - Parallel version checks

3. **Differential updates**
   - For package managers: only update changed packages
   - For toolchains: skip if already latest

**Expected improvement**: 30-50% faster when no updates available

### Phase 3: Advanced Optimizations (v0.7.0+)

1. **Batched operations**
   - Combine multiple commands where possible
   - Reduce subprocess overhead

2. **Async I/O**
   - Non-blocking log writes
   - Streaming command output
   - Async file operations

3. **Memory optimization**
   - Stream large outputs instead of buffering
   - Optimize string allocations
   - Lazy initialization

4. **Network optimization**
   - Connection pooling
   - Parallel downloads
   - Resume failed downloads

## Benchmarking

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench command_execution

# Compare with baseline
cargo bench -- --save-baseline main
# ... make changes ...
cargo bench -- --baseline main
```

### Key Metrics

1. **Total execution time**: End-to-end update duration
2. **Per-tool time**: Individual tool update duration
3. **Startup overhead**: Time before first command
4. **Memory usage**: Peak and average RSS
5. **CPU utilization**: Percentage during execution

### Benchmark Targets

| Metric | Current (v0.4.0) | Target (v0.5.0) | Target (v1.0.0) |
|--------|------------------|-----------------|-----------------|
| Total time (3 tools) | 55s | 30s (-45%) | 25s (-55%) |
| Startup time | 50ms | 50ms | <30ms |
| Memory usage | 15MB | 20MB | <25MB |
| CPU efficiency | Low | Medium | High |

## Performance Testing

### Automated Performance Tests

```bash
# In CI/CD
- name: Performance regression test
  run: |
    cargo bench --bench command_execution > new.txt
    # Compare with baseline
    python scripts/compare_bench.py baseline.txt new.txt
```

### Manual Performance Testing

1. **Test scenario**: Update all tools
   ```bash
   time devtool update
   ```

2. **Test with dry-run** (measures overhead only):
   ```bash
   time devtool update --dry-run
   ```

3. **Test individual tools**:
   ```bash
   time devtool homebrew
   time devtool rustup
   time devtool mise
   ```

4. **Profile with cargo-flamegraph**:
   ```bash
   cargo install flamegraph
   sudo cargo flamegraph --bin devtool
   ```

## Monitoring Performance

### Metrics to Track

1. **Execution time distribution** (percentiles: p50, p90, p99)
2. **Per-tool success rate**
3. **Network transfer time vs. compute time**
4. **Error rate impact on performance**

### Future: Telemetry

Opt-in performance telemetry to help identify bottlenecks:
- Anonymized execution times
- Tool combinations used
- Platform-specific performance data
- Error patterns

## Platform-Specific Considerations

### macOS
- Homebrew updates can be slow (30-60s typical)
- System permission prompts may block execution
- SIP (System Integrity Protection) impacts

### Linux
- Faster package manager operations generally
- Sudo password prompts may interrupt flow
- Varies by distro (apt vs dnf vs pacman)

### Windows
- PowerShell overhead
- Windows Defender scans may slow execution
- Different tool ecosystem

## Best Practices

### For Users

1. **Use parallel mode** (when available):
   ```bash
   devtool update --parallel
   ```

2. **Run during low-activity periods**:
   - Less network contention
   - Less system resource competition

3. **Regular updates** reduce individual update time:
   ```bash
   # Add to crontab/launchd
   0 9 * * * devtool update
   ```

### For Developers

1. **Profile before optimizing**
   ```bash
   cargo flamegraph --bin devtool -- update
   ```

2. **Measure actual impact**
   ```bash
   cargo bench -- --baseline before
   ```

3. **Consider real-world scenarios**
   - Different network speeds
   - Different numbers of packages
   - Different system loads

4. **Optimize hot paths first**
   - Command execution (80% of time)
   - Output parsing (10% of time)
   - Everything else (10% of time)

## Contributing Performance Improvements

When submitting performance optimizations:

1. **Include benchmarks** showing improvement
2. **Test on multiple platforms** if possible
3. **Document trade-offs** (memory vs. speed, etc.)
4. **Check for regressions** in other areas
5. **Update this document** with new findings

### Performance PR Template

```markdown
## Performance Improvement

**What**: Brief description of optimization
**Why**: What bottleneck does this address?
**Impact**: X% faster, Y MB less memory, etc.

### Benchmarks

**Before**:
```
test_name: 1000ms
```

**After**:
```
test_name: 600ms (-40%)
```

### Trade-offs
- Increased code complexity: Low/Medium/High
- Memory impact: +5MB
- Platform compatibility: All/Limited
```

## Resources

- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## Appendix: Performance Profiling Tools

### Recommended Tools

1. **cargo-flamegraph**: Visual profiling
   ```bash
   cargo install flamegraph
   sudo cargo flamegraph --bin devtool
   ```

2. **hyperfine**: Command-line benchmarking
   ```bash
   brew install hyperfine
   hyperfine 'devtool update --dry-run'
   ```

3. **perf** (Linux): System profiling
   ```bash
   perf record -g ./target/release/devtool update
   perf report
   ```

4. **Instruments** (macOS): Xcode profiling tools

5. **cargo-criterion**: Advanced benchmarking
   ```bash
   cargo install cargo-criterion
   cargo criterion
   ```

### Example Profiling Session

```bash
# 1. Build optimized binary
cargo build --release

# 2. Profile with flamegraph
sudo cargo flamegraph --bin devtool -- update --dry-run

# 3. Analyze results
# Opens flamegraph.svg in browser

# 4. Run criterion benchmarks
cargo bench

# 5. Compare with baseline
cargo bench -- --save-baseline optimized
```

---

**Last Updated**: January 2024  
**Next Review**: After v0.5.0 release