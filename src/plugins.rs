#![deny(
    clippy::print_stdout,
    clippy::expect_used,
    clippy::unwrap_used,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

use std::fmt::Display;
use std::sync::Arc;

use anyhow::Error;
use dirs::config_local_dir;
use serde::Deserialize;

use crate::git::Git;

/// default host `github.com`
pub static DEFAULT_HOST: &str = "github.com";
/// default owner `abhinandh-s`
pub static DEFAULT_OWNER: &str = "abhinandh-s";
/// default repo `lazy.tmux`
pub static DEFAULT_REPO: &str = "lazy.tmux";

/// represents a single `Plugin` entry in the ConfigFile
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Plugins {
    owner: Arc<str>,
    repo: Arc<str>,
    platform: Option<Arc<str>>,
    branch: Option<Arc<str>>,
}

impl Plugins {
    /// clones the repo to PluginsDir
    ///
    /// # Errors
    ///
    /// can fail only on clone
    #[inline]
    pub fn install(&self) -> Result<(), Error> {
        Git::new(
            &self.owner,
            &self.repo,
            self.platform.as_deref(),
            self.branch.as_deref(),
        )
        .clone()?;
        Ok(())
    }
}

impl Default for Plugins {
    fn default() -> Self {
        Self {
            owner: Arc::from(DEFAULT_OWNER),
            repo: Arc::from(DEFAULT_REPO),
            platform: Some(Arc::from(DEFAULT_HOST)),
            branch: None,
        }
    }
}

impl Display for Plugins {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Plugin:\n  owner: {}\n  repo: {}\n  platform: {}\n  branch: {}",
            self.owner,
            self.repo,
            self.platform.as_deref().unwrap_or(DEFAULT_HOST),
            self.branch.as_deref().unwrap_or("none"),
        )
    }
}

impl Plugins {
    /// method new
    pub fn new(
        owner: Arc<str>,
        repo: Arc<str>,
        platform: Option<Arc<str>>,
        branch: Option<Arc<str>>,
    ) -> Self {
        Self {
            owner,
            repo,
            platform,
            branch,
        }
    }

    /// getter method for owner
    #[inline]
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// setter method for owner
    #[inline]
    pub fn set_owner(&mut self, owner: Arc<str>) {
        self.owner = owner;
    }

    #[inline]
    /// getter method for repo
    pub fn repo(&self) -> &str {
        &self.repo
    }

    #[inline]
    /// setter method for repo
    pub fn set_repo(&mut self, repo: Arc<str>) {
        self.repo = repo;
    }

    #[inline]
    /// getter method for platform
    pub fn platform(&self) -> Option<&Arc<str>> {
        self.platform.as_ref()
    }

    /// setter method for platform
    #[inline]
    pub fn set_platform(&mut self, platform: Option<Arc<str>>) {
        self.platform = platform;
    }

    /// getter method for branch
    #[inline]
    pub fn branch(&self) -> Option<&Arc<str>> {
        self.branch.as_ref()
    }

    #[inline]
    /// setter method for branch
    pub fn set_branch(&mut self, branch: Option<Arc<str>>) {
        self.branch = branch;
    }
}

/// `ConfigFile` represents the entire list of `Plugins` parsed from config file.
#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigFile {
    plugins: Vec<Plugins>,
}

impl ConfigFile {
    /// parses the config file `$CONFIG_HOME/tmux/plugins.toml`
    /// returns some vector of `Plugin` entries
    #[inline]
    pub fn get_plugins() -> Option<Vec<Plugins>> {
        let config_dir = config_local_dir()?.join("tmux").join("plugins.toml");
        let input = std::fs::read_to_string(config_dir).ok()?;
        let config: ConfigFile = toml::from_str(&input).ok()?;
        Some(config.plugins)
    }

    /// lists all plugins parsed from the config file
    #[inline]
    #[allow(clippy::print_stdout)]
    pub fn list_plugins() {
        if let Some(plugins) = Self::get_plugins() {
            plugins.iter().for_each(|p| {
                println!("{}", p);
            });
        }
    }
}
