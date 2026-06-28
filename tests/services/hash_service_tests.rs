// Verifies deterministic hashing behavior for the file hashing service.
// Connects to: src/services/hash_service.rs
// Created: 2026-06-28

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use file_duplicate_finder::services::hash_service::hash_file;

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
/// Confirms that identical file contents produce matching hashes.
fn hash_file_returns_same_value_for_identical_content() {
    let root = create_temp_directory("hash-match");
    let left_path = root.join("left.txt");
    let right_path = root.join("right.txt");

    fs::write(&left_path, "same content").expect("left file should be written");
    fs::write(&right_path, "same content").expect("right file should be written");

    let left_hash = hash_file(&left_path).expect("left file should hash");
    let right_hash = hash_file(&right_path).expect("right file should hash");

    assert_eq!(left_hash, right_hash);

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}
