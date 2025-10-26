use crate::app::App;
use crate::bible;
use crate::ui::components::{FilteredSelect, FilteredSelectConfig, NumberGrid, SelectItem};
use crate::ui::icons;
use ratatui::Frame;

/// Selector state - which step of the selection process we're in
#[derive(Debug, Clone, PartialEq)]
pub enum SelectorStep {
    Book,
    Chapter,
    Verse,
}

/// Render the verse selector modal (telescope-style)
pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    match app.selector_step {
        SelectorStep::Book => render_book_selector(f, app, area),
        SelectorStep::Chapter => render_chapter_selector(f, app, area),
        SelectorStep::Verse => render_verse_selector(f, app, area),
    }
}

/// Render book selection step with fuzzy search
fn render_book_selector(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // Filter books based on search
    let filtered_books = filter_books(&app.selector_search);

    // Convert to SelectItem
    let items: Vec<SelectItem> = filtered_books
        .iter()
        .map(|(short, full, testament, _)| {
            let icon = icons::category_icon(short);
            SelectItem::new(*full, *short)
                .with_icon(icon)
                .with_description(*testament)
        })
        .collect();

    let config = FilteredSelectConfig {
        title: format!("{} Go To Book", icons::SEARCH),
        items,
        search_query: &app.selector_search,
        selected_index: app.selector_index,
        show_search: true,
        search_placeholder: "Type to search books...",
        help_text: None,
    };

    FilteredSelect::render(f, config, &app.theme, area);
}

/// Render chapter selection step
fn render_chapter_selector(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let book_name = app.selector_selected_book.as_deref().unwrap_or("Unknown");
    let chapter_count = bible::get_chapter_count(book_name);

    let title = format!("{} {} - Select Chapter", icons::BOOK, book_name);

    NumberGrid::render(
        f,
        &title,
        chapter_count as usize,
        app.selector_index,
        &app.theme,
        area,
    );
}

/// Render verse selection step
fn render_verse_selector(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let book_name = app.selector_selected_book.as_deref().unwrap_or("Unknown");
    let chapter_num = app.selector_selected_chapter.unwrap_or(1);

    // Get verse count from loaded chapter
    let verse_count = if let Some(chapter) = &app.current_chapter {
        chapter.verses.len()
    } else {
        50 // Default estimate
    };

    let title = format!("{} {} {} - Select Verse", icons::BOOK, book_name, chapter_num);

    NumberGrid::render(
        f,
        &title,
        verse_count,
        app.selector_index,
        &app.theme,
        area,
    );
}

/// Filter books based on fuzzy search
fn filter_books(search: &str) -> Vec<(&'static str, &'static str, &'static str, u32)> {
    let search_lower = search.to_lowercase();

    bible::BOOK_ORDER
        .iter()
        .filter(|(short, full, _, _)| {
            if search.is_empty() {
                true
            } else {
                full.to_lowercase().contains(&search_lower)
                    || short.to_lowercase().contains(&search_lower)
            }
        })
        .map(|(short, full, testament, chapter_count)| {
            let testament_str = match testament {
                bible::Testament::Old => "Old Testament",
                bible::Testament::New => "New Testament",
            };
            (*short, *full, testament_str, *chapter_count)
        })
        .collect()
}
