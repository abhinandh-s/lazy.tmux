
/// .
use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Cli Commands
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[allow(missing_docs)]
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Cli Subcommands
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
