💤 A modern plugin manager for Tmux *(Work In Progress)*

- ✅ Source plugins  
- ✅ Install plugins  
- ✅ Update plugins  
- ⬜ Clean unused plugins  

> Tested and working on Linux.

---

## Installation

**Requirements**: tmux version 1.9 (or higher), git.

#### Option 1: Install via Cargo

```bash 
cargo install --git https://github.com/abhinandh-s/lazy.tmux
```

#### Option 2: Download a Release

Put this at the bottom of ~/.config/tmux/plugins.toml:

```toml
[[plugins]]
owner = "abhinandh-s"
repo = "lazy.tmux"
platform = "github.com"

[[plugins]]
owner = "tmux-plugins"
repo = "tmux-sensible"
```

plugins are installed into ~/.local/share/tmux/plugins

Put this at the bottom of $XDG_CONFIG_HOME/tmux/tmux.conf

```bash
# Split panes using | and -
bind | split-window -h
bind - split-window -v
unbind '"'
unbind %

# Other settings:

# Initialize lazy.tmux (keep this line at the very bottom of tmux.conf)
run '~/.cargo/bin/lazy-tmux init' 
# If lazy-tmux is in your PATH, just use:
# run 'lazy-tmux init'
```

Reload TMUX environment so TPM is sourced:

```bash
# type this in terminal if tmux is already running
tmux source ~/.tmux.conf
```

## Managing plugins via the command line

```bash
Cli Commands

Usage: lazy-tmux [OPTIONS] [COMMAND]

Commands:
  install  Installs plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`
  update   Updates plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`
  clean    Uninstalls plugins not listed in config file `$CONFIG_HOME/tmux/plugins.toml`
  init     Sources plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`
  help     Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>  Sets a custom config file
  -h, --help           Print help
  -V, --version        Print version
```

<br>

#### License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>

<br>

<sub>

Copyright (c) 2025 Abhinandh S

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

</sub>
