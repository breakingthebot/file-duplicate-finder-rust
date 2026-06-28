// Parses saved JSON scan manifests back into in-memory scan result models.
// Connects to: src/services/manifest_diff.rs, tests/utils/manifest_json_tests.rs
// Created: 2026-06-28

use std::path::PathBuf;

use crate::models::duplicate_group::DuplicateGroup;
use crate::models::scan_metrics::ScanMetrics;
use crate::models::scan_result::ScanResult;

/// Parses the saved JSON manifest format produced by this CLI.
pub fn parse_scan_result_from_json(json_text: &str) -> Result<ScanResult, String> {
    let compact_json = remove_whitespace_outside_strings(json_text);

    let metrics_json = extract_object_value(&compact_json, "metrics")?;
    let groups_json = extract_array_value(&compact_json, "groups")?;

    Ok(ScanResult {
        metrics: parse_metrics(&metrics_json)?,
        duplicate_groups: parse_groups(&groups_json)?,
    })
}

/// Parses the metrics object from manifest JSON.
fn parse_metrics(metrics_json: &str) -> Result<ScanMetrics, String> {
    Ok(ScanMetrics {
        files_scanned: extract_usize_value(metrics_json, "files_scanned")?,
        bytes_scanned: extract_u64_value(metrics_json, "bytes_scanned")?,
        duplicate_groups: extract_usize_value(metrics_json, "duplicate_groups")?,
        duplicate_files: extract_usize_value(metrics_json, "duplicate_files")?,
        duplicate_bytes: extract_u64_value(metrics_json, "duplicate_bytes")?,
        elapsed_milliseconds: extract_u128_value(metrics_json, "elapsed_milliseconds")?,
    })
}

/// Parses the duplicate group array from manifest JSON.
fn parse_groups(groups_json: &str) -> Result<Vec<DuplicateGroup>, String> {
    let mut groups = Vec::new();

    for group_json in split_top_level_objects(groups_json)? {
        groups.push(DuplicateGroup {
            hash: u64::from_str_radix(&extract_string_value(&group_json, "hash")?, 16)
                .map_err(|_| "Invalid manifest hash value.".to_string())?,
            file_size_bytes: extract_u64_value(&group_json, "file_size_bytes")?,
            file_paths: extract_string_array_value(&group_json, "file_paths")?
                .into_iter()
                .map(PathBuf::from)
                .collect(),
        });
    }

    Ok(groups)
}

/// Removes insignificant whitespace while preserving string content.
fn remove_whitespace_outside_strings(input: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut escaped = false;

    for character in input.chars() {
        if in_string {
            result.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
        } else if character == '"' {
            in_string = true;
            result.push(character);
        } else if !character.is_whitespace() {
            result.push(character);
        }
    }

    result
}

/// Extracts an object value by key from a compact JSON object.
fn extract_object_value(json: &str, key: &str) -> Result<String, String> {
    extract_bracketed_value(json, key, '{', '}')
}

/// Extracts an array value by key from a compact JSON object.
fn extract_array_value(json: &str, key: &str) -> Result<String, String> {
    extract_bracketed_value(json, key, '[', ']')
}

/// Extracts a bracketed value by key from a compact JSON object.
fn extract_bracketed_value(
    json: &str,
    key: &str,
    open: char,
    close: char,
) -> Result<String, String> {
    let key_marker = format!("\"{key}\":");
    let start_index = json
        .find(&key_marker)
        .ok_or_else(|| format!("Manifest key not found: {key}"))?
        + key_marker.len();

    let value_slice = json
        .get(start_index..)
        .ok_or_else(|| format!("Manifest value missing for key: {key}"))?;

    let mut depth = 0_i32;
    let mut end_offset = None;
    let mut in_string = false;
    let mut escaped = false;

    for (offset, character) in value_slice.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }

            continue;
        }

        if character == '"' {
            in_string = true;
            continue;
        }

        if character == open {
            depth += 1;
        } else if character == close {
            depth -= 1;
            if depth == 0 {
                end_offset = Some(offset);
                break;
            }
        }
    }

    let end_offset = end_offset.ok_or_else(|| format!("Manifest key was not closed: {key}"))?;

    Ok(value_slice[..=end_offset].to_string())
}

