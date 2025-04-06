use std::path::PathBuf;
use std::process::{Command, ExitStatus, exit};

use anyhow::Error;

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
#[derive(Debug, PartialEq)]
pub struct Plugin<'a> {
    owner: &'a str,
    repo: &'a str,
    platform: &'a str,
}

impl Plugin<'_> {
    // This method will help users to discover the builder
    pub fn builder<'a>() -> PluginBuilder<'a> {
        PluginBuilder::default()
    }
    // This method will help users to discover the builder
    pub fn install(&self) -> Result<(), Error> {
        let status = Git::new(self.owner, self.repo, self.platform).clone();

        if status.unwrap().success() {
            println!("Installed {} successfully!", self.repo);
        } else {
            eprintln!("Failed to clone repository.");
            exit(1); // Exit with a failure status if cloning fails
        }

        Ok(())
    }
}

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
pub struct PluginBuilder<'a> {
    owner: &'a str,
    repo: &'a str,
    platform: &'a str,
}

impl Default for PluginBuilder<'_> {
    fn default() -> Self {
        Self {
            owner: Default::default(),
            repo: Default::default(),
            platform: "github.com",
        }
    }
}

impl<'a> PluginBuilder<'a> {
    pub fn new() -> PluginBuilder<'a> {
        // Set the minimally required fields of Foo.
        PluginBuilder::default()
    }

    pub fn repo(mut self, repo: &'a str) -> PluginBuilder<'a> {
        // Set the name on the builder itself, and return the builder by value.
        self.repo = repo;
        self
    }
    pub fn owner(mut self, owner: &'a str) -> PluginBuilder<'a> {
        // Set the name on the builder itself, and return the builder by value.
        self.owner = owner;
        self
    }
    pub fn platform(mut self, service: &'a str) -> PluginBuilder<'a> {
        // Set the name on the builder itself, and return the builder by value.
        self.platform = service;
        self
    }

    // If we can get away with not consuming the Builder here, that is an
    // advantage. It means we can use the FooBuilder as a template for constructing
    // many Foos.
    pub fn build(self) -> Plugin<'a> {
        // Create a Plugin from the PluginBuilder, applying all settings in PluginBuilder to Plugin.
        Plugin {
            owner: self.owner,
            repo: self.repo,
            platform: self.platform,
        }
    }
}

pub struct Git<'a> {
    owner: &'a str,
    repo: &'a str,
    platform: &'a str,
}

#[allow(clippy::should_implement_trait)]
impl<'a> Git<'a> {
    pub fn new(owner: &'a str, repo: &'a str, platform: &'a str) -> Self {
        Self {
            owner,
            repo,
            platform,
        }
    }

    pub fn clone(&self) -> Result<ExitStatus, Error> {
        let dir: PathBuf = PluginDir::builder()
            .owner(self.owner)
            .repo(self.repo)
            .build()
            .into();
        // let pluginspath: PathBuf = PluginsPathBakOld::new().join(self.repo).join(self.name).into();
        // // Run the 'git clone' command
        let d = Command::new("git")
            .arg("clone")
            .arg(format!(
                "https://{}/{}/{}.git",
                self.platform, self.owner, self.repo
            ))
            .arg(dir)
            .arg("--depth=1")
            .arg("--quiet")
            .status()
            .unwrap();
        Ok(d)
    }
}

#[test]
fn builder_test() {
    let foo: Plugin<'_> = Plugin {
        owner: "abhinandh-s",
        repo: "lazy.tmux",
        platform: "github.com",
    };
    let foo_from_builder: Plugin = PluginBuilder::new()
        .owner("abhinandh-s")
        .repo("lazy.tmux")
        .build();
    assert_eq!(foo_from_builder.platform, "github.com");
    assert_eq!(foo, foo_from_builder);
}
