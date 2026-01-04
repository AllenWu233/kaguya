//! Compress and decompress games saves and configuration backups

use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use std::{
    fs::{File, create_dir_all},
    path::Path,
};
use tar::Archive;

use crate::{models::KaguyaError, utils::path::get_file_name};

/// Compress source file or directory to target directory in tar.gz format
/// The archive file preserves the top-level directory if dst is a directory.
///
/// Usage:
/// ```
/// let src: PathBuf = "~/games/game-a/saves"
/// let dst: PathBuf = "~/.local/share/kaguya/vault/backups/2025-12-25_10-00-00/saves.tar.gz"
///
/// compress_to_tar_gz(src, dst);
/// ```
pub fn compress_to_tar_gz(
    src: &impl AsRef<Path>,
    dst: &impl AsRef<Path>,
) -> Result<(), KaguyaError> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !src.exists() {
        return Err(KaguyaError::PathNotFound(src.to_string_lossy().to_string()));
    }

    // Build encoder
    let tar_gz = File::create(dst)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    // Create archive
    let src_file_name = get_file_name(src).unwrap_or(".".to_string());
    if src.is_file() {
        tar.append_path_with_name(src, src_file_name)?;
    } else {
        tar.append_dir_all(src_file_name, src)?;
    }

    println!(
        "Successfully compressed '{}' to '{}'.",
        src.display(),
        dst.display()
    );

    tar.finish()?;
    Ok(())
}

/// Decompress a tar.gz file to a target directory.
/// Assuming that the archive file always preserves the top-level directory.
///
/// Usage:
/// ```
/// let src: PathBuf = "~/.local/share/kaguya/vault/backups/2025-12-25_10-00-00/saves.tar.gz"
/// let dst: PathBuf = "~/games/game-a"
///
/// // Will decompress to '~/games/game-a/saves'
/// decompress_from_tar_gz(src, dst)?;
/// ```
pub fn decompress_from_tar_gz(
    src: &impl AsRef<Path>,
    dst: &impl AsRef<Path>,
) -> Result<(), KaguyaError> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !src.exists() {
        return Err(KaguyaError::PathNotFound(src.to_string_lossy().to_string()));
    }

    // Avoid nested same-name directories on extraction
    // e.g., ~/games/game-a/saves/saves
    // let unpack_dir = dst.parent().ok_or_else(|| {
    //     KaguyaError::InvalidInput(format!(
    //         "Destination path '{}' has no parent",
    //         dst.display()
    //     ))
    // })?;
    // if !unpack_dir.exists() {
    //     create_dir_all(unpack_dir)?;
    // }

    if !dst.exists() {
        create_dir_all(dst)?;
    }

    // Build decoder
    let tar_gz = File::open(src)?;
    let decoder = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(decoder);

    archive.unpack(dst)?;

    println!(
        "Successfully decompressed '{}' to '{}'.",
        src.display(),
        dst.display()
    );

    Ok(())
}
