# Quick Start Guide

Get started with devtool-rs in minutes!

## Installation

### From crates.io (Recommended)

```bash
cargo install devtool
```

### From source

```bash
git clone https://github.com/jenkinpan/devtool-rs.git
cd devtool-rs
cargo install --path .
```

### From GitHub releases

Download the pre-built binary for your platform from the [releases page](https://github.com/jenkinpan/devtool-rs/releases).

#### Linux

```bash
# x86_64
wget https://github.com/jenkinpan/devtool-rs/releases/latest/download/devtool-linux-x86_64.tar.gz
tar -xzf devtool-linux-x86_64.tar.gz
sudo mv devtool /usr/local/bin/
```

#### macOS

```bash
# Intel
wget https://github.com/jenkinpan/devtool-rs/releases/latest/download/devtool-macos-x86_64.tar.gz
tar -xzf devtool-macos-x86_64.tar.gz
sudo mv devtool /usr/local/bin/

# Apple Silicon (M1/M2/M3)
wget https://github.com/jenkinpan/devtool-rs/releases/latest/download/devtool-macos-aarch64.tar.gz
tar -xzf devtool-macos-aarch64.tar.gz
sudo mv devtool /usr/local/bin/
```

#### Windows

Download `devtool-windows-x86_64.zip` and extract to a directory in your PATH.

## Verify Installation

```bash
devtool --version
```

You should see output like: `devtool 0.4.0`

## Basic Usage

### Update all tools

The simplest command - updates all detected tools:

```bash
devtool
```

or explicitly:

```bash
devtool update
```

**What it does**:
1. Detects installed tools (Homebrew, Rustup, Mise)
2. Updates each detected tool
3. Shows progress bars for each operation
4. Displays a summary of results

### Update specific tools

Update only Homebrew:
```bash
devtool homebrew
```

Update only Rustup:
```bash
devtool rustup
```

Update only Mise:
```bash
devtool mise
```

### Dry run mode

Preview what would be updated without making changes:

```bash
devtool --dry-run
```

Perfect for:
- Testing the tool
- Seeing what will be updated
- Checking if everything is working

### Verbose output

See detailed output from all commands:

```bash
devtool --verbose
```

or short form:

```bash
devtool -v
```

### No color output

Disable colored output (useful for scripts):

```bash
devtool --no-color
```

### Check progress status

Query the current progress (useful in scripts):

```bash
devtool progress-status
```

Returns JSON output:
```json
{
  "current_step": 2,
  "total_steps": 3,
  "current_tool": "rustup",
  "message": "Updating Rust toolchain...",
  "timestamp": "2024-01-15T10:30:45Z"
}
```

## Common Workflows

### Morning routine

Update everything at the start of your workday:

```bash
devtool
```

### Safe exploration

Try it out without making changes:

```bash
devtool --dry-run --verbose
```

### Script integration

Use in a shell script:

```bash
#!/bin/bash
if devtool --dry-run; then
    echo "Dry run successful, proceeding with update..."
    devtool
else
    echo "Dry run failed, skipping update"
    exit 1
fi
```

### Automated updates

Add to crontab for automatic updates:

```bash
# Edit crontab
crontab -e

# Add this line to run every day at 9 AM
0 9 * * * /usr/local/bin/devtool
```

Or on macOS with launchd:

```bash
# Create ~/Library/LaunchAgents/com.devtool.update.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.devtool.update</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/devtool</string>
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

### Check progress from another terminal

While devtool is running in one terminal:

```bash
# In another terminal
watch -n 1 'devtool progress-status | jq'
```

## Supported Tools

### Homebrew (macOS/Linux)

**What gets updated**:
- Package index (`brew update`)
- Installed packages (`brew upgrade`)
- Casks (applications)
- Cleanup old versions (`brew cleanup`)

**Requirements**:
- Homebrew must be installed
- Available at: https://brew.sh

### Rustup (All platforms)

**What gets updated**:
- Stable Rust toolchain
- Rust components (rustfmt, clippy, etc.)

**Requirements**:
- Rustup must be installed
- Available at: https://rustup.rs

### Mise (All platforms)

**What gets updated**:
- All tools managed by Mise (`mise up`)
- This includes Node.js, Python, Ruby, etc.

**Requirements**:
- Mise must be installed
- Available at: https://mise.jdx.dev

## Troubleshooting

### "Command not found: devtool"

Make sure the binary is in your PATH:

```bash
# Check if devtool is in PATH
which devtool

# If not, add installation directory to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Permission errors

On Unix systems, you may need to make the binary executable:

```bash
chmod +x devtool
```

### No tools detected

devtool only updates tools that are installed. Install at least one:

- [Homebrew](https://brew.sh)
- [Rustup](https://rustup.rs)
- [Mise](https://mise.jdx.dev)

### Updates fail

1. Check you have internet connectivity
2. Try running the tool's update command manually
3. Check logs in `~/.cache/devtool/logs/`
4. Run with verbose flag: `devtool --verbose`

### Language/locale issues

devtool auto-detects your system language. To force English:

```bash
LANG=en_US.UTF-8 devtool
```

## Log Files

devtool creates log files for each update:

**Location**: `~/.cache/devtool/logs/`

**Files**:
- `homebrew_YYYYMMDD_HHMMSS.log`
- `rustup_YYYYMMDD_HHMMSS.log`
- `mise_YYYYMMDD_HHMMSS.log`

**View recent logs**:
```bash
ls -lt ~/.cache/devtool/logs/ | head
cat ~/.cache/devtool/logs/homebrew_*.log | tail -50
```

## Getting Help

### Built-in help

```bash
devtool --help
```

### Version information

```bash
devtool --version
```

### Community support

- **Issues**: https://github.com/jenkinpan/devtool-rs/issues
- **Discussions**: https://github.com/jenkinpan/devtool-rs/discussions
- **Documentation**: https://docs.rs/devtool

## What's Next?

- Read the [Architecture](../ARCHITECTURE.md) to understand how devtool works
- Check the [Roadmap](../ROADMAP.md) to see what's coming
- Contribute! See [Contributing Guide](../CONTRIBUTING.md)
- Explore [Performance Optimization](PERFORMANCE.md) for advanced usage

## Tips & Tricks

### Alias for convenience

Add to your shell config (`.bashrc`, `.zshrc`, etc.):

```bash
alias dup='devtool'          # Short and sweet
alias update-all='devtool'   # Descriptive
```

### Notification on completion

On macOS:
```bash
devtool && osascript -e 'display notification "Updates completed" with title "devtool"'
```

On Linux (with notify-send):
```bash
devtool && notify-send "devtool" "Updates completed"
```

### Conditional updates

Only update if it's been more than 24 hours:

```bash
#!/bin/bash
CACHE_FILE="$HOME/.cache/devtool/last_update"
NOW=$(date +%s)

if [ -f "$CACHE_FILE" ]; then
    LAST=$(cat "$CACHE_FILE")
    AGE=$((NOW - LAST))
    
    if [ $AGE -lt 86400 ]; then
        echo "Updated recently, skipping"
        exit 0
    fi
fi

devtool && echo $NOW > "$CACHE_FILE"
```

### Integration with other tools

Use in Makefiles:
```makefile
.PHONY: update-tools
update-tools:
	devtool
	
.PHONY: dev-setup
dev-setup: update-tools
	npm install
	bundle install
```

Use in Docker:
```dockerfile
FROM rust:latest
RUN cargo install devtool
RUN devtool
```

## FAQ

**Q: Does devtool work on Windows?**  
A: Partial support. Rustup and Mise work, but Homebrew is not available on Windows. We plan to add winget support.

**Q: Can I customize which tools to update?**  
A: Currently, devtool updates all detected tools. Configuration file support is planned for v0.5.0.

**Q: Is it safe to run devtool?**  
A: Yes! devtool only calls the standard update commands for each tool. Use `--dry-run` to see exactly what will be executed.

**Q: How long does it take?**  
A: Typically 30-90 seconds depending on how many updates are available and your internet speed.

**Q: Can I run it in parallel?**  
A: Not yet, but parallel execution is planned for v0.5.0, which will significantly reduce update time.

**Q: Does it update system packages (apt, yum, etc.)?**  
A: Not yet, but system package manager support is planned for v0.6.0.

---

**Need more help?** Open an issue on [GitHub](https://github.com/jenkinpan/devtool-rs/issues)!