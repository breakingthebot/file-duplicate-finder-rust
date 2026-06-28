// Recursively walks the filesystem and collects regular files for duplicate analysis.
// Connects to: src/services/duplicate_finder.rs, src/utils/logger.rs
// Created: 2026-06-28

use std::fs;
use std::path::{Path, PathBuf};

use crate::config::scan_filter::ScanFilter;
use crate::utils::logger::log_warning;

/// Walks the target directory tree and returns all readable file paths.
pub fn collect_files(root_path: &Path, scan_filter: &ScanFilter) -> Result<Vec<PathBuf>, String> {
    if !root_path.exists() {
        return Err(format!(
            "Target path does not exist: {}",
            root_path.display()
        ));
    }

    if !root_path.is_dir() {
        return Err(format!(
            "Target path is not a directory: {}",
            root_path.display()
        ));
    }

    let mut files = Vec::new();
    visit_directory(root_path, root_path, &mut files, scan_filter)?;
    Ok(files)
}

/// Visits a directory depth-first and collects regular file paths.
fn visit_directory(
    root_path: &Path,
    current_path: &Path,
    files: &mut Vec<PathBuf>,
    scan_filter: &ScanFilter,
) -> Result<(), String> {
    let entries = fs::read_dir(current_path).map_err(|error| {
        format!(
            "Failed to read directory {}: {error}",
            current_path.display()
        )
    })?;

    for entry_result in entries {
        match entry_result {
            Ok(entry) => {
                let path = entry.path();
                let relative_path = path.strip_prefix(root_path).map_err(|error| {
                    format!(
                        "Failed to build relative path for {}: {error}",
                        path.display()
                    )
                })?;

                if scan_filter.excludes(relative_path) {
                    continue;
                }

                if path.is_dir() {
                    if let Err(error) = visit_directory(root_path, &path, files, scan_filter) {
                        log_warning(
                            "directory_skipped",
                            &[
                                ("path", path.to_string_lossy().as_ref()),
                                ("error", error.as_str()),
                            ],
                        );
                    }
                } else if path.is_file() {
                    files.push(path);
                }
            }
            Err(error) => {
                log_warning(
                    "directory_entry_skipped",
                    &[
                        ("path", current_path.to_string_lossy().as_ref()),
                        ("error", error.to_string().as_str()),
                    ],
                );
            }
        }
    }

    Ok(())
}
