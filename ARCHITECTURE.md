# Architecture

This document provides a high-level overview of devtool-rs architecture.

## Design Principles

1. **Modularity**: Each component has a clear, single responsibility
2. **Testability**: All components are designed to be easily testable
3. **Extensibility**: New package managers can be added with minimal changes
4. **User Experience**: Clear feedback and progress reporting
5. **Robustness**: Graceful error handling and recovery

## High-Level Architecture

```
┌─────────────────────────────────────┐
│          CLI Layer                  │
│      (Argument Parsing)             │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│       Main Controller               │
│   (Orchestrates Updates)            │
└──┬────┬────┬────┬────┬─────────────┘
   │    │    │    │    │
   ▼    ▼    ▼    ▼    ▼
┌────┐┌───┐┌────┐┌────┐┌─────┐
│UI  ││i18││Prog││Run ││Utils│
└────┘└───┘└────┘└────┘└─────┘
              │
    ┌─────────┴─────────┐
    │   Commands        │
    │ (Tool Updates)    │
    └┬────┬────┬────────┘
     │    │    │
     ▼    ▼    ▼
  ┌────┐┌───┐┌────┐
  │Brew││Rus││Mise│
  └────┘└───┘└────┘
```

## Module Overview

### Core Modules

- **cli**: Command-line argument parsing (Clap)
- **i18n**: Internationalization and language detection
- **ui**: User interface output (colors, progress bars)
- **runner**: External command execution abstraction
- **commands**: Tool-specific update implementations
  - `homebrew`: Homebrew package manager
  - `rustup`: Rust toolchain manager
  - `mise`: Development environment manager
- **utils**: Shared utility functions

### Key Design Decisions

**Trait-based Command Execution**
- `Runner` trait allows dependency injection and testing
- Easy to mock for unit tests
- Centralized error handling

**Module Isolation**
- Each tool (Homebrew, Rustup, Mise) is independent
- Tools can be added/removed without affecting others
- Tool detection is non-intrusive (checks if installed)

**Error Handling Strategy**
- Fatal errors: Stop immediately (invalid arguments, initialization)
- Tool errors: Log and continue (tool not installed, update failed)
- Non-critical errors: Log and proceed (parse warnings)

## Adding a New Tool

To add support for a new package manager:

1. Create `src/commands/newtool.rs`
2. Implement detection: `pub fn is_installed() -> bool`
3. Implement update: `pub fn update(runner: &dyn Runner, dry_run: bool) -> Result<()>`
4. Add tests in `#[cfg(test)] mod tests`
5. Register in `src/commands/mod.rs`
6. Update main controller in `src/main.rs`

See `CONTRIBUTING.md` for detailed guidelines.

## Testing

- **Unit tests**: Each module tests its own logic
- **Integration**: Main workflow tested end-to-end
- **Mocking**: `Runner` trait enables command mocking

Run tests: `cargo test`

## Performance

Current implementation is sequential. Parallel execution is planned for v0.5.0.

See `ROADMAP.md` for future improvements.

## Further Reading

- Code documentation: `cargo doc --open`
- Contributing guide: `CONTRIBUTING.md`
- Future plans: `ROADMAP.md`

---

**Note**: This is a living document. For detailed implementation specifics, refer to the source code and inline documentation.