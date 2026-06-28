// Bundles duplicate groups with aggregate metrics for one scan operation.
// Connects to: src/main.rs, src/services/duplicate_finder.rs, src/utils/formatting.rs
// Created: 2026-06-28

use crate::models::duplicate_group::DuplicateGroup;
use crate::models::scan_metrics::ScanMetrics;

/// Represents the complete outcome of one duplicate scan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub metrics: ScanMetrics,
}
