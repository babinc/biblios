use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
    Frame,
};

/// Render the settings modal
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Create centered modal (60% width, 70% height)
    let modal_area = centered_rect(60, 70, area);

    // Clear the area behind the modal
    f.render_widget(Clear, modal_area);

    let mut settings_text = vec![
        Line::from(""),
    ];

    // Theme setting (index 0)
    let is_selected_0 = app.settings_selected_index == 0;
    let theme_prefix = if is_selected_0 { "▶ " } else { "  " };
    settings_text.push(Line::from(vec![
        Span::styled(theme_prefix, app.theme.accent()),
        Span::styled("Theme: ", if is_selected_0 { app.theme.heading() } else { app.theme.text_secondary() }),
        Span::styled(&app.settings.theme, if is_selected_0 { app.theme.accent() } else { app.theme.text() }),
        Span::styled(" Enter", app.theme.text_muted()),
    ]));
    settings_text.push(Line::from(""));

    // Show verse numbers (index 1)
    let is_selected_1 = app.settings_selected_index == 1;
    let verse_nums_prefix = if is_selected_1 { "▶ " } else { "  " };
    let verse_nums_value = if app.settings.show_verse_numbers { "Yes" } else { "No" };
    settings_text.push(Line::from(vec![
        Span::styled(verse_nums_prefix, app.theme.accent()),
        Span::styled("Show Verse Numbers: ", if is_selected_1 { app.theme.heading() } else { app.theme.text_secondary() }),
        Span::styled(verse_nums_value, if is_selected_1 { app.theme.accent() } else { app.theme.text() }),
        Span::styled(" Enter", app.theme.text_muted()),
    ]));
    settings_text.push(Line::from(""));

    // Verse spacing (index 2)
    let is_selected_2 = app.settings_selected_index == 2;
    let spacing_prefix = if is_selected_2 { "▶ " } else { "  " };
    let spacing_value = if app.settings.verse_spacing { "Yes" } else { "No" };
    settings_text.push(Line::from(vec![
        Span::styled(spacing_prefix, app.theme.accent()),
        Span::styled("Verse Spacing: ", if is_selected_2 { app.theme.heading() } else { app.theme.text_secondary() }),
        Span::styled(spacing_value, if is_selected_2 { app.theme.accent() } else { app.theme.text() }),
        Span::styled(" Enter", app.theme.text_muted()),
    ]));

    settings_text.push(Line::from(""));
    settings_text.push(Line::from(""));
    settings_text.push(Line::from(vec![
        Span::styled("Translation: ", app.theme.text_muted()),
        Span::styled(&app.settings.translation, app.theme.text_muted()),
        Span::styled(" (read-only)", app.theme.text_muted()),
    ]));
    settings_text.push(Line::from(""));
    settings_text.push(Line::from(""));
    settings_text.push(Line::from(vec![
        Span::styled("↑/↓", app.theme.accent()),
        Span::styled(":Navigate  ", app.theme.text_muted()),
        Span::styled("Enter", app.theme.accent()),
        Span::styled(":Select  ", app.theme.text_muted()),
        Span::styled("ESC", app.theme.accent()),
        Span::styled(":Close", app.theme.text_muted()),
    ]));

    let block = app.theme.block_modal_with_title("⚙ Settings");

    let paragraph = Paragraph::new(settings_text)
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
