# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0] - 2025-10-18

### Added
- **用户反馈系统**: 实现了完整的用户反馈收集和分析系统
- **内置反馈命令**: 新增 `devtool feedback` 命令，支持交互式反馈收集
- **GitHub Issues 模板**: 创建了标准化的 Bug 报告和功能请求模板
- **反馈分析工具**: 提供了反馈数据分析和可视化工具
- **多语言支持**: 反馈系统支持英文界面，提升国际化体验

### Improved
- **用户体验**: 通过反馈系统持续改进用户体验
- **问题跟踪**: 建立了系统化的问题跟踪和解决机制
- **社区参与**: 鼓励用户参与产品改进和功能建议
- **文档完善**: 更新了相关文档和帮助信息

### Technical Details
- **反馈命令**: 实现了 `devtool feedback` 命令，支持多种反馈类型
- **系统信息收集**: 自动收集操作系统、版本等环境信息
- **反馈报告生成**: 自动生成结构化的反馈报告文件
- **GitHub 集成**: 与 GitHub Issues 和 Discussions 集成
- **分析工具**: 提供了 Python 和 Shell 脚本进行反馈分析

## [0.7.8] - 2025-10-18

### Added
- **版本跟踪系统**: 实现了标准化的升级详情跟踪系统
- **智能版本检测**: 只有在有实际升级时才进行版本比较，提升性能
- **统一升级格式**: 所有工具（Homebrew、Rustup、Mise）使用统一的升级详情格式
- **JSON + 文本格式**: 同时支持 JSON 和文本格式的升级详情文件

### Improved
- **性能优化**: 显著提升编译性能（15.6% 提升）
- **代码质量**: 清理未使用代码，减少警告数量（81.8% 减少）
- **版本跟踪准确性**: 改进了所有工具的版本变化检测逻辑
- **文件操作优化**: 简化了文件保存和错误处理逻辑

### Technical Details
- **条件版本检查**: 只有在检测到实际升级时才进行版本比较
- **统一数据结构**: 使用 `UpgradeDetail` 和 `UpgradeDetails` 结构体
- **性能优化**: 移除了调试输出和未使用的模块
- **代码清理**: 删除了 4 个未使用的模块文件

## [0.7.73] - 2025-10-18

### Added
- **版本跟踪系统**: 实现了标准化的升级详情跟踪系统
- **智能版本检测**: 只有在有实际升级时才进行版本比较，提升性能
- **统一升级格式**: 所有工具（Homebrew、Rustup、Mise）使用统一的升级详情格式
- **JSON + 文本格式**: 同时支持 JSON 和文本格式的升级详情文件

### Improved
- **性能优化**: 显著提升编译性能（15.6% 提升）
- **代码质量**: 清理未使用代码，减少警告数量（81.8% 减少）
- **版本跟踪准确性**: 改进了所有工具的版本变化检测逻辑
- **文件操作优化**: 简化了文件保存和错误处理逻辑

### Technical Details
- **条件版本检查**: 只有在检测到实际升级时才进行版本比较
- **统一数据结构**: 使用 `UpgradeDetail` 和 `UpgradeDetails` 结构体
- **性能优化**: 移除了调试输出和未使用的模块
- **代码清理**: 删除了 4 个未使用的模块文件

## [0.7.72] - 2025-10-17

### Fixed
- **升级详情显示逻辑修复**: 修复了升级详情不显示的问题
- **状态判断优化**: 根据升级详情文件判断是否有真正的升级
- **显示逻辑改进**: 区分索引更新和软件包升级，避免显示"已更新"但没有详情的情况

### Technical Details
- **精确判断逻辑**: 只有存在升级详情文件时才显示为"已更新"
- **状态区分**: 索引更新显示为"已是最新"，软件包升级显示为"已更新"
- **用户体验优化**: 避免了"显示更新但没有详情"的困惑

## [0.7.71] - 2025-10-17

### Fixed
- **Homebrew JSON 解析修复**: 修复了 `brew outdated --json` 输出格式不匹配的问题
- **版本跟踪系统优化**: 改进了 Homebrew 版本对比逻辑，确保准确的升级检测
- **JSON 结构体更新**: 更新了 `OutdatedPackage` 结构体以匹配实际的 Homebrew JSON 输出格式

### Technical Details
- **JSON 格式兼容**: 修复了 `installed_versions` 字段的解析问题
- **版本对比优化**: 改进了升级前后的版本对比逻辑
- **错误处理增强**: 更好的 JSON 解析错误处理