/// Extracts a string value by key from a compact JSON object.
fn extract_string_value(json: &str, key: &str) -> Result<String, String> {
    let raw_value = extract_scalar_value(json, key)?;

    if !raw_value.starts_with('"') || !raw_value.ends_with('"') {
        return Err(format!("Manifest string value missing quotes: {key}"));
    }

    unescape_json_string(&raw_value[1..raw_value.len() - 1])
}

/// Extracts a usize value by key from a compact JSON object.
fn extract_usize_value(json: &str, key: &str) -> Result<usize, String> {
    extract_scalar_value(json, key)?
        .parse::<usize>()
        .map_err(|_| format!("Invalid manifest usize value: {key}"))
}

/// Extracts a u64 value by key from a compact JSON object.
fn extract_u64_value(json: &str, key: &str) -> Result<u64, String> {
    extract_scalar_value(json, key)?
        .parse::<u64>()
        .map_err(|_| format!("Invalid manifest u64 value: {key}"))
}

/// Extracts a u128 value by key from a compact JSON object.
fn extract_u128_value(json: &str, key: &str) -> Result<u128, String> {
    extract_scalar_value(json, key)?
        .parse::<u128>()
        .map_err(|_| format!("Invalid manifest u128 value: {key}"))
}

/// Extracts a scalar value by key from a compact JSON object.
fn extract_scalar_value(json: &str, key: &str) -> Result<String, String> {
    let key_marker = format!("\"{key}\":");
    let start_index = json
        .find(&key_marker)
        .ok_or_else(|| format!("Manifest key not found: {key}"))?
        + key_marker.len();

    let value_slice = json
        .get(start_index..)
        .ok_or_else(|| format!("Manifest value missing for key: {key}"))?;

    let end_index = value_slice
        .find([',', '}', ']'])
        .ok_or_else(|| format!("Manifest scalar value not terminated: {key}"))?;

    Ok(value_slice[..end_index].to_string())
}

/// Extracts a string array value by key from a compact JSON object.
fn extract_string_array_value(json: &str, key: &str) -> Result<Vec<String>, String> {
    let array_json = extract_array_value(json, key)?;
    let array_body = &array_json[1..array_json.len() - 1];

    if array_body.is_empty() {
        return Ok(Vec::new());
    }

    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut escaped = false;

    for character in array_body.chars() {
        if in_string {
            current.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        if character == '"' {
            in_string = true;
            current.push(character);
        } else if character == ',' {
            values.push(parse_json_array_string(&current)?);
            current.clear();
        }
    }

    if !current.is_empty() {
        values.push(parse_json_array_string(&current)?);
    }

    Ok(values)
}

/// Splits a JSON array of objects into individual object strings.
fn split_top_level_objects(array_json: &str) -> Result<Vec<String>, String> {
    let array_body = &array_json[1..array_json.len() - 1];

    if array_body.is_empty() {
        return Ok(Vec::new());
    }

    let mut objects = Vec::new();
    let mut current = String::new();
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut escaped = false;

    for character in array_body.chars() {
        if in_string {
            current.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        if character == '"' {
            in_string = true;
            current.push(character);
            continue;
        }

        if character == '{' {
            depth += 1;
        } else if character == '}' {
            depth -= 1;
        }

        if character == ',' && depth == 0 {
            objects.push(current.clone());
            current.clear();
            continue;
        }

        current.push(character);
    }

    if depth != 0 {
        return Err("Manifest groups array was not balanced.".to_string());
    }

    if !current.is_empty() {
        objects.push(current);
    }

    Ok(objects)
}

/// Parses one quoted JSON string from an array element.
fn parse_json_array_string(raw_value: &str) -> Result<String, String> {
    let trimmed = raw_value.trim();

    if !trimmed.starts_with('"') || !trimmed.ends_with('"') {
        return Err("Manifest array value was not a quoted string.".to_string());
    }

    unescape_json_string(&trimmed[1..trimmed.len() - 1])
}

/// Unescapes the JSON string values this project emits.
fn unescape_json_string(value: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut characters = value.chars();

    while let Some(character) = characters.next() {
        if character != '\\' {
            result.push(character);
            continue;
        }

        let escaped = characters
            .next()
            .ok_or_else(|| "Manifest string ended with an incomplete escape.".to_string())?;

        match escaped {
            '\\' => result.push('\\'),
            '"' => result.push('"'),
            'n' => result.push('\n'),
            'r' => result.push('\r'),
            't' => result.push('\t'),
            _ => return Err("Manifest string contained an unsupported escape.".to_string()),
        }
    }

    Ok(result)
}
