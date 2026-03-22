use clap::Parser;
use g_tools::*;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Xournal { action } => {
            cmd_xournal(action, cli.verbose);
        }
    }
}

// G xournal open 3d32ed7b
// G xournal open 6bda659d
// xournal_open_by_hash 3d32ed7b
