use clap::Parser;
use clap::Subcommand;
use colored::Colorize;
use pathsearch::find_executable_in_path;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

//mod G;
// use crate::G::*;

// pub use crate::G;
// ::XournalAction;
// pub use crate::G;

// use std::env;
// use std::path::PathBuf;
// use std::fs;
// use std::fs::File;
//use std::io::{self, BufRead, BufReader};
// use prettycli::*;
// use std::error::Error;
// use std::path::Path;
// use std::fs;
// use std::path::PathBuf;

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Xournal {
        #[command(subcommand)]
        action: XournalAction,
    },
}

#[derive(Subcommand)]
pub enum XournalAction {
    Open {
        #[arg(required = true, num_args = 1)]
        hash: String,
    },
}

fn show_command(cmd: String) {
    println!("CMD: {}", cmd.green().bold());
}

pub fn bin_xournalpp() -> &'static str {
    match std::env::consts::OS {
        "linux" => "/usr/bin/xournalpp",
        "macos" => "/Applications/Xournal++.app/Contents/MacOS/xournalpp",
        &_ => todo!(),
    }
}

fn install_via_apt(package: &str) {
    match sudo::escalate_if_needed() {
        Ok(_) => {
            show_command(format!("sudo apt install {}", package));

            let _status = Command::new("apt-get")
                .arg("update")
                .spawn()
                .expect("apt-get update failure")
                .wait();

            let _status = Command::new("apt-get")
                .arg("install")
                .arg(package)
                .spawn()
                .expect("apt-get install failure")
                .wait();
        }
        Err(e) => {
            eprintln!("Failed to elevate: {}", e);
            std::process::exit(1);
        }
    }
}

fn install_xournalpp() {
    match std::env::consts::OS {
        "linux" => {
            install_via_apt("xournalpp");
        }
        "macos" => {
            eprintln!("Install from https://github.com/xournalpp/xournalpp/releases/tag/nightly");
            eprintln!("xattr -c /Applications/Xournal++.app");
            eprintln!("codesign --force --deep --sign - /Applications/Xournal++.app");
            std::process::exit(1);
        }
        _ => {
            eprintln!(
                "Error: Failure installing xournallpp in {}",
                std::env::consts::OS
            );
            std::process::exit(1);
        }
    }
}

fn check_executable_exists(executable_name: &str) {
    match find_executable_in_path(executable_name) {
        Some(_path) => {
            // println!("'{}' found in PATH at: {:?}", executable_name, path);
            // Ok(())
        }
        None => {
            match executable_name {
                "xournalpp" => {
                    install_xournalpp();
                }
                _ => todo!(),
            }
            std::process::exit(1);
        }
    }
}

fn locate_related_file(hash: &str) -> Option<String> {
    let expanded_path = shellexpand::tilde("~/pdf_images/index.txt");
    let contents = std::fs::read_to_string(expanded_path.as_ref());
    for line in contents.expect("Failure reading index.txt").lines() {
        if line.starts_with(hash) {
            let filename = line.split_whitespace().nth(1).unwrap();
            let file_path = Path::new(filename);
            if file_path.exists() {
                println!("Found {}", filename);
                return Some(filename.to_string());
            } else {
                println!("Not found {}", filename);
            }
        }
    }
    None
}

pub fn cmd_xournal(action: XournalAction, _verbose: bool) {
    match action {
        XournalAction::Open { hash } => {
            check_executable_exists(bin_xournalpp());
            match locate_related_file(&hash) {
                Some(filename) => {
                    let _ = Command::new(bin_xournalpp())
                        .arg(filename)
                        .stdout(Stdio::null()) // Redirect standard output to null
                        .stderr(Stdio::null()) // Redirect standard error to null
                        .spawn()
                        .expect("Failure to execute xournallpp");
                    // .wait(); // Keep in background
                }
                None => {}
            }
        }
    }
}
