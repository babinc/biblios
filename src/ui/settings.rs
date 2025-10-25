use crate::app::App;
use crate::ui::themes::available_themes;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the settings view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let input_mode = match app.settings.input_mode {
        crate::config::settings::InputMode::Vim => "Vim",
        crate::config::settings::InputMode::Normal => "Normal",
    };

    let themes = available_themes();
    let theme_list = themes.join(", ");

    let settings_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Translation: ", app.theme.secondary_style()),
            Span::styled(&app.settings.translation, app.theme.text_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Input Mode: ", app.theme.secondary_style()),
            Span::styled(input_mode, app.theme.text_style()),
            Span::styled(" (Press 'i' to toggle)", app.theme.verse_number_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Theme: ", app.theme.secondary_style()),
            Span::styled(&app.settings.theme, app.theme.text_style()),
        ]),
        Line::from(vec![
            Span::styled("  Available: ", app.theme.verse_number_style()),
            Span::styled(theme_list, app.theme.verse_number_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Focus Mode: ", app.theme.secondary_style()),
            Span::styled(
                if app.settings.focus_mode { "On" } else { "Off" },
                app.theme.text_style(),
            ),
            Span::styled(" (Press 'f' to toggle)", app.theme.verse_number_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Show Verse Numbers: ", app.theme.secondary_style()),
            Span::styled(
                if app.settings.show_verse_numbers { "Yes" } else { "No" },
                app.theme.text_style(),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Verses Per Page: ", app.theme.secondary_style()),
            Span::styled(app.settings.verses_per_page.to_string(), app.theme.text_style()),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Press ESC to close settings",
            app.theme.verse_number_style(),
        )),
    ];

    let paragraph = Paragraph::new(settings_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(app.theme.border_style())
                .title(" Settings "),
        )
        .style(app.theme.text_style());

    f.render_widget(paragraph, area);
}
