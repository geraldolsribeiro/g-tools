pub mod config;

use crate::config::*;
use clap::Parser;
use clap::Subcommand;
use cli_clipboard::ClipboardContext;
use cli_clipboard::ClipboardProvider;
use colored::Colorize;
use pathsearch::find_executable_in_path;
use regex::RegexBuilder;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

#[derive(Parser)]
// #[command(
//     help_template = "{author-with-newline} {about-section}Version: {version} \n {usage-heading} {usage} \n {all-args} {tab}"
// )]
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
    #[clap(alias("x"))]
    Xournal {
        #[command(subcommand)]
        action: XournalAction,
    },
    Microci {
        #[command(subcommand)]
        action: MicroCIAction,
    },
}

#[derive(Subcommand)]
pub enum XournalAction {
    #[clap(alias("o"))]
    Open {
        #[arg(required = true, num_args = 1)]
        hash: String,
    },
    #[clap(alias("s"))]
    Search {
        #[arg(required = true, num_args = 1)]
        text: String,
    },
    #[clap(alias("b"))]
    Bookmark {
        #[arg(required = true, num_args = 1)]
        hash: String,
    },
}

#[derive(Subcommand)]
pub enum MicroCIAction {
    #[clap(alias("m"))]
    Install,
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

// Returns the path to the `xournalpp` executable based on the current operating system.
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

/// Locates a file related to the given hash by searching an index file
///
/// # Parameters
/// * `hash` - SHA256 hash prefix to search for in index.txt
///
/// # Returns
/// `Some(filename)` if a matching file exists, otherwise `None`
///
pub fn locate_related_file(hash: &str) -> Option<String> {
    let index_txt = &MUTABLE_CONFIG.get()?.lock().unwrap().index_txt;
    let contents = std::fs::read_to_string(index_txt);
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

// osascript -e "tell application \"Xournal++\" to activate"
fn bring_app_to_front(app_name: &str) {
    match std::env::consts::OS {
        "macos" => {
            let script = format!("tell application \"{}\" to activate", app_name);
            Command::new("osascript")
                .arg("-e")
                .arg(&script)
                .output()
                .expect("Failed to execute AppleScript");
        }
        &_ => todo!(),
    }
}

pub fn search_text(pattern: &str) -> Option<Vec<String>> {
    let re = RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .expect("Invalid regex pattern");
    let index_txt = &MUTABLE_CONFIG.get()?.lock().unwrap().index_txt;
    let contents = std::fs::read_to_string(index_txt);
    let mut list: Vec<String> = Vec::new();
    for line in contents.expect("Failure reading index.txt").lines() {
        if re.is_match(line) {
            list.push(String::from(line));
        }
    }
    if list.is_empty() { None } else { Some(list) }
}

pub fn show_bookmark(hash: &str) -> Option<Vec<String>> {
    let re = RegexBuilder::new(hash)
        .case_insensitive(true)
        .build()
        .expect("Invalid regex pattern");
    let bookmarks_txt = &MUTABLE_CONFIG.get()?.lock().unwrap().bookmarks_txt;
    let contents = std::fs::read_to_string(bookmarks_txt);
    let mut list: Vec<String> = Vec::new();
    for line in contents.expect("Failure reading bookmarks.txt").lines() {
        if re.is_match(line) {
            list.push(String::from(line));
        }
    }
    if list.is_empty() { None } else { Some(list) }
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
                        .expect("Failure to execute xournallpp")
                        .wait();

                    bring_app_to_front("Xournal++");

                    println!("Please check Xournal++ window");
                    Ok(())
                }
                None => Err("Hash not found at index.txt"),
            }
        }
        XournalAction::Search { text } => match search_text(&text) {
            Some(lines) => {
                for line in lines {
                    println!("{}", &line);
                }
                Ok(())
            }
            None => Err("Not found"),
        },
        XournalAction::Bookmark { hash } => {
            show_bookmark(&hash);
            Ok(())
        }
    }
}

pub fn cmd_microci(action: MicroCIAction) -> Result<(), &'static str> {
    match action {
        MicroCIAction::Install => {
            match std::env::consts::OS {
                "linux" => match sudo::escalate_if_needed() {
                    Ok(_) => {
                        let url = "https://github.com/geraldolsribeiro/microci/releases/latest/download/microCI";
                        let _status = Command::new("curl")
                            .arg("-fsSL")
                            .arg(url)
                            .arg("-o")
                            .arg("/usr/bin/microCI")
                            .spawn()
                            .expect("curl microci")
                            .wait();
                        let _status = Command::new("chmod")
                            .arg("755")
                            .arg("/usr/bin/microCI")
                            .spawn()
                            .expect("chmod microci")
                            .wait();
                        let _status = Command::new("microCI")
                            .arg("--version")
                            .spawn()
                            .expect("microci --version")
                            .wait();
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("Failed to elevate: {}", e);
                        std::process::exit(1);
                    }
                },
                "macos" => {
                    // https://github.com/geraldolsribeiro/homebrew-tap
                    let _status = Command::new("brew")
                        .arg("install")
                        .arg("geraldolsribeiro/tap/microci")
                        .spawn()
                        .expect("brew install microci")
                        .wait();
                    let _status = Command::new("microCI")
                        .arg("--version")
                        .spawn()
                        .expect("microci --version")
                        .wait();
                    Ok(())
                }
                &_ => todo!(),
            }
        }
    }
}
