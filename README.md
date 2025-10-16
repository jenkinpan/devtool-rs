# devtool 🚀

A modern, efficient developer tool written in Rust that unifies the update process for your development environment tools and package managers with a single command. Say goodbye to multiple update commands and let `devtool` keep everything up to date.

[![Crates.io](https://img.shields.io/crates/v/devtool.svg)](https://crates.io/crates/devtool)
[![CI](https://github.com/jenkinpan/devtool-rs/workflows/CI/badge.svg)](https://github.com/jenkinpan/devtool-rs/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

English | [简体中文](README_zh.md)

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

**Q: Can I customize which tools to update?**  
A: Currently, devtool updates all detected tools. Configuration file support is planned for v0.5.0.

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
