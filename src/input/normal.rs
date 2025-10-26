use super::Action;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handle key events in normal (non-Vim) mode
pub fn handle_key(event: KeyEvent) -> Action {
    match event.code {
        // Navigation with arrow keys (up/down now flow continuously through chapters and books)
        KeyCode::Down => Action::ScrollDown,
        KeyCode::Up => Action::ScrollUp,
        KeyCode::Left => Action::PreviousVerse,
        KeyCode::Right => Action::NextVerse,

        // Page navigation
        KeyCode::PageDown => Action::PageDown,
        KeyCode::PageUp => Action::PageUp,
        KeyCode::Home => Action::GoToTop,
        KeyCode::End => Action::GoToBottom,

        // Go to verse selector
        KeyCode::Char('g') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::OpenVerseSelector,

        // Search
        KeyCode::Char('/') => Action::OpenSearch,
        KeyCode::Char('f') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::OpenSearch,
        KeyCode::F(3) => Action::SearchNext,

        // Bookmarks
        KeyCode::Char('m') => Action::ToggleBookmark,
        KeyCode::Char('b') => Action::OpenBookmarks,

        // Settings
        KeyCode::Char('s') => Action::OpenSettings,

        // Help
        KeyCode::F(1) => Action::OpenHelp,
        KeyCode::Char('?') => Action::OpenHelp,

        // General
        KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Esc => Action::Escape,
        KeyCode::Enter => Action::Enter,
        KeyCode::Tab => Action::Tab,
        KeyCode::BackTab => Action::ShiftTab,

        // Character input (for search, etc.)
        KeyCode::Char(c) => Action::Char(c),
        KeyCode::Backspace => Action::Backspace,
        KeyCode::Delete => Action::Delete,

        _ => Action::None,
    }
}
