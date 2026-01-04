use chrono::{DateTime, Utc};

use crate::models::GameConfig;

#[derive(Debug)]
pub struct Game {
    /// ID for the table
    pub id: Option<i64>,

    /// Game ID, e.g., "outer_wilds"
    pub external_id: String,

    /// Friendly game name
    pub name: String,

    /// Alternative comment
    pub comment: Option<String>,

    /// How many versions to keep when acting prune, cover global config
    pub keep_versions: Option<i64>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&GameConfig> for Game {
    fn from(game: &GameConfig) -> Self {
        let created_at = Utc::now();
        let updated_at = created_at;

        Self {
            id: None,
            external_id: game.id.clone(),
            name: game.name.clone(),
            comment: game.comment.clone(),
            keep_versions: game.keep_versions,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug)]
pub struct GamePath {
    pub id: Option<i64>,
    pub game_id: Option<i64>,
    pub original_path: String,
}

/// Use for `get_db_path_list`
#[derive(Debug)]
pub struct DbPathInfo {
    pub id: i64,
    pub external_id: String,
    pub original_path: String,
}

#[derive(Debug)]
pub struct Backup {
    pub id: Option<i64>,
    pub game_id: i64,
    pub version: String,
    pub timestaqmp: String,
    pub total_size_bytes: i64,
    pub checksum: String,
}

#[derive(Debug)]
pub struct BackupFile {
    pub id: Option<i64>,
    pub backup_id: i64,
    pub original_path: String,
    pub archive_path: String,
    pub size_bytes: i64,
    pub checksum: String,
}

#[derive(Debug)]
pub struct Event {
    pub id: Option<i64>,
    pub event_type: String,
    pub game_id: i64,
    pub backup_id: Option<i64>,
    pub timestamp: String,
}
