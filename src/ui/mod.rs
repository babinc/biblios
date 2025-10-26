pub mod bookmarks;
pub mod components;
pub mod help;
pub mod icons;
pub mod reader;
pub mod search;
pub mod settings;
pub mod theme_picker;
pub mod themes;
pub mod verse_selector;

use crate::app::{App, ViewMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the main UI
pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Key bindings bar
            Constraint::Length(3), // Header
            Constraint::Min(0),     // Main content
            Constraint::Length(1),  // Footer/status
        ])
        .split(f.area());

    render_keybindings(f, app, chunks[0]);
    render_header(f, app, chunks[1]);
    render_content(f, app, chunks[2]);
    render_footer(f, app, chunks[3]);
}

/// Render the key bindings bar
fn render_keybindings(f: &mut Frame, app: &App, area: Rect) {
    let keybindings = vec![
        Span::styled(" g", app.theme.accent()),
        Span::styled(":Go ", app.theme.text_muted()),
        Span::styled("j/k", app.theme.accent()),
        Span::styled(":Scroll ", app.theme.text_muted()),
        Span::styled("m", app.theme.accent()),
        Span::styled(":Bookmark ", app.theme.text_muted()),
        Span::styled("b", app.theme.accent()),
        Span::styled(":Bookmarks ", app.theme.text_muted()),
        Span::styled("/", app.theme.accent()),
        Span::styled(":Search ", app.theme.text_muted()),
        Span::styled("s", app.theme.accent()),
        Span::styled(":Settings ", app.theme.text_muted()),
        Span::styled("?", app.theme.accent()),
        Span::styled(":Help ", app.theme.text_muted()),
        Span::styled("q", app.theme.accent()),
        Span::styled(":Quit ", app.theme.text_muted()),
    ];

    let paragraph = Paragraph::new(Line::from(keybindings))
        .style(app.theme.text().bg(app.theme.bg_secondary));

    f.render_widget(paragraph, area);
}

/// Render the header
fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let title = if let (Some(book), Some(chapter)) = (&app.state.current_book, app.state.current_chapter) {
        format!(" Biblios - {} {} ", book, chapter)
    } else {
        " Biblios ".to_string()
    };

    let header = Paragraph::new(title)
        .style(app.theme.heading().bg(app.theme.bg_secondary))
        .block(Block::default().borders(Borders::ALL).border_style(app.theme.border_style()));

    f.render_widget(header, area);
}

/// Render the main content area based on view mode
fn render_content(f: &mut Frame, app: &App, area: Rect) {
    match app.view_mode {
        ViewMode::Reader => reader::render(f, app, area),
        ViewMode::Search => search::render(f, app, area),
        ViewMode::Bookmarks => bookmarks::render(f, app, area),
        ViewMode::Settings => {
            // Settings is now a modal, so render reader underneath
            reader::render(f, app, area);
        }
        ViewMode::Help => {
            // Help is now a modal, so render reader underneath
            reader::render(f, app, area);
        }
    }

    // Render verse selector modal on top if open
    if app.selector_open {
        verse_selector::render(f, app, f.area());
    }

    // Render settings modal on top if open
    if app.settings_open {
        settings::render(f, app, f.area());
    }

    // Render theme picker modal on top of settings if open
    if app.theme_picker_open {
        theme_picker::render(f, app, f.area());
    }

    // Render help modal on top if open
    if app.help_open {
        help::render(f, app, f.area());
    }
}

/// Render the footer with status information
fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let status = format!(
        " {} | Theme: {} ",
        app.settings.translation,
        app.theme.name
    );

    let footer = Paragraph::new(status)
        .style(app.theme.text_secondary().bg(app.theme.bg_secondary));

    f.render_widget(footer, area);
}
