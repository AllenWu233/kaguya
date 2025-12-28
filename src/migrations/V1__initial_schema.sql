-- =====================================
-- Kaguya Database Initialization Script
-- Version: 1.0
-- =====================================

-- Enable foreign key constraints for the current connection.
-- This is necessary because SQLite disables foreign key constraints by default for backward compatibility.
PRAGMA foreign_keys = ON;

-- Stores basic configuration information for games.
CREATE TABLE game (
    id INTEGER PRIMARY KEY,                           -- Internal auto-incrementing primary key
    external_id TEXT NOT NULL UNIQUE,                 -- External ID (e.g., outer_wilds), globally unique
    name TEXT NOT NULL,                               -- User-friendly name of the game
    comment TEXT,                                     -- User comment
    keep_versions INTEGER,                            -- Default number of backups to keep
    created_at TEXT NOT NULL,                         -- Creation datetime timestamp (DATETIME/TIMESTAMP is stored as TEXT in SQLite)
    updated_at TEXT NOT NULL                          -- Last updated datetime
);

-- Stores a list of original paths to be backed up for each game.
CREATE TABLE path (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL,                         -- Associated game ID
    original_path TEXT NOT NULL,                      -- The original path

    -- Foreign key constraint: If a game is deleted, all its path configurations are also deleted in a cascade.
    FOREIGN KEY (game_id) REFERENCES game(id) ON DELETE CASCADE,

    -- Composite unique constraint: Ensures that duplicate paths are not added for the same game.
    UNIQUE(game_id, original_path)
);

-- Records metadata for each backup operation.
CREATE TABLE backup (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL,                         -- Associated game ID
    version TEXT NOT NULL,                            -- Backup version (A formatted local time string, e.g., 2025-12-25_10-00-00)
    timestamp TEXT NOT NULL,                          -- Timestamp of when the backup was created
    total_size_bytes INTEGER NOT NULL,                -- Total size of the entire backup set
    checksum TEXT NOT NULL,                           -- Checksum for the entire backup set

    -- Foreign key constraint: If a game is deleted, all its backups are NOT deleted in a cascade.
    FOREIGN KEY (game_id) REFERENCES game(id),

    -- Composite unique constraint: Ensures that the version string is unique for each game.
    UNIQUE(game_id, version)
);

-- Records each compressed file contained within a single backup.
CREATE TABLE backup_file (
    id INTEGER PRIMARY KEY,
    backup_id INTEGER NOT NULL,                       -- Associated backup ID
    original_path TEXT NOT NULL,                      -- The original path of the file
    archive_filename TEXT NOT NULL,                   -- The filename of the compressed file stored in the vault
    size_bytes INTEGER NOT NULL,                      -- Size of this compressed file
    checksum TEXT NOT NULL,                           -- Checksum for this compressed file

    -- Foreign key constraint: If a backup is deleted, all its file records are also deleted in a cascade.
    FOREIGN KEY (backup_id) REFERENCES backup(id) ON DELETE CASCADE
);

-- Records the history of all key operations for auditing purposes.
CREATE TABLE event (
    id INTEGER PRIMARY KEY,
    event_type TEXT NOT NULL,                         -- Type of the event (e.g., 'backup', 'restore', 'prune')
    game_id INTEGER NOT NULL,                         -- Associated game ID
    backup_id INTEGER,                                -- Associated backup ID (can be NULL, as not all events are related to a specific backup)
    timestamp TEXT NOT NULL,                          -- Timestamp of when the event occurred

    FOREIGN KEY (game_id) REFERENCES game(id),
    FOREIGN KEY (backup_id) REFERENCES backup(id)
);


-- Create indexes on columns frequently used for queries, JOINs, and sorting to significantly improve performance.

-- Indexes for the path table
CREATE INDEX idx_path_game_id ON path(game_id);

-- Indexes for the backup table
CREATE INDEX idx_backup_game_id ON backup(game_id);
-- This composite index supports both the UNIQUE constraint and fast queries by game + version.
CREATE INDEX idx_backup_game_version ON backup(game_id, version);

-- Indexes for the backup_file table
CREATE INDEX idx_backup_file_backup_id ON backup_file(backup_id);

-- Indexes for the event table
CREATE INDEX idx_event_game_id ON event(game_id);
CREATE INDEX idx_event_backup_id ON event(backup_id);
CREATE INDEX idx_event_timestamp ON event(timestamp);
