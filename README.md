# devtool 🚀

A modern, efficient developer tool written in Rust that unifies the update process for your development environment tools and package managers with a single command. Say goodbye to multiple update commands and let `devtool` keep everything up to date.

[![Crates.io](https://img.shields.io/crates/v/devtool.svg)](https://crates.io/crates/devtool)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

## ✨ Features

- **One-command updates**: Run `devtool` to automatically update multiple supported tools
- **Smart detection**: Automatically detects installed tools (Homebrew, rustup, mise) and skips uninstalled ones
- **Beautiful progress feedback**: Real-time progress bars and status updates in the terminal
- **Detailed logging**: Each step's output is logged for easy troubleshooting
- **Execution summary**: Clear summary showing which tools were updated, already latest, or failed
- **Dry run mode**: Use `--dry-run` to preview steps without making changes
- **External status monitoring**: Query real-time status with `devtool progress-status` for integration with other tools
- **Multi-language support**: Automatically detects system language and displays Chinese or English interface
- **Version information**: Check version with `devtool -V` or `devtool --version`

## 🛠️ Supported Tools

`devtool` currently supports automatic detection and updating of:

- **Homebrew**: Update index, upgrade packages, cleanup old versions
- **Rust (rustup)**: Update `stable` Rust toolchain
- **Mise** (formerly rtx): Run `mise up` to update all managed languages and tools (Node.js, Python, etc.)

## 📦 Installation

### From crates.io (Recommended)

```bash
cargo install devtool
```

Make sure `~/.cargo/bin` is in your `PATH` environment variable.

### From source

```bash
git clone https://github.com/jenkinpan/devtool-rs.git
cd devtool-rs
cargo build --release
cargo install --path .
```

## 🚀 Usage

### Basic Usage

Simply run `devtool` in your terminal to start the update process:

```bash
devtool
```

This is equivalent to running `devtool update`.

### Command Line Options

Customize `devtool` behavior with these options:

| Option        | Short | Description                                                            |
| ------------- | ----- | ---------------------------------------------------------------------- |
| `--dry-run`   | `-n`  | Show steps that would be executed without actually running them        |
| `--verbose`   | `-v`  | Print detailed output for each step during execution                   |
| `--version`   | `-V`  | Show version information                                               |
| `--keep-logs` |       | Keep log files for each step, stored in `~/.cache/devtool/` by default |
| `--no-banner` |       | Don't show startup banner                                              |
| `--compact`   |       | Use more compact output format for non-interactive environments        |
| `--parallel`  |       | (Not implemented) Execute update steps in parallel                     |
| `--no-color`  |       | Disable colored output                                                 |
| `--help`      | `-h`  | Show help information                                                  |

### Examples

**Standard update:**

```bash
$ devtool
🚀 Starting devtool update: 2025-10-04 13:17:20
📋 Will execute 5 steps:
  1) Homebrew: Update index
  2) Homebrew: Upgrade packages
  3) Action: Cleanup old versions
  4) Rust: Update stable toolchain
  5) Mise: Update managed tools
[========================================] 5/5 (100%) | Mise: Update managed tools

🎉 Update completed: 2025-10-04 13:18:04 (Time taken: 14秒)
✅ Updated: Action: Cleanup old versions
⚠️ Already latest: Homebrew: Update index, Homebrew: Upgrade packages, Rust: Update stable toolchain, Mise: Update managed tools
```

**Dry run:**

```bash
devtool --dry-run
```

**Check version:**

```bash
devtool -V
# Output: devtool 0.3.3
```

**Monitor progress (for scripts or status bars):**

Run `devtool` in one terminal, query progress in another:

```bash
devtool progress-status
# Output: Progress status: ProgressStatus { state: "update", percent: Some(60), done: Some(3), total: Some(5), desc: Some("Action: Cleanup old versions"), ts: Some("2025-10-04T13:17:45.123+08:00") }
```

**Language support:**

The tool automatically detects your system language and displays the interface in Chinese or English accordingly.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests on the GitHub repository.

## 📜 License

This project is licensed under both MIT License and Apache License 2.0.

## 🔗 Links

- [Crates.io](https://crates.io/crates/devtool)
- [GitHub Repository](https://github.com/jenkinpan/devtool-rs)
- [Documentation](https://docs.rs/devtool)
