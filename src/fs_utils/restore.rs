use crate::fs_utils::archive::decompress_from_tar_gz;
use crate::models::KaguyaError;
use rand::{Rng, distr::Alphanumeric};
use scopeguard::defer;
use std::fs::{create_dir_all, remove_dir_all, remove_file, rename};
use std::path::Path;
use std::process;

/// Restore single archive from `src` to `dst`.
/// Old saves or configurations will be removed.
///
/// This function assumes the archive was created by kaguya, and therefore
/// its contents are wrapped in a single top-level directory.
///
/// Usage:
/// ```
/// let src: PathBuf = "~/.local/share/kaguya/vault/backups/2025-12-25_10-00-00/saves.tar.gz"
/// let dst: PathBuf = "~/games/game-a/saves"
///
/// // Restore to '~/games/game-a/saves'
/// restore_archive(src, dst)?;
/// ```
pub fn restore_archive(src: &impl AsRef<Path>, dst: &impl AsRef<Path>) -> Result<(), KaguyaError> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    let temp_dir_name = generate_unique_temp_name(".kaguya-restore", 8);
    let temp_dir = dst
        .parent()
        .ok_or_else(|| {
            KaguyaError::InvalidInput(format!(
                "Destination path '{}' has no parent",
                dst.display()
            ))
        })?
        .join(&temp_dir_name);
    create_dir_all(&temp_dir)?;

    // Ensure the temporary directory is always cleaned up.
    defer! {
        remove_dir_all(&temp_dir).ok();
    }

    let unpacked_path = temp_dir.join(dst.file_name().ok_or_else(|| {
        KaguyaError::InvalidInput(format!(
            "Destination path '{}' has no file name",
            dst.display()
        ))
    })?);

    decompress_from_tar_gz(&src, &temp_dir)?;

    if dst.exists() {
        if dst.is_dir() {
            remove_dir_all(dst)?;
        } else if dst.is_file() {
            remove_file(dst)?;
        } else {
            return Err(KaguyaError::InvalidInput(format!(
                "Invalid type of path: {}",
                dst.display()
            )));
        }
    }
    rename(unpacked_path, dst)?;

    Ok(())
}

// Generate a temp name with prefix + process ID + random string
fn generate_unique_temp_name(prefix: &str, rnd_str_len: u32) -> String {
    format!(
        "{}-{}-{}",
        prefix,
        process::id(),
        (0..rnd_str_len)
            .map(|_| rand::rng().sample(Alphanumeric) as char)
            .collect::<String>()
    )
}
