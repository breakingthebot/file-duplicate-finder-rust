// Calculates deterministic file hashes for duplicate detection.
// Connects to: src/services/duplicate_finder.rs
// Created: 2026-06-28

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const BUFFER_SIZE_BYTES: usize = 8 * 1024;
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// Hashes a file's contents using FNV-1a for deterministic duplicate grouping.
pub fn hash_file(file_path: &Path) -> Result<u64, String> {
    let file = File::open(file_path)
        .map_err(|error| format!("Failed to open file {}: {error}", file_path.display()))?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0_u8; BUFFER_SIZE_BYTES];
    let mut hash = FNV_OFFSET_BASIS;

    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .map_err(|error| format!("Failed to read file {}: {error}", file_path.display()))?;

        if bytes_read == 0 {
            break;
        }

        hash = update_fnv1a_hash(hash, &buffer[..bytes_read]);
    }

    Ok(hash)
}

/// Updates an in-progress FNV-1a hash with the next chunk of file bytes.
fn update_fnv1a_hash(current_hash: u64, bytes: &[u8]) -> u64 {
    let mut hash = current_hash;

    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}
