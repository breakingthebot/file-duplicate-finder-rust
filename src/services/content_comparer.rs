// Confirms that two files are byte-for-byte identical after a hash match.
// Connects to: src/services/duplicate_finder.rs
// Created: 2026-06-28

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const BUFFER_SIZE_BYTES: usize = 8 * 1024;

/// Compares two files chunk-by-chunk and returns true when their contents match.
pub fn files_match(left_path: &Path, right_path: &Path) -> Result<bool, String> {
    let left_file = File::open(left_path)
        .map_err(|error| format!("Failed to open file {}: {error}", left_path.display()))?;
    let right_file = File::open(right_path)
        .map_err(|error| format!("Failed to open file {}: {error}", right_path.display()))?;

    let mut left_reader = BufReader::new(left_file);
    let mut right_reader = BufReader::new(right_file);
    let mut left_buffer = [0_u8; BUFFER_SIZE_BYTES];
    let mut right_buffer = [0_u8; BUFFER_SIZE_BYTES];

    loop {
        let left_read = left_reader
            .read(&mut left_buffer)
            .map_err(|error| format!("Failed to read file {}: {error}", left_path.display()))?;
        let right_read = right_reader
            .read(&mut right_buffer)
            .map_err(|error| format!("Failed to read file {}: {error}", right_path.display()))?;

        if left_read != right_read {
            return Ok(false);
        }

        if left_read == 0 {
            return Ok(true);
        }

        if left_buffer[..left_read] != right_buffer[..right_read] {
            return Ok(false);
        }
    }
}
