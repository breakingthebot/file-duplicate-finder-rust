// Defines the outcome of a duplicate remediation run or dry-run plan.
// Connects to: src/services/remediation.rs, src/utils/formatting.rs
// Created: 2026-06-28

use std::path::PathBuf;

/// Represents one duplicate group remediation action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemediationGroup {
    pub kept_path: PathBuf,
    pub deleted_paths: Vec<PathBuf>,
    pub file_size_bytes: u64,
}

/// Represents the full remediation result for one manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemediationResult {
    pub manifest_path: PathBuf,
    pub apply_changes: bool,
    pub groups: Vec<RemediationGroup>,
    pub files_to_delete: usize,
    pub bytes_to_reclaim: u64,
}
