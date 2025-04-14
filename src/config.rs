use std::fmt::Display;

use dirs::config_local_dir;
use serde::Deserialize;

pub static DEFAULT_HOST: Cow<'static, str> = Cow::Borrowed("github.com");

use std::borrow::Cow;


#[derive(Debug, PartialEq, Deserialize)]
pub struct Plugins {
    owner: Cow<'static, str>,
    repo: Cow<'static, str>,
    platform: Option<Cow<'static, str>>,
    branch: Option<Cow<'static, str>>,
}

impl Plugins {
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn repo(&self) -> &str {
        &self.repo
    }

    pub fn platform(&self) -> Option<&Cow<'static, str>> {
        self.platform.as_ref()
    }

    pub fn branch(&self) -> Option<&Cow<'static, str>> {
        self.branch.as_ref()
    }

    pub fn set_branch(&mut self, branch: Option<Cow<'static, str>>) {
        self.branch = branch;
    }

    pub fn set_owner(&mut self, owner: Cow<'static, str>) {
        self.owner = owner;
    }

    pub fn set_repo(&mut self, repo: Cow<'static, str>) {
        self.repo = repo;
    }

    pub fn set_platform(&mut self, platform: Option<Cow<'static, str>>) {
        self.platform = platform;
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigFile {
    plugins: Vec<Plugins>,
}



impl Default for Plugins {
    fn default() -> Self {
        Self {
            owner: Default::default(),
            repo: Default::default(),
            platform: Some(DEFAULT_HOST.clone()),
            branch: None,
        }
    }
}

impl Display for Plugins {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Plugin:\n  owner: {:?}\n  repo: {:?}\n  platform: {:?}\n  branch: {:?}",
            self.owner,
            self.repo,
            match &self.platform {
                Some(p) => p,
                None => "github.com",
            },
            match &self.branch {
                Some(p) => p,
                None => "none",
            }
        )
    }
}

impl ConfigFile {
    pub fn get_plugins() -> Option<Vec<Plugins>> {
        let config_dir = config_local_dir()?.join("tmux").join("plugins.toml");
        let input = std::fs::read_to_string(config_dir).ok()?;
        let config: ConfigFile = toml::from_str(&input).ok()?;
        Some(config.plugins)
    }
    pub fn list_plugins() {
        let plugins = Self::get_plugins();
        if let Some(plugins) = plugins {
            plugins.iter().for_each(|p| {
                println!("{}", p);
            });
        }
    }
}