## [0.7.7] - 2025-10-17

### Enhanced
- **Comprehensive Version Tracking**: Implemented robust version tracking for all tools
  - **Homebrew**: Enhanced with `brew outdated --json` pre-check and version comparison
  - **Rustup**: Improved toolchain version tracking with JSON-based version recording
  - **Mise**: Enhanced tool version tracking with structured version comparison
  - All tools now use consistent version tracking methodology for accurate upgrade detection

### Technical Details
- **Homebrew Optimization**: Uses `brew outdated --json` to pre-check for outdated packages and compare versions
- **Rustup Enhancement**: JSON-based toolchain version recording and comparison
- **Mise Improvement**: Structured tool version tracking with JSON serialization
- **Universal Approach**: All tools now use the same reliable version tracking pattern
- **Better Accuracy**: Eliminates false positives and ensures only actual upgrades are reported

## [0.7.6] - 2025-10-17

### Enhanced
- **Upgrade Details Display**: Comprehensive upgrade detail collection and display for all tools
  - Enhanced upgrade detail detection logic to work with all tools (Homebrew, Rustup, Mise)
  - Improved upgrade detail collection for both parallel and sequential execution modes
  - Optimized Rustup upgrade detail collection with better version change detection
  - Enhanced Mise upgrade detail collection with improved version comparison logic
  - Now correctly displays specific version changes for all tools in unified format

### Technical Details
- **Universal Upgrade Detection**: Removed dependency on specific keywords, now relies on upgrade detail file existence
- **Rustup Optimization**: Improved toolchain version change detection and upgrade detail saving
- **Mise Enhancement**: Better version change detection and output parsing
- **Display Consistency**: Unified upgrade detail display format across all tools

## [0.7.5] - 2025-10-17

### Fixed
- **Upgrade Details Display**: Enhanced upgrade detail collection and display functionality
  - Improved upgrade detail detection logic to support both "updated" and "changed" status
  - Enhanced output parsing to detect upgrade patterns with "->" symbols
  - Optimized `parse_upgrade_line` function to handle tap format package names (e.g., "jenkinpan/tap/devtool")
  - Fixed upgrade detail collection for both parallel and sequential execution modes
  - Now correctly displays specific version changes in format: `package: old_version → new_version`

### Technical Details
- **Upgrade Detection**: Extended detection conditions for upgrade status
- **Output Parsing**: Improved parsing of brew upgrade output with arrow symbols
- **Package Name Handling**: Better support for tap format package names
- **Display Format**: Consistent upgrade detail display format

## [0.7.4] - 2025-10-17

### Fixed
- **Progress Bar Display**: Fixed progress bar duplication and display conflicts
  - Reduced refresh frequency from 1000ms to 2000ms to prevent display conflicts
  - Increased completion delay from 200ms to 500ms to ensure proper state updates
  - Improved progress bar stability during long-running operations
- **Upgrade Information Display**: Enhanced upgrade detail collection and display
  - Added `read_upgrade_details()` function to collect specific package version changes
  - Implemented upgrade detail collection for both parallel and sequential execution modes
  - Enhanced upgrade information display with specific version changes (e.g., `nginx: 1.21.0 → 1.22.0`)
  - Improved user experience with detailed upgrade feedback

### Technical Details
- **Progress Bar Optimization**: Better refresh mechanism to prevent display conflicts
- **Version Information Collection**: Enhanced upgrade detail collection for Homebrew, Rustup, and Mise
- **Display Improvements**: More stable progress indication and detailed upgrade reporting

## [0.7.3] - 2025-10-17

### Fixed
- **Progress Bar Display**: Fixed progress bar duplication and display conflicts
  - Reduced refresh frequency from 1000ms to 2000ms to prevent display conflicts
  - Increased completion delay from 200ms to 500ms to ensure proper state updates
  - Improved progress bar stability during long-running operations
- **Upgrade Information Display**: Enhanced upgrade detail collection and display
  - Added `read_upgrade_details()` function to collect specific package version changes
  - Implemented upgrade detail collection for both parallel and sequential execution modes
  - Enhanced upgrade information display with specific version changes (e.g., `nginx: 1.21.0 → 1.22.0`)
  - Improved user experience with detailed upgrade feedback

