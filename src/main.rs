use clap::Parser;
use std::process::ExitCode;

use g_tools::*;

fn main() -> ExitCode {
    initialize_mutable_config("~/pdf_images/index.txt".to_string());

    let cli = Cli::parse();
    let status = match cli.command {
        Commands::Xournal { action } => cmd_xournal(action, cli.verbose),
    };

    match status {
        Ok(()) => {
            return ExitCode::SUCCESS;
        }
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
        }
    }
}

// G xournal open 3d32ed7b
// G xournal open 6bda659d
// xournal_open_by_hash 3d32ed7b
//
// ${HOME}/bin/book_cover "${full_pdf_filename}" > /dev/null
// ${HOME}/bin/copy_hash_and_filename_to_clipboard "${full_pdf_filename}"
//
// echo "${full_pdf_filename}"
//
// xournalpp "${full_pdf_filename}" 2> /dev/null
// #${HOME}/bin/ pdf_rename_by_content "${full_pdf_filename}" > /dev/null
//
// pretty_filename=$(basename ${full_pdf_filename} .pdf | sed "s/[_]/ /g" )
// echo "$hash_prefix 👉️ $pretty_filename"
// echo "{{/home/geraldo/pdf_images/${hash_prefix}_cover_s.jpg}}"
