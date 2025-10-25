pub mod reader;
pub mod search;
pub mod settings;
pub mod bookmarks;
pub mod themes;
pub mod help;

use crate::app::{App, ViewMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the main UI
pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Footer/status
        ])
        .split(f.area());

    render_header(f, app, chunks[0]);
    render_content(f, app, chunks[1]);
    render_footer(f, app, chunks[2]);
}

/// Render the header
fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let title = if let (Some(book), Some(chapter)) = (&app.state.current_book, app.state.current_chapter) {
        format!(" Biblios - {} {} ", book, chapter)
    } else {
        " Biblios ".to_string()
    };

    let header = Paragraph::new(title)
        .style(app.theme.header_style())
        .block(Block::default().borders(Borders::ALL).border_style(app.theme.border_style()));

    f.render_widget(header, area);
}

/// Render the main content area based on view mode
fn render_content(f: &mut Frame, app: &App, area: Rect) {
    match app.view_mode {
        ViewMode::Reader => reader::render(f, app, area),
        ViewMode::Search => search::render(f, app, area),
        ViewMode::Bookmarks => bookmarks::render(f, app, area),
        ViewMode::Settings => settings::render(f, app, area),
        ViewMode::Help => help::render(f, app, area),
    }
}

/// Render the footer with status information
fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let input_mode = match app.settings.input_mode {
        crate::config::settings::InputMode::Vim => "VIM",
        crate::config::settings::InputMode::Normal => "NORMAL",
    };

    let vim_mode_indicator = if app.settings.input_mode == crate::config::settings::InputMode::Vim {
        if app.vim_normal_mode { " [N] " } else { " [I] " }
    } else {
        ""
    };

    let focus_indicator = if app.settings.focus_mode { " [FOCUS] " } else { "" };

    let status = format!(
        " {} | {} | Theme: {} {}{} | Press ? for Help",
        input_mode,
        app.settings.translation,
        app.theme.name,
        vim_mode_indicator,
        focus_indicator
    );

    let footer = Paragraph::new(status)
        .style(app.theme.secondary_style())
        .block(Block::default().borders(Borders::ALL).border_style(app.theme.border_style()));

    f.render_widget(footer, area);
}
