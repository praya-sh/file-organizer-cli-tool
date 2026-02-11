# tidy

A fast, safe, and simple command-line tool written in Rust that organizes files in a directory into categorized folders based on file extensions.

[![Crates.io](https://img.shields.io/crates/v/tidy-file-organizer.svg)](https://crates.io/crates/tidy-files/)
[![Downloads](https://img.shields.io/crates/d/tidy-file-organizer.svg)](https://crates.io/crates/tidy-files/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## Quick Start

```bash
# Install
cargo install tidy-files

# Run
tidy ~/Downloads
```

## Features

- **Organizes files by type** — Images, Docs, Videos, Audio, Archives, Others
- **Fast** — Built with Rust standard library for optimal performance
- **Dry-run mode** — Preview changes before applying them
- **Safe by default** — Avoids overwriting files with smart renaming
- **Cross-platform** — Works on Windows, macOS, and Linux
- **Single binary** — No runtime dependencies required

## 📦 Example

**Before:**
```
Downloads/
├── photo.png
├── resume.pdf
├── movie.mp4
└── song.mp3
```

**After:**
```
Downloads/
├── Images/
│   └── photo.png
├── Docs/
│   └── resume.pdf
├── Videos/
│   └── movie.mp4
└── Audio/
    └── song.mp3
```

## Installation

### Install from crates.io (Recommended)

```bash
cargo install tidy-files
```

Then use it anywhere:
```bash
tidy
tidy ~/Downloads
```

### Build from Source

**Clone and install:**
```bash
git clone https://github.com/yourusername/tidy
cd tidy
cargo install --path .
```

**Or build manually:**
```bash
cargo build --release
```

The binary will be located at:
- **Windows:** `target/release/tidy.exe`
- **Linux/macOS:** `target/release/tidy`

Add it to your system PATH to use globally.

## Usage

```bash
tidy [directory] [options]
```

### Examples

**Clean current directory:**
```bash
tidy
```

**Clean specific directory:**
```bash
tidy C:\Users\praya\Downloads
tidy ~/Downloads
```

**Preview without moving files:**
```bash
tidy ~/Downloads --dry-run
```

**Overwrite files instead of renaming:**
```bash
tidy ~/Downloads --force
```

## Options

| Flag | Description |
|------|-------------|
| `--dry-run` | Show what would happen without moving files |
| `--force` | Overwrite existing files instead of renaming |

## How It Works

1. **Scans** files in the target directory
2. **Detects** file extensions
3. **Maps** extensions to categories
4. **Creates** folders if missing
5. **Moves** files safely with conflict resolution

## Folder Categories

| Extensions | Folder |
|------------|--------|
| `png`, `jpg`, `jpeg`, `gif`, `bmp`, `svg`, `webp` | `Images` |
| `pdf`, `doc`, `docx`, `txt`, `rtf`, `odt` | `Docs` |
| `mp4`, `mkv`, `avi`, `mov`, `wmv`, `flv` | `Videos` |
| `mp3`, `wav`, `flac`, `aac`, `ogg`, `m4a` | `Audio` |
| `zip`, `rar`, `7z`, `tar`, `gz`, `bz2` | `Archives` |
| All others | `Others` |

## Development

### Clone and build

```bash
git clone <repository-url>
cd tidy
cargo build
```

### Run locally

```bash
cargo run -- .
cargo run -- ~/Downloads --dry-run
```

### Run tests

```bash
cargo test
```

### Format code

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Roadmap

- [ ] Custom category configuration via config file
- [ ] Recursive directory organization
- [ ] Undo/restore functionality
- [ ] Date-based organization option
- [ ] File size-based organization

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

**Prayash Shakya**

- 📦 [crates.io/crates/tidy-files](https://crates.io/crates/tidy-files/)
- 🦀 Built with Rust

---

<div align="center">
  <sub>If you found this useful, consider giving it a ⭐!</sub>
</div>
