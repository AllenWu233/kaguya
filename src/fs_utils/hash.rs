use crate::models::KaguyaError;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

/// Calculates checksum for a file or directory.
/// Directly hashes files; recursively hashes directories.
pub fn calculate_entry_checksum<P: AsRef<Path>>(path: P) -> Result<String, KaguyaError> {
    let p = path.as_ref();

    if !p.exists() {
        return Err(KaguyaError::PathNotFound(p.to_string_lossy().to_string()));
    }

    if p.is_file() {
        calculate_file_hash(p)
    } else if p.is_dir() {
        calculate_dir_checksum(p)
    } else {
        Err(KaguyaError::InvalidInput(format!(
            "Unsupported file type with '{}'",
            p.display()
        )))
    }
}

/// Calculate SHA-256 hash of the file
fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String, KaguyaError> {
    let file = File::open(path.as_ref())?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192]; // 8KB

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Recursively calculates directory checksum.
fn calculate_dir_checksum(dir: &Path) -> Result<String, KaguyaError> {
    let mut hasher = Sha256::new();

    // Use BTreeMap to ensure paths are processed alphabetically (Determinism)
    let mut file_hashes: BTreeMap<String, String> = BTreeMap::new();

    collect_hashes_recursive(dir, dir, &mut file_hashes)?;

    // Feed sorted paths and file hashes into final hasher
    for (rel_path, content_hash) in file_hashes {
        hasher.update(rel_path.as_bytes());
        hasher.update(content_hash.as_bytes());
    }

    Ok(hex::encode(hasher.finalize()))
}

/// Recursively collects file hashes into the accumulator.
fn collect_hashes_recursive(
    root: &Path,
    current_dir: &Path,
    acc: &mut BTreeMap<String, String>,
) -> Result<(), KaguyaError> {
    let entries = fs::read_dir(current_dir)?;

    for entry in entries {
        let path = entry?.path();

        if path.is_file() {
            // Calculate relative path to ensure structure awareness
            let rel_path = path
                .strip_prefix(root)
                .map_err(|e| KaguyaError::InvalidInput(format!("Prefix strip error: {}", e)))?
                .to_string_lossy()
                .to_string();

            let hash = calculate_file_hash(&path)?;
            acc.insert(rel_path, hash);
        } else if path.is_dir() {
            collect_hashes_recursive(root, &path, acc)?;
        }
    }

    Ok(())
}
