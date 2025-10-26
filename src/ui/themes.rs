use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, BorderType};

/// Comprehensive theme system for consistent UI styling across the entire application
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,

    // Background colors
    pub bg_primary: Color,        // Main content background
    pub bg_secondary: Color,       // Headers, sidebars
    pub bg_tertiary: Color,        // Status bars
    pub bg_modal: Color,           // Modal/overlay background
    pub bg_selection: Color,       // Selected item background
    pub bg_current: Color,         // Current verse/item background

    // Text colors
    pub text_primary: Color,       // Main text
    pub text_secondary: Color,     // Context text
    pub text_muted: Color,         // Verse numbers, help text
    pub text_heading: Color,       // Titles, headers
    pub text_current: Color,       // Current verse text
    pub text_inverse: Color,       // Text on colored backgrounds

    // Border colors
    pub border_normal: Color,      // Standard borders
    pub border_active: Color,      // Active/focused borders
    pub border_inactive: Color,    // Inactive borders

    // Accent colors
    pub accent_primary: Color,     // Primary accent (links, highlights)
    pub accent_secondary: Color,   // Secondary accent
    pub accent_success: Color,     // Success states
    pub accent_warning: Color,     // Warning states
    pub accent_error: Color,       // Error states
    pub accent_info: Color,        // Info states

    // Special colors
    pub bookmark_color: Color,     // Bookmark indicator
    pub search_highlight: Color,   // Search result highlight
}

