use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Render the main Bible reader view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    if app.settings.focus_mode {
        render_focus_mode(f, app, area);
    } else {
        render_normal_mode(f, app, area);
    }
}

/// Render normal reading mode with all UI elements
fn render_normal_mode(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Book/chapter navigation
            Constraint::Percentage(80), // Main reading area
        ])
        .split(area);

    render_navigation(f, app, chunks[0]);
    render_verses(f, app, chunks[1]);
}

/// Render focus mode (just verses, no navigation)
fn render_focus_mode(f: &mut Frame, app: &App, area: Rect) {
    render_verses(f, app, area);
}

/// Render the navigation sidebar
fn render_navigation(f: &mut Frame, app: &App, area: Rect) {
    let books = vec![
        "Genesis", "Exodus", "Leviticus", "Numbers", "Deuteronomy",
        "Joshua", "Judges", "Ruth", "1 Samuel", "2 Samuel",
        "1 Kings", "2 Kings", "1 Chronicles", "2 Chronicles",
        "Ezra", "Nehemiah", "Esther", "Job", "Psalms", "Proverbs",
        "Ecclesiastes", "Song of Solomon", "Isaiah", "Jeremiah",
        "Lamentations", "Ezekiel", "Daniel", "Hosea", "Joel",
        "Amos", "Obadiah", "Jonah", "Micah", "Nahum", "Habakkuk",
        "Zephaniah", "Haggai", "Zechariah", "Malachi",
        "Matthew", "Mark", "Luke", "John", "Acts", "Romans",
        "1 Corinthians", "2 Corinthians", "Galatians", "Ephesians",
        "Philippians", "Colossians", "1 Thessalonians",
        "2 Thessalonians", "1 Timothy", "2 Timothy", "Titus",
        "Philemon", "Hebrews", "James", "1 Peter", "2 Peter",
        "1 John", "2 John", "3 John", "Jude", "Revelation",
    ];

    let items: Vec<ListItem> = books
        .iter()
        .map(|book| {
            let style = if Some(book.to_string()) == app.state.current_book {
                app.theme.highlight_style()
            } else {
                app.theme.text_style()
            };
            ListItem::new(*book).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(app.theme.border_style())
                .title(" Books "),
        )
        .style(app.theme.text_style());

    f.render_widget(list, area);
}

/// Render the verses in the main area
fn render_verses(f: &mut Frame, app: &App, area: Rect) {
    let verses = if let Some(chapter) = &app.current_chapter {
        let height = area.height.saturating_sub(2) as usize; // Account for borders
        let start = app.scroll_offset;
        let end = (start + height).min(chapter.verses.len());

        chapter.verses[start..end]
            .iter()
            .map(|verse| {
                let verse_num_str = if app.settings.show_verse_numbers {
                    format!("{:3} ", verse.reference.verse)
                } else {
                    String::new()
                };

                let is_bookmarked = app.bookmarks.is_bookmarked(&verse.reference);
                let bookmark_indicator = if is_bookmarked { " " } else { "  " };

                let mut line = vec![
                    Span::styled(bookmark_indicator, app.theme.bookmark_style()),
                ];

                if app.settings.show_verse_numbers {
                    line.push(Span::styled(verse_num_str, app.theme.verse_number_style()));
                }

                line.push(Span::styled(&verse.text, app.theme.text_style()));

                Line::from(line)
            })
            .collect::<Vec<_>>()
    } else {
        vec![
            Line::from(""),
            Line::from("Welcome to Biblios!"),
            Line::from(""),
            Line::from("No chapter is currently loaded."),
            Line::from(""),
            Line::from("Quick Start:"),
            Line::from("  - Press ? or F1 for help"),
            Line::from("  - Use navigation keys to browse verses"),
            Line::from("  - In Vim mode: use [ and ] to change chapters"),
            Line::from("  - In Normal mode: use Ctrl+Left/Right to change chapters"),
            Line::from(""),
            Line::from("Note: This is a sample database with limited content."),
        ]
    };

    let title = if let (Some(book), Some(chapter_num)) = (&app.state.current_book, app.state.current_chapter) {
        format!(" {} {} ", book, chapter_num)
    } else {
        " Reader ".to_string()
    };

    let paragraph = Paragraph::new(verses)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(app.theme.border_style())
                .title(title),
        )
        .style(app.theme.text_style());

    f.render_widget(paragraph, area);
}
