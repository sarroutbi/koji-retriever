# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`koji-retriever` is a Rust CLI tool that downloads RPM packages from Koji build URLs (Fedora and Red Hat's build system). It parses HTML from Koji build pages, extracts package download links, and downloads the RPM files.

## Build and Test Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build

# Run tests
cargo test                     # Run all tests
cargo test --release          # Run tests in release mode

# Run specific test
cargo test url_existing_test

# Format code
cargo fmt

# Linting
cargo clippy

# Security audit
cargo audit
```

## Coverage Testing

```bash
# Setup (one-time)
cargo install grcov
rustup component add llvm-tools-preview

# Generate coverage
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="koji-retriever-%p-%m.profraw"
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

## Architecture

### Core Flow
1. **main.rs**: Entry point. Uses `curl` to fetch the Koji build page HTML
2. **links.rs**: Core logic for parsing and downloading
   - `get_link_lines()`: Filters HTML lines containing RPM links
   - `get_links()`: Extracts actual URLs from filtered HTML lines
   - `download_links()`: Downloads each RPM file (with optional filtering)
3. **verbose.rs**: Global verbose flag using `lazy_static` Mutex

### Link Extraction
The tool searches for HTML lines containing:
- `<a href` tags
- `.rpm` extension
- Known Koji domains: `fedoraproject.org`, `download.devel.redhat.com`, `download.eng.bos.redhat.com`, `kojihub.stream.rdu2.redhat.com`, `dc.redhat.com` (brewweb)

### Key Design Patterns
- Uses `clap` derive macros for CLI argument parsing
- Global state for verbose flag via `lazy_static` Mutex
- Stream-based download using `curl` write callbacks
- Test mode (-t) allows validation without actual downloads
- Filter mode (-f) restricts downloads to URLs matching a pattern
- Redirect mode (-r) follows HTTP redirects during download

### CLI Arguments
- `-u, --url`: Koji build URL (required)
- `-f, --filter`: Filter to match URLs
- `-d, --directory`: Download directory (defaults to current)
- `-r, --redirect`: Follow HTTP redirects
- `-t, --test`: Test mode (no actual downloads)
- `-v, --verbose`: Enable verbose output

## Testing

Tests are located in `tests/` directory:
- `koji-retriever-test.rs`: Main CLI integration tests using `assert_cmd`
- `links-tests.rs`: Link parsing tests
- `verbose-test.rs`: Verbose flag tests

Tests use real Koji URLs and download to `/tmp`, so they require network access. The test suite includes commands like `ls` and `rm` to verify actual file downloads.

## Dependencies

Key dependencies:
- `clap`: CLI argument parsing with derive macros
- `curl`: HTTP downloads with streaming support
- `lazy_static`: Global state management
- `assert_cmd`, `predicates`, `cargo-bin`: Test utilities
