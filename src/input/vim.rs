use super::Action;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handle key events in Vim mode
pub fn handle_key(event: KeyEvent) -> Action {
    match event.code {
        // Navigation (j/k now flow continuously through chapters and books)
        KeyCode::Char('j') => Action::ScrollDown,
        KeyCode::Char('k') => Action::ScrollUp,
        KeyCode::Char('h') => Action::PreviousVerse,
        KeyCode::Char('l') => Action::NextVerse,

        // Fast navigation
        KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::PageDown,
        KeyCode::Char('u') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::PageUp,
        KeyCode::Char('g') => Action::OpenVerseSelector,
        KeyCode::Char('G') => Action::GoToBottom,

        // Search
        KeyCode::Char('/') => Action::OpenSearch,
        KeyCode::Char('n') => Action::SearchNext,
        KeyCode::Char('N') => Action::SearchPrevious,

        // Bookmarks
        KeyCode::Char('m') => Action::ToggleBookmark,
        KeyCode::Char('b') => Action::OpenBookmarks,

        // Settings
        KeyCode::Char('s') => Action::OpenSettings,

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
