// Verifies command-line parsing for the duplicate finder configuration layer.
// Connects to: src/config/cli.rs
// Created: 2026-06-28

use std::path::PathBuf;

use file_duplicate_finder::config::cli::{parse_cli_args, OutputFormat};

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
    assert_eq!(arguments.output_format, OutputFormat::Text);
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
