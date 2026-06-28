// Parses and validates command-line arguments for the duplicate finder CLI.
// Connects to: src/main.rs
// Created: 2026-06-28

use std::path::PathBuf;

const HELP_FLAG: &str = "--help";
const SHORT_HELP_FLAG: &str = "-h";
const VERSION_FLAG: &str = "--version";
const SHORT_VERSION_FLAG: &str = "-V";
const MIN_SIZE_FLAG: &str = "--min-size";

/// Holds validated command-line arguments for the application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArguments {
    pub target_path: Option<PathBuf>,
    pub minimum_size_bytes: u64,
    pub show_help: bool,
    pub show_version: bool,
}

impl CliArguments {
    /// Returns the command help text shown for `--help`.
    pub fn help_text() -> &'static str {
        "Usage: file-duplicate-finder [OPTIONS] <DIRECTORY>\n\nOptions:\n  --min-size <BYTES>  Only hash files at or above this size\n  -h, --help          Show help output\n  -V, --version       Show version output"
    }
}

/// Converts raw CLI tokens into validated application arguments.
pub fn parse_cli_args(raw_args: Vec<String>) -> Result<CliArguments, String> {
    let mut arguments = CliArguments {
        target_path: None,
        minimum_size_bytes: 1,
        show_help: false,
        show_version: false,
    };

    let mut index = 0;

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
            MIN_SIZE_FLAG => {
                let value = raw_args
                    .get(index + 1)
                    .ok_or_else(|| "Missing value for --min-size.".to_string())?;
                arguments.minimum_size_bytes = parse_minimum_size(value)?;
                index += 2;
            }
            token if token.starts_with('-') => {
                return Err(format!("Unknown option: {token}"));
            }
            token => {
                if arguments.target_path.is_some() {
                    return Err("Only one target directory may be provided.".to_string());
                }

                arguments.target_path = Some(PathBuf::from(token));
                index += 1;
            }
        }
    }

    Ok(arguments)
}

/// Parses the minimum file size filter from CLI text into bytes.
fn parse_minimum_size(value: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|_| format!("Invalid --min-size value: {value}"))
}
