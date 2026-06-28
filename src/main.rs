// Starts the CLI, parses arguments, runs the duplicate scan, and prints results.
// Connects to: src/config/cli.rs, src/services/duplicate_finder.rs, src/utils/
// Created: 2026-06-28

use std::process::ExitCode;
use std::time::Instant;

use file_duplicate_finder::config::cli::{parse_cli_args, CliArguments, OutputFormat};
use file_duplicate_finder::services::duplicate_finder::{build_scan_metrics, run_duplicate_scan};
use file_duplicate_finder::utils::formatting::{
    format_duplicate_report_as_json, format_duplicate_report_as_text,
};
use file_duplicate_finder::utils::logger::{log_error, log_info};
use file_duplicate_finder::utils::output_writer::write_report_to_path;
use file_duplicate_finder::utils::release_metadata::build_version_output;

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
        println!("{}", build_version_output());
        return Ok(());
    }

    let root_path = arguments
        .target_path
        .ok_or_else(|| "A target directory is required. Run with --help for usage.".to_string())?;

    log_info(
        "scan_started",
        &[("root", root_path.to_string_lossy().as_ref())],
    );

    let started_at = Instant::now();
    let mut scan_result = run_duplicate_scan(
        &root_path,
        arguments.minimum_size_bytes,
        &arguments.scan_filter,
    )?;
    scan_result.metrics = build_scan_metrics(
        &scan_result.duplicate_groups,
        scan_result.metrics.files_scanned,
        scan_result.metrics.bytes_scanned,
        started_at.elapsed().as_millis(),
    );

    let report = match arguments.output_format {
        OutputFormat::Text => format_duplicate_report_as_text(&scan_result),
        OutputFormat::Json => format_duplicate_report_as_json(&scan_result),
    };

    if let Some(output_path) = &arguments.output_path {
        write_report_to_path(output_path, &report)?;
        log_info(
            "report_written",
            &[("path", output_path.to_string_lossy().as_ref())],
        );
    }

    println!("{report}");

    log_info(
        "scan_completed",
        &[
            (
                "duplicate_groups",
                &scan_result.metrics.duplicate_groups.to_string(),
            ),
            (
                "files_scanned",
                &scan_result.metrics.files_scanned.to_string(),
            ),
            (
                "bytes_scanned",
                &scan_result.metrics.bytes_scanned.to_string(),
            ),
            (
                "elapsed_milliseconds",
                &scan_result.metrics.elapsed_milliseconds.to_string(),
            ),
        ],
    );

    Ok(())
}
