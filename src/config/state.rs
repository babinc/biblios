use crate::bible::VerseReference;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Tracks the user's reading position and session state
///
/// This state is persisted between application runs, allowing the user
/// to resume reading where they left off.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingState {
    /// The specific verse reference currently being read (optional)
    pub current_verse: Option<VerseReference>,

    /// The book currently being read (e.g., "John", "Gen")
    pub current_book: Option<String>,

    /// The chapter number currently being read
    pub current_chapter: Option<u32>,

    /// Index of the verse being read within the current chapter (0-based)
    /// This is the verse that will be highlighted and centered on screen
    #[serde(rename = "scroll_offset")] // Keep old name for backward compatibility
    pub current_verse_index: usize,

    /// ISO 8601 timestamp of last update
    pub last_updated: String,
}

impl Default for ReadingState {
    fn default() -> Self {
        Self {
            current_verse: None,
            current_book: Some("John".to_string()),
            current_chapter: Some(1),
            current_verse_index: 0,
            last_updated: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl ReadingState {
    /// Load state from file or create default
    pub fn load() -> Result<Self> {
        let path = Self::state_path()?;

        if path.exists() {
            let content = fs::read_to_string(&path)
                .context("Failed to read state file")?;

            toml::from_str(&content)
                .context("Failed to parse state TOML")
        } else {
            Ok(Self::default())
        }
    }

    /// Save state to file
    pub fn save(&self) -> Result<()> {
        let path = Self::state_path()?;

        let mut state_to_save = self.clone();
        state_to_save.last_updated = chrono::Utc::now().to_rfc3339();

        let content = toml::to_string_pretty(&state_to_save)
            .context("Failed to serialize state")?;

        fs::write(&path, content)
            .context("Failed to write state file")?;

        Ok(())
    }

    /// Update current reading position
    pub fn update_position(&mut self, book: String, chapter: u32, verse: Option<u32>) {
        self.current_book = Some(book.clone());
        self.current_chapter = Some(chapter);

        if let Some(v) = verse {
            self.current_verse = Some(VerseReference::new(book, chapter, v));
        }
    }

    /// Get the state file path
    fn state_path() -> Result<PathBuf> {
        let config_dir = super::config_dir()?;
        Ok(config_dir.join("state.toml"))
    }
}
