// Verifies the shipped binary version flag behavior for release-facing output.
// Connects to: src/main.rs, src/utils/release_metadata.rs
// Created: 2026-06-28

use std::process::Command;

#[test]
/// Confirms that the compiled binary prints the standardized version banner.
fn binary_version_flag_prints_release_banner() {
    let output = Command::new(env!("CARGO_BIN_EXE_file-duplicate-finder"))
        .arg("--version")
        .output()
        .expect("binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid utf-8");
    assert_eq!(stdout.trim(), "file-duplicate-finder 1.0.0");
}
