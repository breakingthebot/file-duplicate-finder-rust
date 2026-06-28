// Verifies duplicate grouping behavior for the main duplicate finder service.
// Connects to: src/services/duplicate_finder.rs
// Created: 2026-06-28

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use file_duplicate_finder::config::scan_filter::ScanFilter;
use file_duplicate_finder::services::duplicate_finder::find_duplicate_groups;

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
/// Confirms that duplicate groups are discovered across nested directories.
fn find_duplicate_groups_returns_matching_files() {
    let root = create_temp_directory("duplicates");
    let nested = root.join("nested");
    fs::create_dir_all(&nested).expect("nested test directory should be created");

    let first_duplicate = root.join("a.txt");
    let second_duplicate = nested.join("b.txt");
    let unique_file = root.join("c.txt");

    fs::write(&first_duplicate, "duplicate text").expect("first duplicate should be written");
    fs::write(&second_duplicate, "duplicate text").expect("second duplicate should be written");
    fs::write(&unique_file, "unique text").expect("unique file should be written");

    let groups = find_duplicate_groups(&root, 1, &ScanFilter::empty())
        .expect("duplicate scan should succeed");

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].file_paths.len(), 2);
    assert!(groups[0].file_paths.contains(&first_duplicate));
    assert!(groups[0].file_paths.contains(&second_duplicate));

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}

#[test]
/// Confirms that larger same-size candidate sets still produce stable duplicate groups.
fn find_duplicate_groups_handles_parallel_hashing_candidates() {
    let root = create_temp_directory("parallel-duplicates");

    for index in 0..8 {
        let duplicate_path = root.join(format!("duplicate-{index}.txt"));
        fs::write(&duplicate_path, "parallel duplicate payload")
            .expect("duplicate file should be written");
    }

    for index in 0..4 {
        let unique_path = root.join(format!("unique-{index}.txt"));
        fs::write(&unique_path, format!("unique payload {index:02}"))
            .expect("unique file should be written");
    }

    let groups = find_duplicate_groups(&root, 1, &ScanFilter::empty())
        .expect("duplicate scan should succeed");

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].file_paths.len(), 8);
    assert_eq!(
        groups[0].file_paths.first(),
        Some(&root.join("duplicate-0.txt"))
    );
    assert_eq!(
        groups[0].file_paths.last(),
        Some(&root.join("duplicate-7.txt"))
    );

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}

#[test]
/// Confirms that excluded names and relative paths are skipped during duplicate scanning.
fn find_duplicate_groups_skips_excluded_names_and_paths() {
    let root = create_temp_directory("excluded-duplicates");
    let ignored_directory = root.join("ignored");
    let nested_cache_directory = root.join("nested").join("cache");
    fs::create_dir_all(&ignored_directory).expect("ignored directory should be created");
    fs::create_dir_all(&nested_cache_directory).expect("nested cache directory should be created");

    let included_first = root.join("keep-a.txt");
    let included_second = root.join("keep-b.txt");
    let ignored_file = ignored_directory.join("keep-c.txt");
    let cached_file = nested_cache_directory.join("keep-d.txt");

    fs::write(&included_first, "kept duplicate").expect("included first file should be written");
    fs::write(&included_second, "kept duplicate").expect("included second file should be written");
    fs::write(&ignored_file, "kept duplicate").expect("ignored file should be written");
    fs::write(&cached_file, "kept duplicate").expect("cached file should be written");

    let mut scan_filter = ScanFilter::empty();
    scan_filter
        .add_exclusion("ignored")
        .expect("name exclusion should parse");
    scan_filter
        .add_exclusion("nested/cache")
        .expect("path exclusion should parse");

    let groups =
        find_duplicate_groups(&root, 1, &scan_filter).expect("duplicate scan should succeed");

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].file_paths.len(), 2);
    assert!(groups[0].file_paths.contains(&included_first));
    assert!(groups[0].file_paths.contains(&included_second));
    assert!(!groups[0].file_paths.contains(&ignored_file));
    assert!(!groups[0].file_paths.contains(&cached_file));

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}
