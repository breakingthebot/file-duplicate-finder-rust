// Verifies report rendering behavior for the duplicate output formatter.
// Connects to: src/utils/formatting.rs, src/models/duplicate_group.rs
// Created: 2026-06-28

use std::path::PathBuf;

use file_duplicate_finder::models::duplicate_group::DuplicateGroup;
use file_duplicate_finder::utils::formatting::{
    format_duplicate_report_as_json, format_duplicate_report_as_text,
};

#[test]
/// Confirms that duplicate groups are rendered in the user-facing report.
fn format_duplicate_report_renders_group_details() {
    let group = DuplicateGroup {
        hash: 42,
        file_size_bytes: 128,
        file_paths: vec![PathBuf::from("left.txt"), PathBuf::from("right.txt")],
    };

    let report = format_duplicate_report_as_text(&[group]);

    assert!(report.contains("Found 1 duplicate group(s):"));
    assert!(report.contains("size=128 bytes"));
    assert!(report.contains("left.txt"));
    assert!(report.contains("right.txt"));
}

#[test]
/// Confirms that duplicate groups can be rendered as JSON for automation.
fn format_duplicate_report_as_json_renders_group_details() {
    let group = DuplicateGroup {
        hash: 42,
        file_size_bytes: 128,
        file_paths: vec![PathBuf::from("left.txt"), PathBuf::from("right.txt")],
    };

    let report = format_duplicate_report_as_json(&[group]);

    assert!(report.contains("\"duplicate_group_count\":1"));
    assert!(report.contains("\"hash\":\"000000000000002a\""));
    assert!(report.contains("\"file_size_bytes\":128"));
    assert!(report.contains("\"file_paths\":[\"left.txt\",\"right.txt\"]"));
}
