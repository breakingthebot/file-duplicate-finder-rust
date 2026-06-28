// Formats duplicate scan results into user-facing CLI output.
// Connects to: src/main.rs, src/models/duplicate_group.rs
// Created: 2026-06-28

use crate::models::duplicate_group::DuplicateGroup;

/// Builds a readable text report for duplicate scan results.
pub fn format_duplicate_report_as_text(groups: &[DuplicateGroup]) -> String {
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

/// Builds a JSON report for duplicate scan results without external dependencies.
pub fn format_duplicate_report_as_json(groups: &[DuplicateGroup]) -> String {
    let duplicate_groups = groups
        .iter()
        .map(format_group_as_json)
        .collect::<Vec<String>>()
        .join(",");

    format!(
        "{{\"duplicate_group_count\":{},\"groups\":[{}]}}",
        groups.len(),
        duplicate_groups
    )
}

/// Builds the JSON object for one duplicate group.
fn format_group_as_json(group: &DuplicateGroup) -> String {
    let file_paths = group
        .file_paths
        .iter()
        .map(|path| format!("\"{}\"", escape_json_string(&path.to_string_lossy())))
        .collect::<Vec<String>>()
        .join(",");

    format!(
        "{{\"hash\":\"{:016x}\",\"file_size_bytes\":{},\"file_paths\":[{}]}}",
        group.hash, group.file_size_bytes, file_paths
    )
}

/// Escapes JSON string content so file paths remain valid JSON values.
fn escape_json_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
