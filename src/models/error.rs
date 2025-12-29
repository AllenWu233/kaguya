//! Custom error type for Kaguya

use thiserror::Error;

/// Unified error type for the Kaguya application.
#[derive(Error, Debug)]
pub enum KaguyaError {
    /// Represents an error from the file system I/O.
    #[error("File system error: {0}")]
    Io(#[from] std::io::Error),

    /// Represents an error from parsing a TOML configuration file.
    #[error("Could not parse config file: {0}")]
    TomlParseError(#[from] toml::de::Error),

    /// Represents an error from serializing to TOML.
    #[error("Could not serialize data to TOML: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),

    /// An error for when a path cannot be found.
    #[error("Could not find path: {0}")]
    PathNotFound(String),

    /// An error for when cannot get a file name from path
    #[error("Cound not determine file name from {0}")]
    FileNameError(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// Represents a custom business logic error.
    #[error("Game with id '{0}' not found.")]
    GameNotFound(String),

    #[error("Backup with ID '{0}' not found.")]
    BackupNotFound(i64),

    #[error("No paths configured for game with external_id '{0}'.")]
    NoPathsConfigured(String),
}
