// Hashes candidate files across worker threads and groups them by content hash.
// Connects to: src/services/duplicate_finder.rs, src/services/hash_service.rs, src/utils/logger.rs
// Created: 2026-06-28

use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;

use crate::services::hash_service::hash_file;
use crate::utils::logger::{log_debug, log_warning};

/// Hashes candidate files in parallel and groups successful results by hash value.
pub fn group_paths_by_hash_parallel(file_paths: &[PathBuf]) -> HashMap<u64, Vec<PathBuf>> {
    let worker_count = determine_worker_count(file_paths.len());
    let chunk_size = file_paths.len().div_ceil(worker_count);
    let mut worker_handles = Vec::new();

    log_debug(
        "hash_workers_started",
        &[
            ("workers", &worker_count.to_string()),
            ("candidates", &file_paths.len().to_string()),
        ],
    );

    for candidate_chunk in file_paths.chunks(chunk_size) {
        let owned_chunk = candidate_chunk.to_vec();
        worker_handles.push(thread::spawn(move || hash_chunk(owned_chunk)));
    }

    group_hashed_entries(collect_hashed_entries(worker_handles))
}

/// Picks a reasonable worker count based on hardware and candidate volume.
fn determine_worker_count(candidate_count: usize) -> usize {
    let available_workers = thread::available_parallelism()
        .map(|parallelism| parallelism.get())
        .unwrap_or(1);

    available_workers.max(1).min(candidate_count.max(1))
}

/// Hashes one chunk of candidate paths inside a worker thread.
fn hash_chunk(candidate_paths: Vec<PathBuf>) -> Vec<(PathBuf, Result<u64, String>)> {
    candidate_paths
        .into_iter()
        .map(|file_path| {
            let hash_result = hash_file(&file_path);
            (file_path, hash_result)
        })
        .collect()
}

/// Joins worker threads and combines their hash results into one list.
fn collect_hashed_entries(
    worker_handles: Vec<thread::JoinHandle<Vec<(PathBuf, Result<u64, String>)>>>,
) -> Vec<(PathBuf, Result<u64, String>)> {
    let mut hashed_entries = Vec::new();

    for worker_handle in worker_handles {
        match worker_handle.join() {
            Ok(mut worker_entries) => {
                hashed_entries.append(&mut worker_entries);
            }
            Err(_) => {
                log_warning(
                    "hash_worker_failed",
                    &[("error", "A hashing worker thread panicked and was skipped.")],
                );
            }
        }
    }

    hashed_entries.sort_by(|left, right| left.0.cmp(&right.0));
    hashed_entries
}

/// Groups successful hash results and logs files that could not be hashed.
fn group_hashed_entries(
    hashed_entries: Vec<(PathBuf, Result<u64, String>)>,
) -> HashMap<u64, Vec<PathBuf>> {
    let mut groups: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for (file_path, hash_result) in hashed_entries {
        match hash_result {
            Ok(hash) => {
                groups.entry(hash).or_default().push(file_path);
            }
            Err(error) => {
                log_warning(
                    "file_skipped_during_hash",
                    &[
                        ("path", file_path.to_string_lossy().as_ref()),
                        ("error", error.as_str()),
                    ],
                );
            }
        }
    }

    groups
}