### Technical Details
- **Progress Bar Optimization**: Better refresh mechanism to prevent display conflicts
- **Version Information Collection**: Enhanced upgrade detail collection for Homebrew, Rustup, and Mise
- **Display Improvements**: More stable progress indication and detailed upgrade reporting

## [0.7.2] - 2025-10-16

### Added
- **Version Information**: Added `-V` and `--version` parameters to display version information
  - Users can now check the current version with `devtool -V` or `devtool --version`
  - Version information is automatically retrieved from Cargo.toml
  - Follows standard CLI conventions for version display

## [0.7.1] - 2025-10-16

### Fixed
- **CLI Help Information**: Fixed help command display issues
  - Corrected incomplete help information display
  - Updated CLI about description to English for better internationalization
  - Improved grammar in help text ("updating" instead of "development in update")
  - Maintained Chinese command descriptions for localized user experience

## [0.7.0] - 2025-10-16

### Added
- **Enhanced Progress Reporting**: Multi-progress bar support using indicatif
  - Real-time progress updates for all running tasks
  - Individual task progress tracking with elapsed time
  - Detailed tool descriptions in progress display
  - Support for both parallel and sequential execution modes
- **Performance Benchmarks**: Comprehensive benchmark suite for parallel vs sequential execution
  - Performance validation with up to 10x speed improvements
  - Memory usage patterns and optimization
  - Benchmark coverage for different task counts and durations
- **Enhanced Rustup Support**: Support for all installed toolchains
  - Updates all toolchains (stable, nightly, beta) instead of just stable
  - Detailed version change reporting for each toolchain
  - Better detection of toolchain updates and changes
- **Updated Shell Completions**: All completion files updated with new parameters
  - Added `--sequential` flag support
  - Added `--jobs` parameter support
  - Updated descriptions for all new features

### Changed
- **Default Behavior**: Parallel execution is now enabled by default
  - Default concurrency level set to 3 jobs
  - `--sequential` flag available to override parallel mode
  - Improved user experience with faster default execution
- **Progress Display**: Replaced custom progress bars with indicatif
  - More reliable progress tracking
  - Better visual feedback with real-time updates
  - Consistent progress display across all execution modes

### Fixed
- **Code Quality**: Zero Clippy warnings with comprehensive linting
  - Removed unused code and modules
  - Fixed all compiler warnings
  - Improved error handling and reporting
- **Success/Failure Reporting**: Enhanced accuracy of tool status reporting
  - Better distinction between 'updated' and 'already latest' states
  - More descriptive output messages
  - Improved result classification logic

### Technical Details
- **Dependencies**: Added `indicatif = "0.18"` for progress bars
- **Performance**: Up to 10x faster execution with parallel mode
- **Code Quality**: Comprehensive cleanup of unused code and modules
- **Testing**: Enhanced test coverage with performance benchmarks

## [0.6.1] - 2025-10-16

### Fixed
- **Improved Success/Failure Reporting Logic**: Enhanced accuracy of tool status reporting
  - Fixed tools incorrectly showing as 'failed' when they were actually up-to-date
  - Distinguish between 'updated' and 'already latest' states for better user feedback
  - Updated output messages to be more descriptive:
    - `"Homebrew updated"` vs `"Homebrew already latest"`
    - `"Rustup updated"` vs `"Rustup already latest"`
    - `"Mise updated"` vs `"Mise already latest"`
  - Fixed result classification logic to properly categorize tools based on actual changes
  - Maintained success status for both updated and unchanged tools
  - Consistent behavior in both sequential and parallel execution modes

### Technical Details
- Enhanced `execute_tool_update` function to detect actual changes vs no-changes
- Improved result processing logic to correctly classify tool outcomes
- Better user experience with accurate status reporting
- Fixed misleading 'updated' status for tools that were already latest

## [0.6.0] - 2025-10-16

### Added
- **Parallel Execution Framework**: Revolutionary parallel execution system for faster updates
  - New `--parallel` flag to enable concurrent tool updates
  - `--jobs <N>` parameter to control maximum concurrent tasks (default: 4)
  - Intelligent dependency management with `DependencyGraph` for optimal execution order
  - Async/await architecture using Tokio runtime for high-performance execution
  - Comprehensive error handling and result aggregation for parallel tasks
  - Support for both parallel and sequential execution modes
- **Advanced Task Scheduling**: Smart task management system
  - `ParallelScheduler` with configurable concurrency limits
  - Resource-aware scheduling to prevent system overload
  - Automatic dependency resolution for complex update sequences
  - Graceful error handling with detailed task result reporting
