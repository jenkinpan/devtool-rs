module completions {
  # A CLI tool for development in update rustup toolchain, mise maintained tools and homebrew packages.
  export extern devtool [
    --help (-h) # Print help
    --version (-V) # Print version
    ...args: string
  ]

  # upgrade development tools (default command)
  export extern "devtool update" [
    --dry-run (-n) # 模拟执行，不实际运行命令 dry-run
    --verbose (-v) # output more details
    --no-color # no color output
    --keep-logs # keep logs at ~/.cache/devtool/
    --parallel # 并行执行更新步骤（尚未实现）
    --no-banner # 不显示启动横幅
    --compact # 使用紧凑输出格式（适用于非交互环境）
    --help (-h) # Print help
    ...args: string
  ]

  def "nu-complete devtool completion shell" [] {
    ["bash" "zsh" "fish" "powershell" "elvish" "nushell"]
  }

  # generate shell completion scripts
  export extern "devtool completion" [
    shell: string@"nu-complete devtool completion shell"
    --help (-h) # Print help (see more with '--help')
    ...args: string
  ]

  # show progress status
  export extern "devtool progress-status" [
    --help (-h) # Print help
    ...args: string
  ]
}

use completions *
