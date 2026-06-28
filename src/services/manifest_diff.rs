// Loads saved JSON manifests and compares their duplicate groups.
// Connects to: src/models/manifest_diff.rs, src/models/scan_result.rs, src/utils/manifest_json.rs
// Created: 2026-06-28

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::models::duplicate_group::DuplicateGroup;
use crate::models::manifest_diff::ManifestDiff;
use crate::utils::manifest_json::parse_scan_result_from_json;

/// Compares two saved JSON manifests and returns added and removed duplicate groups.
pub fn diff_manifest_files(before_path: &Path, after_path: &Path) -> Result<ManifestDiff, String> {
    let before_manifest = load_manifest(before_path)?;
    let after_manifest = load_manifest(after_path)?;

    let before_groups = map_groups_by_signature(before_manifest.duplicate_groups);
    let after_groups = map_groups_by_signature(after_manifest.duplicate_groups);
    let mut added_groups = Vec::new();
    let mut removed_groups = Vec::new();

    for (signature, group) in &after_groups {
        if !before_groups.contains_key(signature) {
            added_groups.push(group.clone());
        }
    }

    for (signature, group) in &before_groups {
        if !after_groups.contains_key(signature) {
            removed_groups.push(group.clone());
        }
    }

    sort_groups(&mut added_groups);
    sort_groups(&mut removed_groups);

    Ok(ManifestDiff {
        before_label: before_path.display().to_string(),
        after_label: after_path.display().to_string(),
        added_groups,
        removed_groups,
    })
}

/// Reads and parses one saved manifest file.
fn load_manifest(manifest_path: &Path) -> Result<crate::models::scan_result::ScanResult, String> {
    let manifest_text = fs::read_to_string(manifest_path).map_err(|error| {
        format!(
            "Failed to read manifest file {}: {error}",
            manifest_path.display()
        )
    })?;

    parse_scan_result_from_json(&manifest_text)
}

/// Maps duplicate groups by a stable signature for diffing.
fn map_groups_by_signature(groups: Vec<DuplicateGroup>) -> HashMap<String, DuplicateGroup> {
    let mut groups_by_signature = HashMap::new();

    for group in groups {
        groups_by_signature.insert(build_group_signature(&group), group);
    }

    groups_by_signature
}

/// Builds a stable signature from hash, size, and sorted file paths.
fn build_group_signature(group: &DuplicateGroup) -> String {
    let mut file_paths = group
        .file_paths
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<String>>();
    file_paths.sort();

    format!(
        "{:016x}|{}|{}",
        group.hash,
        group.file_size_bytes,
        file_paths.join("|")
    )
}

/// Sorts groups so diff output remains stable.
fn sort_groups(groups: &mut [DuplicateGroup]) {
    groups.sort_by(|left, right| {
        right
            .file_size_bytes
            .cmp(&left.file_size_bytes)
            .then_with(|| left.file_paths.first().cmp(&right.file_paths.first()))
    });
}