- **Enhanced Performance**: Significant speed improvements
  - Up to 3x faster execution with parallel mode
  - Efficient resource utilization with configurable job limits
  - Optimized memory usage for large-scale updates
  - Background task execution with progress tracking

### Changed
- **Architecture**: Complete refactoring to support async execution
  - Main function converted to `#[tokio::main]` for async runtime
  - Tool execution logic abstracted into `execute_tool_update` function
  - Parallel and sequential execution paths with unified result handling
  - Improved error handling with detailed task result reporting
- **Dependencies**: Added async runtime support
  - Added `tokio = { version = "1.0", features = ["full"] }` for async execution
  - Maintained backward compatibility with existing functionality
  - Enhanced CLI with new parallel execution options

### Technical Details
- **New Modules**: Added `src/parallel/mod.rs` with comprehensive parallel execution framework
- **Tool Management**: `Tool` enum with `Homebrew`, `Rustup`, and `Mise` variants
- **Dependency System**: `DependencyGraph` for managing tool update dependencies
- **Task Results**: `TaskResult` struct for detailed execution reporting
- **Scheduler**: `ParallelScheduler` for managing concurrent task execution

## [0.5.6] - 2025-10-08

### Added
- **Shell completion support**: Added comprehensive shell completion generation for multiple shells
  - Support for bash, zsh, fish, powershell, elvish, and nushell
  - New `devtool completion <shell>` command to generate completion scripts
  - Chinese descriptions in completion scripts for better user experience
  - Complete documentation for installation and usage of completion scripts
- **Nushell support**: Added dedicated support for nushell completion generation
  - Uses `clap_complete_nushell` crate for modern nushell completion format
  - Generates `.nu` files compatible with nushell's module system
  - Includes intelligent completion suggestions and parameter descriptions

### Changed
- **CLI structure**: Refactored CLI to use subcommands for better organization
  - `devtool update` - Main update functionality (default behavior)
  - `devtool completion <shell>` - Generate shell completion scripts
  - `devtool progress-status` - Check update progress
- **Dependencies**: Updated and added new dependencies
  - Added `clap_complete = "4.5"` for shell completion generation
  - Added `clap_complete_nushell = "4.5"` for nushell support
  - Updated `criterion = "0.7"` for benchmarking

### Fixed
- **Compiler warnings**: Resolved all unused variable warnings
  - Marked `parallel` and `compact` parameters as intentionally unused with underscore prefix
  - Clean compilation with no warnings

### Documentation
- **Completion guide**: Added comprehensive `COMPLETIONS.md` documentation
  - Installation instructions for all supported shells
  - Usage examples and configuration guides
  - Nushell-specific setup instructions
  - Chinese language support in completion scripts

## [0.4.1] - 2025-10-04

### Added
- Chinese README (`README_zh.md`) for better support of Chinese-speaking users
- Language switcher links in both English and Chinese README files
- Comprehensive troubleshooting section in README
- FAQ section with 10 common questions
- Tips & tricks section for automation and integration examples

### Changed
- **Major documentation cleanup**: Simplified and reorganized project documentation
  - Removed `docs/` directory and consolidated content into main documentation files
  - Simplified `ARCHITECTURE.md` from 13K to 3.8K (removed excessive implementation details)
  - Expanded `README.md` from 5.2K to 10K with practical user guidance
  - Removed internal work summary and premature performance documentation
  - Reduced total documentation size by ~32K (-51%) while improving clarity
- Better documentation structure now matches project size (~1,500 LOC)

### Fixed
- Replaced unmaintained `atty` dependency with standard library's `IsTerminal` trait
- Fixed CI security-audit task permission issues
- Resolved cargo-audit warnings (RUSTSEC-2024-0375, RUSTSEC-2021-0145)

### Internal
- Improved CI/CD pipeline reliability
- Enhanced release automation with multi-platform builds
- Added criterion as dev-dependency for performance benchmarking

## [0.4.0] - 2025-10-04

### Changed
- **Major refactoring**: Restructured the entire codebase into modular components for better maintainability
  - Split monolithic `main.rs` (~1,299 lines) into focused modules
  - Organized code into logical modules: `cli`, `i18n`, `ui`, `progress`, `runner`, `commands`, `utils`
  - Each tool (Homebrew, Rustup, Mise) now has its own dedicated module
  - Improved code reusability and testability

