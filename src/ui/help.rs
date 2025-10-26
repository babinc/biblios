use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
    Frame,
};

/// Render the help modal
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Create centered modal (80% width, 85% height)
    let modal_area = centered_rect(80, 85, area);

    // Clear the area behind the modal
    f.render_widget(Clear, modal_area);

    let help_text = help_text_content(app);

    let block = app.theme.block_modal_with_title("? Help");

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .style(app.theme.text());

    f.render_widget(paragraph, modal_area);
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn help_text_content(app: &App) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Biblios - TUI Bible Reader", app.theme.heading()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("NAVIGATION", app.theme.text_secondary()),
        ]),
        Line::from("  j/k or ↓/↑    - Scroll through verses continuously"),
        Line::from("  h/l or ←/→    - Previous/next verse"),
        Line::from("  Ctrl+d/Ctrl+u - Page down/up"),
        Line::from("  Ctrl+u/PgUp   - Scroll up by page"),
        Line::from("  G             - Go to bottom"),
        Line::from(""),
        Line::from(vec![
            Span::styled("FEATURES", app.theme.text_secondary()),
        ]),
        Line::from("  g             - Go to (book/chapter/verse selector)"),
        Line::from("  /             - Search verses"),
        Line::from("  n/N           - Next/previous search result"),
        Line::from("  m             - Toggle bookmark on current verse"),
        Line::from("  b             - View bookmarks"),
        Line::from("  s             - Open settings"),
        Line::from(""),
        Line::from(vec![
            Span::styled("GENERAL", app.theme.text_secondary()),
        ]),
        Line::from("  ?             - Show this help"),
        Line::from("  q or Ctrl+C   - Quit"),
        Line::from("  ESC           - Close modal/go back"),
        Line::from(""),
        Line::from(vec![
            Span::styled("TIPS", app.theme.accent()),
        ]),
        Line::from("  • Both vim (j/k) and arrow keys work everywhere"),
        Line::from("  • Press 'g' to quickly jump to any book/chapter/verse"),
        Line::from("  • In the selector: arrow keys navigate, letters filter"),
        Line::from("  • Bookmarks ('m') persist across sessions"),
        Line::from("  • Search ('/') works across the entire Bible"),
        Line::from(""),
        Line::from(Span::styled(
            "Press ESC to close",
            app.theme.text_muted(),
        )),
    ]
}
