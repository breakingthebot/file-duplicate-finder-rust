// Verifies release-facing metadata formatting such as the CLI version banner.
// Connects to: src/utils/release_metadata.rs
// Created: 2026-06-28

use file_duplicate_finder::utils::release_metadata::build_version_output;

#[test]
/// Confirms that the version banner contains the package name and version.
fn build_version_output_renders_name_and_version() {
    let version_output = build_version_output();

    assert_eq!(version_output, "file-duplicate-finder 1.0.0");
}
