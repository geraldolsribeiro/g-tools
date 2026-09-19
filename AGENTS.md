# Repository Development Guide

This document describes the project and the conventions to follow when making changes.

## Project overview

`g-tools` is a Rust CLI toolbox that wraps functionality for Xournal++ PDF annotation software and microCI. It maintains an index of PDF files by hash and provides search and bookmark commands. The command-line binary is `G`.

## Development commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo check                    # Type-check without producing a release
cargo test                     # Run all tests
cargo test <TEST_NAME>         # Run one test
cargo test -- --nocapture      # Show test output
cargo fmt                      # Format Rust code
cargo clippy                   # Run lint checks
cargo update                   # Update dependencies
```

The equivalent Make targets are:

```bash
make build
make run
make install                  # cargo install --path .
make publish                  # cargo publish --allow-dirty
make acceptance-test          # Install and run test_01.sh
```

Run the application locally with:

```bash
cargo run -- --port <PORT> --dir <DIRECTORY>
```

Before submitting changes, format the code and run the relevant tests; for broader changes, run the complete test suite and Clippy.

## Repository structure

- `src/lib.rs` — CLI definition and application logic, including Xournal++ and microCI operations.
- `src/config.rs` — Thread-safe mutable configuration backed by `OnceLock<Mutex<Config>>`.
- `tests/` — Integration tests.
- `_g_completions.sh` — Bash completion setup.
- `Makefile` — Common build, install, publish, and acceptance-test targets.

## CLI commands

The `Commands` enum is defined in `src/lib.rs`.

- `xournal open <hash>` (`x`) — Open a PDF using a SHA256 hash prefix.
- `xournal search <text>` (`s`) — Search text across indexed PDFs.
- `xournal bookmark <hash>` (`b`) — Show bookmarks for a PDF.
- `microci install` (`m`) — Install microCI using the platform-specific method.

## Configuration and data files

Application startup initializes configuration for `~/pdf_images/`:

- `~/pdf_images/` — Base directory for PDF-related data.
- `~/pdf_images/index.txt` — PDF index containing hash prefixes and filenames.
- `~/pdf_images/bookmarks.txt` — Bookmark index.

Configuration is accessed through the thread-safe mutable configuration in `src/config.rs`.

## Platform behavior

### Xournal++

The Xournal++ command verifies that `xournalpp` is available and may install it when missing. It resolves PDFs from `index.txt`, copies the hash and filename to the clipboard, launches Xournal++, and brings its window to the front on macOS.

- Linux expects Xournal++ at `/usr/bin/xournalpp` and installs it with `apt` when needed.
- macOS uses a manually installed GitHub release; the application must be codesigned and have extended attributes removed as required.

### microCI

- Linux downloads the binary to `/usr/bin/microCI` and requires `sudo`.
- macOS installs it through the Homebrew tap `geraldolsribeiro/tap/microci`.

## Dependencies

Runtime and CLI dependencies are declared in `Cargo.toml`, including `clap`, `cli-clipboard`, `colored`, `pathsearch`, `regex`, `shellexpand`, and `sudo`. Update the manifest and lockfile together when changing dependencies.

## Bash completion

Install completions with:

```bash
mkdir -p ~/.local/share/bash-completion/completions
cp _g_completions.sh ~/.local/share/bash-completion/completions/G
source ~/.bashrc  # or ~/.zshrc
```

## Change guidelines

- Keep changes focused and preserve existing CLI behavior unless the change explicitly requires otherwise.
- Follow idiomatic Rust and run `cargo fmt` on modified code.
- Add or update tests for behavior changes.
- Avoid committing generated files, local configuration, or machine-specific data.
- Do not alter the proprietary license terms.
