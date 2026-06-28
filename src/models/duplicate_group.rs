// Defines the duplicate file group model returned by the scanning service.
// Connects to: src/services/duplicate_finder.rs, src/utils/formatting.rs
// Created: 2026-06-28

use std::path::PathBuf;

/// Represents one set of files that share identical content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateGroup {
    pub hash: u64,
    pub file_size_bytes: u64,
    pub file_paths: Vec<PathBuf>,
}
