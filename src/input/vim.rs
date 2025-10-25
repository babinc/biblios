use super::Action;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handle key events in Vim mode
pub fn handle_key(event: KeyEvent, normal_mode: bool) -> Action {
    if normal_mode {
        handle_normal_mode(event)
    } else {
        handle_insert_mode(event)
    }
}

/// Handle keys in Vim normal mode
fn handle_normal_mode(event: KeyEvent) -> Action {
    match event.code {
        // Navigation
        KeyCode::Char('j') => Action::ScrollDown,
        KeyCode::Char('k') => Action::ScrollUp,
        KeyCode::Char('h') => Action::PreviousVerse,
        KeyCode::Char('l') => Action::NextVerse,

        // Fast navigation
        KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::PageDown,
        KeyCode::Char('u') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::PageUp,
        KeyCode::Char('g') => Action::GoToTop,
        KeyCode::Char('G') => Action::GoToBottom,

        // Chapter/Book navigation
        KeyCode::Char(']') => Action::NextChapter,
        KeyCode::Char('[') => Action::PreviousChapter,
        KeyCode::Char('}') => Action::NextBook,
        KeyCode::Char('{') => Action::PreviousBook,

        // Search
        KeyCode::Char('/') => Action::OpenSearch,
        KeyCode::Char('n') => Action::SearchNext,
        KeyCode::Char('N') => Action::SearchPrevious,

        // Bookmarks
        KeyCode::Char('m') => Action::ToggleBookmark,
        KeyCode::Char('b') => Action::OpenBookmarks,

        // Settings
        KeyCode::Char('s') => Action::OpenSettings,
        KeyCode::Char('i') => Action::ToggleInputMode,
        KeyCode::Char('f') => Action::ToggleFocusMode,

        // Help
        KeyCode::Char('?') => Action::OpenHelp,
        KeyCode::F(1) => Action::OpenHelp,

        // General
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Esc => Action::Escape,
        KeyCode::Enter => Action::Enter,
        KeyCode::Tab => Action::Tab,
        KeyCode::BackTab => Action::ShiftTab,

        _ => Action::None,
    }
}

/// Handle keys in Vim insert mode (for search input, etc.)
fn handle_insert_mode(event: KeyEvent) -> Action {
    match event.code {
        KeyCode::Char(c) => Action::Char(c),
        KeyCode::Backspace => Action::Backspace,
        KeyCode::Delete => Action::Delete,
        KeyCode::Esc => Action::Escape,
        KeyCode::Enter => Action::Enter,
        KeyCode::Tab => Action::Tab,
        KeyCode::BackTab => Action::ShiftTab,
        _ => Action::None,
    }
}
