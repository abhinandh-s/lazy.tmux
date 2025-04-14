use std::path::PathBuf;
use std::process::Command;

use crate::path::PluginDir;

pub struct Git<'a> {
    owner: &'a str,
    repo: &'a str,
    platform: Option<&'a str>,
    branch: Option<&'a str>,
}

#[allow(clippy::should_implement_trait)]
impl<'a> Git<'a> {
    pub fn new(
        owner: &'a str,
        repo: &'a str,
        platform: Option<&'a str>,
        branch: Option<&'a str>,
    ) -> Self {
        Self {
            owner,
            repo,
            platform,
            branch,
        }
    }

    #[inline]
    pub fn clone(&self) -> Result<(), anyhow::Error> {
        let dir: PathBuf = PluginDir::builder()
            .owner(self.owner)
            .repo(self.repo)
            .build()
            .into();
        if dir.exists() {
            return Err(anyhow::anyhow!("exists"));
        }
        let branch = self.branch.unwrap_or_default();
        // let pluginspath: PathBuf = PluginsPathBakOld::new().join(self.repo).join(self.name).into();
        // // Run the 'git clone' command
        let d = Command::new("git")
            .arg("clone")
            .arg(format!(
                "https://{}/{}/{}.git",
                self.platform
                    .unwrap_or(&std::borrow::Cow::Borrowed("github.com")),
                self.owner,
                self.repo
            ))
            .arg(dir)
            .arg("--depth=1")
            .arg("--quiet")
            .status()
            .unwrap();

        if d.success() {
            println!("Installed {} successfully!", self.repo);
        } else {
            eprintln!("Failed to clone repository.");
            std::process::exit(1); // Exit with a failure status if cloning fails
        }

        Ok(())
    }

    #[inline]
    pub fn pull(&self) -> Result<std::process::ExitStatus, anyhow::Error> {
        let dir: PathBuf = PluginDir::builder()
            .owner(self.owner)
            .repo(self.repo)
            .build()
            .into();
        let d = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("pull")
            .arg("--depth=1")
            // .arg("--quiet")
            .status()
            .unwrap();
        Ok(d)
    }
}
