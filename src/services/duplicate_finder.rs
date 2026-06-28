// Orchestrates file collection, size grouping, hashing, and duplicate confirmation.
// Connects to: src/models/duplicate_group.rs, src/services/*, src/utils/logger.rs
// Created: 2026-06-28

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::scan_filter::ScanFilter;
use crate::models::duplicate_group::DuplicateGroup;
use crate::models::scan_metrics::ScanMetrics;
use crate::models::scan_result::ScanResult;
use crate::services::content_comparer::files_match;
use crate::services::directory_scanner::collect_files;
use crate::services::hash_grouping::group_paths_by_hash_parallel;
use crate::utils::logger::log_debug;

/// Finds duplicate groups and summary metrics under the provided root path.
pub fn run_duplicate_scan(
    root_path: &Path,
    minimum_size_bytes: u64,
    scan_filter: &ScanFilter,
) -> Result<ScanResult, String> {
    let file_paths = collect_files(root_path, scan_filter)?;
    let files_scanned = file_paths.len();
    let size_groups = group_paths_by_size(file_paths, minimum_size_bytes)?;
    let mut duplicate_groups = Vec::new();
    let mut bytes_scanned = 0_u64;

    for (file_size_bytes, same_size_paths) in size_groups {
        bytes_scanned += file_size_bytes * same_size_paths.len() as u64;

        if same_size_paths.len() < 2 {
            continue;
        }

        log_debug(
            "size_group_considered",
            &[
                ("file_size_bytes", &file_size_bytes.to_string()),
                ("candidates", &same_size_paths.len().to_string()),
            ],
        );

        let hashed_groups = group_paths_by_hash_parallel(&same_size_paths);

        for (hash, hashed_paths) in hashed_groups {
            if hashed_paths.len() < 2 {
                continue;
            }

            let confirmed_paths = confirm_duplicate_paths(&hashed_paths)?;
            if confirmed_paths.len() > 1 {
                duplicate_groups.push(DuplicateGroup {
                    hash,
                    file_size_bytes,
                    file_paths: confirmed_paths,
                });
            }
        }
    }

    duplicate_groups.sort_by(|left, right| {
        right
            .file_size_bytes
            .cmp(&left.file_size_bytes)
            .then_with(|| left.file_paths.first().cmp(&right.file_paths.first()))
    });

    Ok(ScanResult {
        metrics: build_scan_metrics(&duplicate_groups, files_scanned, bytes_scanned, 0),
        duplicate_groups,
    })
}

/// Groups files by byte size so hashing work only happens for plausible duplicates.
fn group_paths_by_size(
    file_paths: Vec<PathBuf>,
    minimum_size_bytes: u64,
) -> Result<HashMap<u64, Vec<PathBuf>>, String> {
    let mut groups: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for file_path in file_paths {
        let metadata = fs::metadata(&file_path).map_err(|error| {
            format!(
                "Failed to read metadata for {}: {error}",
                file_path.display()
            )
        })?;

        if metadata.len() < minimum_size_bytes {
            continue;
        }

        groups.entry(metadata.len()).or_default().push(file_path);
    }

    Ok(groups)
}

/// Splits a hash bucket into true duplicate groups using byte-for-byte comparison.
fn confirm_duplicate_paths(candidate_paths: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    let mut confirmed_groups: Vec<Vec<PathBuf>> = Vec::new();

    for candidate_path in candidate_paths {
        let mut placed = false;

        for confirmed_group in &mut confirmed_groups {
            let representative = confirmed_group
                .first()
                .ok_or_else(|| "Confirmed duplicate group was unexpectedly empty.".to_string())?;

            if files_match(representative, candidate_path)? {
                confirmed_group.push(candidate_path.clone());
                placed = true;
                break;
            }
        }

        if !placed {
            confirmed_groups.push(vec![candidate_path.clone()]);
        }
    }

    let largest_group = confirmed_groups
        .into_iter()
        .max_by_key(|group| group.len())
        .unwrap_or_default();

    Ok(sort_paths(largest_group))
}

/// Sorts paths so output stays stable even when hashing happens in parallel.
fn sort_paths(mut file_paths: Vec<PathBuf>) -> Vec<PathBuf> {
    file_paths.sort();
    file_paths
}

/// Builds the aggregate metrics for a completed duplicate scan.
pub fn build_scan_metrics(
    duplicate_groups: &[DuplicateGroup],
    files_scanned: usize,
    bytes_scanned: u64,
    elapsed_milliseconds: u128,
) -> ScanMetrics {
    let duplicate_groups_count = duplicate_groups.len();
    let duplicate_files = duplicate_groups
        .iter()
        .map(|group| group.file_paths.len())
        .sum::<usize>();
    let duplicate_bytes = duplicate_groups
        .iter()
        .map(|group| group.file_size_bytes * group.file_paths.len() as u64)
        .sum::<u64>();

    ScanMetrics {
        files_scanned,
        bytes_scanned,
        duplicate_groups: duplicate_groups_count,
        duplicate_files,
        duplicate_bytes,
        elapsed_milliseconds,
    }
}
