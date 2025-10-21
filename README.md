# devtool 🚀

A modern, efficient developer tool written in Rust that unifies the update process for your development environment tools and package managers with a single command. Say goodbye to multiple update commands and let `devtool` keep everything up to date.

[![Crates.io](https://img.shields.io/crates/v/devtool.svg)](https://crates.io/crates/devtool)
[![CI](https://github.com/jenkinpan/devtool-rs/workflows/CI/badge.svg)](https://github.com/jenkinpan/devtool-rs/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

English | [简体中文](README_zh.md)

## ✨ Features

- **One-command updates**: Run `devtool` to automatically update multiple supported tools
- **🚀 Parallel execution**: Default parallel execution for up to 10x faster updates with concurrent tool execution
- **Smart detection**: Automatically detects installed tools (Homebrew, rustup, mise) and skips uninstalled ones
- **Enhanced progress feedback**: Multi-progress bars with real-time updates and elapsed time tracking
- **📊 Detailed upgrade tracking**: Shows exactly what was upgraded with before/after version information
- **🔍 Smart version detection**: Only performs version comparison when actual upgrades occur, improving performance
- **📝 Comprehensive logging**: Each step's output is logged for easy troubleshooting
- **🗂️ Unified log storage**: All log files are stored in a unified location (`~/Library/Caches/devtool/` on macOS, `~/.cache/devtool/` on Linux)
- **🔍 Organized by tool**: Log files are organized by tool (Homebrew, Rustup, Mise) with timestamped directories
- **🔗 Latest symlinks**: Each tool directory includes a `latest` symlink pointing to the most recent execution
- **📋 Execution summary**: Clear summary showing which tools were updated, already latest, or failed
- **🧪 Dry run mode**: Use `--dry-run` to preview steps without making changes
- **⚙️ Flexible execution modes**: Choose between parallel (default) or sequential execution with `--sequential`
- **🔧 Configurable concurrency**: Set the number of concurrent jobs with `--jobs` (default: 3)
- **📡 External status monitoring**: Query real-time status with `devtool progress-status` for integration with other tools
- **🌍 Multi-language support**: Automatically detects system language and displays Chinese or English interface
- **⌨️ Shell completion**: Comprehensive completion support for bash, zsh, fish, powershell, elvish, and nushell
- **ℹ️ Version information**: Check version with `devtool -V` or `devtool --version`
- **📝 User feedback system**: Built-in feedback collection with `devtool feedback` command
- **🐛 Issue templates**: Standardized GitHub Issues templates for bug reports and feature requests
- **📊 Feedback analysis**: Tools for analyzing user feedback and improving the product

## 🛠️ Supported Tools

`devtool` currently supports automatic detection and updating of:

- **Homebrew**: Update index, upgrade packages, cleanup old versions
- **Rust (rustup)**: Update all installed Rust toolchains (stable, nightly, beta)
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

### Parallel Execution (Default in v0.7.0)

Parallel execution is now the default mode for faster updates:

```bash
# Default parallel execution with 3 concurrent jobs
devtool

# Control the number of concurrent jobs
devtool --jobs 5

# Sequential execution (override parallel mode)
devtool --sequential

# Parallel execution with dry run
devtool --jobs 3 --dry-run
```

**Performance Benefits:**
- Up to 10x faster execution with parallel mode (default)
- Configurable concurrency with `--jobs` parameter (default: 3)
- Choose between parallel (default) or sequential execution
- Intelligent dependency management
- Maintains all safety features of sequential mode

### Command Line Options

Customize `devtool` behavior with these options:

| Option         | Short | Description                                                            |
| -------------- | ----- | ---------------------------------------------------------------------- |
| `--dry-run`    | `-n`  | Show steps that would be executed without actually running them        |
| `--verbose`    | `-v`  | Print detailed output for each step during execution                   |
| `--version`    | `-V`  | Show version information                                               |
| `--keep-logs`  |       | Keep log files for each step, stored in unified cache directory by default |
| `--no-banner`  |       | Don't show startup banner                                              |
| `--compact`    |       | Use more compact output format for non-interactive environments        |
| `--parallel`   |       | Execute update steps in parallel (default)                             |
| `--sequential` |       | Execute update steps sequentially (override parallel mode)            |
| `--jobs`       |       | Number of concurrent jobs for parallel execution (default: 3)         |
| `--no-color`   |       | Disable colored output                                                 |
| `--help`       | `-h`  | Show help information                                                  |

## 📊 Upgrade Details Tracking

`devtool` now provides detailed upgrade tracking, showing exactly what was upgraded with before/after version information:

### Upgrade Detail Display

When upgrades occur, `devtool` shows detailed information about what was changed:

```bash
🎉 更新完成：2025-10-18 14:00:30 (耗时: 12秒)
✅ 已更新：Homebrew, Rustup
ℹ️ 无更新应用。

📋 升级详情：
Homebrew：升级软件包
  - git: 2.45.0 → 2.45.1
  - node: 20.10.0 → 20.11.0
  - python: 3.12.0 → 3.12.1

Rust：更新工具链
  - stable-aarch64-apple-darwin: 1.90.0 → 1.91.0
  - nightly-aarch64-apple-darwin: 1.92.0 → 1.93.0
```

### Smart Version Detection

- **Performance optimized**: Only performs version comparison when actual upgrades occur
- **Accurate tracking**: Distinguishes between index updates and actual package upgrades
- **Unified format**: All tools (Homebrew, Rustup, Mise) use consistent upgrade detail format
- **Multiple formats**: Supports both JSON and text formats for upgrade details

### Supported Upgrade Types

- **Version upgrades**: `package: old_version → new_version`
- **New installations**: `package: new installation → version`
- **Toolchain updates**: Shows Rust toolchain version changes
- **Tool updates**: Shows Mise-managed tool version changes

## 📁 Log Storage System

`devtool` provides a comprehensive log storage system for easy troubleshooting and debugging:

### Unified Log Storage

All log files are stored in a unified location:
- **macOS**: `~/Library/Caches/devtool/`
- **Linux**: `~/.cache/devtool/`

### Directory Structure

Logs are organized by tool with timestamped directories:

```
~/Library/Caches/devtool/
├── homebrew/
│   ├── 1761008090/
│   │   ├── brew_cleanup.log
│   │   ├── brew_detailed_debug.log
│   │   ├── brew_outdated.log
│   │   ├── brew_update.log
│   │   ├── brew_upgrade.log
│   │   └── outdated_packages.json
│   └── latest -> 1761008090/
├── rustup/
│   ├── 1761008078/
│   │   └── rustup_update.log
│   └── latest -> 1761008078/
├── mise/
│   ├── 1761008076/
│   │   └── mise_up.log
│   └── latest -> 1761008076/
└── feedback/
    └── devtool_feedback_*.md
```

### Log File Types

Each tool generates specific log files:

- **Homebrew**:
  - `brew_update.log`: Homebrew update output
  - `brew_upgrade.log`: Homebrew upgrade output
  - `brew_cleanup.log`: Homebrew cleanup output
  - `brew_outdated.log`: Outdated packages detection
  - `brew_detailed_debug.log`: Detailed debugging information
  - `outdated_packages.json`: JSON format of outdated packages

- **Rustup**:
  - `rustup_update.log`: Rustup update output

- **Mise**:
  - `mise_up.log`: Mise update output

### Using Logs for Troubleshooting

1. **Access latest logs**: Use the `latest` symlink for the most recent execution
   ```bash
   ls ~/Library/Caches/devtool/homebrew/latest/
   ```

2. **View specific log files**: Check individual log files for detailed information
   ```bash
   cat ~/Library/Caches/devtool/homebrew/latest/brew_detailed_debug.log
   ```

3. **Historical logs**: Browse timestamped directories for previous executions
   ```bash
   ls ~/Library/Caches/devtool/homebrew/
   ```

### Enabling Log Storage

Use the `--keep-logs` flag to enable log storage:

```bash
devtool update --keep-logs
```

This will save all log files to the unified cache directory for later analysis.

### Examples

**Standard update (parallel execution by default):**

```bash
$ devtool
🚀 Starting devtool update: 2025-10-16 16:59:41
📋 Will execute 3 steps:
  1) Homebrew update & upgrade & cleanup
  2) Rustup all toolchains update
  3) Mise tools update
🚀 Parallel execution mode (max concurrent: 3)
[00:00:11] [#########################] 100% ✅ Homebrew 完成
[00:00:12] [#########################] 100% ✅ Rustup 完成
[00:00:12] [#########################] 100% ✅ Mise 完成

🎉 Update completed: 2025-10-16 16:59:45 (Time taken: 19秒)
ℹ️ No updates applied.
⚠️ Already latest: Rustup, Mise
✅ Updated: Homebrew
```

**Dry run:**

```bash
devtool --dry-run
```

**User feedback:**

```bash
# Interactive feedback collection
devtool feedback

# Direct feedback submission
devtool feedback --type bug --message "Found an issue with Homebrew updates" --verbose

# Feature request
devtool feedback --type feature --message "Add support for npm updates"
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

## 🐚 Shell Completion

`devtool` provides comprehensive shell completion support for all major shells:

### Generate Completion Scripts

```bash
# Generate completion for your shell
devtool completion bash    # For bash
devtool completion zsh     # For zsh
devtool completion fish    # For fish
devtool completion powershell  # For PowerShell
devtool completion elvish  # For Elvish
devtool completion nushell # For Nushell
```

### Setup Completion

**Bash:**
```bash
# Add to ~/.bashrc
source <(devtool completion bash)
```

**Zsh:**
```bash
# Add to ~/.zshrc
source <(devtool completion zsh)
```

**Fish:**
```bash
# Add to ~/.config/fish/config.fish
devtool completion fish | source
```

**Nushell:**
```nu
# Add to your config.nu
use ~/.config/nushell/completions/devtool-completions.nu *
```

## 🔧 Troubleshooting

### Command not found

If you get `command not found: devtool` after installation:

```bash
# Check if ~/.cargo/bin is in your PATH
echo $PATH

# Add to your shell profile (e.g., ~/.bashrc, ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"

# Reload your shell configuration
source ~/.bashrc  # or source ~/.zshrc
```

### Permission errors

If you encounter permission errors:

```bash
# Make sure the binary is executable (Unix/Linux/macOS)
chmod +x ~/.cargo/bin/devtool

# On macOS, if blocked by Gatekeeper:
xattr -d com.apple.quarantine ~/.cargo/bin/devtool
```

### No tools detected

`devtool` only updates tools that are already installed. To get started:

- Install [Homebrew](https://brew.sh) (macOS/Linux)
- Install [Rustup](https://rustup.rs) (all platforms)
- Install [Mise](https://mise.jdx.dev) (all platforms)

### Updates fail

If updates fail consistently:

1. Try running the tool's update command manually to see the actual error
2. Check your internet connection
3. Verify you have sufficient disk space
4. Run with `--verbose` flag for detailed output: `devtool --verbose`
5. Check log files in `~/.cache/devtool/logs/` for detailed error messages

### Language/locale issues

Force English output if automatic language detection doesn't work:

```bash
LANG=en_US.UTF-8 devtool
```

## 💡 Tips & Tricks

### Create an alias

Add to your shell profile for quick access:

```bash
alias dup='devtool'
alias update-dev='devtool'
```

### Automated updates

**Using cron (Linux/macOS):**

```bash
# Edit crontab
crontab -e

# Add this line to run daily at 9 AM
0 9 * * * /home/username/.cargo/bin/devtool
```

**Using launchd (macOS):**

Create `~/Library/LaunchAgents/com.devtool.update.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.devtool.update</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/yourname/.cargo/bin/devtool</string>
    </array>
    <key>StartCalendarInterval</key>
    <dict>
        <key>Hour</key>
        <integer>9</integer>
        <key>Minute</key>
        <integer>0</integer>
    </dict>
</dict>
</plist>
```

Then load it: `launchctl load ~/Library/LaunchAgents/com.devtool.update.plist`

### Integration with other tools

**In Makefiles:**

```makefile
.PHONY: update-tools
update-tools:
	devtool

.PHONY: dev-setup
dev-setup: update-tools
	npm install
	bundle install
```

**In scripts:**

```bash
#!/bin/bash
# Safe update script
if devtool --dry-run; then
    echo "Dry run successful, proceeding..."
    devtool
else
    echo "Dry run failed, skipping update"
    exit 1
fi
```

### Notifications on completion

**macOS:**

```bash
devtool && osascript -e 'display notification "Updates completed" with title "devtool"'
```

**Linux (with notify-send):**

```bash
devtool && notify-send "devtool" "Updates completed"
```

## ❓ FAQ

**Q: Does devtool work on Windows?**  
A: Partial support. Rustup and Mise work, but Homebrew is not available on Windows. We plan to add winget support in the future.

**Q: Is it safe to run devtool?**  
A: Yes! devtool only calls the standard update commands for each tool. Use `--dry-run` to see exactly what will be executed before running.

**Q: How long does it take?**  
A: Typically 30-90 seconds depending on how many updates are available and your internet speed.

**Q: Can I run it in parallel?**  
A: Not yet, but parallel execution is planned for v0.5.0, which will significantly reduce update time.

**Q: Does it update system packages (apt, yum, etc.)?**  
A: Not yet, but system package manager support is planned for v0.6.0.

**Q: Where are log files stored?**  
A: Log files are stored in `~/.cache/devtool/logs/` (Linux/macOS) or `%LOCALAPPDATA%\devtool\logs\` (Windows).

**Q: How do I report a bug?**  
A: Please [open an issue](https://github.com/jenkinpan/devtool-rs/issues/new?template=bug_report.md) on GitHub with details about your environment and the error.

**Q: How can I contribute?**  
A: See our [CONTRIBUTING.md](CONTRIBUTING.md) guide! We welcome bug reports, feature requests, and pull requests.

## 📖 Documentation

- [Architecture Overview](ARCHITECTURE.md) - System design and module structure
- [Contributing Guide](CONTRIBUTING.md) - How to contribute to the project
- [Roadmap](ROADMAP.md) - Future development plans
- [Changelog](CHANGELOG.md) - Version history and release notes

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests on the GitHub repository.

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on:
- Reporting bugs
- Suggesting features
- Submitting pull requests
- Development setup
- Adding new package managers


## 🔗 Links

- [Crates.io](https://crates.io/crates/devtool)
- [GitHub Repository](https://github.com/jenkinpan/devtool-rs)
- [Documentation](https://docs.rs/devtool)
- [Report Issues](https://github.com/jenkinpan/devtool-rs/issues)
- [Discussions](https://github.com/jenkinpan/devtool-rs/discussions)
