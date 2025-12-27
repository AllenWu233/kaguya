//! Compress and decompress games saves and configuration backups

use flate2::{Compression, write::GzEncoder};
use std::{fs::File, path::PathBuf};

use crate::models::KaguyaError;

/// Compress source file or directory to target directory in tar.gz format
///
/// Usage:
/// ```
/// let source: PathBuf = "~/games/game-a/saves"
/// let target: PathBuf = "~/.local/share/kaguya/vault/backups/2025-12-25_10-00-00/saves.tar.gz"
///
/// compress_to_tar_gz(source, target);
/// ```
pub fn compress_to_tar_gz(source: &PathBuf, target: &PathBuf) -> Result<(), KaguyaError> {
    if !source.exists() {
        return Err(KaguyaError::PathNotFound(
            source.to_string_lossy().to_string(),
        ));
    }

    let tar_gz = File::create(target)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    if source.is_file() {
        // tar.append_file(source, &mut File::open(source)?)?;
        let file_name = source
            .file_name()
            .ok_or_else(|| KaguyaError::FileNameError(source.to_string_lossy().to_string()))?;
        tar.append_path_with_name(source, file_name)?;
    } else {
        tar.append_dir_all(".", source)?;
    }

    tar.finish()?;
    Ok(())
}
