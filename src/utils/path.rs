use crate::models::{
    DEFAULT_CONFIG_DIR, DEFAULT_CONFIG_FILE, DEFAULT_VAULT_DIR, DEFAULT_VAULT_SUBDIR, GameConfig,
    KaguyaError,
};
use dirs::{config_dir, home_dir};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

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
pub fn shrink_path<P>(path: &P) -> Result<PathBuf, KaguyaError>
where
    P: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    if let Some(home_dir) = home_dir()
        && let Ok(relative_path) = path.strip_prefix(&home_dir)
    {
        return Ok(PathBuf::from("~").join(relative_path));
    }
    Ok(path.to_path_buf())
}

/// Expands a path string that may contain a tilde `~` into a `PathBuf`.
///
/// The tilde `~` will be expanded to the user's home directory.
/// Otherwise, the original path is returned unchanged.
pub fn expand_path<P>(path: &P) -> Result<PathBuf, KaguyaError>
where
    P: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    if let Some(home_dir) = home_dir()
        && let Ok(relative_path) = path.strip_prefix("~")
    {
        return Ok(home_dir.join(relative_path));
    }
    Ok(path.to_path_buf())
}

/// Converts a relative path to an absolu
///
/// - If the path is already absolute, it is normalized (canonicalized) if it exists.
/// - If the path is relative, it is joined with the current working directory.
/// - If the path does not exist (so canonicalization fails), it returns the
///   expanded absolute path as-is (containing `..` or `.` if not resolved).te path.
pub fn to_absolute_path<P>(path: &P) -> Result<PathBuf, KaguyaError>
where
    P: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();

    if path.is_absolute() {
        return path.canonicalize().or_else(|_| Ok(path.to_path_buf()));
    }

    // Path is relative
    let current_dir = current_dir()?;
    let absolute_path = current_dir.join(path);

    // Try to normalize. If the path doesn't exist yet (e.g., add a new path to backup),
    // canonicalize fails, so we return the joined absolute path directly.
    absolute_path.canonicalize().or_else(|_| Ok(absolute_path))
}

/// Transforms an optional vector of paths using a provided function.
///
/// This generic function allows you to apply any of the previously defined
/// path utility functions (like `shrink_path`, `expand_path`, or `to_absolute_path`)
/// to a list of paths within an `Option`.
///
/// # Arguments
/// * `paths` - An optional vector of paths (e.g., from config).
/// * `transform_fn` - A function that takes a `&Path` and returns a `Result<PathBuf, KaguyaError>`.
///
/// # Examples
///
/// ```
/// // Convert to absolute paths
/// let abs_paths = transform_paths_option(paths, to_absolute_path)?;
///
/// // Shrink paths (replace home dir with ~)
/// let short_paths = transform_paths_option(paths, shrink_path)?;
/// ```
pub fn transform_paths_option<F>(
    paths: Option<Vec<PathBuf>>,
    transform_fn: F,
) -> Result<Option<Vec<PathBuf>>, KaguyaError>
where
    F: Fn(&Path) -> Result<PathBuf, KaguyaError>,
{
    paths
        .map(|vec| vec.into_iter().map(|p| transform_fn(&p)).collect())
        .transpose()
}

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
