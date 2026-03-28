pub mod config;

use crate::config::*;

use clap::Parser;
use clap::Subcommand;
// use cli_clipboard::Clipboard;
use cli_clipboard::ClipboardContext;
use cli_clipboard::ClipboardProvider;
use colored::Colorize;
use pathsearch::find_executable_in_path;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

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

pub fn copy_text_to_clipboard(text: String) -> Result<(), Box<dyn Error>> {
    let mut ctx = cli_clipboard::ClipboardContext::new()?;
    ctx.set_contents(text.to_owned())?;
    Ok(())
}

pub fn copy_text_from_clipboard() -> Result<String, Box<dyn Error>> {
    let mut ctx = ClipboardContext::new()?;
    let contents = ctx.get_contents()?;
    Ok(contents)
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
    let index_txt_path = &MUTABLE_CONFIG.get()?.lock().unwrap().index_txt_path;
    let expanded_path = shellexpand::tilde(index_txt_path);
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

pub fn cmd_xournal(action: XournalAction, _verbose: bool) -> Result<(), &'static str> {
    match action {
        XournalAction::Open { hash } => {
            check_executable_exists(bin_xournalpp());
            match locate_related_file(&hash) {
                Some(filename) => {
                    let hash_and_filename = format!("{}\n{}", hash, filename);
                    let _ = copy_text_to_clipboard(hash_and_filename);

                    let _ = Command::new(bin_xournalpp())
                        .arg(filename)
                        .stdout(Stdio::null()) // Redirect standard output to null
                        .stderr(Stdio::null()) // Redirect standard error to null
                        .spawn()
                        .expect("Failure to execute xournallpp");
                    // .wait(); // Keep in background
                    Ok(())
                }
                None => Err("Hash not found at index.txt"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let path = "~/pdf_images/index.txt".to_string();
        initialize_mutable_config(path.clone());
        let index_txt_path = &MUTABLE_CONFIG
            .get()
            .expect("Error in config")
            .lock()
            .unwrap()
            .index_txt_path;
        assert_eq!(index_txt_path, &path);
    }

    #[test]
    fn test_copy_text_to_clipboard() {
        let text1 = "Ipsum lorem".to_string();
        let _ = copy_text_to_clipboard(text1);
        let text2 = copy_text_from_clipboard();
        assert_eq!(text2.unwrap(), "Ipsum lorem".to_string());
    }

    #[test]
    fn test_cmd_xournal() {
        let result = cmd_xournal(
            XournalAction::Open {
                hash: "12345678".to_string(),
            },
            false,
        );
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, "Hash not found at index.txt");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_locate_related_file() {
        // ...
    }
}
