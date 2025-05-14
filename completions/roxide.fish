# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_roxide_global_optspecs
	string join \n c/config= h/help V/version
end

function __fish_roxide_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_roxide_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_roxide_using_subcommand
	set -l cmd (__fish_roxide_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c roxide -n "__fish_roxide_needs_command" -s c -l config -d 'Sets a custom config file' -r -F
complete -c roxide -n "__fish_roxide_needs_command" -s h -l help -d 'Print help'
complete -c roxide -n "__fish_roxide_needs_command" -s V -l version -d 'Print version'
complete -c roxide -n "__fish_roxide_needs_command" -f -a "install" -d 'Installs plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
complete -c roxide -n "__fish_roxide_needs_command" -f -a "update" -d 'Updates plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
complete -c roxide -n "__fish_roxide_needs_command" -f -a "clean" -d 'Uninstalls plugins not listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
complete -c roxide -n "__fish_roxide_needs_command" -f -a "init" -d 'Sources plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
complete -c roxide -n "__fish_roxide_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c roxide -n "__fish_roxide_using_subcommand install" -s h -l help -d 'Print help'
complete -c roxide -n "__fish_roxide_using_subcommand update" -s h -l help -d 'Print help'
complete -c roxide -n "__fish_roxide_using_subcommand clean" -s h -l help -d 'Print help'
complete -c roxide -n "__fish_roxide_using_subcommand init" -s h -l help -d 'Print help'
complete -c roxide -n "__fish_roxide_using_subcommand help; and not __fish_seen_subcommand_from install update clean init help" -f -a "install" -d 'Installs plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
complete -c roxide -n "__fish_roxide_using_subcommand help; and not __fish_seen_subcommand_from install update clean init help" -f -a "update" -d 'Updates plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
complete -c roxide -n "__fish_roxide_using_subcommand help; and not __fish_seen_subcommand_from install update clean init help" -f -a "clean" -d 'Uninstalls plugins not listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
complete -c roxide -n "__fish_roxide_using_subcommand help; and not __fish_seen_subcommand_from install update clean init help" -f -a "init" -d 'Sources plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
complete -c roxide -n "__fish_roxide_using_subcommand help; and not __fish_seen_subcommand_from install update clean init help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
