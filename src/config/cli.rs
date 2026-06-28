// Parses and validates command-line arguments for the duplicate finder CLI.
// Connects to: src/main.rs
// Created: 2026-06-28

use std::path::PathBuf;

use crate::config::file_config::load_file_config;
use crate::config::scan_filter::ScanFilter;

const HELP_FLAG: &str = "--help";
const SHORT_HELP_FLAG: &str = "-h";
const VERSION_FLAG: &str = "--version";
const SHORT_VERSION_FLAG: &str = "-V";
const MIN_SIZE_FLAG: &str = "--min-size";
const FORMAT_FLAG: &str = "--format";
const EXCLUDE_FLAG: &str = "--exclude";
const OUTPUT_FLAG: &str = "--output";
const CONFIG_FLAG: &str = "--config";

/// Declares the supported output formats for duplicate scan results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

/// Holds validated command-line arguments for the application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArguments {
    pub config_path: Option<PathBuf>,
    pub target_path: Option<PathBuf>,
    pub minimum_size_bytes: u64,
    pub output_format: OutputFormat,
    pub output_path: Option<PathBuf>,
    pub scan_filter: ScanFilter,
    pub show_help: bool,
    pub show_version: bool,
}

impl CliArguments {
    /// Returns the command help text shown for `--help`.
    pub fn help_text() -> &'static str {
        "Usage: file-duplicate-finder [OPTIONS] <DIRECTORY>\n\nOptions:\n  --config <PATH>         Load defaults from a key=value config file\n  --min-size <BYTES>      Only hash files at or above this size\n  --format <text|json>    Select report output format\n  --output <PATH>         Write the rendered report to a file\n  --exclude <RULE>        Skip a name like 'target' or a relative path like 'nested/cache'\n  -h, --help              Show help output\n  -V, --version           Show version output"
    }
}

/// Converts raw CLI tokens into validated application arguments.
pub fn parse_cli_args(raw_args: Vec<String>) -> Result<CliArguments, String> {
    let config_path = find_config_path(&raw_args)?;
    let mut arguments = CliArguments {
        config_path,
        target_path: None,
        minimum_size_bytes: 1,
        output_format: OutputFormat::Text,
        output_path: None,
        scan_filter: ScanFilter::empty(),
        show_help: false,
        show_version: false,
    };
    apply_config_defaults(&mut arguments)?;

    let mut index = 0;
    let mut cli_target_path_set = false;

    while index < raw_args.len() {
        match raw_args[index].as_str() {
            HELP_FLAG | SHORT_HELP_FLAG => {
                arguments.show_help = true;
                index += 1;
            }
            VERSION_FLAG | SHORT_VERSION_FLAG => {
                arguments.show_version = true;
                index += 1;
            }
            CONFIG_FLAG => {
                index += 2;
            }
            MIN_SIZE_FLAG => {
                let value = raw_args
                    .get(index + 1)
                    .ok_or_else(|| "Missing value for --min-size.".to_string())?;
                arguments.minimum_size_bytes = parse_minimum_size(value)?;
                index += 2;
            }
            FORMAT_FLAG => {
                let value = raw_args
                    .get(index + 1)
                    .ok_or_else(|| "Missing value for --format.".to_string())?;
                arguments.output_format = parse_output_format(value)?;
                index += 2;
            }
            OUTPUT_FLAG => {
                let value = raw_args
                    .get(index + 1)
                    .ok_or_else(|| "Missing value for --output.".to_string())?;
                arguments.output_path = Some(parse_output_path(value)?);
                index += 2;
            }
            EXCLUDE_FLAG => {
                let value = raw_args
                    .get(index + 1)
                    .ok_or_else(|| "Missing value for --exclude.".to_string())?;
                arguments.scan_filter.add_exclusion(value)?;
                index += 2;
            }
            token if token.starts_with('-') => {
                return Err(format!("Unknown option: {token}"));
            }
            token => {
                if cli_target_path_set {
                    return Err("Only one target directory may be provided.".to_string());
                }

                arguments.target_path = Some(PathBuf::from(token));
                cli_target_path_set = true;
                index += 1;
            }
        }
    }

    Ok(arguments)
}

/// Scans raw CLI tokens for the optional config path flag.
fn find_config_path(raw_args: &[String]) -> Result<Option<PathBuf>, String> {
    let mut index = 0;

    while index < raw_args.len() {
        if raw_args[index] == CONFIG_FLAG {
            let value = raw_args
                .get(index + 1)
                .ok_or_else(|| "Missing value for --config.".to_string())?;
            return Ok(Some(parse_config_path(value)?));
        }

        index += 1;
    }

    Ok(None)
}

/// Applies config file defaults before CLI flags are processed as overrides.
fn apply_config_defaults(arguments: &mut CliArguments) -> Result<(), String> {
    let Some(config_path) = &arguments.config_path else {
        return Ok(());
    };

    let file_config = load_file_config(config_path)?;

    if let Some(target_path) = file_config.target_path {
        arguments.target_path = Some(PathBuf::from(target_path));
    }

    if let Some(minimum_size_bytes) = file_config.minimum_size_bytes {
        arguments.minimum_size_bytes = parse_minimum_size(&minimum_size_bytes)?;
    }

    if let Some(output_format) = file_config.output_format {
        arguments.output_format = parse_output_format(&output_format)?;
    }

    if let Some(output_path) = file_config.output_path {
        arguments.output_path = Some(parse_output_path(&output_path)?);
    }

    for exclusion in file_config.exclusions {
        arguments.scan_filter.add_exclusion(&exclusion)?;
    }

    Ok(())
}

/// Parses the minimum file size filter from CLI text into bytes.
fn parse_minimum_size(value: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|_| format!("Invalid --min-size value: {value}"))
}

/// Parses the requested output format from CLI text into a supported variant.
fn parse_output_format(value: &str) -> Result<OutputFormat, String> {
    match value {
        "text" => Ok(OutputFormat::Text),
        "json" => Ok(OutputFormat::Json),
        _ => Err(format!(
            "Invalid --format value: {value}. Expected 'text' or 'json'."
        )),
    }
}

/// Parses the requested report output path from CLI text into a validated path.
fn parse_output_path(value: &str) -> Result<PathBuf, String> {
    let trimmed_value = value.trim();

    if trimmed_value.is_empty() {
        return Err("Invalid --output value: path cannot be empty.".to_string());
    }

    Ok(PathBuf::from(trimmed_value))
}

/// Parses the requested config file path from CLI text into a validated path.
fn parse_config_path(value: &str) -> Result<PathBuf, String> {
    let trimmed_value = value.trim();

    if trimmed_value.is_empty() {
        return Err("Invalid --config value: path cannot be empty.".to_string());
    }

    Ok(PathBuf::from(trimmed_value))
}
