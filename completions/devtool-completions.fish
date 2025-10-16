# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_devtool_global_optspecs
	string join \n h/help V/version
end

function __fish_devtool_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_devtool_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_devtool_using_subcommand
	set -l cmd (__fish_devtool_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c devtool -n "__fish_devtool_needs_command" -s h -l help -d 'Print help'
complete -c devtool -n "__fish_devtool_needs_command" -s V -l version -d 'Print version'
complete -c devtool -n "__fish_devtool_needs_command" -f -a "update" -d '更新开发工具（默认命令）'
complete -c devtool -n "__fish_devtool_needs_command" -f -a "completion" -d '生成 shell 补全脚本'
complete -c devtool -n "__fish_devtool_needs_command" -f -a "progress-status" -d '显示进度状态'
complete -c devtool -n "__fish_devtool_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c devtool -n "__fish_devtool_using_subcommand update" -l jobs -d '并行任务数量限制' -r
complete -c devtool -n "__fish_devtool_using_subcommand update" -s n -l dry-run -d '模拟执行，不实际运行命令'
complete -c devtool -n "__fish_devtool_using_subcommand update" -s v -l verbose -d '详细输出模式'
complete -c devtool -n "__fish_devtool_using_subcommand update" -l no-color -d '禁用彩色输出'
complete -c devtool -n "__fish_devtool_using_subcommand update" -l keep-logs -d '保留日志文件到 ~/.cache/devtool/'
complete -c devtool -n "__fish_devtool_using_subcommand update" -l parallel -d '并行执行更新步骤 (默认启用)'
complete -c devtool -n "__fish_devtool_using_subcommand update" -l sequential -d '顺序执行更新步骤 (覆盖并行模式)'
complete -c devtool -n "__fish_devtool_using_subcommand update" -l no-banner -d '不显示启动横幅'
complete -c devtool -n "__fish_devtool_using_subcommand update" -l compact -d '使用紧凑输出格式（适用于非交互环境）'
complete -c devtool -n "__fish_devtool_using_subcommand update" -s h -l help -d 'Print help'
complete -c devtool -n "__fish_devtool_using_subcommand completion" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c devtool -n "__fish_devtool_using_subcommand progress-status" -s h -l help -d 'Print help'
complete -c devtool -n "__fish_devtool_using_subcommand help; and not __fish_seen_subcommand_from update completion progress-status help" -f -a "update" -d '更新开发工具（默认命令）'
complete -c devtool -n "__fish_devtool_using_subcommand help; and not __fish_seen_subcommand_from update completion progress-status help" -f -a "completion" -d '生成 shell 补全脚本'
complete -c devtool -n "__fish_devtool_using_subcommand help; and not __fish_seen_subcommand_from update completion progress-status help" -f -a "progress-status" -d '显示进度状态'
complete -c devtool -n "__fish_devtool_using_subcommand help; and not __fish_seen_subcommand_from update completion progress-status help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
