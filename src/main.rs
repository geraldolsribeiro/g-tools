use clap::Parser;
use clap::Subcommand;
use pathsearch::find_executable_in_path;
use prettycli::*;
use std::error::Error;
use std::process::Command;
// use std::path::Path;
// use colored::Colorize;
// use std::fs;
// use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "G Tools",
    long_about = None
)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Xournal {
        #[command(subcommand)]
        action: XournalAction,
    },
}

#[derive(Subcommand)]
enum XournalAction {
    Open {
        #[arg(required = true, num_args = 1)]
        hash: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Xournal { action } => {
            cmd_xournal(action, cli.verbose);
        }
    }
}

fn check_executable_exists(executable_name: &str) -> Result<(), Box<dyn Error>> {
    match find_executable_in_path(executable_name) {
        Some(path) => {
            println!("'{}' found in PATH at: {:?}", executable_name, path);
            Ok(())
        }
        None => {
            match executable_name {
                "xournalpp" => {
                    error("xournalpp not found in PATH");
                    command("sudo apt install xournalpp");
                    sudo::escalate_if_needed()?;
                    let mut apt = Command::new("apt-get")
                        .arg("update")
                        .spawn()
                        .expect("Failed to update apt");

                    let _status = apt.wait().expect("Failed to update apt");

                    apt = Command::new("apt-get")
                        .arg("install")
                        .arg("xournalpp")
                        .spawn()
                        .expect("Failed to install xournalpp");

                    let _status = apt.wait().expect("Failed to install xournalpp");
                }
                _ => {
                    println!("Unknown executable");
                }
            }
            std::process::exit(1);
        }
    }
}

fn cmd_xournal(action: XournalAction, _verbose: bool) {
    match action {
        XournalAction::Open { hash } => {
            let _ = check_executable_exists("xournalpp");

            println!("xournal open {}", hash);
            // A default configuration can be generated using Command::new(program)
            let mut child = Command::new("echo")
                .arg("Hello world") // Arguments are added separately
                .spawn() // Spawn the child process
                .expect("Failed to execute command");

            // The parent process can continue doing other work here

            // You can wait for the child process to exit and get its status
            let _status = child.wait().expect("Failed to wait on child");
            // println!("Child process exited with status: {}", status);
        }
    }
}

// G xournal open 3d32ed7b
// xournal_open_by_hash 3d32ed7b
