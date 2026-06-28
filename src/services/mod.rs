// Exposes the service modules that implement scanning and duplicate detection.
// Connects to: src/main.rs, tests/services/
// Created: 2026-06-28

pub mod content_comparer;
pub mod directory_scanner;
pub mod duplicate_finder;
pub mod hash_grouping;
pub mod hash_service;
pub mod manifest_diff;
