// Formats release-facing metadata such as the CLI version banner.
// Connects to: src/main.rs, tests/utils/release_metadata_tests.rs
// Created: 2026-06-28

/// Builds the standardized version banner shown by the CLI.
pub fn build_version_output() -> String {
    format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}
