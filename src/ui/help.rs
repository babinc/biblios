use crate::app::App;
use crate::config::settings::InputMode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the help view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let is_vim_mode = app.settings.input_mode == InputMode::Vim;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),     // Help content
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    render_help_content(f, app, is_vim_mode, chunks[0]);
    render_help_footer(f, app, chunks[1]);
}

fn render_help_content(f: &mut Frame, app: &App, is_vim_mode: bool, area: Rect) {
    let help_text = if is_vim_mode {
        vim_help_text(app)
    } else {
        normal_help_text(app)
    };

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(app.theme.border_style())
                .title(" Biblios Help - Press ESC or ? to close "),
        )
        .style(app.theme.text_style());

    f.render_widget(paragraph, area);
}

fn render_help_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer_text = "Press 'i' to toggle between Vim and Normal input modes";

    let footer = Paragraph::new(footer_text)
        .style(app.theme.secondary_style())
        .block(Block::default().borders(Borders::ALL).border_style(app.theme.border_style()));

    f.render_widget(footer, area);
}

fn vim_help_text(app: &App) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Biblios - TUI Bible Reader", app.theme.header_style()),
        ]),
        Line::from(vec![
            Span::styled("Vim Mode Keybindings", app.theme.highlight_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("NAVIGATION", app.theme.secondary_style()),
        ]),
        Line::from("  j/k           - Scroll down/up one verse"),
        Line::from("  h/l           - Previous/next verse"),
        Line::from("  Ctrl+d/Ctrl+u - Page down/up"),
        Line::from("  g/G           - Go to top/bottom"),
        Line::from("  [ / ]         - Previous/next chapter"),
        Line::from("  { / }         - Previous/next book"),
        Line::from(""),
        Line::from(vec![
            Span::styled("FEATURES", app.theme.secondary_style()),
        ]),
        Line::from("  /             - Open search"),
        Line::from("  n/N           - Next/previous search result"),
        Line::from("  m             - Toggle bookmark on current verse"),
        Line::from("  b             - View bookmarks"),
        Line::from("  s             - Open settings"),
        Line::from("  f             - Toggle focus mode"),
        Line::from("  i             - Toggle input mode (Vim/Normal)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("GENERAL", app.theme.secondary_style()),
        ]),
        Line::from("  ?             - Show this help"),
        Line::from("  q             - Quit"),
        Line::from("  ESC           - Close modal/go back"),
        Line::from(""),
        Line::from(vec![
            Span::styled("TIPS", app.theme.highlight_style()),
        ]),
        Line::from("  • Use [ and ] to navigate chapters easily"),
        Line::from("  • Press 'm' to bookmark verses you want to remember"),
        Line::from("  • Use focus mode (f) for distraction-free reading"),
        Line::from("  • Search (/) works across all verses"),
    ]
}

fn normal_help_text(app: &App) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Biblios - TUI Bible Reader", app.theme.header_style()),
        ]),
        Line::from(vec![
            Span::styled("Normal Mode Keybindings", app.theme.highlight_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("NAVIGATION", app.theme.secondary_style()),
        ]),
        Line::from("  Arrow Keys     - Navigate verses"),
        Line::from("  PageUp/Down    - Scroll by page"),
        Line::from("  Home/End       - Go to top/bottom"),
        Line::from("  Ctrl+Left/Right - Previous/next chapter"),
        Line::from("  Ctrl+Up/Down   - Previous/next book"),
        Line::from(""),
        Line::from(vec![
            Span::styled("FEATURES", app.theme.secondary_style()),
        ]),
        Line::from("  Ctrl+F        - Open search"),
        Line::from("  F3            - Next search result"),
        Line::from("  Ctrl+B        - Toggle bookmark on current verse"),
        Line::from("  Ctrl+M        - View bookmarks"),
        Line::from("  Ctrl+,        - Open settings"),
        Line::from("  F2            - Toggle focus mode"),
        Line::from(""),
        Line::from(vec![
            Span::styled("GENERAL", app.theme.secondary_style()),
        ]),
        Line::from("  F1 or ?       - Show this help"),
        Line::from("  Ctrl+Q/Ctrl+C - Quit"),
        Line::from("  ESC           - Close modal/go back"),
        Line::from(""),
        Line::from(vec![
            Span::styled("TIPS", app.theme.highlight_style()),
        ]),
        Line::from("  • Use Ctrl+Arrow keys to navigate chapters and books"),
        Line::from("  • Bookmark verses with Ctrl+B for easy reference"),
        Line::from("  • Use focus mode (F2) for distraction-free reading"),
        Line::from("  • Search (Ctrl+F) works across all verses"),
        Line::from("  • Switch to Vim mode for faster navigation (press 'i' in settings)"),
    ]
}
