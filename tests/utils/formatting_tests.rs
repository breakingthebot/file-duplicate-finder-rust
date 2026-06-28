// Verifies report rendering behavior for the duplicate output formatter.
// Connects to: src/utils/formatting.rs, src/models/duplicate_group.rs
// Created: 2026-06-28

use std::path::PathBuf;

use file_duplicate_finder::models::duplicate_group::DuplicateGroup;
use file_duplicate_finder::models::manifest_diff::ManifestDiff;
use file_duplicate_finder::models::scan_metrics::ScanMetrics;
use file_duplicate_finder::models::scan_result::ScanResult;
use file_duplicate_finder::utils::formatting::{
    format_duplicate_report_as_json, format_duplicate_report_as_text, format_manifest_diff_as_json,
    format_manifest_diff_as_text,
};

#[test]
/// Confirms that duplicate groups are rendered in the user-facing report.
fn format_duplicate_report_renders_group_details() {
    let report = format_duplicate_report_as_text(&build_scan_result());

    assert!(report.contains("Found 1 duplicate group(s):"));
    assert!(report.contains("size=128 bytes"));
    assert!(report.contains("left.txt"));
    assert!(report.contains("right.txt"));
    assert!(report.contains("Summary:"));
    assert!(report.contains("files_scanned=4"));
    assert!(report.contains("duplicate_bytes=256"));
}

#[test]
/// Confirms that duplicate groups can be rendered as JSON for automation.
fn format_duplicate_report_as_json_renders_group_details() {
    let report = format_duplicate_report_as_json(&build_scan_result());

    assert!(report.contains("\"metrics\":"));
    assert!(report.contains("\"hash\":\"000000000000002a\""));
    assert!(report.contains("\"file_size_bytes\":128"));
    assert!(report.contains("\"file_paths\":[\"left.txt\",\"right.txt\"]"));
    assert!(report.contains("\"files_scanned\":4"));
    assert!(report.contains("\"duplicate_bytes\":256"));
}

#[test]
/// Confirms that manifest diffs are rendered in text form.
fn format_manifest_diff_as_text_renders_changes() {
    let report = format_manifest_diff_as_text(&build_manifest_diff());

    assert!(report.contains("Added duplicate group(s): 1"));
    assert!(report.contains("Removed duplicate group(s): 1"));
    assert!(report.contains("added-a.txt"));
    assert!(report.contains("removed-a.txt"));
}

#[test]
/// Confirms that manifest diffs are rendered in JSON form.
fn format_manifest_diff_as_json_renders_changes() {
    let report = format_manifest_diff_as_json(&build_manifest_diff());

    assert!(report.contains("\"added_groups\":["));
    assert!(report.contains("\"removed_groups\":["));
    assert!(report.contains("\"added-a.txt\""));
    assert!(report.contains("\"removed-a.txt\""));
}

/// Builds a reusable scan result fixture for formatting tests.
fn build_scan_result() -> ScanResult {
    ScanResult {
        duplicate_groups: vec![DuplicateGroup {
            hash: 42,
            file_size_bytes: 128,
            file_paths: vec![PathBuf::from("left.txt"), PathBuf::from("right.txt")],
        }],
        metrics: ScanMetrics {
            files_scanned: 4,
            bytes_scanned: 512,
            duplicate_groups: 1,
            duplicate_files: 2,
            duplicate_bytes: 256,
            elapsed_milliseconds: 12,
        },
    }
}

/// Builds a reusable manifest diff fixture for formatting tests.
fn build_manifest_diff() -> ManifestDiff {
    ManifestDiff {
        before_label: "before.json".to_string(),
        after_label: "after.json".to_string(),
        added_groups: vec![DuplicateGroup {
            hash: 7,
            file_size_bytes: 16,
            file_paths: vec![PathBuf::from("added-a.txt"), PathBuf::from("added-b.txt")],
        }],
        removed_groups: vec![DuplicateGroup {
            hash: 8,
            file_size_bytes: 16,
            file_paths: vec![
                PathBuf::from("removed-a.txt"),
                PathBuf::from("removed-b.txt"),
            ],
        }],
    }
}
