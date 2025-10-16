module completions {

  # A CLI tool for development in update rustup toolchain, mise maintained tools and homebrew packages.
  export extern devtool [
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # 更新开发工具（默认命令）
  export extern "devtool update" [
    --dry-run(-n)             # 模拟执行，不实际运行命令
    --verbose(-v)             # 详细输出模式
    --no-color                # 禁用彩色输出
    --keep-logs               # 保留日志文件到 ~/.cache/devtool/
    --parallel                # 并行执行更新步骤 (默认启用)
    --sequential              # 顺序执行更新步骤 (覆盖并行模式)
    --jobs: string            # 并行任务数量限制
    --no-banner               # 不显示启动横幅
    --compact                 # 使用紧凑输出格式（适用于非交互环境）
    --help(-h)                # Print help
  ]

  def "nu-complete devtool completion shell" [] {
    [ "bash" "zsh" "fish" "powershell" "elvish" "nushell" ]
  }

  # 生成 shell 补全脚本
  export extern "devtool completion" [
    shell: string@"nu-complete devtool completion shell" # Shell 类型
    --help(-h)                # Print help (see more with '--help')
  ]

  # 显示进度状态
  export extern "devtool progress-status" [
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "devtool help" [
  ]

  # 更新开发工具（默认命令）
  export extern "devtool help update" [
  ]

  # 生成 shell 补全脚本
  export extern "devtool help completion" [
  ]

  # 显示进度状态
  export extern "devtool help progress-status" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "devtool help help" [
  ]

}

export use completions *
