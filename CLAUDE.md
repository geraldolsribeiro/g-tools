# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`g-tools` is a Rust CLI toolbox ("Geraldo's toolbox") that wraps and extends functionality around Xournal++ PDF annotation software and microCI, originally ported from bash scripts. It manages an index of PDF files by hash and provides search/bookmark capabilities.

## Common Commands

### Build & Run
```bash
# Build the project
cargo build

# Run the tool (binary is `G`)
cargo run

# Install globally
cargo install --path .
```

### Makefile Targets
```bash
make build      # cargo build
make run        # cargo run
make install    # cargo install --path .
make publish    # cargo publish --allow-dirty
make acceptance-test  # installs then runs test_01.sh
```

### Testing
```bash
# Run all tests
cargo test

# Run a specific test
cargo test <test_name>

# Run with verbose output
cargo test -- --nocapture
```

## Architecture

### Structure
- **`src/lib.rs`**: Main CLI logic, command parsing via clap, and core functionality (xournal operations, microCI installation)
- **`src/config.rs`**: Thread-safe mutable config using `OnceLock<Mutex<Config>>`, manages paths to PDF images directory, index.txt, and bookmarks.txt

### Key Components

#### Commands (`Commands` enum in lib.rs)
- `Xournal`: Operations on Xournal++ PDF files
  - `open <hash>`: Open a PDF by its SHA256 hash prefix (alias: `x`)
  - `search <text>`: Search text across all indexed PDFs (alias: `s`)
  - `bookmark <hash>`: Show bookmarks for a specific hash (alias: `b`)
- `microci`: Install microCI tool
  - `install` (alias: `m`): Install microCI via apt (Linux) or Homebrew tap (macOS)

#### Configuration Flow
1. `initialize_mutable_config("~/pdf_images/")` sets up paths to:
   - `~/pdf_images/` - base directory
   - `~/pdf_images/index.txt` - file listing PDFs by hash prefix
   - `~/pdf_images/bookmarks.txt` - bookmark index

2. Config is stored in a thread-safe mutex for concurrent access

#### Xournal Operations (`cmd_xournal`)
- Validates xournalpp binary exists (auto-installs if missing)
- Locates PDF from index.txt using hash prefix
- Copies hash+filename to clipboard
- Launches xournalpp with the file
- Brings Xournal++ window to front on macOS

#### MicroCI Installation (`cmd_microci`)
- Linux: Downloads binary directly to `/usr/bin/microCI` (sudo required)
- macOS: Installs via Homebrew tap `geraldolsribeiro/tap/microci`

## Dependencies (Cargo.toml)
```toml
clap = { version="4.6.0", features = ["derive", "cargo"] }  # CLI parsing
cli-clipboard = "0.4.0"                                     # Clipboard operations
colored = "3.1.1"                                           # Colored terminal output
pathsearch = "0.2.0"                                        # Find executables in PATH
regex = "1.12.3"                                            # Pattern matching
shellexpand = "3.1.2"                                      # Tilde expansion (~)
sudo = "0.6.0"                                              # Sudo escalation for installs
```

## Platform-Specific Notes

### Linux
- xournalpp: Installed via `apt` at `/usr/bin/xournalpp`
- microCI: Downloaded to `/usr/bin/microCI` with 755 permissions

### macOS
- xournalpp: Manual install from GitHub releases, must be codesigned and stripped of attributes
- microCI: Homebrew tap installation

## Bash Completion
File: `_g_completions.sh`
```bash
mkdir -p ~/.local/share/bash-completion/completions
cp _g_completions.sh ~/.local/share/bash-completion/completions/G
source ~/.bashrc  # or ~/.zshrc
```

## License
Proprietary/Confidential (All Rights Reserved)
