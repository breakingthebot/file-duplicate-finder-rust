// Loads simple key-value configuration files for the duplicate finder CLI.
// Connects to: src/config/cli.rs, tests/config/file_config_tests.rs
// Created: 2026-06-28

use std::fs;
use std::path::Path;

/// Stores raw configuration values loaded from a config file before CLI resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileConfig {
    pub target_path: Option<String>,
    pub minimum_size_bytes: Option<String>,
    pub output_format: Option<String>,
    pub output_path: Option<String>,
    pub exclusions: Vec<String>,
}

impl FileConfig {
    /// Creates an empty configuration value set.
    pub fn empty() -> Self {
        Self {
            target_path: None,
            minimum_size_bytes: None,
            output_format: None,
            output_path: None,
            exclusions: Vec::new(),
        }
    }
}

/// Loads and parses a simple key-value config file from disk.
pub fn load_file_config(config_path: &Path) -> Result<FileConfig, String> {
    let file_contents = fs::read_to_string(config_path).map_err(|error| {
        format!(
            "Failed to read config file {}: {error}",
            config_path.display()
        )
    })?;

    parse_file_config(&file_contents)
}

/// Parses simple key-value config text into raw configuration values.
pub fn parse_file_config(file_contents: &str) -> Result<FileConfig, String> {
    let mut file_config = FileConfig::empty();

    for (line_index, raw_line) in file_contents.lines().enumerate() {
        let trimmed_line = raw_line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        let Some((raw_key, raw_value)) = trimmed_line.split_once('=') else {
            return Err(format!(
                "Invalid config line {}: expected key=value syntax.",
                line_index + 1
            ));
        };

        let key = raw_key.trim();
        let value = raw_value.trim();

        if value.is_empty() {
            return Err(format!(
                "Invalid config line {}: value cannot be empty.",
                line_index + 1
            ));
        }

        match key {
            "target_path" => file_config.target_path = Some(value.to_string()),
            "min_size" => file_config.minimum_size_bytes = Some(value.to_string()),
            "format" => file_config.output_format = Some(value.to_string()),
            "output" => file_config.output_path = Some(value.to_string()),
            "exclude" => file_config.exclusions.push(value.to_string()),
            _ => {
                return Err(format!(
                    "Invalid config line {}: unknown key '{key}'.",
                    line_index + 1
                ));
            }
        }
    }

    Ok(file_config)
}
