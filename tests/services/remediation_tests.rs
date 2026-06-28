// Verifies dry-run and apply behavior for duplicate remediation.
// Connects to: src/services/remediation.rs
// Created: 2026-06-28

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use file_duplicate_finder::services::remediation::remediate_manifest;

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
/// Confirms that dry-run remediation plans deletions without removing files.
fn remediate_manifest_builds_dry_run_plan() {
    let root = create_temp_directory("remediation-dry-run");
    let keep_path = root.join("a.txt");
    let delete_path = root.join("b.txt");
    let manifest_path = root.join("scan.json");

    fs::write(&keep_path, "same").expect("keep file should be written");
    fs::write(&delete_path, "same").expect("delete file should be written");
    let escaped_keep_path = keep_path.to_string_lossy().replace('\\', "\\\\");
    let escaped_delete_path = delete_path.to_string_lossy().replace('\\', "\\\\");
    fs::write(
        &manifest_path,
        format!(
            "{{\"metrics\":{{\"files_scanned\":2,\"bytes_scanned\":10,\"duplicate_groups\":1,\"duplicate_files\":2,\"duplicate_bytes\":10,\"elapsed_milliseconds\":1}},\"groups\":[{{\"hash\":\"1111111111111111\",\"file_size_bytes\":5,\"file_paths\":[\"{}\",\"{}\"]}}]}}",
            escaped_keep_path,
            escaped_delete_path
        ),
    )
    .expect("manifest should be written");

    let remediation_result =
        remediate_manifest(&manifest_path, false).expect("dry-run remediation should succeed");

    assert!(!remediation_result.apply_changes);
    assert_eq!(remediation_result.files_to_delete, 1);
    assert!(keep_path.exists());
    assert!(delete_path.exists());

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}

#[test]
/// Confirms that apply remediation deletes redundant files and keeps the first sorted path.
fn remediate_manifest_applies_file_deletions() {
    let root = create_temp_directory("remediation-apply");
    let keep_path = root.join("a.txt");
    let delete_path = root.join("b.txt");
    let manifest_path = root.join("scan.json");

    fs::write(&keep_path, "same").expect("keep file should be written");
    fs::write(&delete_path, "same").expect("delete file should be written");
    let escaped_keep_path = keep_path.to_string_lossy().replace('\\', "\\\\");
    let escaped_delete_path = delete_path.to_string_lossy().replace('\\', "\\\\");
    fs::write(
        &manifest_path,
        format!(
            "{{\"metrics\":{{\"files_scanned\":2,\"bytes_scanned\":10,\"duplicate_groups\":1,\"duplicate_files\":2,\"duplicate_bytes\":10,\"elapsed_milliseconds\":1}},\"groups\":[{{\"hash\":\"1111111111111111\",\"file_size_bytes\":5,\"file_paths\":[\"{}\",\"{}\"]}}]}}",
            escaped_keep_path,
            escaped_delete_path
        ),
    )
    .expect("manifest should be written");

    let remediation_result =
        remediate_manifest(&manifest_path, true).expect("apply remediation should succeed");

    assert!(remediation_result.apply_changes);
    assert!(keep_path.exists());
    assert!(!delete_path.exists());

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}
