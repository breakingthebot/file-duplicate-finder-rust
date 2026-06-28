// Verifies parsing behavior for saved JSON manifests.
// Connects to: src/utils/manifest_json.rs
// Created: 2026-06-28

use file_duplicate_finder::utils::manifest_json::parse_scan_result_from_json;

#[test]
/// Confirms that saved JSON manifests can be parsed back into scan results.
fn parse_scan_result_from_json_reads_metrics_and_groups() {
    let scan_result = parse_scan_result_from_json(
        "{\"metrics\":{\"files_scanned\":3,\"bytes_scanned\":23,\"duplicate_groups\":1,\"duplicate_files\":2,\"duplicate_bytes\":12,\"elapsed_milliseconds\":1},\"groups\":[{\"hash\":\"64c1ccfeba96a0f8\",\"file_size_bytes\":6,\"file_paths\":[\"a.txt\",\"b.txt\"]}]}",
    )
    .expect("manifest json should parse");

    assert_eq!(scan_result.metrics.files_scanned, 3);
    assert_eq!(scan_result.metrics.duplicate_groups, 1);
    assert_eq!(scan_result.duplicate_groups.len(), 1);
    assert_eq!(scan_result.duplicate_groups[0].file_paths.len(), 2);
}
