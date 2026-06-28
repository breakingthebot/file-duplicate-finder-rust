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
7. Review the summary block at the end of each run to see scan volume, duplicate volume, and elapsed time.
8. Use `cargo run -- --format json --output reports/scan.json <directory>` to save a reusable manifest artifact.
9. Run `cargo run -- --version` to confirm the exact installed CLI release.
10. Use `cargo run -- --config .\file-duplicate-finder.conf.example <directory>` to load reusable defaults from a config file.
11. Use `cargo run -- --diff before.json after.json --format json` to compare two saved JSON manifests.
12. Use `cargo run -- --remediate scan.json` to preview duplicate deletions, then add `--apply` only when you are ready to remove files.

## Environment Variables
No environment variables are required for this project. See `.env.example`.

## Running Locally
```powershell
cargo fmt
cargo test
cargo run -- --version
cargo run -- .\sample-directory
cargo run -- --format json .\sample-directory
cargo run -- --exclude target --exclude nested/cache .\sample-directory
cargo run -- --format json --output .\reports\scan.json .\sample-directory
cargo run -- --config .\file-duplicate-finder.conf.example .\sample-directory
cargo run -- --diff .\reports\before.json .\reports\after.json --format json
cargo run -- --remediate .\reports\scan.json
cargo run -- --remediate .\reports\scan.json --apply
```

## Deployed
Not applicable. This project is a local CLI tool.

## Architecture Notes
This build is a small command-line tool that walks a folder, groups files by size, hashes only the groups that might actually contain duplicates, and then double-checks matching hashes with a byte-for-byte comparison before reporting them. I split it into small Rust modules so the CLI parsing, logging, directory walking, hashing, duplicate detection, and output formatting can each change independently without turning `main.rs` into a junk drawer.

The tool now covers the whole safe workflow from scan to export to diff to remediation. This iteration adds `--remediate <MANIFEST>` as a manifest-based cleanup path that uses the exported JSON artifact as the reviewed input, shows a dry-run plan by default, and only deletes redundant files when `--apply` is present. That keeps the destructive part explicit and auditable instead of hiding it behind a live scan.

## Notes
- The tool uses a deterministic internal FNV-1a content hash and then confirms duplicates with a byte comparison to avoid false positives from hash collisions.
- Hidden files are scanned like any other file.
- Permission errors are logged and skipped so one bad path does not stop the whole scan.
- Supported output modes are `text` and `json`.
- Hashing uses Rust's standard library worker threads and scales up to the machine's available parallelism.
- `--exclude name` skips any file or directory with that exact name, and `--exclude path/to/node` skips that relative path from the scan root.
- Every run now reports `files_scanned`, `bytes_scanned`, `duplicate_groups`, `duplicate_files`, `duplicate_bytes`, and `elapsed_milliseconds`.
- `--output <PATH>` writes the same rendered report to disk and creates missing parent directories automatically.
- `--version` now prints a standardized release banner in the form `file-duplicate-finder 1.0.0`.
- `--config <PATH>` loads defaults from a simple key-value file where repeated `exclude=...` lines are allowed and CLI flags take precedence.
- `--diff <BEFORE> <AFTER>` compares two saved JSON manifests; it is intentionally scoped to the machine-readable export format, not the human-readable text report.
- `--remediate <MANIFEST>` uses a saved JSON manifest, keeps the first sorted file in each duplicate group, defaults to dry-run, and only deletes files when `--apply` is included.