impl Theme {
    /// Create a new theme with the given name and colors
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),

            // Defaults (dark theme)
            bg_primary: Color::Rgb(15, 15, 25),
            bg_secondary: Color::Rgb(20, 20, 30),
            bg_tertiary: Color::Rgb(25, 25, 35),
            bg_modal: Color::Rgb(20, 20, 30),
            bg_selection: Color::Rgb(40, 40, 60),
            bg_current: Color::Rgb(40, 40, 60),

            text_primary: Color::Rgb(220, 220, 240),
            text_secondary: Color::Rgb(200, 200, 220),
            text_muted: Color::Rgb(120, 120, 150),
            text_heading: Color::Rgb(150, 200, 255),
            text_current: Color::Rgb(255, 255, 255),
            text_inverse: Color::Rgb(0, 0, 0),

            border_normal: Color::Rgb(80, 80, 120),
            border_active: Color::Rgb(100, 200, 255),
            border_inactive: Color::Rgb(60, 60, 90),

            accent_primary: Color::Rgb(100, 200, 255),
            accent_secondary: Color::Rgb(150, 200, 255),
            accent_success: Color::Rgb(100, 200, 100),
            accent_warning: Color::Rgb(255, 200, 100),
            accent_error: Color::Rgb(255, 100, 100),
            accent_info: Color::Rgb(100, 150, 255),

            bookmark_color: Color::Rgb(255, 200, 100),
            search_highlight: Color::Rgb(255, 255, 100),
        }
    }

    // ==================== Style Builders ====================
    // These methods provide consistent styling for all UI elements

    /// Default text style for primary content
    pub fn text(&self) -> Style {
        Style::default()
            .fg(self.text_primary)
            .bg(self.bg_primary)
    }

    /// Secondary text style for context
    pub fn text_secondary(&self) -> Style {
        Style::default()
            .fg(self.text_secondary)
            .bg(self.bg_primary)
    }

    /// Muted text style for hints and labels
    pub fn text_muted(&self) -> Style {
        Style::default().fg(self.text_muted)
    }

    /// Heading text style for titles
    pub fn heading(&self) -> Style {
        Style::default()
            .fg(self.text_heading)
            .add_modifier(Modifier::BOLD)
    }

    /// Current item style (highlighted)
    pub fn current(&self) -> Style {
        Style::default()
            .fg(self.text_current)
            .bg(self.bg_current)
            .add_modifier(Modifier::BOLD)
    }

    /// Selected item style
    pub fn selected(&self) -> Style {
        Style::default()
            .fg(self.accent_primary)
            .bg(self.bg_selection)
            .add_modifier(Modifier::BOLD)
    }

    /// Accent text style
    pub fn accent(&self) -> Style {
        Style::default()
            .fg(self.accent_primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Success message style
    pub fn success(&self) -> Style {
        Style::default()
            .fg(self.accent_success)
            .add_modifier(Modifier::BOLD)
    }

    /// Warning message style
    pub fn warning(&self) -> Style {
        Style::default()
            .fg(self.accent_warning)
            .add_modifier(Modifier::BOLD)
    }

    /// Error message style
    pub fn error(&self) -> Style {
        Style::default()
            .fg(self.accent_error)
            .add_modifier(Modifier::BOLD)
    }

    /// Info message style
    pub fn info(&self) -> Style {
        Style::default()
            .fg(self.accent_info)
    }

    /// Bookmark indicator style
    pub fn bookmark(&self) -> Style {
        Style::default().fg(self.bookmark_color)
    }

    /// Verse number style
    pub fn verse_number(&self) -> Style {
        Style::default().fg(self.text_muted)
    }

    /// Verse number for current verse
    pub fn verse_number_current(&self) -> Style {
        Style::default()
            .fg(self.accent_warning)
            .add_modifier(Modifier::BOLD)
    }

    // ==================== Block Builders ====================
    // Standard block builders that automatically apply theme

    /// Create a standard block with rounded borders
    pub fn block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.border_normal))
            .style(Style::default().bg(self.bg_primary))
    }

    /// Create an active/focused block with highlighted border
    pub fn block_active(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.border_active))
            .style(Style::default().bg(self.bg_primary))
    }

    /// Create a secondary block (for sidebars, panels)
    pub fn block_secondary(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.border_normal))
            .style(Style::default().bg(self.bg_secondary))
    }

    /// Create a modal block (for popups, dialogs)
    pub fn block_modal(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.border_active))
            .style(Style::default().bg(self.bg_modal))
    }

    /// Create a titled block with consistent heading style
    pub fn block_with_title(&self, title: impl Into<String>) -> Block<'static> {
        self.block()
            .title(format!(" {} ", title.into()))
            .title_style(self.heading())
    }

    /// Create a titled active block
    pub fn block_active_with_title(&self, title: impl Into<String>) -> Block<'static> {
        self.block_active()
            .title(format!(" {} ", title.into()))
            .title_style(self.heading())
    }

    /// Create a titled modal block
    pub fn block_modal_with_title(&self, title: impl Into<String>) -> Block<'static> {
        self.block_modal()
            .title(format!(" {} ", title.into()))
            .title_style(self.heading())
    }

    // ==================== Component Styles ====================

    /// Style for status bar
    pub fn status_bar(&self) -> Style {
        Style::default().bg(self.bg_tertiary)
    }

    /// Style for header bar
    pub fn header(&self) -> Style {
        Style::default().bg(self.bg_secondary)
    }

    /// Style for mode indicator (NORMAL, INSERT, etc.)
    pub fn mode_indicator(&self, is_normal: bool) -> Style {
        if is_normal {
            Style::default()
                .fg(self.text_inverse)
                .bg(self.accent_success)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.text_inverse)
                .bg(self.accent_warning)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for focus mode indicator
    pub fn focus_indicator(&self) -> Style {
        Style::default()
            .fg(self.accent_warning)
    }

    // ==================== Legacy Methods ====================
    // For backward compatibility with existing code

    /// Get style for normal text (legacy)
    pub fn text_style(&self) -> Style {
        self.text()
    }

    /// Get style for headers/titles (legacy)
    pub fn header_style(&self) -> Style {
        self.heading()
    }

    /// Get style for highlighted/selected items (legacy)
    pub fn highlight_style(&self) -> Style {
        self.selected()
    }

    /// Get style for borders (legacy)
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border_normal)
    }

    /// Get style for verse numbers (legacy)
    pub fn verse_number_style(&self) -> Style {
        self.verse_number()
    }

    /// Get style for bookmarks (legacy)
    pub fn bookmark_style(&self) -> Style {
        self.bookmark()
    }

    /// Get style for secondary text (legacy)
    pub fn secondary_style(&self) -> Style {
        self.text_secondary()
    }
}

