# Architecture Documentation

This document describes the architecture and design principles of devtool-rs.

## Overview

devtool-rs is a command-line tool designed to unify the update process for various development tools and package managers. The architecture emphasizes modularity, testability, and extensibility.

## Design Principles

1. **Modularity**: Each component has a clear, single responsibility
2. **Testability**: All components are designed to be easily testable
3. **Extensibility**: New package managers can be added with minimal changes
4. **User Experience**: Clear feedback and progress reporting
5. **Robustness**: Graceful error handling and recovery

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│                    (Argument Parsing)                        │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                      Main Controller                         │
│              (Orchestrates Update Process)                   │
└───┬──────────────┬──────────────┬──────────────┬───────────┘
    │              │              │              │
    ▼              ▼              ▼              ▼
┌───────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│  UI   │    │  i18n   │    │ Progress│    │ Runner  │
│Module │    │ Module  │    │ Module  │    │ Module  │
└───────┘    └─────────┘    └─────────┘    └─────────┘
                                                  │
                    ┌─────────────────────────────┘
                    │
    ┌───────────────▼───────────────┐
    │      Commands Module          │
    │  (Tool-Specific Implementations)│
    └───┬───────┬───────┬───────────┘
        │       │       │
        ▼       ▼       ▼
   ┌─────┐ ┌──────┐ ┌──────┐
   │Brew │ │Rustup│ │ Mise │
   └─────┘ └──────┘ └──────┘
```

## Module Structure

### 1. CLI Module (`src/cli/`)

**Purpose**: Parse and validate command-line arguments

**Key Components**:
- `Args`: Main argument structure using Clap
- Command definitions
- Flag definitions (dry-run, verbose, no-color, etc.)

**Responsibilities**:
- Parse command-line arguments
- Validate input
- Display help and version information

**Example**:
```rust
pub struct Args {
    pub command: String,
    pub dry_run: bool,
    pub verbose: bool,
    pub no_color: bool,
}
```

### 2. i18n Module (`src/i18n/`)

**Purpose**: Handle internationalization and localization

**Key Components**:
- Language detection (system locale, environment variables)
- Localized string storage
- Translation functions

**Supported Languages**:
- English (en)
- Chinese (zh)

**Responsibilities**:
- Detect user's preferred language
- Provide translated strings
- Fall back to English if translation missing

### 3. UI Module (`src/ui/`)

**Purpose**: Handle all user interface output

**Submodules**:
- `colors`: Colored output functions
- `progress`: Progress bar management

**Key Functions**:
- `print_success()`: Green success messages
- `print_info()`: Cyan information messages
- `print_warning()`: Yellow warning messages
- `print_error()`: Red error messages
- `print_banner()`: Application banner

**Responsibilities**:
- Consistent color scheme
- Terminal capability detection
- Progress indication
- Status reporting

### 4. Progress Module (`src/ui/progress/`)

**Purpose**: Manage progress bars and status tracking

**Key Components**:
- `Bar`: Progress bar wrapper
- `ProgressStatus`: Serializable status for external queries

**Features**:
- Real-time progress updates
- External status queries via JSON
- Multiple progress bar support (future)

### 5. Runner Module (`src/runner/`)

**Purpose**: Abstract command execution

**Key Components**:
- `Runner` trait: Interface for command execution
- `ShellRunner`: Default implementation
- `run_command()`: High-level execution function

**Benefits**:
- Testability (mock runners for tests)
- Consistent error handling
- Centralized logging
- Future async support

**Example**:
```rust
pub trait Runner {
    fn run(&self, command: &str, args: &[&str]) -> Result<Output>;
}
```

### 6. Commands Module (`src/commands/`)

**Purpose**: Implement tool-specific update logic

**Structure**:
```
commands/
├── mod.rs          # Common types and interfaces
├── homebrew.rs     # Homebrew implementation
├── rustup.rs       # Rustup implementation
└── mise.rs         # Mise implementation
```

**Each Tool Module Implements**:
- `is_installed()`: Check if tool is available
- `update()`: Perform the update
- Version detection (where applicable)
- Error handling specific to the tool

**Common Pattern**:
```rust
pub fn update(runner: &dyn Runner, dry_run: bool) -> Result<()> {
    if !is_installed() {
        return Ok(()); // Skip if not installed
    }
    
    // Perform update steps
    // ...
}
```

### 7. Utils Module (`src/utils/`)

**Purpose**: Shared utility functions

**Key Functions**:
- `get_cache_dir()`: Get application cache directory
- File system helpers
- Date/time utilities
- Common string operations

## Data Flow

### 1. Application Startup

```
User Input → CLI Parser → Args Structure → Main Function
```

### 2. Update Process

```
Main
  ├─→ Detect Language (i18n)
  ├─→ Print Banner (UI)
  ├─→ Initialize Runner
  └─→ For Each Tool:
        ├─→ Check if installed (Commands)
        ├─→ Create progress bar (Progress)
        ├─→ Execute update (Runner)
        ├─→ Update progress (Progress)
        ├─→ Log output (Utils)
        └─→ Report status (UI)
```

### 3. Progress Status Query

```
Query Request → Read Status File → Parse JSON → Return Status
```

## Error Handling Strategy

### Levels of Error Handling

1. **Fatal Errors**: Stop execution immediately
   - Invalid command-line arguments
   - Cannot create cache directory
   - Cannot initialize logging

2. **Tool Errors**: Skip tool and continue
   - Tool not installed
   - Update command failed
   - Network timeout

3. **Non-Critical Errors**: Log and continue
   - Cannot parse version output
   - Log file write failed

### Error Propagation

```rust
// Use anyhow::Result for flexible error handling
use anyhow::{Result, Context};

