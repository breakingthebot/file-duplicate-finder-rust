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
5. Use `cargo run -- --format json <directory>` when you want machine-readable output.

## Environment Variables
No environment variables are required for this project. See `.env.example`.

## Running Locally
```powershell
cargo fmt
cargo test
cargo run -- .\sample-directory
cargo run -- --format json .\sample-directory
```

## Deployed
Not applicable. This project is a local CLI tool.

## Architecture Notes
This build is a small command-line tool that walks a folder, groups files by size, hashes only the groups that might actually contain duplicates, and then double-checks matching hashes with a byte-for-byte comparison before reporting them. I split it into small Rust modules so the CLI parsing, logging, directory walking, hashing, duplicate detection, and output formatting can each change independently without turning `main.rs` into a junk drawer.

The first iteration established a correct baseline, the second iteration made the output automation-friendly, and this iteration improves the expensive hashing phase by spreading same-size candidate files across worker threads. The concurrency is isolated in its own service module so the core scan flow stays readable, while the results are sorted before reporting so parallel work does not make the output flaky.

## Notes
- The tool uses a deterministic internal FNV-1a content hash and then confirms duplicates with a byte comparison to avoid false positives from hash collisions.
- Hidden files are scanned like any other file.
- Permission errors are logged and skipped so one bad path does not stop the whole scan.
- Supported output modes are `text` and `json`.
- Hashing uses Rust's standard library worker threads and scales up to the machine's available parallelism.
