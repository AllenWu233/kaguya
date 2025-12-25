use crate::models::{DEFAULT_VAULT_DIR, DEFAULT_VAULT_SUBDIR, KaguyaError};
use std::path::PathBuf;

/// Get Kaguya vault path, defaults to '~/.local/share/kaguya/vault' for Linux
pub fn get_vault_path(path: &Option<PathBuf>) -> Result<PathBuf, KaguyaError> {
    if let Some(p) = path {
        return Ok(p.clone());
    }

    let data_dir = dirs::data_local_dir().ok_or_else(|| {
        KaguyaError::DirectoryNotFound("Could not find local data directory.".to_string())
    })?;

    let default_path = data_dir.join(DEFAULT_VAULT_DIR).join(DEFAULT_VAULT_SUBDIR);

    Ok(default_path)
}

// pub fn get_vault_path(path: &Option<PathBuf>) -> PathBuf {
//     if let Some(p) = path {
//         return p.to_path_buf();
//     }
//
//     if let Some(d) = dirs::data_dir() {
//         return d.join(DEFAULT_VAULT_DIR).join(DEFAULT_VAULT_SUBDIR);
//     }
//
//     return "".into();
// }
