use std::path::PathBuf;
use std::process::Command;

use anyhow::Error;
use lazy_tmux::config;
use lazy_tmux::path::PluginDir;

fn main() {
    install_plugins().unwrap();
    source_plugins().unwrap().iter().for_each(|path| {
        let status = Command::new(path.as_os_str()).status().unwrap();
        if !status.success() {
            eprintln!("Warning: Failed to source");
        } else {
            eprintln!("Success: sourced");
        }
    })
}

fn install_plugins() -> Result<(), Error> {
    let conf = config::ConfigFile::get_plugins().unwrap();
    for p in conf {
        p.install()?;
    }
    Ok(())
}

use walkdir::{DirEntry, WalkDir};

fn source_plugins() -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut v = Vec::new();
    let conf = config::ConfigFile::get_plugins().unwrap();

    for p in conf {
        let l: PluginDir = p.into();
        let walker = WalkDir::new(l.as_path()).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let e = entry?;
            if is_tmux_file(&e) {
                v.push(e.path().to_path_buf());
                println!("{}", e.path().display());
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
