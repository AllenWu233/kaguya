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

    /// An error for when a directory cannot be found.
    #[error("Could not find directory: {0}")]
    DirectoryNotFound(String),

    /// Represents a custom business logic error.
    #[error("A game with ID '{0}' not found.")]
    GameNotFound(String),
}
