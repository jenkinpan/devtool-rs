# devtool 🚀

<!--toc:start-->

- [devtool 🚀](#devtool-🚀)
  - [✨ 特性](#特性)
  - [🛠️ 支持的工具](#🛠️-支持的工具)
  - [📦 安装](#📦-安装)
    - [从 crates.io 安装（推荐）](#从-cratesio-安装推荐)
    - [从源码安装](#从源码安装)
  - [🚀 使用](#🚀-使用)
    - [基本用法](#基本用法)
    - [命令行选项](#命令行选项)
    - [示例](#示例)
  - [🔧 故障排查](#🔧-故障排查)
    - [找不到命令](#找不到命令)
    - [权限错误](#权限错误)
    - [未检测到工具](#未检测到工具)
    - [更新失败](#更新失败)
    - [语言/区域设置问题](#语言区域设置问题)
  - [💡 技巧和窍门](#💡-技巧和窍门)
    - [创建别名](#创建别名)
    - [自动更新](#自动更新)
    - [与其他工具集成](#与其他工具集成)
    - [完成时通知](#完成时通知)
  - [❓ 常见问题](#常见问题)
  - [📖 文档](#📖-文档)
  - [🤝 贡献](#🤝-贡献)
  - [📜 许可证](#📜-许可证)
  - [🔗 链接](#🔗-链接)
  <!--toc:end-->

一个用 Rust 编写的现代化、高效的开发者工具，通过一条命令统一更新开发环境中的工具和包管理器。告别多条更新命令，让 `devtool` 帮你保持一切最新。

[![Crates.io](https://img.shields.io/crates/v/devtool.svg)](https://crates.io/crates/devtool)
[![CI](https://github.com/jenkinpan/devtool-rs/workflows/CI/badge.svg)](https://github.com/jenkinpan/devtool-rs/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![codecov](https://codecov.io/gh/jenkinpan/devtool-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/jenkinpan/devtool-rs)

[English](README.md) | 简体中文

## ✨ 特性

- **一键更新**：运行 `devtool` 自动更新多个支持的工具
- **🚀 并行执行**：使用 `--parallel` 标志实现高达 3 倍的更新速度，支持并发工具执行
- **智能检测**：自动检测已安装的工具（Homebrew、rustup、mise），跳过未安装的工具
- **📊 详细升级跟踪**：显示具体升级了什么，包含升级前后的版本信息
- **🔍 智能版本检测**：只有在实际发生升级时才进行版本比较，提升性能
- **精美的进度反馈**：终端中显示实时进度条和状态更新
- **详细日志**：每个步骤的输出都会被记录，方便排查问题
- **🗂️ 统一日志存储**：所有日志文件统一保存到 `~/Library/Caches/devtool/`（macOS）或 `~/.cache/devtool/`（Linux）
- **🔍 按工具分类**：日志文件按工具（Homebrew、Rustup、Mise）分类，使用时间戳目录组织
- **🔗 最新符号链接**：每个工具目录包含 `latest` 符号链接，指向最新的执行结果
- **执行摘要**：清晰显示哪些工具已更新、已是最新版本或更新失败
- **试运行模式**：使用 `--dry-run` 预览执行步骤而不实际运行
- **外部状态监控**：使用 `devtool progress-status` 查询实时状态，可集成到其他工具
- **多语言支持**：自动检测系统语言，显示中文或英文界面
- **版本信息**：使用 `devtool -V` 或 `devtool --version` 查看版本
- **📝 用户反馈系统**：内置反馈收集功能，使用 `devtool feedback` 命令
- **🐛 问题模板**：标准化的 GitHub Issues 模板，用于 Bug 报告和功能请求
- **📊 反馈分析**：提供反馈数据分析工具，持续改进产品

## 🛠️ 支持的工具

`devtool` 目前支持自动检测和更新以下工具：

- **Homebrew**：更新索引、升级软件包、清理旧版本
- **Rust (rustup)**：更新 `stable` Rust 工具链
- **Mise**（原 rtx）：运行 `mise up` 更新所有管理的语言和工具（Node.js、Python 等）

## 📦 安装

### 从 crates.io 安装（推荐）

```bash
cargo install devtool
```

确保 `~/.cargo/bin` 在你的 `PATH` 环境变量中。

### 从源码安装

```bash
git clone https://github.com/jenkinpan/devtool-rs.git
cd devtool-rs
cargo build --release
cargo install --path .
```

## 🚀 使用

### 基本用法

在终端中运行 `devtool` 启动更新过程：

```bash
devtool
```

这等同于运行 `devtool update`。

## 📊 升级详情跟踪

`devtool` 现在提供详细的升级跟踪功能，显示具体升级了什么以及升级前后的版本信息：

### 升级详情显示

当发生升级时，`devtool` 会显示详细的变更信息：

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

### 智能版本检测

- **性能优化**：只有在实际发生升级时才进行版本比较
- **准确跟踪**：区分索引更新和实际软件包升级
- **统一格式**：所有工具（Homebrew、Rustup、Mise）使用一致的升级详情格式
- **多格式支持**：同时支持 JSON 和文本格式的升级详情

### 支持的升级类型

- **版本升级**：`软件包: 旧版本 → 新版本`
- **新安装**：`软件包: 新安装 → 版本`
- **工具链更新**：显示 Rust 工具链版本变化
- **工具更新**：显示 Mise 管理的工具版本变化

### 并行执行（v0.6.0 新功能）

为了更快的更新速度，使用新的并行执行模式：

```bash
# 启用并行执行，默认 4 个并发任务
devtool --parallel

# 控制并发任务数量
devtool --parallel --jobs 2

# 并行执行试运行
devtool --parallel --jobs 3 --dry-run
```

**性能优势：**
- 并行模式下执行速度提升高达 3 倍
- 通过 `--jobs` 参数可配置并发数
- 智能依赖管理
- 保持顺序模式的所有安全特性

### 命令行选项

使用以下选项自定义 `devtool` 的行为：

| 选项          | 简写 | 描述                                                   |
| ------------- | ---- | ------------------------------------------------------ |
| `--dry-run`   | `-n` | 显示将要执行的步骤但不实际运行                         |
| `--verbose`   | `-v` | 在执行过程中打印详细输出                               |
| `--version`   | `-V` | 显示版本信息                                           |
| `--keep-logs` |      | 保留每个步骤的日志文件，统一存储在缓存目录中 |
| `--no-banner` |      | 不显示启动横幅                                         |
| `--compact`   |      | 在非交互式环境中使用更紧凑的输出格式                   |
| `--parallel`  |      | 并行执行更新步骤（v0.6.0 新功能）                     |
| `--jobs`      |      | 并行任务数量限制（默认：4）                           |
| `--no-color`  |      | 禁用彩色输出                                           |
| `--help`      | `-h` | 显示帮助信息                                           |

### 示例

**标准更新：**

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

**试运行：**

```bash
devtool --dry-run
```

**用户反馈：**

```bash
# 交互式反馈收集
devtool feedback

# 直接提交反馈
devtool feedback --type bug --message "发现 Homebrew 更新问题" --verbose

# 功能请求
devtool feedback --type feature --message "添加 npm 更新支持"
```

**查看版本：**

```bash
devtool -V
# 输出: devtool 0.4.0
```

**监控进度（用于脚本或状态栏）：**

在一个终端运行 `devtool`，在另一个终端查询进度：

```bash
devtool progress-status
# 输出: Progress status: ProgressStatus { state: "update", percent: Some(60), done: Some(3), total: Some(5), desc: Some("Action: Cleanup old versions"), ts: Some("2025-10-04T13:17:45.123+08:00") }
```

**语言支持：**

工具会自动检测你的系统语言，并相应地显示中文或英文界面。

## 📁 日志存储系统

`devtool` 提供了全面的日志存储系统，便于故障排查和调试：

### 统一日志存储

所有日志文件统一保存到以下位置：
- **macOS**: `~/Library/Caches/devtool/`
- **Linux**: `~/.cache/devtool/`

### 目录结构

日志按工具分类，使用时间戳目录组织：

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

### 日志文件类型

每个工具生成特定的日志文件：

- **Homebrew**:
  - `brew_update.log`: Homebrew 更新输出
  - `brew_upgrade.log`: Homebrew 升级输出
  - `brew_cleanup.log`: Homebrew 清理输出
  - `brew_outdated.log`: 过时软件包检测
  - `brew_detailed_debug.log`: 详细调试信息
  - `outdated_packages.json`: 过时软件包的 JSON 格式

- **Rustup**:
  - `rustup_update.log`: Rustup 更新输出

- **Mise**:
  - `mise_up.log`: Mise 更新输出

### 使用日志进行故障排查

1. **访问最新日志**：使用 `latest` 符号链接查看最新执行结果
   ```bash
   ls ~/Library/Caches/devtool/homebrew/latest/
   ```

2. **查看特定日志文件**：检查单个日志文件获取详细信息
   ```bash
   cat ~/Library/Caches/devtool/homebrew/latest/brew_detailed_debug.log
   ```

3. **历史日志**：浏览时间戳目录查看之前的执行记录
   ```bash
   ls ~/Library/Caches/devtool/homebrew/
   ```

### 启用日志存储

使用 `--keep-logs` 标志启用日志存储：

```bash
devtool update --keep-logs
```

这将把所有日志文件保存到统一缓存目录中，供后续分析使用。

## 🔧 故障排查

### 找不到命令

如果安装后出现 `command not found: devtool`：

```bash
# 检查 ~/.cargo/bin 是否在 PATH 中
echo $PATH

# 添加到 shell 配置文件（例如 ~/.bashrc、~/.zshrc）
export PATH="$HOME/.cargo/bin:$PATH"

# 重新加载 shell 配置
source ~/.bashrc  # 或 source ~/.zshrc
```

### 权限错误

如果遇到权限错误：

```bash
# 确保二进制文件可执行（Unix/Linux/macOS）
chmod +x ~/.cargo/bin/devtool

# 在 macOS 上，如果被 Gatekeeper 阻止：
xattr -d com.apple.quarantine ~/.cargo/bin/devtool
```

### 未检测到工具

`devtool` 只更新已安装的工具。开始使用前请安装：

- 安装 [Homebrew](https://brew.sh)（macOS/Linux）
- 安装 [Rustup](https://rustup.rs)（所有平台）
- 安装 [Mise](https://mise.jdx.dev)（所有平台）

### 更新失败

如果更新持续失败：

1. 尝试手动运行工具的更新命令查看实际错误
2. 检查网络连接
3. 验证磁盘空间是否充足
4. 使用 `--verbose` 标志获取详细输出：`devtool --verbose`
5. 检查 `~/.cache/devtool/logs/` 中的日志文件获取详细错误信息

### 语言/区域设置问题

如果自动语言检测不起作用，强制使用英文输出：

```bash
LANG=en_US.UTF-8 devtool
```

强制使用中文输出：

```bash
LANG=zh_CN.UTF-8 devtool
```

## 💡 技巧和窍门

### 创建别名

添加到 shell 配置文件以快速访问：

```bash
alias dup='devtool'
alias update-dev='devtool'
```

### 自动更新

**使用 cron（Linux/macOS）：**

```bash
# 编辑 crontab
crontab -e

# 添加此行以每天早上 9 点运行
0 9 * * * /home/username/.cargo/bin/devtool
```

**使用 launchd（macOS）：**

创建 `~/Library/LaunchAgents/com.devtool.update.plist`：

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

然后加载：`launchctl load ~/Library/LaunchAgents/com.devtool.update.plist`

### 与其他工具集成

**在 Makefile 中：**

```makefile
.PHONY: update-tools
update-tools:
 devtool

.PHONY: dev-setup
dev-setup: update-tools
 npm install
 bundle install
```

**在脚本中：**

```bash
#!/bin/bash
# 安全更新脚本
if devtool --dry-run; then
    echo "试运行成功，继续执行..."
    devtool
else
    echo "试运行失败，跳过更新"
    exit 1
fi
```

### 完成时通知

**macOS：**

```bash
devtool && osascript -e 'display notification "更新完成" with title "devtool"'
```

**Linux（使用 notify-send）：**

```bash
devtool && notify-send "devtool" "更新完成"
```

## ❓ 常见问题

**问：devtool 支持 Windows 吗？**  
答：部分支持。Rustup 和 Mise 可以工作，但 Homebrew 在 Windows 上不可用。我们计划在未来添加 winget 支持。

**问：运行 devtool 安全吗？**  
答：是的！devtool 只调用每个工具的标准更新命令。使用 `--dry-run` 可以在运行前查看将要执行的内容。

**问：需要多长时间？**  
答：通常 30-90 秒，取决于可用更新的数量和网络速度。

**问：可以并行运行吗？**  
答：是的！v0.6.0 已经支持并行执行。使用 `--parallel` 标志可以同时运行多个工具更新，速度提升高达 3 倍。使用 `--jobs` 参数可以控制并发任务数量。

**问：它会更新系统软件包（apt、yum 等）吗？**  
答：目前还不行，但系统包管理器支持计划在 v0.6.0 中推出。

**问：日志文件存储在哪里？**  
答：日志文件存储在 `~/.cache/devtool/logs/`（Linux/macOS）或 `%LOCALAPPDATA%\devtool\logs\`（Windows）。

**问：如何报告错误？**  
答：请在 GitHub 上[提交 issue](https://github.com/jenkinpan/devtool-rs/issues/new?template=bug_report.md)，并提供环境和错误的详细信息。

**问：如何贡献？**  
答：请查看我们的 [CONTRIBUTING.md](CONTRIBUTING.md) 指南！我们欢迎错误报告、功能请求和拉取请求。

## 📖 文档

- [架构概述](ARCHITECTURE.md) - 系统设计和模块结构
- [贡献指南](CONTRIBUTING.md) - 如何为项目做贡献
- [路线图](ROADMAP.md) - 未来开发计划
- [变更日志](CHANGELOG.md) - 版本历史和发布说明

## 🤝 贡献

欢迎贡献！请随时在 GitHub 仓库提交 issue 和拉取请求。

查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细指南：

- 报告错误
- 建议功能
- 提交拉取请求
- 开发环境设置
- 添加新的包管理器

## 📜 许可证

本项目采用 Apache License 2.0 许可证。

## 🔗 链接

- [Crates.io](https://crates.io/crates/devtool)
- [GitHub 仓库](https://github.com/jenkinpan/devtool-rs)
- [文档](https://docs.rs/devtool)
- [报告问题](https://github.com/jenkinpan/devtool-rs/issues)
- [讨论区](https://github.com/jenkinpan/devtool-rs/discussions)

