use crate::app::App;
use crate::ui::icons;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Padding, Paragraph, Wrap},
    Frame,
};

/// Render the main Bible reader view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    render_normal_mode(f, app, area);
}

/// Render normal reading mode with all UI elements
fn render_normal_mode(f: &mut Frame, app: &App, area: Rect) {
    // Main layout: top bar + content + status bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Top bar
            Constraint::Min(0),     // Content (full width)
            Constraint::Length(1),  // Status bar
        ])
        .split(area);

    // Render elegant top bar
    render_top_bar(f, app, main_chunks[0]);

    // Render full-width reading area (no sidebar)
    render_reading_area(f, app, main_chunks[1]);

    // Render status bar
    render_status_bar(f, app, main_chunks[2]);
}

/// Render elegant top bar with title and metadata
fn render_top_bar(f: &mut Frame, app: &App, area: Rect) {
    let title = if let (Some(book), Some(chapter)) = (&app.state.current_book, app.state.current_chapter) {
        format!("{} {} {}", icons::BIBLE, book, chapter)
    } else {
        format!("{} Biblios", icons::BIBLE)
    };

    let title_widget = Paragraph::new(title)
        .block(app.theme.block_secondary())
        .alignment(Alignment::Center)
        .style(app.theme.heading());

    f.render_widget(title_widget, area);
}

/// Render the main reading area with elegant verse display
fn render_reading_area(f: &mut Frame, app: &App, area: Rect) {
    render_verses(f, app, area);
}

/// Render verses with modern, elegant styling
fn render_verses(f: &mut Frame, app: &App, area: Rect) {
    let verses_text = if let Some(chapter) = &app.current_chapter {
        let height = area.height.saturating_sub(2) as usize;
        let current_idx = app.current_verse_index;

        // Use scroll offset for viewport - verses stay in place
        let start = app.scroll_offset;
        let end = (start + height).min(chapter.verses.len());

        let mut lines = Vec::new();

        for (idx, verse) in chapter.verses[start..end].iter().enumerate() {
            let actual_verse_idx = start + idx;
            let is_current = actual_verse_idx == current_idx;

            // Create verse number
            let verse_num = if app.settings.show_verse_numbers {
                format!("{:>3} ", verse.reference.verse)
            } else {
                String::new()
            };

            // Style based on whether this is the current verse
            let (num_style, text_style, bg_color) = if is_current {
                (
                    app.theme.verse_number_current(),
                    app.theme.current(),
                    Some(app.theme.bg_current),
                )
            } else {
                (
                    app.theme.verse_number(),
                    app.theme.text_secondary(),
                    None,
                )
            };

            // Create the line with proper styling
            let mut spans = vec![];

            // Add bookmark indicator
            let is_bookmarked = app.bookmarks.is_bookmarked(&verse.reference);
            let bookmark_icon = if is_bookmarked {
                format!("{} ", icons::BOOKMARK)
            } else {
                "   ".to_string()
            };
            spans.push(Span::styled(
                bookmark_icon,
                app.theme.bookmark(),
            ));

            // Add verse number
            if app.settings.show_verse_numbers {
                spans.push(Span::styled(verse_num, num_style));
            }

            // Add verse text
            let text_span = if let Some(bg) = bg_color {
                Span::styled(&verse.text, text_style.bg(bg))
            } else {
                Span::styled(&verse.text, text_style)
            };
            spans.push(text_span);

            lines.push(Line::from(spans));

            // Add spacing between verses if enabled (always, not conditional on is_current)
            if app.settings.verse_spacing && idx < chapter.verses[start..end].len() - 1 {
                lines.push(Line::from(""));
            }
        }

        Text::from(lines)
    } else {
        // Welcome screen with modern styling
        let welcome_lines = vec![
            Line::from(""),
            Line::from(
                Span::styled(
                    format!("{} Welcome to Biblios", icons::BIBLE),
                    app.theme.heading(),
                )
            ).alignment(Alignment::Center),
            Line::from(""),
            Line::from(
                Span::styled(
                    "A modern terminal Bible reader",
                    app.theme.text_secondary(),
                )
            ).alignment(Alignment::Center),
            Line::from(""),
            Line::from(""),
            Line::from(
                Span::styled(
                    "Quick Start",
                    app.theme.heading(),
                )
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled("  j/k ", app.theme.accent()),
                Span::styled(" or ", app.theme.text_muted()),
                Span::styled("↓/↑  ", app.theme.accent()),
                Span::styled("Flow through verses", app.theme.text_secondary()),
            ]),
            Line::from(vec![
                Span::styled("  g   ", app.theme.accent()),
                Span::styled("       ", Style::default()),
                Span::styled("Go to (book/chapter/verse)", app.theme.text_secondary()),
            ]),
            Line::from(vec![
                Span::styled("  m   ", app.theme.accent()),
                Span::styled("       ", Style::default()),
                Span::styled("Bookmark current verse", app.theme.text_secondary()),
            ]),
            Line::from(vec![
                Span::styled("  /   ", app.theme.accent()),
                Span::styled("       ", Style::default()),
                Span::styled("Search verses", app.theme.text_secondary()),
            ]),
            Line::from(vec![
                Span::styled("  ?   ", app.theme.accent()),
                Span::styled("       ", Style::default()),
                Span::styled("Show help", app.theme.text_secondary()),
            ]),
        ];

        Text::from(welcome_lines)
    };

    // Create the paragraph widget
    let title = if let (Some(book), Some(chapter_num)) = (&app.state.current_book, app.state.current_chapter) {
        format!(" {} {} ", book, chapter_num)
    } else {
        " Reader ".to_string()
    };

    let mut block = app.theme.block_with_title(title);
    block = block.padding(Padding::horizontal(2));

    let paragraph = Paragraph::new(verses_text)
        .block(block)
        .style(app.theme.text())
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

/// Render elegant status bar with contextual information
fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    // Get current verse info
    let verse_info = if let Some(chapter) = &app.current_chapter {
        if let Some(verse) = chapter.verses.get(app.current_verse_index) {
            format!(" {}  ", verse.reference)
        } else {
            " Loading... ".to_string()
        }
    } else {
        " No chapter loaded ".to_string()
    };

    // Input mode indicator
    let mode_text = if app.settings.input_mode == crate::config::settings::InputMode::Vim {
        if app.vim_normal_mode {
            " NORMAL "
        } else {
            " INSERT "
        }
    } else {
        " ARROW "
    };

    let mode_style = app.theme.mode_indicator(app.vim_normal_mode);

    // Help hint
    let help_hint = " ? for help ";

    // Calculate spacing
    let total_len = mode_text.len() + verse_info.len() + help_hint.len() + 2;
    let spacing = if area.width as usize > total_len {
        " ".repeat(area.width as usize - total_len)
    } else {
        String::new()
    };

    let status_line = Line::from(vec![
        Span::styled(mode_text, mode_style),
        Span::styled(" ", Style::default()),
        Span::styled(verse_info.clone(), app.theme.accent()),
        Span::styled(spacing, Style::default()),
        Span::styled(help_hint, app.theme.text_muted()),
    ]);

    let status_widget = Paragraph::new(status_line)
        .style(app.theme.status_bar());

    f.render_widget(status_widget, area);
}
