use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Current translation ID
    pub translation: String,

    /// Input mode: "normal" or "vim"
    pub input_mode: InputMode,

    /// Current theme
    pub theme: String,

    /// Font size (for future terminal size adjustments)
    pub font_size: u16,

    /// Show verse numbers
    pub show_verse_numbers: bool,

    /// Add spacing between verses
    pub verse_spacing: bool,

    /// Number of verses per page
    pub verses_per_page: usize,

    /// Search result limit
    pub search_limit: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputMode {
    Normal,
    Vim,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            translation: "KJV".to_string(),
            input_mode: InputMode::Normal,
            theme: "default".to_string(),
            font_size: 14,
            show_verse_numbers: true,
            verse_spacing: true,
            verses_per_page: 20,
            search_limit: 100,
        }
    }
}

impl Settings {
    /// Load settings from file or create default
    pub fn load() -> Result<Self> {
        let path = Self::settings_path()?;

        if path.exists() {
            let content = fs::read_to_string(&path)
                .context("Failed to read settings file")?;

            toml::from_str(&content)
                .context("Failed to parse settings TOML")
        } else {
            let settings = Self::default();
            settings.save()?;
            Ok(settings)
        }
    }

    /// Save settings to file
    pub fn save(&self) -> Result<()> {
        let path = Self::settings_path()?;

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize settings")?;

        fs::write(&path, content)
            .context("Failed to write settings file")?;

        Ok(())
    }

    /// Get the settings file path
    fn settings_path() -> Result<PathBuf> {
        let config_dir = super::config_dir()?;
        Ok(config_dir.join("settings.toml"))
    }

    /// Toggle input mode
    pub fn toggle_input_mode(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Normal => InputMode::Vim,
            InputMode::Vim => InputMode::Normal,
        };
    }
}
