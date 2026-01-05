//! Requests for core services

use std::path::PathBuf;

/// Represents a request to add a new game, coming directly from the CLI
#[derive(Debug)]
pub struct AddGameRequest {
    pub id: String,
    pub name: Option<String>,
    pub paths: Option<Vec<PathBuf>>,
    pub comment: Option<String>,
}

/// Represents a request to list games, coming directly from [`ConfigSubcommands`]
#[derive(Debug)]
pub struct ListGameRequest {
    pub long: bool,
}

/// Represents a request to remove a game, coming directly from [`ConfigSubcommands`]
#[derive(Debug)]
pub struct RmGameRequest {
    pub id: String,
    pub purge: bool,
}

/// Represents a request to action backup, coming directly from the CLI
#[derive(Debug)]
pub struct BackupRequest {
    pub id: Option<String>,
    pub paths: Option<Vec<PathBuf>>,
}

/// Represents a request to action restore, coming directly from the CLI
#[derive(Debug)]
pub struct RestoreRequest {
    pub id: String,
    pub version: Option<String>,
    pub paths: Option<Vec<PathBuf>>,
}
