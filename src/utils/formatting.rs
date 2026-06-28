// Formats duplicate scan and manifest diff results into user-facing CLI output.
// Connects to: src/main.rs, src/models/scan_result.rs, src/models/manifest_diff.rs
// Created: 2026-06-28

use crate::models::duplicate_group::DuplicateGroup;
use crate::models::manifest_diff::ManifestDiff;
use crate::models::scan_metrics::ScanMetrics;
use crate::models::scan_result::ScanResult;

/// Builds a readable text report for duplicate scan results.
pub fn format_duplicate_report_as_text(scan_result: &ScanResult) -> String {
    if scan_result.duplicate_groups.is_empty() {
        return format!(
            "No duplicate files found.\n\n{}",
            format_metrics_as_text(&scan_result.metrics)
        );
    }

    let mut lines = vec![format!(
        "Found {} duplicate group(s):",
        scan_result.duplicate_groups.len()
    )];

    for (index, group) in scan_result.duplicate_groups.iter().enumerate() {
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

    lines.push(String::new());
    lines.push(format_metrics_as_text(&scan_result.metrics));

    lines.join("\n")
}

/// Builds a JSON report for duplicate scan results without external dependencies.
pub fn format_duplicate_report_as_json(scan_result: &ScanResult) -> String {
    let duplicate_groups = scan_result
        .duplicate_groups
        .iter()
        .map(format_group_as_json)
        .collect::<Vec<String>>()
        .join(",");

    format!(
        "{{\"metrics\":{},\"groups\":[{}]}}",
        format_metrics_as_json(&scan_result.metrics),
        duplicate_groups
    )
}

/// Builds a readable text report for manifest diff results.
pub fn format_manifest_diff_as_text(manifest_diff: &ManifestDiff) -> String {
    let mut lines = vec![
        format!("Comparing manifests:"),
        format!("  before={}", manifest_diff.before_label),
        format!("  after={}", manifest_diff.after_label),
        String::new(),
        format!(
            "Added duplicate group(s): {}",
            manifest_diff.added_groups.len()
        ),
    ];

    append_group_lines(&mut lines, &manifest_diff.added_groups);
    lines.push(String::new());
    lines.push(format!(
        "Removed duplicate group(s): {}",
        manifest_diff.removed_groups.len()
    ));
    append_group_lines(&mut lines, &manifest_diff.removed_groups);

    lines.join("\n")
}

/// Builds a JSON report for manifest diff results without external dependencies.
pub fn format_manifest_diff_as_json(manifest_diff: &ManifestDiff) -> String {
    let added_groups = manifest_diff
        .added_groups
        .iter()
        .map(format_group_as_json)
        .collect::<Vec<String>>()
        .join(",");
    let removed_groups = manifest_diff
        .removed_groups
        .iter()
        .map(format_group_as_json)
        .collect::<Vec<String>>()
        .join(",");

    format!(
        "{{\"before\":\"{}\",\"after\":\"{}\",\"added_groups\":[{}],\"removed_groups\":[{}]}}",
        escape_json_string(&manifest_diff.before_label),
        escape_json_string(&manifest_diff.after_label),
        added_groups,
        removed_groups
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

/// Appends formatted duplicate group lines to an existing text buffer.
fn append_group_lines(lines: &mut Vec<String>, groups: &[DuplicateGroup]) {
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
}

/// Builds the text block for aggregate scan metrics.
fn format_metrics_as_text(metrics: &ScanMetrics) -> String {
    [
        "Summary:".to_string(),
        format!("  files_scanned={}", metrics.files_scanned),
        format!("  bytes_scanned={}", metrics.bytes_scanned),
        format!("  duplicate_groups={}", metrics.duplicate_groups),
        format!("  duplicate_files={}", metrics.duplicate_files),
        format!("  duplicate_bytes={}", metrics.duplicate_bytes),
        format!("  elapsed_milliseconds={}", metrics.elapsed_milliseconds),
    ]
    .join("\n")
}

/// Builds the JSON object for aggregate scan metrics.
fn format_metrics_as_json(metrics: &ScanMetrics) -> String {
    format!(
        "{{\"files_scanned\":{},\"bytes_scanned\":{},\"duplicate_groups\":{},\"duplicate_files\":{},\"duplicate_bytes\":{},\"elapsed_milliseconds\":{}}}",
        metrics.files_scanned,
        metrics.bytes_scanned,
        metrics.duplicate_groups,
        metrics.duplicate_files,
        metrics.duplicate_bytes,
        metrics.elapsed_milliseconds
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
