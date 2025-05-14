
use builtin;
use str;

set edit:completion:arg-completer[roxide] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'roxide'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'roxide'= {
            cand -c 'Sets a custom config file'
            cand --config 'Sets a custom config file'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand install 'Installs plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
            cand update 'Updates plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
            cand clean 'Uninstalls plugins not listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
            cand init 'Sources plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'roxide;install'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'roxide;update'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'roxide;clean'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'roxide;init'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'roxide;help'= {
            cand install 'Installs plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
            cand update 'Updates plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
            cand clean 'Uninstalls plugins not listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
            cand init 'Sources plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'roxide;help;install'= {
        }
        &'roxide;help;update'= {
        }
        &'roxide;help;clean'= {
        }
        &'roxide;help;init'= {
        }
        &'roxide;help;help'= {
        }
    ]
    $completions[$command]
}
