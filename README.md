# g-tools

A port of a collection of bash scripts utilities designed to enhance PDF annotation, reference management,  microCI management, and more.

<img width="600px" src="https://raw.githubusercontent.com/geraldolsribeiro/g-tools/refs/heads/master/docs/imgs/logo_white_bg.png">

<p align="center">
[![Crates.io](https://img.shields.io/crates/v/g-tools)](https://crates.io/crates/g-tools)
[![Documentation](https://img.shields.io/docsrs/g-tools)](https://docs.rs/g-tools)
[![License](https://img.shields.io/crates/l/g-tools)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/g-tools)](https://crates.io/crates/g-tools)
[![GitHub Stars](https://img.shields.io/github/stars/geraldolsribeiro/g-tools)](https://github.com/geraldolsribeiro/g-tools/stargazers)

</p>

## Installation

to install `g-tools`, follow these steps:

1. **Install via Cargo:**

```bash
cargo install g-tools
```

## Features

- **PDF Annotation with Xournal++**
  - `open <hash>`: Open a PDF by its SHA256 hash prefix.
  - `search <text>`: Search text across all indexed PDFs.
  - `bookmark <hash>`: Show bookmarks for a specific hash.

- **microCI Management**
  - `install`: Install microCI tool (from github on Linux or Homebrew tap on macOS).

## Usage

run the tool directly:

```bash
G
```

## License

This project is licensed under the [LICENSE](LICENSE).
