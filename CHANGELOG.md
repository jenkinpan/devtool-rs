# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.13] - 2025-10-19

### Added
- 进度条平滑过渡功能
- 进度条状态验证机制
- 全局进度条管理
- 代码质量优化工具

### Changed
- 实现进度条平滑过渡，从 25% 到 50% 到 75% 到 100%
- 修复重复进度条显示问题
- 统一并行和顺序执行模式的进度条管理
- 优化进度条显示稳定性

### Fixed
- 修复进度条重复显示问题
- 消除所有 lint 和 clippy 警告
- 修复进度条状态转换验证
- 优化进度条创建和销毁逻辑

### Improved
- 显著提升进度条显示质量
- 改进代码可维护性
- 优化用户体验
- 增强系统稳定性

## [0.8.12] - 2025-10-18

### Added
- 进度条显示样式美化
- 现代化进度条设计
- 无边框极简主义设计

### Changed
- 进度条字符从单调的 `#` 升级为美观的 `█▓▒░`
- 进一步升级为现代字符 `▰▱▰▱`
- 实现无边框现代化设计

### Improved
- 显著提升用户体验和视觉效果
- 符合现代UI设计趋势
- 提供更优秀的进度反馈
- 实现极简主义的现代化进度条

## [0.8.11] - 2025-10-18

### Fixed
- 修复编译警告和代码质量问题
- 修复重复进度条显示问题
- 移除未使用的字段和方法
- 优化代码结构和性能

### Improved
- 改进进度条管理系统
- 提升代码质量和可维护性
- 统一代码格式化
- 应用所有 Clippy 建议

### Technical Details
- 移除了 `ProgressBarManager` 中未使用的 `start_time` 和 `completion_delay` 字段
- 移除了未使用的 getter 方法
- 使用 `std::slice::from_ref` 替代 `clone()` 以提高性能
- 优化了测试代码以适配新的代码结构
- 确保所有代码符合 Rust 最佳实践

## [0.8.10] - 2025-10-18

### Added
- 用户反馈系统
- 内置反馈收集功能
- 交互式反馈收集
- 自动系统信息收集
- 结构化反馈报告生成

### Fixed
- 修复重复进度条显示问题
- 改进 Homebrew 命令执行

### Improved
- 进度条状态管理和同步
- 用户反馈收集体验

## [0.8.0] - 2025-10-18

### Added
- 扩展工具支持
- 用户反馈系统
- 改进的进度条显示

### Changed
- 优化了工具更新流程
- 改进了用户体验

## [0.7.8] - 2025-10-18

### Added
- 版本跟踪系统
- 性能优化

### Improved
- 工具更新性能
- 用户体验

## [0.7.0] - 2025-10-18

### Added
- 并行执行框架
- 多进度条支持
- Shell 补全支持
- 性能基准测试

### Improved
- 代码质量
- 错误处理
- Rustup 支持

## [0.6.0] - 2025-10-18

### Added
- 并行执行模式
- 依赖图管理
- 异步/等待架构

### Improved
- 执行性能（最高 10 倍提升）
- 进度报告
- 用户体验
