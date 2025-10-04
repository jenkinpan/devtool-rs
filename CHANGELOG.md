# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1] - 2024-01-XX

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

## [0.4.0] - 2024-01-XX

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

## [0.3.5] - Previous Release

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

## [0.3.0] - Initial Stable Release

### Added
- Initial release of devtool-rs
- Basic update functionality for development tools
- Command-line interface with Clap
- Colored output support

---

## Upgrade Notes

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

[Unreleased]: https://github.com/jenkinpan/devtool-rs/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/jenkinpan/devtool-rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/jenkinpan/devtool-rs/compare/v0.3.5...v0.4.0
[0.3.5]: https://github.com/jenkinpan/devtool-rs/compare/v0.3.0...v0.3.5
[0.3.0]: https://github.com/jenkinpan/devtool-rs/releases/tag/v0.3.0