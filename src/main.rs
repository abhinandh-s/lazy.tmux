/*
Load config file	✅ Done
Parse plugin info	✅ Done
Clone/install plugins	✅ Done
Update plugins	✅ Done
CLI command parsing	✅ Done
Source .tmux plugins	🟡 Partially
Clean unused plugins	❌ Not done
Error handling	🟡 Incomplete
Logging / verbosity control	❌ Not done
Tests	❌ Missing
Parallel install/update	❌ Not done
Lockfile support	❌ Not done

*/

use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use lazy_tmux::args::Cli;
use lazy_tmux::plugins::ConfigFile;
use lazy_tmux::{install_plugins, update_plugins};
use walkdir::{DirEntry, WalkDir};
use lazy_tmux::path::PluginDir;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(lazy_tmux::args::Commands::Install) => {
            let re = install_plugins();
            match re {
                Ok(_) => {
                    dbg!("success");
                }
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
