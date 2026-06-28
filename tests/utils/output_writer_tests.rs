// Verifies file output behavior for saved duplicate scan reports.
// Connects to: src/utils/output_writer.rs
// Created: 2026-06-28

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use file_duplicate_finder::utils::output_writer::write_report_to_path;

/// Creates a unique temporary test directory under the system temp path.
fn create_temp_directory(test_name: &str) -> PathBuf {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let path =
        std::env::temp_dir().join(format!("file-duplicate-finder-{test_name}-{unique_suffix}"));
    fs::create_dir_all(&path).expect("temporary test directory should be created");
    path
}

#[test]
/// Confirms that reports can be written to nested output file paths.
fn write_report_to_path_creates_parent_directories_and_writes_file() {
    let root = create_temp_directory("output-writer");
    let output_path = root.join("reports").join("scan.json");

    write_report_to_path(&output_path, "{\"ok\":true}").expect("report should be written");

    let saved_report = fs::read_to_string(&output_path).expect("report file should be readable");
    assert_eq!(saved_report, "{\"ok\":true}");

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}

#[test]
/// Confirms that existing directories are rejected as report output targets.
fn write_report_to_path_rejects_directory_targets() {
    let root = create_temp_directory("output-directory");

    let error = write_report_to_path(&root, "report body").expect_err("directory should fail");

    assert!(error.contains("directory"));

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}