### Added
- Comprehensive test suite with 33+ unit tests covering:
  - CLI argument parsing
  - Internationalization (i18n)
  - UI output functions
  - Command runner
  - Individual tool update logic
- Comprehensive CI/CD pipeline with GitHub Actions
  - Multi-platform testing (Ubuntu, macOS, Windows)
  - Code quality checks: clippy, rustfmt, documentation
  - Security audit with cargo-audit
  - Code coverage tracking with Codecov
- Enhanced release workflow
  - Automatic multi-platform binary builds (Linux, macOS, Windows)
  - Automatic GitHub Release creation with changelog
  - Automated crates.io publishing
- Project documentation
  - CHANGELOG.md following Keep a Changelog format
  - CONTRIBUTING.md with development guidelines
  - ROADMAP.md outlining future development plans
  - ARCHITECTURE.md describing system design
  - Issue templates for bugs and feature requests
  - Pull request template for contributions
- Performance benchmarking framework with Criterion
  - Command execution benchmarks
  - I/O operation benchmarks
  - String operation benchmarks

### Fixed
- String escaping issues in localization module
- Clap version flag conflicts
- Clippy warnings: removed unnecessary borrows

### Internal
- Improved error handling across all modules
- Better separation of concerns
- Enhanced code documentation
- Prepared foundation for future parallel execution support

## [0.3.5] - 2025-10-04

### Features
- Support for updating Homebrew packages
- Support for updating Rustup toolchains
- Support for updating Mise-managed tools
- Dry-run mode (`--dry-run`) for testing without making changes
- Progress status command for checking ongoing updates
- Multi-language support (English and Chinese)
- Colored terminal output
- Progress bar for long-running operations

### Commands
- `devtool homebrew` - Update Homebrew packages
- `devtool rustup` - Update Rust toolchains
- `devtool mise` - Update Mise tools
- `devtool progress-status` - Check progress of updates

### Options
- `--dry-run` - Show what would be updated without actually updating
- `--force` - Force update even if already up-to-date
- `--verbose` - Show detailed output
- `--help` - Show help information
- `--version` - Show version information

## [0.3.0] - 2025-10-03

### Added
- Initial release of devtool-rs
- Basic update functionality for development tools
- Command-line interface with Clap
- Colored output support

---

## Upgrade Notes

### 0.4.1 → 0.5.6

This is a feature release that adds shell completion support while maintaining full backward compatibility. All existing functionality works exactly as before.

**New Features:**
- Shell completion generation for 6 different shells (bash, zsh, fish, powershell, elvish, nushell)
- New `devtool completion <shell>` command
- Comprehensive completion documentation

**Breaking Changes:**
- None. All existing commands and options work exactly as before.

**Migration Guide:**
- No action required for existing users
- Optional: Install shell completion scripts for better user experience
- See `COMPLETIONS.md` for installation instructions

### 0.4.0 → 0.4.1

This is a documentation and maintenance release. All existing functionality works exactly as before.

Changes:
- Better documentation for Chinese users
- Cleaner, more focused documentation structure
- Fixed dependency security warnings
- No breaking changes or new features

No action is required when upgrading.

### 0.3.5 → 0.4.0

This is a refactoring release that maintains full backward compatibility. All existing commands and options work exactly as before. The main changes are internal code organization improvements that will enable:

- Easier addition of new package managers
- Better testing and maintenance
- Future parallel execution support
- Improved error handling and reporting

No action is required from users upgrading from 0.3.5 to 0.4.0.

---

[Unreleased]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.72...HEAD
[0.7.72]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.71...v0.7.72
[0.7.71]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.7...v0.7.71
[0.7.7]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.6...v0.7.7
[0.7.6]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.5...v0.7.6
[0.7.5]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.4...v0.7.5
[0.7.4]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/jenkinpan/devtool-rs/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/jenkinpan/devtool-rs/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/jenkinpan/devtool-rs/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/jenkinpan/devtool-rs/compare/v0.5.6...v0.6.0
[0.5.6]: https://github.com/jenkinpan/devtool-rs/compare/v0.4.1...v0.5.6
[0.4.1]: https://github.com/jenkinpan/devtool-rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/jenkinpan/devtool-rs/compare/v0.3.5...v0.4.0
[0.3.5]: https://github.com/jenkinpan/devtool-rs/compare/v0.3.0...v0.3.5
[0.3.0]: https://github.com/jenkinpan/devtool-rs/releases/tag/v0.3.0