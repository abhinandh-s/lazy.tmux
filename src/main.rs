use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::{Command, exit};

use anyhow::{Error, Ok, anyhow};
use clap::{Parser, Subcommand};
use dirs::config_local_dir;

fn main() {

    // make_config_path().unwrap();
    // Plugin2::new("sainnhe/tmux-fzf").install().unwrap();
    // Plugin2::new("olimorris/tmux-pomodoro-plus").install().unwrap();
    // Plugin2::new("tmux-plugins/tmux-cpu").install().unwrap();
}

#[allow(clippy::vec_init_then_push)]
fn get_repo_names<'de>() -> Vec<&'de str> {
    let mut names = Vec::new();
    names.push("catppuccin/tmux");
    names.push("tmux-plugins/tmux-sensible");
    names
}

fn make_config_path() -> Result<(), Error> {
    let c_dir = config_local_dir().ok_or(anyhow!("no config dir"))?;
    let tmux_dir = c_dir.join("tmux").join("plugins");
    if !tmux_dir.exists() {
        create_dir_all(&tmux_dir)?;
    }

    install_plugins(get_repo_names())?;

    Ok(())
}

pub struct PluginsPathBakOld {
    path: PathBuf,
}

#[allow(clippy::from_over_into)]
impl Into<PathBuf> for PluginsPathBakOld {
    fn into(self) -> PathBuf {
        self.path.to_path_buf()
    }
}

impl PluginsPathBakOld {
    pub fn new() -> Self {
        let plugin_dir = config_local_dir().unwrap().join("tmux").join("plugins");
        PluginsPathBakOld { path: plugin_dir }
    }
    pub fn join(&mut self, name: &str) -> Self {
        let path = self.path.join(name);
        PluginsPathBakOld { path }
    }
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}

impl Default for PluginsPathBakOld {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> From<&'a str> for PluginsPathBakOld {
    fn from(value: &'a str) -> Self {
        let n = value.split_once('/').map(|f| (f.0, f.1)).unwrap();
        PluginsPathBakOld::new().join(n.0).join(n.1)
    }
}

pub struct Plugin2<'a> {
    name: &'a str,
    status: PluginsStatus,
}

impl<'a> Plugin2<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name, status: PluginsStatus::NotInstalled }
    }
    pub fn install(&self) -> Result<(), Error> {
        let per_plugin_dir = PluginsPathBakOld::new().join(self.name);
        if !per_plugin_dir.exists() {
            // The URL of the repository you want to clone
            let repo_url = format!("https://github.com/{}.git", self.name);

            println!("Installing... {}", self.name);

            // The directory where you want to clone the repository
            let target_dir = Into::<PathBuf>::into(per_plugin_dir);

            // Run the 'git clone' command
            let status = Command::new("git")
                .arg("clone")
                .arg(repo_url)
                .arg(target_dir)
                .arg("--depth=1")
                .arg("--quiet")
                .status()
                .expect("Failed to execute git clone ");

            if status.success() {
                println!("Installed {} successfully!", self.name);
            } else {
                eprintln!("Failed to clone repository.");
                exit(1); // Exit with a failure status if cloning fails
            }
        }

        Ok(())
    }
}

pub struct Git<'a> {
    name: &'a str,
    repo: &'a str,
}

#[allow(clippy::should_implement_trait)]
impl<'a> Git<'a> {
    pub fn new(name: &'a str, repo: &'a str) -> Self {
        Self { name, repo }
    }
    pub fn clone(&self) -> Result<(), Error> {
        let pluginspath: PathBuf = PluginsPathBakOld::new().join(self.repo).join(self.name).into();
        // Run the 'git clone' command
        let status = Command::new("git")
            .arg("clone")
            .arg(format!("https://github.com/{}/{}.git", self.repo, self.name))
            .arg(pluginspath)
            .arg("--depth=1")
            .arg("--quiet")
            .status()
            .expect("Failed to execute git clone ");

        if status.success() {
            println!("Installed {} successfully!", self.name);
        } else {
            eprintln!("Failed to clone repository.");
            exit(1); // Exit with a failure status if cloning fails
        }
        Ok(())
    }
}

enum PluginsStatus {
    Installed,
    NotInstalled,
}

fn install_plugins(names: Vec<&str>) -> Result<(), Error> {
    let mut plugin_dir = PluginsPathBakOld::default();

    for repo_name in names {
        let per_plugin_dir = plugin_dir.join(repo_name);
        if !per_plugin_dir.exists() {
            // The URL of the repository you want to clone
            let repo_url = format!("https://github.com/{}.git", repo_name);

            println!("Installing... {}", repo_name);

            // The directory where you want to clone the repository
            let target_dir = Into::<PathBuf>::into(per_plugin_dir);

            // Run the 'git clone' command
            let status = Command::new("git")
                .arg("clone")
                .arg(repo_url)
                .arg(target_dir)
                .arg("--depth=1")
                .arg("--quiet")
                .status()
                .expect("Failed to execute git clone ");

            if status.success() {
                println!("Installed {} successfully!", repo_name);
            } else {
                eprintln!("Failed to clone repository.");
                exit(1); // Exit with a failure status if cloning fails
            }
        }
    }
    Ok(())
}
