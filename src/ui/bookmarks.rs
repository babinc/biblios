use crate::app::App;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

/// Render the bookmarks view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let bookmarks = app.bookmarks.all();

    let items: Vec<ListItem> = if bookmarks.is_empty() {
        vec![ListItem::new(Line::from(
            "No bookmarks yet. Press 'm' or Ctrl+B to bookmark a verse.",
        ))]
    } else {
        bookmarks
            .iter()
            .enumerate()
            .map(|(i, bookmark)| {
                let style = if i == app.bookmark_selected {
                    app.theme.highlight_style()
                } else {
                    app.theme.text_style()
                };

                let text = if let Some(note) = &bookmark.note {
                    format!("{} - {}", bookmark.reference, note)
                } else {
                    bookmark.reference.to_string()
                };

                ListItem::new(text).style(style)
            })
            .collect()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(app.theme.border_style())
                .title(format!(" Bookmarks ({}) - ESC to close ", bookmarks.len())),
        )
        .style(app.theme.text_style());

    f.render_widget(list, area);
}
