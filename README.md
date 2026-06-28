# README.md
<!-- Documents setup, usage, architecture, and testing for the duplicate finder CLI. Connects to: Cargo.toml, src/, tests/, .github/workflows/ci.yml. Created: 2026-06-28 -->

# File Duplicate Finder

CLI tool that walks a directory tree, hashes file contents, and reports duplicate files.

## Stack
- Rust 1.89+ toolchain
- Standard library only
- GitHub Actions for CI

## Setup
1. Install Rust with `rustup`.
2. Clone the repository.
3. From the project root, run `cargo test`.
4. Run the CLI with `cargo run -- <directory>`.

## Environment Variables
No environment variables are required for this project. See `.env.example`.

## Running Locally
```powershell
cargo fmt
cargo test
cargo run -- .\sample-directory
```

## Deployed
Not applicable. This project is a local CLI tool.

## Architecture Notes
This build is a small command-line tool that walks a folder, groups files by size, hashes only the groups that might actually contain duplicates, and then double-checks matching hashes with a byte-for-byte comparison before reporting them. I split it into small Rust modules so the CLI parsing, logging, directory walking, hashing, duplicate detection, and output formatting can each change independently without turning `main.rs` into a junk drawer.

The goal in the first iteration was to get a real working baseline in place, not a toy demo. That is why the scanner skips unreadable paths with warnings, the duplicate finder avoids unnecessary hashing work by filtering on size first, and the tests cover the logic-bearing modules directly so CI can catch regressions quickly.

## Notes
- The tool uses a deterministic internal FNV-1a content hash and then confirms duplicates with a byte comparison to avoid false positives from hash collisions.
- Hidden files are scanned like any other file.
- Permission errors are logged and skipped so one bad path does not stop the whole scan.