pub fn update_tool() -> Result<()> {
    some_operation()
        .context("Failed to update tool")?;
    Ok(())
}
```

## Testing Strategy

### Unit Tests

Each module has its own test suite testing:
- Individual functions
- Edge cases
- Error conditions

**Location**: `#[cfg(test)] mod tests` in each module

### Integration Tests

Test the interaction between modules:
- Full update workflow
- CLI argument parsing
- Progress reporting

**Location**: `tests/` directory (future)

### Benchmark Tests

Performance benchmarks for optimization:
- Command execution speed
- I/O operations
- String parsing

**Location**: `benches/` directory

## Configuration Management

### Current Implementation

- Command-line flags only
- Environment variables for language detection

### Future Configuration File

Planned `~/.config/devtool/config.toml`:
```toml
[general]
parallel = true
max_jobs = 4
verbose = false

[homebrew]
enabled = true
auto_cleanup = true

[rustup]
enabled = true
toolchains = ["stable", "nightly"]
```

## Logging

### Log Locations

- **Individual tool logs**: `~/.cache/devtool/logs/<tool>_<timestamp>.log`
- **Status file**: `~/.cache/devtool/progress_status.json`

### Log Format

```
[2024-01-15 10:30:45] INFO: Starting Homebrew update
[2024-01-15 10:30:46] DEBUG: Running: brew update
[2024-01-15 10:31:02] INFO: Homebrew update completed successfully
```

## Future Architecture Improvements

### 1. Async/Parallel Execution

**Goal**: Run independent updates concurrently

**Changes Needed**:
- Convert `Runner` trait to async
- Implement task dependency graph
- Use Tokio for async runtime
- Handle concurrent progress bars

**Example**:
```rust
#[async_trait]
pub trait AsyncRunner {
    async fn run(&self, command: &str, args: &[&str]) -> Result<Output>;
}
```

### 2. Plugin System

**Goal**: Allow third-party tool implementations

**Architecture**:
```
Plugin API
├── Plugin Trait
├── Plugin Discovery
├── Plugin Loading
└── Plugin Execution
```

**Benefits**:
- Community contributions
- No need to fork for new tools
- Faster feature development

### 3. Configuration System

**Goal**: Persistent user preferences

**Components**:
- Config file parser (TOML)
- Config validation
- Config merging (file + CLI)
- Config migration

## Performance Considerations

### Current Performance

- **Startup time**: ~50ms
- **Memory usage**: ~10MB base
- **Sequential execution**: Sum of all tool update times

### Optimization Opportunities

1. **Parallel Execution**: 50-70% time reduction for independent tools
2. **Incremental Updates**: Only check changed packages
3. **Caching**: Cache tool version checks
4. **Async I/O**: Reduce blocking on slow operations

## Security Considerations

### Current Security Measures

1. **No credential storage**: Relies on system authentication
2. **Shell command sanitization**: Limited to known commands
3. **No network code**: Uses system package managers

### Future Security Enhancements

1. **Command whitelisting**: Only allow known-safe commands
2. **Privilege escalation handling**: Proper sudo management
3. **Supply chain security**: Verify package manager authenticity
4. **Audit logging**: Track all system modifications

## Extensibility Points

### Adding a New Package Manager

1. Create new module in `src/commands/`
2. Implement detection function
3. Implement update function
4. Add tests
5. Update main controller

**Minimal Example**:
```rust
// src/commands/npm.rs
use crate::runner::Runner;
use anyhow::Result;

pub fn is_installed() -> bool {
    which::which("npm").is_ok()
}

pub fn update(runner: &dyn Runner, dry_run: bool) -> Result<()> {
    if !is_installed() {
        return Ok(());
    }
    
    runner.run("npm", &["update", "-g"])?;
    Ok(())
}
```

### Adding a New Language

1. Add translations to `src/i18n/mod.rs`
2. Update language detection logic
3. Add tests

## Dependencies

### Core Dependencies

- **clap**: CLI argument parsing
- **anyhow**: Error handling
- **colored**: Terminal colors
- **serde/serde_json**: Serialization for status
- **chrono**: Date/time handling

### Development Dependencies

- **criterion**: Benchmarking
- **tempfile**: Test fixtures

### Dependency Philosophy

- Prefer well-maintained, popular crates
- Minimize dependency count
- Avoid dependencies with C bindings when possible
- Regular dependency audits

## Build and Release Process

### Build Targets

- Linux (x86_64, GNU and musl)
- macOS (x86_64, aarch64/M1)
- Windows (x86_64)

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite
4. Run benchmarks (compare with previous)
5. Create and push tag
6. GitHub Actions handles:
   - Multi-platform builds
   - GitHub Release creation
   - crates.io publication

## Monitoring and Observability

### Current Status

- Exit codes for success/failure
- Log files for troubleshooting
- JSON status for external monitoring

### Future Enhancements

- Structured logging (tracing crate)
- Metrics collection (update duration, success rate)
- Telemetry (opt-in, privacy-preserving)

## Conclusion

The architecture of devtool-rs is designed to be simple, modular, and extensible. The current design supports the core use case well while providing clear paths for future enhancements like parallel execution, plugin systems, and advanced configuration.

For questions or suggestions about the architecture, please open an issue or discussion on GitHub.

---

**Document Version**: 1.0  
**Last Updated**: January 2024  
**Maintainer**: devtool-rs contributors