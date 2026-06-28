// Defines aggregate scan metrics for reporting duplicate scan work and outcomes.
// Connects to: src/models/scan_result.rs, src/services/duplicate_finder.rs, src/utils/formatting.rs
// Created: 2026-06-28

/// Stores summary metrics about one duplicate scan run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanMetrics {
    pub files_scanned: usize,
    pub bytes_scanned: u64,
    pub duplicate_groups: usize,
    pub duplicate_files: usize,
    pub duplicate_bytes: u64,
    pub elapsed_milliseconds: u128,
}
