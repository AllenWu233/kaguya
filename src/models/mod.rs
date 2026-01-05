//! Config / Request / Service struct, constants, and custom error type

pub use constants::*;
pub use db::{Game, GamePath};
pub use error::KaguyaError;
pub use requests::{AddGameRequest, BackupRequest, ListGameRequest, RmGameRequest};
pub use vault_config::{GameConfig, VaultConfig};

pub mod constants;
pub mod db;
pub mod error;
pub mod events;
pub mod global_config;
pub mod requests;
pub mod vault_config;
