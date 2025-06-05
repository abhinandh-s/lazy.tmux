//! .

#![deny(
    clippy::print_stdout,
    clippy::expect_used,
)]

use std::process::Command;
use anyhow::Error;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

use self::path::PluginDir;
use self::plugins::{ConfigFile, Plugins};

pub mod args;
pub mod git;
/// all methods regarding plugins path
pub mod path;
/// all methods regarding plugins
pub mod plugins;

pub mod error;


pub fn install_plugins() -> Result<(), Error> {
    if let Some(plugins) = ConfigFile::get_plugins() {
        plugins
            .par_iter()
            .try_for_each(|plugin: &Plugins| -> Result<(), Error> {
                let dir: PluginDir = plugin.clone().into();
                if !dir.exists() | is_fully_cloned_repo(plugin.clone().into()) {
                    plugin.install()?;
                }
                Ok(())
            })?;
    }
    Ok(())
}

pub fn install_plugins_one_by_one() -> Result<(), Error> {
    let conf = ConfigFile::get_plugins().unwrap();
    for p in conf {
        let e: PluginDir = p.clone().into();
        if !e.exists() | is_fully_cloned_repo(p.clone().into()) {
            p.install()?;
        }
    }

    Ok(())
}


fn is_fully_cloned_repo(mut repo_path: PluginDir) -> bool {
    let mut git_dir = repo_path.join(".git");

    if !git_dir.exists() {
        return false;
    }

    // Check if it's a shallow clone
    if git_dir.join("shallow").exists() {
        return false;
    }

    // Run `git fsck` to check integrity
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path.to_string())
        .arg("fsck")
        .arg("--full")
        .status();

    match status {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

pub fn update_plugins() -> Result<(), Error> {
    let conf = ConfigFile::get_plugins().unwrap();
    for p in conf {
        let e: PluginDir = p.clone().into();
        if e.exists() | is_fully_cloned_repo(p.clone().into()) {
            p.update()?;
        }
        {}
    }
    Ok(())
}
