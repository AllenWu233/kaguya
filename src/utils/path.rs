use crate::models::{
    DEFAULT_CONFIG_DIR, DEFAULT_CONFIG_FILE, DEFAULT_VAULT_DIR, DEFAULT_VAULT_SUBDIR, GameConfig,
    KaguyaError,
};
use std::path::{Path, PathBuf};

/// Get Kaguya config path, defaults to '$XDG_CONFIG_HOME/kaguya/config.toml' for Linux.
pub fn get_config_path(path: &Option<impl AsRef<Path>>) -> Result<PathBuf, KaguyaError> {
    if let Some(p) = path {
        Ok(p.as_ref().to_path_buf())
    } else {
        let config_dir = dirs::config_dir().ok_or_else(|| {
            KaguyaError::PathNotFound("Could not find local data directory.".to_string())
        })?;

        let default_path = config_dir
            .join(DEFAULT_CONFIG_DIR)
            .join(DEFAULT_CONFIG_FILE);

        Ok(default_path)
    }
}

/// Get Kaguya vault path, defaults to '~/.local/share/kaguya/vault' for Linux.
pub fn get_vault_path(path: &Option<impl AsRef<Path>>) -> Result<PathBuf, KaguyaError> {
    if let Some(p) = path {
        Ok(p.as_ref().to_path_buf())
    } else {
        let data_dir = dirs::data_local_dir().ok_or_else(|| {
            KaguyaError::PathNotFound("Could not find local data directory.".to_string())
        })?;

        let default_path = data_dir.join(DEFAULT_VAULT_DIR).join(DEFAULT_VAULT_SUBDIR);

        Ok(default_path)
    }
}

/// Get file name from a path string
pub fn get_file_name(path: impl AsRef<Path>) -> Option<String> {
    path.as_ref()
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
}

/// Finds a game by ID in the configuration game list and return a mutable reference.
pub fn find_game_mut<'a>(games: &'a mut [GameConfig], id: &str) -> Option<&'a mut GameConfig> {
    games.iter_mut().find(|g| g.id == id)
}

/// Finds a game by ID in the configuration game list and return a reference.
pub fn find_game_ref<'a>(games: &'a [GameConfig], id: &str) -> Option<&'a GameConfig> {
    games.iter().find(|g| g.id == id)
}
