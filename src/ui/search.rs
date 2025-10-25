use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Render the search view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Min(0),     // Search results
        ])
        .split(area);

    render_search_input(f, app, chunks[0]);
    render_search_results(f, app, chunks[1]);
}

/// Render the search input field
fn render_search_input(f: &mut Frame, app: &App, area: Rect) {
    let input = Paragraph::new(app.search_query.as_str())
        .style(app.theme.text_style())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(app.theme.border_style())
                .title(" Search (ESC to close) "),
        );

    f.render_widget(input, area);

    // Show cursor in search box
    if !app.vim_normal_mode || app.view_mode == crate::app::ViewMode::Search {
        f.set_cursor_position((
            area.x + app.search_query.len() as u16 + 1,
            area.y + 1,
        ));
    }
}

/// Render search results
fn render_search_results(f: &mut Frame, app: &App, area: Rect) {
    let results: Vec<ListItem> = if app.search_results.is_empty() {
        vec![ListItem::new(Line::from("No results found. Type to search..."))]
    } else {
        app.search_results
            .iter()
            .enumerate()
            .map(|(i, reference)| {
                let style = if i == app.search_selected {
                    app.theme.highlight_style()
                } else {
                    app.theme.text_style()
                };
                ListItem::new(reference.to_string()).style(style)
            })
            .collect()
    };

    let list = List::new(results)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(app.theme.border_style())
                .title(format!(" Results ({}) ", app.search_results.len())),
        )
        .style(app.theme.text_style());

    f.render_widget(list, area);
}