// ==================== Built-in Themes ====================

/// Default dark theme with blue accents
pub fn dark_theme() -> Theme {
    Theme::new("Dark")
        // Already using good defaults
}

/// Gruvbox dark theme (popular developer theme)
pub fn gruvbox_dark() -> Theme {
    let mut theme = Theme::new("Gruvbox Dark");

    // Gruvbox color palette
    theme.bg_primary = Color::Rgb(40, 40, 40);       // bg0
    theme.bg_secondary = Color::Rgb(50, 48, 47);     // bg1
    theme.bg_tertiary = Color::Rgb(60, 56, 54);      // bg2
    theme.bg_modal = Color::Rgb(50, 48, 47);
    theme.bg_selection = Color::Rgb(80, 73, 69);     // bg3
    theme.bg_current = Color::Rgb(102, 92, 84);      // bg4

    theme.text_primary = Color::Rgb(235, 219, 178);  // fg1
    theme.text_secondary = Color::Rgb(213, 196, 161); // fg2
    theme.text_muted = Color::Rgb(168, 153, 132);    // fg4
    theme.text_heading = Color::Rgb(251, 241, 199);  // fg0
    theme.text_current = Color::Rgb(251, 241, 199);
    theme.text_inverse = Color::Rgb(40, 40, 40);

    theme.border_normal = Color::Rgb(80, 73, 69);
    theme.border_active = Color::Rgb(142, 192, 124); // green
    theme.border_inactive = Color::Rgb(60, 56, 54);

    theme.accent_primary = Color::Rgb(131, 165, 152);  // aqua
    theme.accent_secondary = Color::Rgb(142, 192, 124); // green
    theme.accent_success = Color::Rgb(142, 192, 124);   // green
    theme.accent_warning = Color::Rgb(250, 189, 47);    // yellow
    theme.accent_error = Color::Rgb(251, 73, 52);       // red
    theme.accent_info = Color::Rgb(131, 165, 152);      // aqua

    theme.bookmark_color = Color::Rgb(254, 128, 25);    // orange
    theme.search_highlight = Color::Rgb(250, 189, 47);

    theme
}

/// Nord theme (popular Arctic-inspired theme)
pub fn nord_theme() -> Theme {
    let mut theme = Theme::new("Nord");

    // Nord color palette
    theme.bg_primary = Color::Rgb(46, 52, 64);       // nord0
    theme.bg_secondary = Color::Rgb(59, 66, 82);     // nord1
    theme.bg_tertiary = Color::Rgb(67, 76, 94);      // nord2
    theme.bg_modal = Color::Rgb(59, 66, 82);
    theme.bg_selection = Color::Rgb(76, 86, 106);    // nord3
    theme.bg_current = Color::Rgb(76, 86, 106);

    theme.text_primary = Color::Rgb(236, 239, 244);  // nord6
    theme.text_secondary = Color::Rgb(229, 233, 240); // nord5
    theme.text_muted = Color::Rgb(216, 222, 233);    // nord4
    theme.text_heading = Color::Rgb(143, 188, 187);  // nord7
    theme.text_current = Color::Rgb(236, 239, 244);
    theme.text_inverse = Color::Rgb(46, 52, 64);

    theme.border_normal = Color::Rgb(76, 86, 106);
    theme.border_active = Color::Rgb(136, 192, 208); // nord8
    theme.border_inactive = Color::Rgb(67, 76, 94);

    theme.accent_primary = Color::Rgb(136, 192, 208); // nord8
    theme.accent_secondary = Color::Rgb(129, 161, 193); // nord9
    theme.accent_success = Color::Rgb(163, 190, 140);  // nord14
    theme.accent_warning = Color::Rgb(235, 203, 139);  // nord13
    theme.accent_error = Color::Rgb(191, 97, 106);     // nord11
    theme.accent_info = Color::Rgb(136, 192, 208);     // nord8

    theme.bookmark_color = Color::Rgb(208, 135, 112); // nord12
    theme.search_highlight = Color::Rgb(235, 203, 139);

    theme
}

