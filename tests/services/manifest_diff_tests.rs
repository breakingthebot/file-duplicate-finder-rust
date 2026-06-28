// Verifies comparison behavior for saved duplicate scan manifests.
// Connects to: src/services/manifest_diff.rs
// Created: 2026-06-28

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use file_duplicate_finder::services::manifest_diff::diff_manifest_files;

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
/// Confirms that manifest diffs identify added and removed duplicate groups.
fn diff_manifest_files_reports_added_and_removed_groups() {
    let root = create_temp_directory("manifest-diff");
    let before_path = root.join("before.json");
    let after_path = root.join("after.json");

    fs::write(
        &before_path,
        "{\"metrics\":{\"files_scanned\":2,\"bytes_scanned\":12,\"duplicate_groups\":1,\"duplicate_files\":2,\"duplicate_bytes\":12,\"elapsed_milliseconds\":1},\"groups\":[{\"hash\":\"aaaaaaaaaaaaaaaa\",\"file_size_bytes\":6,\"file_paths\":[\"old-a.txt\",\"old-b.txt\"]}]}",
    )
    .expect("before manifest should be written");
    fs::write(
        &after_path,
        "{\"metrics\":{\"files_scanned\":4,\"bytes_scanned\":24,\"duplicate_groups\":1,\"duplicate_files\":2,\"duplicate_bytes\":12,\"elapsed_milliseconds\":1},\"groups\":[{\"hash\":\"bbbbbbbbbbbbbbbb\",\"file_size_bytes\":6,\"file_paths\":[\"new-a.txt\",\"new-b.txt\"]}]}",
    )
    .expect("after manifest should be written");

    let manifest_diff =
        diff_manifest_files(&before_path, &after_path).expect("manifest diff should succeed");

    assert_eq!(manifest_diff.added_groups.len(), 1);
    assert_eq!(manifest_diff.removed_groups.len(), 1);
    assert_eq!(
        manifest_diff.added_groups[0].file_paths[0],
        PathBuf::from("new-a.txt")
    );
    assert_eq!(
        manifest_diff.removed_groups[0].file_paths[0],
        PathBuf::from("old-a.txt")
    );

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}
