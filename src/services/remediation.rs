// Plans and optionally applies duplicate-file remediation from a saved manifest.
// Connects to: src/models/remediation_result.rs, src/utils/manifest_json.rs
// Created: 2026-06-28

use std::fs;
use std::path::{Path, PathBuf};

use crate::models::duplicate_group::DuplicateGroup;
use crate::models::remediation_result::{RemediationGroup, RemediationResult};
use crate::utils::manifest_json::parse_scan_result_from_json;

/// Loads a manifest, builds the remediation plan, and optionally deletes duplicate files.
pub fn remediate_manifest(
    manifest_path: &Path,
    apply_changes: bool,
) -> Result<RemediationResult, String> {
    let manifest_text = fs::read_to_string(manifest_path).map_err(|error| {
        format!(
            "Failed to read manifest file {}: {error}",
            manifest_path.display()
        )
    })?;
    let scan_result = parse_scan_result_from_json(&manifest_text)?;
    let remediation_groups = build_remediation_groups(scan_result.duplicate_groups);
    let files_to_delete = remediation_groups
        .iter()
        .map(|group| group.deleted_paths.len())
        .sum::<usize>();
    let bytes_to_reclaim = remediation_groups
        .iter()
        .map(|group| group.file_size_bytes * group.deleted_paths.len() as u64)
        .sum::<u64>();

    if apply_changes {
        apply_remediation_groups(&remediation_groups)?;
    }

    Ok(RemediationResult {
        manifest_path: manifest_path.to_path_buf(),
        apply_changes,
        groups: remediation_groups,
        files_to_delete,
        bytes_to_reclaim,
    })
}

/// Builds one remediation group per duplicate group by keeping the first sorted path.
fn build_remediation_groups(duplicate_groups: Vec<DuplicateGroup>) -> Vec<RemediationGroup> {
    duplicate_groups
        .into_iter()
        .filter_map(|group| build_remediation_group(group.file_paths, group.file_size_bytes))
        .collect()
}

/// Builds one remediation action from a sorted set of duplicate paths.
fn build_remediation_group(
    mut file_paths: Vec<PathBuf>,
    file_size_bytes: u64,
) -> Option<RemediationGroup> {
    if file_paths.len() < 2 {
        return None;
    }

    file_paths.sort();
    let kept_path = file_paths.first()?.clone();
    let deleted_paths = file_paths.into_iter().skip(1).collect::<Vec<PathBuf>>();

    Some(RemediationGroup {
        kept_path,
        deleted_paths,
        file_size_bytes,
    })
}

/// Applies the planned remediation actions by deleting the redundant files.
fn apply_remediation_groups(groups: &[RemediationGroup]) -> Result<(), String> {
    validate_remediation_targets(groups)?;

    for group in groups {
        for deleted_path in &group.deleted_paths {
            fs::remove_file(deleted_path).map_err(|error| {
                format!(
                    "Failed to delete duplicate file {}: {error}",
                    deleted_path.display()
                )
            })?;
        }
    }

    Ok(())
}

/// Validates that remediation targets exist and do not overlap with kept files.
fn validate_remediation_targets(groups: &[RemediationGroup]) -> Result<(), String> {
    for group in groups {
        if !group.kept_path.exists() {
            return Err(format!(
                "Kept file from manifest does not exist: {}",
                group.kept_path.display()
            ));
        }

        for deleted_path in &group.deleted_paths {
            if deleted_path == &group.kept_path {
                return Err(format!(
                    "Refusing to delete the kept file: {}",
                    deleted_path.display()
                ));
            }

            if !deleted_path.exists() {
                return Err(format!(
                    "Duplicate file from manifest does not exist: {}",
                    deleted_path.display()
                ));
            }
        }
    }

    Ok(())
}
