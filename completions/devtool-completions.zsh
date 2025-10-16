#compdef devtool

autoload -U is-at-least

_devtool() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_devtool_commands" \
"*::: :->devtool" \
&& ret=0
    case $state in
    (devtool)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:devtool-command-$line[1]:"
        case $line[1] in
            (update)
_arguments "${_arguments_options[@]}" : \
'--jobs=[并行任务数量限制]:JOBS:_default' \
'-n[模拟执行，不实际运行命令]' \
'--dry-run[模拟执行，不实际运行命令]' \
'-v[详细输出模式]' \
'--verbose[详细输出模式]' \
'--no-color[禁用彩色输出]' \
'--keep-logs[保留日志文件到 ~/.cache/devtool/]' \
'--parallel[并行执行更新步骤 (默认启用)]' \
'--sequential[顺序执行更新步骤 (覆盖并行模式)]' \
'--no-banner[不显示启动横幅]' \
'--compact[使用紧凑输出格式（适用于非交互环境）]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(completion)
_arguments "${_arguments_options[@]}" : \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':shell -- Shell 类型:((bash\:"Bash shell"
zsh\:"Zsh shell"
fish\:"Fish shell"
powershell\:"PowerShell"
elvish\:"Elvish shell"
nushell\:"Nushell"))' \
&& ret=0
;;
(progress-status)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_devtool__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:devtool-help-command-$line[1]:"
        case $line[1] in
            (update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(completion)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(progress-status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_devtool_commands] )) ||
_devtool_commands() {
    local commands; commands=(
'update:更新开发工具（默认命令）' \
'completion:生成 shell 补全脚本' \
'progress-status:显示进度状态' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'devtool commands' commands "$@"
}
(( $+functions[_devtool__completion_commands] )) ||
_devtool__completion_commands() {
    local commands; commands=()
    _describe -t commands 'devtool completion commands' commands "$@"
}
(( $+functions[_devtool__help_commands] )) ||
_devtool__help_commands() {
    local commands; commands=(
'update:更新开发工具（默认命令）' \
'completion:生成 shell 补全脚本' \
'progress-status:显示进度状态' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'devtool help commands' commands "$@"
}
(( $+functions[_devtool__help__completion_commands] )) ||
_devtool__help__completion_commands() {
    local commands; commands=()
    _describe -t commands 'devtool help completion commands' commands "$@"
}
(( $+functions[_devtool__help__help_commands] )) ||
_devtool__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'devtool help help commands' commands "$@"
}
(( $+functions[_devtool__help__progress-status_commands] )) ||
_devtool__help__progress-status_commands() {
    local commands; commands=()
    _describe -t commands 'devtool help progress-status commands' commands "$@"
}
(( $+functions[_devtool__help__update_commands] )) ||
_devtool__help__update_commands() {
    local commands; commands=()
    _describe -t commands 'devtool help update commands' commands "$@"
}
(( $+functions[_devtool__progress-status_commands] )) ||
_devtool__progress-status_commands() {
    local commands; commands=()
    _describe -t commands 'devtool progress-status commands' commands "$@"
}
(( $+functions[_devtool__update_commands] )) ||
_devtool__update_commands() {
    local commands; commands=()
    _describe -t commands 'devtool update commands' commands "$@"
}

if [ "$funcstack[1]" = "_devtool" ]; then
    _devtool "$@"
else
    compdef _devtool devtool
fi
