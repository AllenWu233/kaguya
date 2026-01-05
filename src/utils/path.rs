use crate::models::{
    DEFAULT_CONFIG_DIR, DEFAULT_CONFIG_FILE, DEFAULT_VAULT_DIR, DEFAULT_VAULT_SUBDIR, GameConfig,
    KaguyaError,
};
use dirs::{config_dir, home_dir};
use std::path::{Path, PathBuf};

/// Shrink an absolute path back to a tilde-prefixed one.
///
/// If the path is inside the user's home directory, it will be replaced with `~`.
/// Otherwise, the original path is returned unchanged.
///
/// # Examples
///
/// ```
/// # use std::path::PathBuf;
/// let home = dir::home_dir().unwrap();
/// let path = home.join("Documents");
/// assert_eq!(shrink_path(&path), PathBuf::from("~/Documents"));
///
/// let config_home = dirs::config_dir().unwrap();
/// let path = config_home.join("kaguya");
/// assert_eq!(shrink_path(&path), PathBuf::from("~/.config/kaguya"));
/// ```
pub fn shrink_path<P>(path: &P) -> PathBuf
where
    P: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    if let Some(home_dir) = home_dir()
        && let Ok(relative_path) = path.strip_prefix(&home_dir)
    {
        return PathBuf::from("~").join(relative_path);
    }
    path.to_path_buf()
}

/// Expands a path string that may contain a tilde `~` into a `PathBuf`.
///
/// The tilde `~` will be expanded to the user's home directory.
/// Otherwise, the original path is returned unchanged.
pub fn expand_path<P>(path: &P) -> PathBuf
where
    P: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    if let Some(home_dir) = home_dir()
        && let Ok(relative_path) = path.strip_prefix("~")
    {
        return home_dir.join(relative_path);
    }
    path.to_path_buf()
}

// /// Expands a path string (which may contain `~`) and a relative path
// /// into a full, absolute `PathBuf`.
// ///
// /// This is useful for resolving paths that are relative to a known base directory,
// /// like a game's saves directory.
// pub fn expand_path_with_base(path: &impl AsRef<Path>) -> Result<PathBuf, KaguyaError> {
//     let path = path.as_ref();
//
//     // If the path is already absolute, just normalize it and return.
//     if path.is_absolute() {
//         return Ok(path.to_path_buf());
//     }
//
//     // If the path is relative, join it with the base directory.
//     Ok(base_dir.as_ref().join(path))
// }

/// Get Kaguya config path, defaults to '$XDG_CONFIG_HOME/kaguya/config.toml' for Linux.
pub fn get_global_config_path(path: &Option<impl AsRef<Path>>) -> Result<PathBuf, KaguyaError> {
    if let Some(p) = path {
        Ok(p.as_ref().to_path_buf())
    } else {
        let config_dir = config_dir().ok_or_else(|| {
            KaguyaError::PathNotFound("Could not find local data directory.".to_string())
        })?;

        let default_path = config_dir
            .join(DEFAULT_CONFIG_DIR)
            .join(DEFAULT_CONFIG_FILE);

        Ok(default_path)
    }
}

/// Get Kaguya vault dir, defaults to '~/.local/share/kaguya/vault' for Linux.
pub fn get_vault_dir(path: &Option<impl AsRef<Path>>) -> Result<PathBuf, KaguyaError> {
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

/// Finds a game by ID in the game list and return a mutable reference.
pub fn find_game_mut<'a>(games: &'a mut [GameConfig], id: &str) -> Option<&'a mut GameConfig> {
    games.iter_mut().find(|g| g.id == id)
}

/// Finds a game by ID in the game list and return a reference.
pub fn find_game_ref<'a>(games: &'a [GameConfig], id: &str) -> Option<&'a GameConfig> {
    games.iter().find(|g| g.id == id)
}
