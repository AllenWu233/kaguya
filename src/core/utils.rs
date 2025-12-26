use crate::models::{
    DEFAULT_CONFIG_DIR, DEFAULT_CONFIG_FILE, DEFAULT_VAULT_DIR, DEFAULT_VAULT_SUBDIR, GameConfig,
    KaguyaError,
};
use std::path::{Path, PathBuf};

/// Get Kaguya config path, defaults to '$XDG_CONFIG_HOME/kaguya/config.toml' for Linux.
pub fn get_config_path<P: AsRef<Path>>(path: &Option<P>) -> Result<PathBuf, KaguyaError> {
    if let Some(p) = path {
        Ok(p.as_ref().to_path_buf())
    } else {
        let config_dir = dirs::config_dir().ok_or_else(|| {
            KaguyaError::DirectoryNotFound("Could not find local data directory.".to_string())
        })?;

        let default_path = config_dir
            .join(DEFAULT_CONFIG_DIR)
            .join(DEFAULT_CONFIG_FILE);

        Ok(default_path)
    }
}

/// Get Kaguya vault path, defaults to '~/.local/share/kaguya/vault' for Linux.
pub fn get_vault_path<P: AsRef<Path>>(path: &Option<P>) -> Result<PathBuf, KaguyaError> {
    if let Some(p) = path {
        Ok(p.as_ref().to_path_buf())
    } else {
        let data_dir = dirs::data_local_dir().ok_or_else(|| {
            KaguyaError::DirectoryNotFound("Could not find local data directory.".to_string())
        })?;

        let default_path = data_dir.join(DEFAULT_VAULT_DIR).join(DEFAULT_VAULT_SUBDIR);

        Ok(default_path)
    }
}

/// Finds a game by ID in the configuration game list and return a mutable reference.
pub fn find_game_mut<'a>(games: &'a mut [GameConfig], id: &str) -> Option<&'a mut GameConfig> {
    games.iter_mut().find(|g| g.id == id)
}
