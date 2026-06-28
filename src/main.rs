// Starts the CLI, parses arguments, runs the duplicate scan, and prints results.
// Connects to: src/config/cli.rs, src/services/duplicate_finder.rs, src/utils/
// Created: 2026-06-28

use std::process::ExitCode;
use std::time::Instant;

use file_duplicate_finder::config::cli::{parse_cli_args, CliArguments, OutputFormat};
use file_duplicate_finder::services::duplicate_finder::{build_scan_metrics, run_duplicate_scan};
use file_duplicate_finder::services::manifest_diff::diff_manifest_files;
use file_duplicate_finder::services::remediation::remediate_manifest;
use file_duplicate_finder::utils::formatting::{
    format_duplicate_report_as_json, format_duplicate_report_as_text, format_manifest_diff_as_json,
    format_manifest_diff_as_text, format_remediation_result_as_json,
    format_remediation_result_as_text,
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

    if let Some((before_path, after_path)) = arguments.diff_paths.clone() {
        return run_manifest_diff(arguments, &before_path, &after_path);
    }

    if let Some(manifest_path) = arguments.remediation_manifest_path.clone() {
        return run_remediation(arguments, &manifest_path);
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

/// Loads a saved manifest, builds a remediation plan, and optionally applies deletions.
fn run_remediation(arguments: CliArguments, manifest_path: &std::path::Path) -> Result<(), String> {
    log_info(
        "remediation_started",
        &[
            ("manifest", manifest_path.to_string_lossy().as_ref()),
            (
                "apply_changes",
                if arguments.apply_changes {
                    "true"
                } else {
                    "false"
                },
            ),
        ],
    );

    let remediation_result = remediate_manifest(manifest_path, arguments.apply_changes)?;
    let report = match arguments.output_format {
        OutputFormat::Text => format_remediation_result_as_text(&remediation_result),
        OutputFormat::Json => format_remediation_result_as_json(&remediation_result),
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
        "remediation_completed",
        &[
            (
                "files_to_delete",
                &remediation_result.files_to_delete.to_string(),
            ),
            (
                "bytes_to_reclaim",
                &remediation_result.bytes_to_reclaim.to_string(),
            ),
        ],
    );

    Ok(())
}

/// Loads and compares two saved manifests, then prints or exports the diff report.
fn run_manifest_diff(
    arguments: CliArguments,
    before_path: &std::path::Path,
    after_path: &std::path::Path,
) -> Result<(), String> {
    log_info(
        "manifest_diff_started",
        &[
            ("before", before_path.to_string_lossy().as_ref()),
            ("after", after_path.to_string_lossy().as_ref()),
        ],
    );

    let manifest_diff = diff_manifest_files(before_path, after_path)?;
    let report = match arguments.output_format {
        OutputFormat::Text => format_manifest_diff_as_text(&manifest_diff),
        OutputFormat::Json => format_manifest_diff_as_json(&manifest_diff),
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
        "manifest_diff_completed",
        &[
            (
                "added_groups",
                &manifest_diff.added_groups.len().to_string(),
            ),
            (
                "removed_groups",
                &manifest_diff.removed_groups.len().to_string(),
            ),
        ],
    );

    Ok(())
}
