#![deny(
    clippy::print_stdout,
    clippy::expect_used,
    clippy::unwrap_used,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

use std::fmt::Display;
use std::fs::create_dir_all;
use std::path::PathBuf;

use dirs::config_local_dir;

use crate::plugins::Plugins;

/// Represents the dir where plugins are stored
pub struct PluginDir {
    path: PathBuf,
}

impl Display for PluginDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl From<Plugins> for PluginDir {
    fn from(value: Plugins) -> Self {
        PluginDir::builder()
            .owner(value.owner())
            .repo(value.repo())
            .build()
    }
}

impl PluginDir {
    /// .
    /// # Panics
    /// will panic if it can't find config dir
    #[allow(clippy::unwrap_used)]
    pub fn new() -> Self {
        let plugin_dir = config_local_dir().unwrap().join("tmux").join("plugins");
        if !plugin_dir.exists() {
            create_dir_all(&plugin_dir).unwrap();
        }
        PluginDir { path: plugin_dir }
    }
    /// .
    #[inline]
    pub fn join(&mut self, name: &str) -> Self {
        let path = self.path.join(name);
        PluginDir { path }
    }
    /// .
    #[inline]
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
    /// This method will help users to discover the builder
    #[inline]
    pub fn builder<'a>() -> PluginDirBuilder<'a> {
        PluginDirBuilder::default()
    }

    /// .
    #[inline]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// .
    #[inline]
    pub fn as_path(&self) -> &std::path::Path {
        self.path.as_path()
    }

    /// .
    #[inline]
    pub fn has_root(&self) -> bool {
        self.path.has_root()
    }

    /// .
    #[inline]
    pub fn display(&self) -> std::path::Display<'_> {
        self.path.display()
    }
}

impl Default for PluginDir {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::from_over_into)]
impl Into<PathBuf> for PluginDir {
    fn into(self) -> PathBuf {
        self.path.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_path_new() {
        let p = PluginDir::builder()
            .repo("lazy.tmux")
            .owner("abhinandh-s")
            .build();
        assert_eq!(
            <PluginDir as Into<PathBuf>>::into(p),
            std::path::Path::new("/home/abhi/.config/tmux/plugins/abhinandh-s/lazy.tmux")
        );
    }
}

/// Represents a Plugin that contains details about its owner, repository
/// and the platform it is hosted on.
#[derive(Default)]
pub struct PluginDirBuilder<'a> {
    owner: &'a str,
    repo: &'a str,
}

impl<'a> PluginDirBuilder<'a> {
    /// Set the minimally required fields of Foo.
    #[inline]
    pub fn new() -> PluginDirBuilder<'a> {
        PluginDirBuilder::default()
    }

    /// Set the `repo` name on the builder itself, and return the builder by value.
    #[inline]
    pub fn repo(mut self, repo: &'a str) -> PluginDirBuilder<'a> {
        self.repo = repo;
        self
    }

    /// Set the `owner` name on the builder itself, and return the builder by value.
    #[inline]
    pub fn owner(mut self, owner: &'a str) -> PluginDirBuilder<'a> {
        self.owner = owner;
        self
    }

    // If we can get away with not consuming the Builder here, that is an
    // advantage. It means we can use the FooBuilder as a template for constructing
    // many Foos.
    /// .
    #[inline]
    pub fn build(self) -> PluginDir {
        // Create a Plugin from the PluginBuilder, applying all settings in PluginBuilder to Plugin.
        PluginDir::new().join(self.owner).join(self.repo)
    }
}
