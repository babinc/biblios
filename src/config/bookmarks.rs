use crate::bible::VerseReference;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Represents a bookmark with optional notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub reference: VerseReference,
    pub note: Option<String>,
    pub created_at: String, // ISO 8601 timestamp
}

impl Bookmark {
    pub fn new(reference: VerseReference) -> Self {
        Self {
            reference,
            note: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_note(reference: VerseReference, note: String) -> Self {
        Self {
            reference,
            note: Some(note),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Manages bookmarks
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BookmarkManager {
    bookmarks: Vec<Bookmark>,
}

impl BookmarkManager {
    /// Load bookmarks from file or create empty
    pub fn load() -> Result<Self> {
        let path = Self::bookmarks_path()?;

        if path.exists() {
            let content = fs::read_to_string(&path)
                .context("Failed to read bookmarks file")?;

            toml::from_str(&content)
                .context("Failed to parse bookmarks TOML")
        } else {
            Ok(Self::default())
        }
    }

    /// Save bookmarks to file
    pub fn save(&self) -> Result<()> {
        let path = Self::bookmarks_path()?;

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize bookmarks")?;

        fs::write(&path, content)
            .context("Failed to write bookmarks file")?;

        Ok(())
    }

    /// Add a bookmark
    pub fn add(&mut self, bookmark: Bookmark) {
        // Remove existing bookmark for the same reference
        self.bookmarks.retain(|b| b.reference != bookmark.reference);
        self.bookmarks.push(bookmark);
    }

    /// Remove a bookmark
    pub fn remove(&mut self, reference: &VerseReference) -> bool {
        let len_before = self.bookmarks.len();
        self.bookmarks.retain(|b| &b.reference != reference);
        len_before != self.bookmarks.len()
    }

    /// Check if a verse is bookmarked
    pub fn is_bookmarked(&self, reference: &VerseReference) -> bool {
        self.bookmarks.iter().any(|b| &b.reference == reference)
    }

    /// Get all bookmarks
    pub fn all(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    /// Get the bookmarks file path
    fn bookmarks_path() -> Result<PathBuf> {
        let config_dir = super::config_dir()?;
        Ok(config_dir.join("bookmarks.toml"))
    }
}
