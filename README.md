# devtool-rs 🚀

`devtool-rs` 是一个用 Rust 编写的现代化、高效的开发者工具，旨在通过一个简单的命令，统一更新您开发环境中常用的工具链和软件包管理器。告别繁琐的多个更新命令，让 `devtool` 为您保持一切最新。

---

## ✨ 功能特性

- **一键更新**: 只需运行 `devtool`，即可自动更新多个受支持的工具。
- **智能检测**: 自动检测系统中已安装的工具（如 Homebrew, rustup, mise），并跳过未安装的。
- **清晰的进度反馈**: 在终端中提供美观的实时进度条和状态更新。
- **详细的日志记录**: 每个步骤的输出都会被记录到日志文件中，方便问题排查。
- **执行摘要**: 命令完成后，提供清晰的摘要，告知您哪些工具被更新、哪些已是最新或执行失败。
- **试运行模式 (Dry Run)**: 使用 `--dry-run` 标志可以查看将要执行的步骤，而不会实际更改任何内容。
- **外部状态监控**: 通过 `devtool progress-status` 命令，可以查询当前更新任务的实时状态，方便与其他工具（如状态栏脚本）集成。

## 🛠️ 支持的工具

`devtool` 目前支持自动检测并更新以下工具：

- **Homebrew**: 更新索引、升级软件包、清理旧版本。
- **Rust (rustup)**: 更新 `stable` 版本的 Rust 工具链。
- **Mise** (前身为 rtx): 运行 `mise up` 来更新由它管理的所有语言和工具（如 Node.js, Python 等）。

## 📦 安装

确保您的系统中已安装 Rust 和 Cargo。然后，您可以通过 Cargo 从 `crates.io` 安装 `devtool`：

```bash
cargo install devtool
```

安装完成后，请确保 `~/.cargo/bin` 目录在您的 `PATH` 环境变量中。

## 🚀 使用方法

### 基本用法

在终端中直接运行 `devtool` 即可开始更新流程：

```bash
devtool
```

这等同于运行 `devtool update`。

### 命令行选项

您可以通过以下选项自定义 `devtool` 的行为：

| 选项          | 简写 | 描述                                                                 |
| ------------- | ---- | -------------------------------------------------------------------- |
| `--dry-run`   | `-n` | 显示将要执行的步骤，但不实际运行。                                   |
| `--verbose`   | `-v` | 在执行时实时打印每个步骤的详细输出。                                 |
| `--keep-logs` |      | 保留每个步骤的日志文件，默认存放在 `~/.cache/devtool/` 目录下。      |
| `--no-banner` |      | 运行时不显示启动横幅。                                               |
| `--compact`   |      | 使用更紧凑的输出格式，适合在非交互式环境或不支持 ANSI 的终端中运行。 |
| `--parallel`  |      | (暂未实现) 并行执行更新步骤。                                        |
| `--no-color`  |      | 禁用彩色输出。                                                       |
| `--help`      | `-h` | 显示帮助信息。                                                       |

### 示例

**标准更新:**

```bash
$ devtool
🚀 开始 devtool 更新（Rust 版本）：2023-10-27 15:30:00
📋 将执行 5 个步骤：
  1) Homebrew：更新索引
  2) Homebrew：升级软件包
  3) Action：清理旧版本
  4) Rust：更新 stable 工具链
  5) Mise：更新托管工具
[========================================] 5/5 (100%) | Mise：更新托管工具

🎉 更新完成：2023-10-27 15:32:10
✅ 已更新：Homebrew：升级软件包, Mise：更新托管工具
🛠️ 已执行动作：Action：清理旧版本
⚠️ 已是最新：Homebrew：更新索引, Rust：更新 stable 工具链
🔎 Mise 简要更新：node: 20.8.0 → 20.9.0, python: 3.11.5 → 3.11.6
```

**试运行:**

```bash
 devtool --dry-run
```

**查看实时进度 (用于脚本或状态栏):**

在一个终端运行 `devtool`，在另一个终端可以查询进度：

```bash
 devtool progress-status
Progress status: ProgressStatus { state: "update", percent: Some(60), done: Some(3), total: Some(5), desc: Some("Action：清理旧版本"), ts: Some("2023-10-27T15:31:05.123+08:00") }
```

## 🤝 贡献

欢迎提交问题 (Issues) 和拉取请求 (Pull Requests)！如果您有任何想法或建议，请随时在 GitHub 仓库中提出。

## 📜 许可证

本项目采用 MIT License 和 Apache License 2.0 双重许可。
