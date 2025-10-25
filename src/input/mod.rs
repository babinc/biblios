pub mod vim;
pub mod normal;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Actions that can be performed in the app
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    // Navigation
    NextVerse,
    PreviousVerse,
    NextChapter,
    PreviousChapter,
    NextBook,
    PreviousBook,
    ScrollDown,
    ScrollUp,
    PageDown,
    PageUp,
    GoToTop,
    GoToBottom,

    // Search
    OpenSearch,
    CloseSearch,
    SearchNext,
    SearchPrevious,

    // Bookmarks
    ToggleBookmark,
    OpenBookmarks,
    CloseBookmarks,

    // Settings
    OpenSettings,
    CloseSettings,
    ToggleInputMode,
    ToggleFocusMode,
    NextTheme,
    PreviousTheme,

    // Help
    OpenHelp,
    CloseHelp,

    // General
    Quit,
    Enter,
    Escape,
    Tab,
    ShiftTab,

    // Character input (for search, etc.)
    Char(char),
    Backspace,
    Delete,

    // No action
    None,
}

/// Process key event based on input mode
pub fn process_key_event(
    event: KeyEvent,
    vim_mode: bool,
    vim_normal_mode: bool,
) -> Action {
    if vim_mode {
        vim::handle_key(event, vim_normal_mode)
    } else {
        normal::handle_key(event)
    }
}

/// Check if key event is a quit command
pub fn is_quit_key(event: KeyEvent) -> bool {
    matches!(
        event,
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } | KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }
    )
}
