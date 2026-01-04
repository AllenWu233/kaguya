//! Requests for core services

use std::path::PathBuf;

/// Represents a request to add a new game, coming directly from the CLI
#[derive(Debug)]
pub struct AddGameRequest<'a> {
    pub id: &'a str,
    pub name: Option<&'a str>,
    pub paths: Option<&'a Vec<PathBuf>>,
    pub comment: Option<&'a str>,
}

/// Represents a request to list games, coming directly from [`ConfigSubcommands`]
#[derive(Debug)]
pub struct ListGameRequest<'a> {
    pub long: &'a bool,
}

/// Represents a request to remove a game, coming directly from [`ConfigSubcommands`]
#[derive(Debug)]
pub struct RmGameRequest<'a> {
    pub id: &'a str,
    pub purge: &'a bool,
}

/// Represents a request to action backup, coming directly from the CLI
#[derive(Debug)]
pub struct BackupRequest<'a> {
    pub id: Option<&'a str>,
    pub paths: Option<&'a Vec<PathBuf>>,
}

/// Represents a request to action restore, coming directly from the CLI
#[derive(Debug)]
pub struct RestoreRequest<'a> {
    pub id: &'a str,
    pub version: Option<&'a str>,
    pub paths: Option<&'a Vec<PathBuf>>,
}
