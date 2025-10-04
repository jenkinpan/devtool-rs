# Contributing to devtool-rs

First off, thank you for considering contributing to devtool-rs! It's people like you that make devtool-rs such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by common sense and mutual respect. Be kind and considerate in your interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples** to demonstrate the steps
- **Describe the behavior you observed** and what behavior you expected
- **Include screenshots** if applicable
- **Include your environment details**: OS, Rust version, devtool version

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Use a clear and descriptive title**
- **Provide a detailed description** of the suggested enhancement
- **Explain why this enhancement would be useful** to most devtool users
- **List some examples** of how it would be used

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. Ensure the test suite passes
4. Make sure your code follows the existing code style
5. Write a good commit message

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Git

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/devtool-rs.git
cd devtool-rs

# Build the project
cargo build

# Run tests
cargo test

# Run clippy for linting
cargo clippy --all-targets --all-features -- -D warnings

# Check code formatting
cargo fmt --check
```

### Project Structure

```
devtool-rs/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli/              # CLI argument parsing
│   ├── i18n/             # Internationalization
│   ├── ui/               # User interface (colors, progress)
│   ├── runner/           # Command execution
│   ├── commands/         # Tool-specific implementations
│   │   ├── homebrew.rs   # Homebrew support
│   │   ├── rustup.rs     # Rustup support
│   │   └── mise.rs       # Mise support
│   └── utils/            # Utility functions
├── tests/                # Integration tests
└── .github/              # CI/CD workflows
```

## Coding Guidelines

### Style Guide

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Run `cargo fmt` before committing
- Use meaningful variable and function names
- Add comments for complex logic

### Testing

- Write unit tests for new functions
- Add integration tests for new features
- Ensure all tests pass before submitting PR
- Aim for good test coverage

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Documentation

- Add doc comments (`///`) for public functions and types
- Include examples in doc comments when helpful
- Update README.md if adding new features
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format

### Commit Messages

Follow these guidelines for commit messages:

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests when applicable

Examples:
```
Add support for npm package manager

- Implement npm detection
- Add update command for npm
- Add tests for npm functionality

Fixes #123
```

## Adding Support for a New Package Manager

To add support for a new package manager:

1. Create a new module in `src/commands/` (e.g., `npm.rs`)
2. Implement the detection and update logic
3. Add the module to `src/commands/mod.rs`
4. Add tests for the new functionality
5. Update documentation (README.md, CHANGELOG.md)
6. Update the help text if needed

Example structure:
```rust
use crate::runner::Runner;
use anyhow::Result;

pub fn is_installed() -> bool {
    // Check if the tool is installed
}

pub fn update(runner: &dyn Runner, dry_run: bool) -> Result<()> {
    // Implement update logic
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_installed() {
        // Your test here
    }
}
```

## Running CI Locally

Before pushing, ensure all CI checks will pass:

```bash
# Format check
cargo fmt --check

# Lint check
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --verbose

# Build release
cargo build --release

# Check documentation
cargo doc --no-deps --all-features
```

## Release Process

Releases are managed by maintainers. The process is:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release notes
3. Commit changes: `git commit -am "Release v0.x.0"`
4. Create and push tag: `git tag v0.x.0 && git push origin v0.x.0`
5. GitHub Actions will automatically:
   - Build binaries for multiple platforms
   - Create a GitHub Release
   - Publish to crates.io

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Check existing issues and discussions first

## Recognition

Contributors will be recognized in:
- GitHub contributors list
- Release notes (for significant contributions)
- Our appreciation and gratitude! 🎉

Thank you for contributing to devtool-rs! 🚀