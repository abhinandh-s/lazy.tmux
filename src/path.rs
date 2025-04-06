use std::path::PathBuf;

use dirs::config_local_dir;

pub struct PluginDir {
    path: PathBuf,
}

impl PluginDir {
    pub fn new() -> Self {
        let plugin_dir = config_local_dir().unwrap().join("tmux").join("plugins");
        PluginDir { path: plugin_dir }
    }
    pub fn join(&mut self, name: &str) -> Self {
        let path = self.path.join(name);
        PluginDir { path }
    }
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
    // This method will help users to discover the builder
    pub fn builder<'a>() -> PluginDirBuilder<'a> {
        PluginDirBuilder::default()
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
    pub fn new() -> PluginDirBuilder<'a> {
        // Set the minimally required fields of Foo.
        PluginDirBuilder::default()
    }

    pub fn repo(mut self, repo: &'a str) -> PluginDirBuilder<'a> {
        // Set the name on the builder itself, and return the builder by value.
        self.repo = repo;
        self
    }
    pub fn owner(mut self, owner: &'a str) -> PluginDirBuilder<'a> {
        // Set the name on the builder itself, and return the builder by value.
        self.owner = owner;
        self
    }

    // If we can get away with not consuming the Builder here, that is an
    // advantage. It means we can use the FooBuilder as a template for constructing
    // many Foos.
    pub fn build(self) -> PluginDir {
        // Create a Plugin from the PluginBuilder, applying all settings in PluginBuilder to Plugin.
        PluginDir::new().join(self.owner).join(self.repo)
    }
}
