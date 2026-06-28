// Defines the result of comparing two saved scan manifests.
// Connects to: src/services/manifest_diff.rs, src/utils/formatting.rs
// Created: 2026-06-28

use crate::models::duplicate_group::DuplicateGroup;

/// Represents the added and removed duplicate groups between two manifests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestDiff {
    pub before_label: String,
    pub after_label: String,
    pub added_groups: Vec<DuplicateGroup>,
    pub removed_groups: Vec<DuplicateGroup>,
}