/// Solarized dark theme
pub fn solarized_dark() -> Theme {
    let mut theme = Theme::new("Solarized Dark");

    // Solarized color palette
    theme.bg_primary = Color::Rgb(0, 43, 54);        // base03
    theme.bg_secondary = Color::Rgb(7, 54, 66);      // base02
    theme.bg_tertiary = Color::Rgb(88, 110, 117);    // base01
    theme.bg_modal = Color::Rgb(7, 54, 66);
    theme.bg_selection = Color::Rgb(88, 110, 117);   // base01
    theme.bg_current = Color::Rgb(101, 123, 131);    // base00

    theme.text_primary = Color::Rgb(131, 148, 150);  // base0
    theme.text_secondary = Color::Rgb(147, 161, 161); // base1
    theme.text_muted = Color::Rgb(101, 123, 131);    // base00
    theme.text_heading = Color::Rgb(238, 232, 213);  // base2
    theme.text_current = Color::Rgb(253, 246, 227);  // base3
    theme.text_inverse = Color::Rgb(0, 43, 54);

    theme.border_normal = Color::Rgb(88, 110, 117);
    theme.border_active = Color::Rgb(38, 139, 210);  // blue
    theme.border_inactive = Color::Rgb(7, 54, 66);

    theme.accent_primary = Color::Rgb(38, 139, 210);  // blue
    theme.accent_secondary = Color::Rgb(42, 161, 152); // cyan
    theme.accent_success = Color::Rgb(133, 153, 0);    // green
    theme.accent_warning = Color::Rgb(181, 137, 0);    // yellow
    theme.accent_error = Color::Rgb(220, 50, 47);      // red
    theme.accent_info = Color::Rgb(38, 139, 210);      // blue

    theme.bookmark_color = Color::Rgb(203, 75, 22);    // orange
    theme.search_highlight = Color::Rgb(181, 137, 0);

    theme
}

/// Light theme for bright environments
pub fn light_theme() -> Theme {
    let mut theme = Theme::new("Light");

    theme.bg_primary = Color::Rgb(250, 250, 250);
    theme.bg_secondary = Color::Rgb(240, 240, 245);
    theme.bg_tertiary = Color::Rgb(230, 230, 240);
    theme.bg_modal = Color::Rgb(245, 245, 250);
    theme.bg_selection = Color::Rgb(220, 230, 255);
    theme.bg_current = Color::Rgb(210, 220, 255);

    theme.text_primary = Color::Rgb(40, 40, 50);
    theme.text_secondary = Color::Rgb(60, 60, 70);
    theme.text_muted = Color::Rgb(120, 120, 140);
    theme.text_heading = Color::Rgb(20, 80, 160);
    theme.text_current = Color::Rgb(10, 10, 20);
    theme.text_inverse = Color::Rgb(255, 255, 255);

    theme.border_normal = Color::Rgb(180, 180, 200);
    theme.border_active = Color::Rgb(40, 120, 220);
    theme.border_inactive = Color::Rgb(200, 200, 215);

    theme.accent_primary = Color::Rgb(40, 120, 220);
    theme.accent_secondary = Color::Rgb(60, 140, 240);
    theme.accent_success = Color::Rgb(40, 160, 60);
    theme.accent_warning = Color::Rgb(220, 140, 20);
    theme.accent_error = Color::Rgb(220, 40, 40);
    theme.accent_info = Color::Rgb(60, 140, 220);

    theme.bookmark_color = Color::Rgb(220, 120, 20);
    theme.search_highlight = Color::Rgb(255, 240, 100);

    theme
}

