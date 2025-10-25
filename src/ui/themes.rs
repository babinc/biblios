use ratatui::style::{Color, Modifier, Style};

/// Theme for the application
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub border: Color,
    pub highlight: Color,
    pub verse_number: Color,
    pub bookmark_indicator: Color,
}

impl Theme {
    /// Default theme (light mode inspired)
    pub fn default() -> Self {
        Self {
            name: "default".to_string(),
            background: Color::Reset,
            foreground: Color::White,
            primary: Color::Cyan,
            secondary: Color::Blue,
            accent: Color::Yellow,
            border: Color::Gray,
            highlight: Color::LightBlue,
            verse_number: Color::DarkGray,
            bookmark_indicator: Color::Yellow,
        }
    }

    /// Dark theme
    pub fn dark() -> Self {
        Self {
            name: "dark".to_string(),
            background: Color::Black,
            foreground: Color::White,
            primary: Color::Cyan,
            secondary: Color::Blue,
            accent: Color::Magenta,
            border: Color::Gray,
            highlight: Color::LightCyan,
            verse_number: Color::DarkGray,
            bookmark_indicator: Color::Yellow,
        }
    }

    /// Nord theme (popular color scheme)
    pub fn nord() -> Self {
        Self {
            name: "nord".to_string(),
            background: Color::Rgb(46, 52, 64),
            foreground: Color::Rgb(236, 239, 244),
            primary: Color::Rgb(136, 192, 208),
            secondary: Color::Rgb(129, 161, 193),
            accent: Color::Rgb(163, 190, 140),
            border: Color::Rgb(76, 86, 106),
            highlight: Color::Rgb(143, 188, 187),
            verse_number: Color::Rgb(216, 222, 233),
            bookmark_indicator: Color::Rgb(235, 203, 139),
        }
    }

    /// Gruvbox theme
    pub fn gruvbox() -> Self {
        Self {
            name: "gruvbox".to_string(),
            background: Color::Rgb(40, 40, 40),
            foreground: Color::Rgb(235, 219, 178),
            primary: Color::Rgb(184, 187, 38),
            secondary: Color::Rgb(152, 151, 26),
            accent: Color::Rgb(215, 153, 33),
            border: Color::Rgb(80, 73, 69),
            highlight: Color::Rgb(131, 165, 152),
            verse_number: Color::Rgb(146, 131, 116),
            bookmark_indicator: Color::Rgb(250, 189, 47),
        }
    }

    /// Solarized Dark theme
    pub fn solarized_dark() -> Self {
        Self {
            name: "solarized-dark".to_string(),
            background: Color::Rgb(0, 43, 54),
            foreground: Color::Rgb(131, 148, 150),
            primary: Color::Rgb(38, 139, 210),
            secondary: Color::Rgb(42, 161, 152),
            accent: Color::Rgb(133, 153, 0),
            border: Color::Rgb(7, 54, 66),
            highlight: Color::Rgb(108, 113, 196),
            verse_number: Color::Rgb(88, 110, 117),
            bookmark_indicator: Color::Rgb(181, 137, 0),
        }
    }

    /// Monokai theme
    pub fn monokai() -> Self {
        Self {
            name: "monokai".to_string(),
            background: Color::Rgb(39, 40, 34),
            foreground: Color::Rgb(248, 248, 242),
            primary: Color::Rgb(102, 217, 239),
            secondary: Color::Rgb(174, 129, 255),
            accent: Color::Rgb(249, 38, 114),
            border: Color::Rgb(73, 72, 62),
            highlight: Color::Rgb(166, 226, 46),
            verse_number: Color::Rgb(117, 113, 94),
            bookmark_indicator: Color::Rgb(230, 219, 116),
        }
    }

    /// Get style for normal text
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.foreground)
    }

    /// Get style for headers/titles
    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for highlighted/selected items
    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.highlight)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for borders
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Get style for verse numbers
    pub fn verse_number_style(&self) -> Style {
        Style::default()
            .fg(self.verse_number)
            .add_modifier(Modifier::DIM)
    }

    /// Get style for bookmarks
    pub fn bookmark_style(&self) -> Style {
        Style::default()
            .fg(self.bookmark_indicator)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for secondary text
    pub fn secondary_style(&self) -> Style {
        Style::default().fg(self.secondary)
    }
}

/// Get theme by name
pub fn get_theme(name: &str) -> Theme {
    match name.to_lowercase().as_str() {
        "dark" => Theme::dark(),
        "nord" => Theme::nord(),
        "gruvbox" => Theme::gruvbox(),
        "solarized-dark" | "solarized" => Theme::solarized_dark(),
        "monokai" => Theme::monokai(),
        _ => Theme::default(),
    }
}

/// Get list of all available theme names
pub fn available_themes() -> Vec<String> {
    vec![
        "default".to_string(),
        "dark".to_string(),
        "nord".to_string(),
        "gruvbox".to_string(),
        "solarized-dark".to_string(),
        "monokai".to_string(),
    ]
}
