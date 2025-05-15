use std::fmt::Display;
use std::fs::create_dir_all;
use std::path::PathBuf;

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
        let plugin_dir = dirs::config_local_dir()
            .unwrap()
            .join("tmux")
            .join("plugins");
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

    pub fn as_path(&self) -> &std::path::Path {
        self.path.as_path()
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
#[allow(clippy::unwrap_used)]
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
            dirs::config_dir()
                .unwrap()
                .join("tmux/plugins/abhinandh-s/lazy.tmux")
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

/// Provides utility functions to determine standard directory paths used by lazy.tmux,
/// such as the data directory for installed plugins and the configuration directory.
///
/// This module uses the user's XDG base directories (as defined by the `dirs` crate)
/// to locate or create the necessary folders.
///
/// # Errors
/// Returns `PluginError` variants when required directories cannot be found or created.
pub mod tmux_dirs {
    use std::path::PathBuf;

    use crate::error::PluginError;
 
    /// Returns the plugin data directory path.
    ///
    /// Resolves to `$XDG_DATA_HOME/tmux/plugins`, typically something like
    /// `~/.local/share/tmux/plugins`.
    ///
    /// # Errors
    /// Returns [`PluginError::CantFindDataDir`] if the data directory can't be located.
    pub fn data_dir() -> Result<PathBuf, PluginError> {
        dirs::data_dir()
            .map(|dir| dir.join("tmux").join("plugins"))
            .ok_or(PluginError::CantFindDataDir)
    }

    /// Returns the tmux configuration directory path and ensures it exists.
    ///
    /// Resolves to `$XDG_CONFIG_HOME/tmux`, typically something like
    /// `~/.config/tmux`. If the directory doesn't exist, it attempts to create it.
    ///
    /// # Errors
    /// - Returns [`PluginError::CantFindConfigDir`] if the config directory can't be found.
    /// - Returns [`PluginError::CantCreateDir`] if the directory cannot be created.
    pub fn config_dir() -> Result<PathBuf, PluginError> {
        dirs::config_local_dir()
            .map(|dir| -> Result<PathBuf, PluginError> {
                let path = dir.join("tmux");
                if !path.exists() {
                    std::fs::create_dir_all(&path).map_err(PluginError::CantCreateDir)?
                }
                Ok(path)
            })
            .ok_or(PluginError::CantFindConfigDir)?
    }

    /// Returns the path to a specific plugin's installation directory.
    ///
    /// Given a GitHub `repo` and `owner`, this returns the path where the plugin
    /// should be installed under the plugins directory.
    ///
    /// Example: `data_dir()/repo/owner`
    ///
    /// # Errors
    /// Returns [`PluginError::CantFindDataDir`] if the base plugin directory can't be resolved.
    pub fn plugin_dir(repo: &str, owner: &str) -> Result<PathBuf, PluginError> {
        Ok(data_dir()?.join(repo).join(owner))
    }
}
