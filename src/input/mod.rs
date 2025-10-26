pub mod vim;
pub mod normal;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Actions that can be performed in the app
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    // Navigation (ScrollDown/ScrollUp now flow continuously through entire Bible)
    NextVerse,
    PreviousVerse,
    ScrollDown,
    ScrollUp,
    PageDown,
    PageUp,
    GoToTop,
    GoToBottom,

    // Search
    OpenSearch,
    SearchNext,
    SearchPrevious,

    // Bookmarks
    ToggleBookmark,
    OpenBookmarks,

    // Verse Selector
    OpenVerseSelector,
    SelectorUp,
    SelectorDown,
    SelectorSelect,
    SelectorBack,

    // Settings
    OpenSettings,

    // Help
    OpenHelp,

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

/// Process key event - combines both vim and normal keybindings
pub fn process_key_event(event: KeyEvent) -> Action {
    // Try vim bindings first
    let vim_action = vim::handle_key(event);
    if vim_action != Action::None {
        return vim_action;
    }

    // Fall back to normal/arrow bindings
    normal::handle_key(event)
}

/// Process key event when selector is open - prioritizes text input over keybindings
pub fn process_selector_key_event(event: KeyEvent) -> Action {
    use crossterm::event::{KeyCode, KeyModifiers};

    match event.code {
        // Navigation with arrow keys ONLY (j/k are used for typing)
        KeyCode::Down => Action::ScrollDown,
        KeyCode::Up => Action::ScrollUp,

        // Selection and escape
        KeyCode::Enter => Action::Enter,
        KeyCode::Esc => Action::Escape,

        // Text editing
        KeyCode::Backspace => Action::Backspace,
        KeyCode::Delete => Action::Delete,

        // Tab navigation
        KeyCode::Tab => Action::Tab,
        KeyCode::BackTab => Action::ShiftTab,

        // Page navigation
        KeyCode::PageDown => Action::PageDown,
        KeyCode::PageUp => Action::PageUp,

        // All characters (including j/k) are treated as text input
        KeyCode::Char(c) if event.modifiers.contains(KeyModifiers::CONTROL) => {
            // Allow Ctrl+C and Ctrl+Q for quit
            match c {
                'c' | 'q' => Action::Quit,
                _ => Action::None,
            }
        }
        KeyCode::Char(c) => Action::Char(c),

        _ => Action::None,
    }
}

/// Process key event when modal (settings/help) is open
pub fn process_modal_key_event(event: KeyEvent, allow_navigation: bool) -> Action {
    use crossterm::event::{KeyCode, KeyModifiers};

    match event.code {
        // Close modal
        KeyCode::Esc => Action::Escape,
        KeyCode::Char('?') if !allow_navigation => Action::Escape,

        // Allow quit
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,

        // Navigation and selection (only if allowed, e.g., in settings modal)
        KeyCode::Up | KeyCode::Char('k') if allow_navigation => Action::ScrollUp,
        KeyCode::Down | KeyCode::Char('j') if allow_navigation => Action::ScrollDown,
        KeyCode::Enter if allow_navigation => Action::Enter,
        KeyCode::Left | KeyCode::Char('h') if allow_navigation => Action::PreviousVerse,
        KeyCode::Right | KeyCode::Char('l') if allow_navigation => Action::NextVerse,

        // Ignore ALL other keys to prevent affecting background
        _ => Action::None,
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
