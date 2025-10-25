pub mod settings;
pub mod bookmarks;
pub mod state;

pub use settings::Settings;
pub use bookmarks::{Bookmark, BookmarkManager};
pub use state::ReadingState;

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Get the config directory for Biblios
pub fn config_dir() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("biblios");

    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .context("Failed to create config directory")?;
    }

    Ok(dir)
}

/// Get the data directory for Biblios (for Bible databases)
pub fn data_dir() -> Result<PathBuf> {
    let dir = dirs::data_dir()
        .context("Could not determine data directory")?
        .join("biblios");

    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .context("Failed to create data directory")?;
    }

    Ok(dir)
}
