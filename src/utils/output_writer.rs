// Writes rendered reports to disk for saved scan artifacts and automation workflows.
// Connects to: src/main.rs, tests/utils/output_writer_tests.rs
// Created: 2026-06-28

use std::fs;
use std::path::Path;

/// Writes a rendered duplicate report to the requested output path.
pub fn write_report_to_path(output_path: &Path, report: &str) -> Result<(), String> {
    ensure_parent_directory_exists(output_path)?;
    ensure_output_path_is_not_directory(output_path)?;

    fs::write(output_path, report).map_err(|error| {
        format!(
            "Failed to write report to {}: {error}",
            output_path.display()
        )
    })
}

/// Creates parent directories for the output file when they do not already exist.
fn ensure_parent_directory_exists(output_path: &Path) -> Result<(), String> {
    let Some(parent_directory) = output_path.parent() else {
        return Ok(());
    };

    if parent_directory.as_os_str().is_empty() {
        return Ok(());
    }

    fs::create_dir_all(parent_directory).map_err(|error| {
        format!(
            "Failed to create output directory {}: {error}",
            parent_directory.display()
        )
    })
}

/// Rejects output targets that resolve to existing directories instead of files.
fn ensure_output_path_is_not_directory(output_path: &Path) -> Result<(), String> {
    if output_path.is_dir() {
        return Err(format!(
            "Output path points to a directory, not a file: {}",
            output_path.display()
        ));
    }

    Ok(())
}