/// Dracula theme (popular purple theme)
pub fn dracula_theme() -> Theme {
    let mut theme = Theme::new("Dracula");

    theme.bg_primary = Color::Rgb(40, 42, 54);       // background
    theme.bg_secondary = Color::Rgb(68, 71, 90);     // current line
    theme.bg_tertiary = Color::Rgb(68, 71, 90);
    theme.bg_modal = Color::Rgb(68, 71, 90);
    theme.bg_selection = Color::Rgb(68, 71, 90);
    theme.bg_current = Color::Rgb(98, 114, 164);     // comment (lighter)

    theme.text_primary = Color::Rgb(248, 248, 242);  // foreground
    theme.text_secondary = Color::Rgb(248, 248, 242);
    theme.text_muted = Color::Rgb(98, 114, 164);     // comment
    theme.text_heading = Color::Rgb(139, 233, 253);  // cyan
    theme.text_current = Color::Rgb(255, 255, 255);
    theme.text_inverse = Color::Rgb(40, 42, 54);

    theme.border_normal = Color::Rgb(98, 114, 164);
    theme.border_active = Color::Rgb(189, 147, 249); // purple
    theme.border_inactive = Color::Rgb(68, 71, 90);

    theme.accent_primary = Color::Rgb(189, 147, 249); // purple
    theme.accent_secondary = Color::Rgb(139, 233, 253); // cyan
    theme.accent_success = Color::Rgb(80, 250, 123);   // green
    theme.accent_warning = Color::Rgb(241, 250, 140);  // yellow
    theme.accent_error = Color::Rgb(255, 85, 85);      // red
    theme.accent_info = Color::Rgb(139, 233, 253);     // cyan

    theme.bookmark_color = Color::Rgb(255, 184, 108); // orange
    theme.search_highlight = Color::Rgb(241, 250, 140);

    theme
}

/// Monokai theme
pub fn monokai_theme() -> Theme {
    let mut theme = Theme::new("Monokai");

    theme.bg_primary = Color::Rgb(39, 40, 34);       // background
    theme.bg_secondary = Color::Rgb(49, 50, 44);
    theme.bg_tertiary = Color::Rgb(73, 72, 62);
    theme.bg_modal = Color::Rgb(49, 50, 44);
    theme.bg_selection = Color::Rgb(73, 72, 62);
    theme.bg_current = Color::Rgb(73, 72, 62);

    theme.text_primary = Color::Rgb(248, 248, 242);  // foreground
    theme.text_secondary = Color::Rgb(248, 248, 242);
    theme.text_muted = Color::Rgb(117, 113, 94);
    theme.text_heading = Color::Rgb(166, 226, 46);   // green
    theme.text_current = Color::Rgb(255, 255, 255);
    theme.text_inverse = Color::Rgb(39, 40, 34);

    theme.border_normal = Color::Rgb(73, 72, 62);
    theme.border_active = Color::Rgb(102, 217, 239);  // cyan
    theme.border_inactive = Color::Rgb(49, 50, 44);

    theme.accent_primary = Color::Rgb(102, 217, 239);  // cyan
    theme.accent_secondary = Color::Rgb(174, 129, 255); // purple
    theme.accent_success = Color::Rgb(166, 226, 46);    // green
    theme.accent_warning = Color::Rgb(230, 219, 116);   // yellow
    theme.accent_error = Color::Rgb(249, 38, 114);      // pink
    theme.accent_info = Color::Rgb(102, 217, 239);      // cyan

    theme.bookmark_color = Color::Rgb(253, 151, 31);    // orange
    theme.search_highlight = Color::Rgb(230, 219, 116);

    theme
}

/// Get a theme by name
pub fn get_theme(name: &str) -> Theme {
    match name.to_lowercase().as_str() {
        "dark" => dark_theme(),
        "gruvbox" | "gruvbox-dark" => gruvbox_dark(),
        "nord" => nord_theme(),
        "solarized" | "solarized-dark" => solarized_dark(),
        "light" => light_theme(),
        "dracula" => dracula_theme(),
        "monokai" => monokai_theme(),
        _ => dark_theme(), // Default to dark theme
    }
}

/// Get list of available theme names
pub fn available_themes() -> Vec<&'static str> {
    vec![
        "Dark",
        "Gruvbox Dark",
        "Nord",
        "Solarized Dark",
        "Light",
        "Dracula",
        "Monokai",
    ]
}
