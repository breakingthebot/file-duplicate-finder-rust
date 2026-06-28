// Verifies parsing behavior for simple key-value config files.
// Connects to: src/config/file_config.rs
// Created: 2026-06-28

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use file_duplicate_finder::config::file_config::{load_file_config, parse_file_config};

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
/// Confirms that repeated exclusions and supported keys are parsed from config text.
fn parse_file_config_reads_supported_keys() {
    let file_config = parse_file_config(
        "# comment\n\
         target_path=./workspace\n\
         min_size=2048\n\
         format=json\n\
         output=./reports/scan.json\n\
         exclude=target\n\
         exclude=nested/cache\n",
    )
    .expect("config text should parse");

    assert_eq!(file_config.target_path, Some("./workspace".to_string()));
    assert_eq!(file_config.minimum_size_bytes, Some("2048".to_string()));
    assert_eq!(file_config.output_format, Some("json".to_string()));
    assert_eq!(
        file_config.output_path,
        Some("./reports/scan.json".to_string())
    );
    assert_eq!(
        file_config.exclusions,
        vec!["target".to_string(), "nested/cache".to_string()]
    );
}

#[test]
/// Confirms that config files can be loaded from disk.
fn load_file_config_reads_file_contents() {
    let root = create_temp_directory("load-config");
    let config_path = root.join("scan.conf");
    fs::write(&config_path, "target_path=./workspace\n").expect("config file should be written");

    let file_config = load_file_config(&config_path).expect("config file should load");

    assert_eq!(file_config.target_path, Some("./workspace".to_string()));

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}
