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
6. Use `cargo run -- --exclude target --exclude nested/cache <directory>` to skip unwanted folders or files.

## Environment Variables
No environment variables are required for this project. See `.env.example`.

## Running Locally
```powershell
cargo fmt
cargo test
cargo run -- .\sample-directory
cargo run -- --format json .\sample-directory
cargo run -- --exclude target --exclude nested/cache .\sample-directory
```

## Deployed
Not applicable. This project is a local CLI tool.

## Architecture Notes
This build is a small command-line tool that walks a folder, groups files by size, hashes only the groups that might actually contain duplicates, and then double-checks matching hashes with a byte-for-byte comparison before reporting them. I split it into small Rust modules so the CLI parsing, logging, directory walking, hashing, duplicate detection, and output formatting can each change independently without turning `main.rs` into a junk drawer.

The first iteration established a correct baseline, the second iteration made the output automation-friendly, the third iteration improved hashing throughput with worker threads, and this iteration makes scanning more practical by letting users skip known-noise paths. The exclusion logic is normalized once from CLI input, applied during directory walking before any hashing happens, and kept separate from the duplicate engine so filtering rules do not leak into the rest of the pipeline.

## Notes
- The tool uses a deterministic internal FNV-1a content hash and then confirms duplicates with a byte comparison to avoid false positives from hash collisions.
- Hidden files are scanned like any other file.
- Permission errors are logged and skipped so one bad path does not stop the whole scan.
- Supported output modes are `text` and `json`.
- Hashing uses Rust's standard library worker threads and scales up to the machine's available parallelism.
- `--exclude name` skips any file or directory with that exact name, and `--exclude path/to/node` skips that relative path from the scan root.
