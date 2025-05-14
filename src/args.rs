use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Installs plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`
    Install,
    /// Updates plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`
    Update,
    /// Uninstalls plugins not listed in config file `$CONFIG_HOME/tmux/plugins.toml`
    Clean,
    /// Sources plugins listed in config file `$CONFIG_HOME/tmux/plugins.toml`
    Init,
}
