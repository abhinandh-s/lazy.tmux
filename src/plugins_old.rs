use anyhow::Error;
use dirs::config_local_dir;
use serde::Deserialize;
use std::borrow::Cow;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, exit};

use crate::path::PluginDir;
/// Represents a Plugin that contains details about its owner, repository
/// and the platform it is hosted on.
///
/// # Fields
///
/// - `owner`: The owner or creator of the repository (e.g., a username or organization).
/// - `repo`: The name of the repository.
/// - `platform`: The platform where the repository is hosted (e.g., GitHub, GitLab).
///
/// use the builder pattern to create a `Plugin`:
///
/// ```rust
///  use lazy_tmux::plugins::PluginBuilder;
/// let plugin = PluginBuilder::new()
///     .owner("abhinandh-s")
///     .repo("lazy.tmux")
///     .platform("github.com")
///     .build();
/// ```
#[derive(Debug, PartialEq, Deserialize)]
pub struct Plugin<'a> {
    owner: Cow<'a, str>,
    repo: Cow<'a, str>,
    platform: Option<Cow<'a, str>>,
    branch: Option<Cow<'a, str>>,
}

impl<'a> Plugin<'a> {
    pub fn new(
        owner: Cow<'a, str>,
        repo: Cow<'a, str>,
        platform: Option<Cow<'a, str>>,
        branch: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            owner,
            repo,
            platform,
            branch,
        }
    }

    // This method will help users to discover the builder
    pub fn builder() -> PluginBuilder<'a> {
        PluginBuilder::default()
    }

    pub fn install(&self) -> Result<(), Error> {
        let status = Git::new(&self.owner, &self.repo, self.platform.as_deref()).clone();

        match status {
            Ok(_) => print!(""),
            Err(err) => println!("already {}", err),
        }

        Ok(())
    }
}

impl Display for Plugin<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Plugin:\n  owner: {:?}\n  repo: {:?}\n  platform: {:?}\n  branch: {:?}",
            self.owner,
            self.repo,
            match &self.platform {
                Some(p) => p,
                None => &std::borrow::Cow::Borrowed("github.com"),
            },
            match &self.branch {
                Some(p) => p,
                None => &std::borrow::Cow::Borrowed("none"),
            }
        )
    }
}

#[derive(Debug)]
pub struct PluginBuilder<'a, State = Owner> {
    owner: Option<Cow<'a, str>>,
    repo: Option<Cow<'a, str>>,
    platform: Option<Cow<'a, str>>,
    branch: Option<Cow<'a, str>>,
    state: std::marker::PhantomData<State>,
}

#[derive(Debug)]
pub struct Owner;
#[derive(Debug)]
pub struct Repo;
#[derive(Debug)]
pub struct Ready;

impl<'a> PluginBuilder<'a, Owner> {
    pub fn owner<S: Into<Cow<'a, str>>>(&self, owner: S) -> PluginBuilder<'_, Repo> {
        PluginBuilder {
            owner: Some(owner.into()),
            ..Default::default()
        }
    }
}

impl<'a> PluginBuilder<'a, Repo> {
    pub fn repo<S: Into<Cow<'a, str>>>(&self, repo: S) -> PluginBuilder<'_, Ready> {
        PluginBuilder {
            owner: self.owner.clone(),
            repo: Some(repo.into()),
            platform: self.platform.clone(),
            branch: self.branch.clone(),
            state: std::marker::PhantomData,
        }
    }
}

impl<'a> PluginBuilder<'a, Ready> {
    #[inline]
    pub fn platform<S: Into<Cow<'a, str>>>(mut self, platform: S) -> Self {
        self.platform = Some(platform.into());
        self
    }

    #[inline]
    pub fn branch<S: Into<Cow<'a, str>>>(mut self, branch: S) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// Create a Plugin from the PluginBuilder, applying all settings in PluginBuilder to Plugin.
    #[inline]
    pub fn build(self) -> Plugin<'a> {
        self.into()
    }
}

#[allow(clippy::from_over_into)]
impl<'a, S> Into<Plugin<'a>> for PluginBuilder<'a, S>
where
    S: std::fmt::Debug,
{
    fn into(self) -> Plugin<'a> {
        assert!(self.owner.is_some());
        assert!(self.repo.is_some());
        Plugin {
            owner: self.owner.unwrap(),
            repo: self.repo.unwrap(),
            platform: self.platform,
            branch: self.branch,
        }
    }
}

impl<S> Default for PluginBuilder<'_, S> {
    fn default() -> Self {
        Self {
            owner: Default::default(),
            repo: Default::default(),
            platform: Some(Cow::Borrowed("github.com")),
            branch: None,
            state: std::marker::PhantomData,
        }
    }
}

