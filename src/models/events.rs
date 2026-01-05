use std::path::PathBuf;

/// Represents a single event that occurred during a backup operation.
#[derive(Debug, Clone)]
pub enum BackupEvent {
    /// A new backup was created.
    Created {
        external_id: String,
        total_files: usize,
        total_size_bytes: u64,
    },

    /// A single file was successfully backed up.
    FileBackedUp {
        original_path: PathBuf,
        archive_path: PathBuf,
        size_bytes: u64,
    },

    /// A file was skipped because it was not found.
    FileSkipped {
        original_path: PathBuf,
        reason: String,
    },

    Error {
        original_path: PathBuf,
        error_string: String,
    },
}
