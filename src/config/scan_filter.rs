// Defines scan exclusion rules used to skip matching files and directories.
// Connects to: src/config/cli.rs, src/services/directory_scanner.rs, src/main.rs
// Created: 2026-06-28

use std::path::Path;

/// Stores normalized exclusion rules for directory scanning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanFilter {
    pub excluded_names: Vec<String>,
    pub excluded_relative_paths: Vec<Vec<String>>,
}

impl ScanFilter {
    /// Creates an empty scan filter that excludes nothing.
    pub fn empty() -> Self {
        Self {
            excluded_names: Vec::new(),
            excluded_relative_paths: Vec::new(),
        }
    }

    /// Adds one exclusion rule from CLI text as either a name or relative path rule.
    pub fn add_exclusion(&mut self, raw_rule: &str) -> Result<(), String> {
        let normalized_components = normalize_rule_components(raw_rule)?;

        if raw_rule.contains(['\\', '/']) {
            self.excluded_relative_paths.push(normalized_components);
        } else {
            self.excluded_names.push(
                normalized_components
                    .first()
                    .cloned()
                    .ok_or_else(|| "Exclude rule was unexpectedly empty.".to_string())?,
            );
        }

        Ok(())
    }

    /// Returns true when the relative path should be skipped during scanning.
    pub fn excludes(&self, relative_path: &Path) -> bool {
        let normalized_components = normalize_relative_path(relative_path);

        if normalized_components
            .iter()
            .any(|component| self.excluded_names.iter().any(|rule| rule == component))
        {
            return true;
        }

        self.excluded_relative_paths
            .iter()
            .any(|rule_components| normalized_components.starts_with(rule_components))
    }
}

/// Normalizes an exclusion rule into lowercase path components.
fn normalize_rule_components(raw_rule: &str) -> Result<Vec<String>, String> {
    let normalized_components = raw_rule
        .split(['\\', '/'])
        .filter(|component| !component.is_empty() && *component != ".")
        .map(|component| component.to_lowercase())
        .collect::<Vec<String>>();

    if normalized_components.is_empty() {
        return Err(format!("Invalid --exclude value: {raw_rule}"));
    }

    Ok(normalized_components)
}

/// Normalizes a relative filesystem path into lowercase components.
fn normalize_relative_path(relative_path: &Path) -> Vec<String> {
    relative_path
        .components()
        .filter_map(|component| {
            let text = component.as_os_str().to_str()?;
            if text.is_empty() || text == "." {
                None
            } else {
                Some(text.to_lowercase())
            }
        })
        .collect()
}
