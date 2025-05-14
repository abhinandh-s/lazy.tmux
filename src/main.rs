use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use lazy_tmux::args::Cli;
use lazy_tmux::plugins::ConfigFile;

use anyhow::Error;
use lazy_tmux::path::PluginDir;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(lazy_tmux::args::Commands::Install) => {
            let re = install_plugins();
            match re {
                Ok(_) => (),
                Err(err) => println!("Error: {}", err),
            }
        }
        Some(lazy_tmux::args::Commands::Init) => {
            if install_plugins().is_ok() {
                source_plugins()
            }
        }
        Some(lazy_tmux::args::Commands::Update) => {
            println!("update");
        let re = update_plugins();
            match re {
                Err(err) => {
                    eprintln!("Failed to update plugins: {}", err);
                }
                Ok(_) => println!("updated"),
            }
        }
        Some(lazy_tmux::args::Commands::Clean) => {
            unimplemented!()
        }
        None => (),
    }
}

fn source_plugins() {
    // we might write it to a log file or something
    get_tmux_executable().unwrap().iter().for_each(|path| {
        if path.is_file() {
        dbg!("Trying to run: {}", path.display());
            
        let _status = Command::new(path.as_os_str()).status().unwrap();
        }
        // if !status.success() {
        //     eprintln!("Warning: Failed to source");
        // } else {
        //     eprintln!("Success: sourced");
        // }
    })
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
fn install_plugins() -> Result<(), Error> {
    let conf = ConfigFile::get_plugins().unwrap();
    for p in conf {
        let e: PluginDir = p.clone().into();
        if !e.exists() | is_fully_cloned_repo(p.clone().into()) {
            p.install()?;
        }
        {}
    }
    Ok(())
}

fn update_plugins() -> Result<(), Error> {
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


use walkdir::{DirEntry, WalkDir};

fn get_tmux_executable() -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut v = Vec::new();
    let conf = ConfigFile::get_plugins().unwrap_or_default();

    for p in conf {
        let l: PluginDir = p.into();
        let walker = WalkDir::new(l.as_path()).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let e = entry?;
            if is_tmux_file(&e) {
                v.push(e.path().to_path_buf());
            }
        }
    }

    Ok(v)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_tmux_file(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext_str| ext_str == "tmux")
        .unwrap_or(false)
}
