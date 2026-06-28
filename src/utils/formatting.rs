// Formats duplicate scan results into user-facing CLI output.
// Connects to: src/main.rs, src/models/duplicate_group.rs
// Created: 2026-06-28

use crate::models::duplicate_group::DuplicateGroup;

/// Builds a readable text report for duplicate scan results.
pub fn format_duplicate_report(groups: &[DuplicateGroup]) -> String {
    if groups.is_empty() {
        return "No duplicate files found.".to_string();
    }

    let mut lines = vec![format!("Found {} duplicate group(s):", groups.len())];

    for (index, group) in groups.iter().enumerate() {
        lines.push(format!(
            "\nGroup {} | size={} bytes | hash={:016x}",
            index + 1,
            group.file_size_bytes,
            group.hash
        ));

        for path in &group.file_paths {
            lines.push(format!("  {}", path.display()));
        }
    }

    lines.join("\n")
}
