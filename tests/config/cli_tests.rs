// Verifies command-line parsing for the duplicate finder configuration layer.
// Connects to: src/config/cli.rs
// Created: 2026-06-28

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};

use file_duplicate_finder::config::cli::{parse_cli_args, OutputFormat};

/// Creates a unique temporary test directory under the system temp path.
fn create_temp_directory(test_name: &str) -> PathBuf {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let path = env::temp_dir().join(format!("file-duplicate-finder-{test_name}-{unique_suffix}"));
    fs::create_dir_all(&path).expect("temporary test directory should be created");
    path
}

#[test]
/// Confirms that the parser accepts a directory path and minimum size flag.
fn parse_cli_args_reads_directory_and_minimum_size() {
    let arguments = parse_cli_args(vec![
        "--min-size".to_string(),
        "10".to_string(),
        "sample".to_string(),
    ])
    .expect("arguments should parse");

    assert_eq!(arguments.minimum_size_bytes, 10);
    assert_eq!(arguments.config_path, None);
    assert_eq!(arguments.output_format, OutputFormat::Text);
    assert_eq!(arguments.output_path, None);
    assert_eq!(arguments.scan_filter.excluded_names.len(), 0);
    assert_eq!(arguments.target_path, Some(PathBuf::from("sample")));
}

#[test]
/// Confirms that the parser accepts the JSON output format option.
fn parse_cli_args_reads_json_output_format() {
    let arguments = parse_cli_args(vec![
        "--format".to_string(),
        "json".to_string(),
        "sample".to_string(),
    ])
    .expect("arguments should parse");

    assert_eq!(arguments.output_format, OutputFormat::Json);
    assert_eq!(arguments.target_path, Some(PathBuf::from("sample")));
}

#[test]
/// Confirms that repeated exclude rules are normalized into name and path filters.
fn parse_cli_args_reads_exclude_rules() {
    let arguments = parse_cli_args(vec![
        "--exclude".to_string(),
        "target".to_string(),
        "--exclude".to_string(),
        "nested/cache".to_string(),
        "sample".to_string(),
    ])
    .expect("arguments should parse");

    assert_eq!(arguments.scan_filter.excluded_names, vec!["target"]);
    assert_eq!(
        arguments.scan_filter.excluded_relative_paths,
        vec![vec!["nested".to_string(), "cache".to_string()]]
    );
}

#[test]
/// Confirms that the parser accepts a report output path.
fn parse_cli_args_reads_output_path() {
    let arguments = parse_cli_args(vec![
        "--output".to_string(),
        "reports/scan.json".to_string(),
        "sample".to_string(),
    ])
    .expect("arguments should parse");

    assert_eq!(
        arguments.output_path,
        Some(PathBuf::from("reports/scan.json"))
    );
}

#[test]
/// Confirms that config file defaults are loaded before CLI overrides are applied.
fn parse_cli_args_merges_config_defaults_with_cli_overrides() {
    let root = create_temp_directory("config-merge");
    let config_path = root.join("scan.conf");

    fs::write(
        &config_path,
        "target_path=from-config\nmin_size=64\nformat=text\noutput=reports/from-config.txt\nexclude=target\n",
    )
    .expect("config file should be written");

    let arguments = parse_cli_args(vec![
        "--config".to_string(),
        config_path.to_string_lossy().to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        "reports/from-cli.json".to_string(),
        "from-cli".to_string(),
    ])
    .expect("arguments should parse");

    assert_eq!(arguments.config_path, Some(config_path.clone()));
    assert_eq!(arguments.target_path, Some(PathBuf::from("from-cli")));
    assert_eq!(arguments.minimum_size_bytes, 64);
    assert_eq!(arguments.output_format, OutputFormat::Json);
    assert_eq!(
        arguments.output_path,
        Some(PathBuf::from("reports/from-cli.json"))
    );
    assert_eq!(arguments.scan_filter.excluded_names, vec!["target"]);

    fs::remove_dir_all(root).expect("temporary directory should be removed");
}
