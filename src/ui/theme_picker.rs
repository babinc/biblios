use crate::app::App;
use crate::ui::themes::available_themes;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
    Frame,
};

/// Render the theme picker modal
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Create centered modal (50% width, 60% height)
    let modal_area = centered_rect(50, 60, area);

    // Clear the area behind the modal
    f.render_widget(Clear, modal_area);

    let themes = available_themes();
    let mut theme_lines = vec![
        Line::from(""),
    ];

    for (idx, theme_name) in themes.iter().enumerate() {
        let is_selected = idx == app.theme_picker_index;
        let prefix = if is_selected { "▶ " } else { "  " };

        theme_lines.push(Line::from(vec![
            Span::styled(prefix, app.theme.accent()),
            Span::styled(
                *theme_name,
                if is_selected { app.theme.heading() } else { app.theme.text() }
            ),
        ]));
    }

    theme_lines.push(Line::from(""));
    theme_lines.push(Line::from(""));
    theme_lines.push(Line::from(vec![
        Span::styled("↑/↓", app.theme.accent()),
        Span::styled(":Navigate  ", app.theme.text_muted()),
        Span::styled("Enter", app.theme.accent()),
        Span::styled(":Apply  ", app.theme.text_muted()),
        Span::styled("ESC", app.theme.accent()),
        Span::styled(":Cancel", app.theme.text_muted()),
    ]));

    let block = app.theme.block_modal_with_title("Select Theme");

    let paragraph = Paragraph::new(theme_lines)
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
