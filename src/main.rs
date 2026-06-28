// Starts the CLI, parses arguments, runs the duplicate scan, and prints results.
// Connects to: src/config/cli.rs, src/services/duplicate_finder.rs, src/utils/
// Created: 2026-06-28

use std::process::ExitCode;

use file_duplicate_finder::config::cli::{parse_cli_args, CliArguments};
use file_duplicate_finder::services::duplicate_finder::find_duplicate_groups;
use file_duplicate_finder::utils::formatting::format_duplicate_report;
use file_duplicate_finder::utils::logger::{log_error, log_info};

/// Runs the application entrypoint and returns a process-friendly exit code.
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            log_error("application_failed", &[("reason", message.as_str())]);
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}

/// Parses CLI input, performs the scan, and prints the final report.
fn run() -> Result<(), String> {
    let arguments = parse_cli_args(std::env::args().skip(1).collect())?;

    if arguments.show_help {
        println!("{}", CliArguments::help_text());
        return Ok(());
    }

    if arguments.show_version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let root_path = arguments
        .target_path
        .ok_or_else(|| "A target directory is required. Run with --help for usage.".to_string())?;

    log_info(
        "scan_started",
        &[("root", root_path.to_string_lossy().as_ref())],
    );

    let groups = find_duplicate_groups(&root_path, arguments.minimum_size_bytes)?;
    let report = format_duplicate_report(&groups);
    println!("{report}");

    log_info(
        "scan_completed",
        &[("duplicate_groups", &groups.len().to_string())],
    );

    Ok(())
}
