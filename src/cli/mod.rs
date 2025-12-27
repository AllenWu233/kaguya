//! CLI entry

// pub use handlers::config::handle_config;
pub use context::AppContext;
pub use handlers::config::handle_config;
pub use handlers::vault::handle_vault;
pub use parser::{Cli, Commands, ConfigSubcommands};

pub mod context;
pub mod handlers;
pub mod parser;
