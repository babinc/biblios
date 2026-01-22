use crate::bible::{loader::BibleLoader, Chapter, VerseReference};
use crate::config::{BookmarkManager, ReadingState, Settings};
use crate::input::Action;
use crate::ui::themes::{get_theme, Theme};
use crate::ui::verse_selector::SelectorStep;
use anyhow::Result;

/// Main application state
///
/// The App struct coordinates all application state including user configuration,
/// Bible data access, and UI state. It serves as the central controller, responding
/// to user input actions and managing the reading experience.
///
/// # State Management
/// - **Persistent State**: `settings`, `bookmarks`, and `state` are saved to disk
/// - **Session State**: `current_chapter`, `current_verse_index` are ephemeral
/// - **UI State**: `view_mode`, `search_query` control the current view
pub struct App {
    /// Current settings
    pub settings: Settings,

    /// Bookmark manager
    pub bookmarks: BookmarkManager,

    /// Reading state
    pub state: ReadingState,

    /// Current theme
    pub theme: Theme,

    /// Bible data loader
    pub loader: Option<BibleLoader>,

    /// Currently loaded chapter
    pub current_chapter: Option<Chapter>,

    /// Current view mode
    pub view_mode: ViewMode,

    /// Search query
    pub search_query: String,

    /// Search results
    pub search_results: Vec<VerseReference>,

    /// Selected search result index
    pub search_selected: usize,

    /// Current verse index (the verse being read, highlighted)
    pub current_verse_index: usize,

    /// Scroll offset (first visible verse index)
    pub scroll_offset: usize,

    /// Selected bookmark index (when viewing bookmarks)
    pub bookmark_selected: usize,

    /// Vim normal mode (true) or insert mode (false)
    pub vim_normal_mode: bool,

    /// Verse selector modal state
    pub selector_open: bool,
    pub selector_step: SelectorStep,
    pub selector_search: String,
    pub selector_index: usize,
    pub selector_selected_book: Option<String>,
    pub selector_selected_chapter: Option<u32>,

    /// Settings modal state
    pub settings_open: bool,

    /// Help modal state
    pub help_open: bool,

    /// Settings modal state
    pub settings_selected_index: usize,

    /// Theme picker modal state
    pub theme_picker_open: bool,
    pub theme_picker_index: usize,

    /// Whether the app should quit
    pub should_quit: bool,

    /// Whether using sample data (no full Bible downloaded)
    pub using_sample_data: bool,

    /// Download modal state
    pub download_modal_open: bool,
    pub download_status: DownloadStatus,
    pub download_selected_index: usize,
}

/// Status of Bible download
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    /// Ready to download
    Ready,
    /// Currently downloading
    Downloading(String),
    /// Download completed successfully
    Complete,
    /// Download failed with error message
    Failed(String),
}

/// Different view modes available in the application
///
/// The application can be in one of several modes, each displaying different
/// content and accepting different input. The Reader mode is the primary mode
/// where Bible text is displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Main Bible reading view with centered verse display
    Reader,
    /// Search interface for finding verses
    Search,
    /// List of saved bookmarks
    Bookmarks,
    /// Application settings and preferences
    Settings,
    /// Help screen with keyboard shortcuts
    Help,
}

impl App {
    /// Create a new app instance
    pub fn new() -> Result<Self> {
        let settings = Settings::load()?;
        let bookmarks = BookmarkManager::load()?;
        let state = ReadingState::load()?;
        let theme = get_theme(&settings.theme);

        Ok(Self {
            settings,
            bookmarks,
            state,
            theme,
            loader: None,
            current_chapter: None,
            view_mode: ViewMode::Reader,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected: 0,
            current_verse_index: 0,
            scroll_offset: 0,
            bookmark_selected: 0,
            vim_normal_mode: true,
            selector_open: false,
            selector_step: SelectorStep::Book,
            selector_search: String::new(),
            selector_index: 0,
            selector_selected_book: None,
            selector_selected_chapter: None,
            settings_open: false,
            help_open: false,
            settings_selected_index: 0,
            theme_picker_open: false,
            theme_picker_index: 0,
            should_quit: false,
            using_sample_data: false,
            download_modal_open: false,
            download_status: DownloadStatus::Ready,
            download_selected_index: 0,
        })
    }

    /// Initialize with a Bible database
    pub fn with_bible(mut self, db_path: &str, is_sample: bool) -> Result<Self> {
        let loader = BibleLoader::new(db_path)?;
        self.loader = Some(loader);
        self.using_sample_data = is_sample;
        self.load_current_chapter()?;
        Ok(self)
    }

    /// Load the current chapter based on reading state
    pub fn load_current_chapter(&mut self) -> Result<()> {
        if let (Some(loader), Some(book), Some(chapter)) = (
            &self.loader,
            &self.state.current_book,
            self.state.current_chapter,
        ) {
            self.current_chapter = Some(loader.load_chapter(book, chapter)?);
            self.current_verse_index = self.state.current_verse_index;
        }
        Ok(())
    }

    /// Handle an action
    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        // If selector is open, intercept navigation actions
        if self.selector_open {
            match action {
                Action::Escape => {
                    return self.handle_selector_back();
                }
                Action::Enter => {
                    return self.handle_selector_select();
                }
                Action::ScrollDown => {
                    return self.handle_selector_down();
                }
                Action::ScrollUp => {
                    return self.handle_selector_up();
                }
                Action::Char(c) if self.selector_step == SelectorStep::Book => {
                    self.selector_search.push(c);
                    self.selector_index = 0; // Reset selection when search changes
                    return Ok(());
                }
                Action::Backspace if self.selector_step == SelectorStep::Book => {
                    self.selector_search.pop();
                    self.selector_index = 0;
                    return Ok(());
                }
                Action::Char(c) if c.is_ascii_digit() => {
                    // Quick number jump for chapters/verses
                    if let Some(digit) = c.to_digit(10) {
                        self.selector_index = (digit as usize).saturating_sub(1);
                    }
                    return Ok(());
                }
                _ => return Ok(()),
            }
        }

        // If theme picker is open, handle navigation (check BEFORE settings!)
        if self.theme_picker_open {
            match action {
                Action::ScrollUp => {
                    let themes = crate::ui::themes::available_themes();
                    if self.theme_picker_index > 0 {
                        self.theme_picker_index -= 1;
                        // Live preview
                        self.theme = crate::ui::themes::get_theme(themes[self.theme_picker_index]);
                    }
                    return Ok(());
                }
                Action::ScrollDown => {
                    let themes = crate::ui::themes::available_themes();
                    if self.theme_picker_index < themes.len() - 1 {
                        self.theme_picker_index += 1;
                        // Live preview
                        self.theme = crate::ui::themes::get_theme(themes[self.theme_picker_index]);
                    }
                    return Ok(());
                }
                Action::Enter => {
                    // Persist and close
                    let themes = crate::ui::themes::available_themes();
                    self.settings.theme = themes[self.theme_picker_index].to_string();
                    self.settings.save()?;
                    self.theme_picker_open = false;
                    return Ok(());
                }
                Action::Escape => {
                    // Revert to saved theme and close
                    self.theme = crate::ui::themes::get_theme(&self.settings.theme);
                    self.theme_picker_open = false;
                    return Ok(());
                }
                Action::Quit => {
                    self.should_quit = true;
                    return Ok(());
                }
                _ => return Ok(()),
            }
        }

        // If settings modal is open, handle navigation and changes
        if self.settings_open {
            match action {
                Action::Escape => {
                    self.settings_open = false;
                    return Ok(());
                }
                Action::ScrollUp => {
                    if self.settings_selected_index > 0 {
                        self.settings_selected_index -= 1;
                    }
                    return Ok(());
                }
                Action::ScrollDown => {
                    // Max index is 2: theme (0), verse numbers (1), verse spacing (2)
                    if self.settings_selected_index < 2 {
                        self.settings_selected_index += 1;
                    }
                    return Ok(());
                }
                Action::Enter | Action::NextVerse => {
                    // Activate/toggle/cycle selected setting
                    match self.settings_selected_index {
                        0 => {
                            // Open theme picker
                            let themes = crate::ui::themes::available_themes();
                            self.theme_picker_index = themes.iter().position(|t| t == &self.settings.theme).unwrap_or(0);
                            self.theme_picker_open = true;
                        }
                        1 => {
                            // Toggle verse numbers
                            self.settings.show_verse_numbers = !self.settings.show_verse_numbers;
                            self.settings.save()?;
                        }
                        2 => {
                            // Toggle verse spacing
                            self.settings.verse_spacing = !self.settings.verse_spacing;
                            self.settings.save()?;
                        }
                        _ => {}
                    }
                    return Ok(());
                }
                Action::PreviousVerse => {
                    // No reverse actions for simplified settings
                    return Ok(());
                }
                Action::Quit => {
                    self.should_quit = true;
                    return Ok(());
                }
                // Block all other actions
                _ => return Ok(()),
            }
        }

        // If help modal is open, only allow closing and quit
        if self.help_open {
            match action {
                Action::Escape => {
                    self.help_open = false;
                    return Ok(());
                }
                Action::Quit => {
                    self.should_quit = true;
                    return Ok(());
                }
                // Block all other actions
                _ => return Ok(()),
            }
        }

        // If download modal is open, handle its actions
        if self.download_modal_open {
            match action {
                Action::Escape => {
                    // Can only close if not downloading
                    if !matches!(self.download_status, DownloadStatus::Downloading(_)) {
                        self.download_modal_open = false;
                        self.download_status = DownloadStatus::Ready;
                    }
                    return Ok(());
                }
                Action::ScrollUp => {
                    // Navigate translation list (only when ready)
                    if matches!(self.download_status, DownloadStatus::Ready) {
                        let count = crate::bible::translations::AVAILABLE_TRANSLATIONS.len();
                        if self.download_selected_index > 0 {
                            self.download_selected_index -= 1;
                        } else {
                            self.download_selected_index = count - 1;
                        }
                    }
                    return Ok(());
                }
                Action::ScrollDown => {
                    // Navigate translation list (only when ready)
                    if matches!(self.download_status, DownloadStatus::Ready) {
                        let count = crate::bible::translations::AVAILABLE_TRANSLATIONS.len();
                        self.download_selected_index = (self.download_selected_index + 1) % count;
                    }
                    return Ok(());
                }
                Action::Enter => {
                    match &self.download_status {
                        DownloadStatus::Ready => {
                            // Start download with selected translation
                            let translations = crate::bible::translations::AVAILABLE_TRANSLATIONS;
                            if let Some(translation) = translations.get(self.download_selected_index) {
                                return self.start_bible_download_for(translation);
                            }
                            return Ok(());
                        }
                        DownloadStatus::Complete => {
                            // Close modal and reload Bible
                            self.download_modal_open = false;
                            self.download_status = DownloadStatus::Ready;
                            return self.reload_bible();
                        }
                        DownloadStatus::Failed(_) => {
                            // Reset to ready state to try again
                            self.download_status = DownloadStatus::Ready;
                            return Ok(());
                        }
                        DownloadStatus::Downloading(_) => {
                            // Ignore while downloading
                            return Ok(());
                        }
                    }
                }
                Action::Quit => {
                    self.should_quit = true;
                    return Ok(());
                }
                _ => return Ok(()),
            }
        }

        match action {
            Action::Quit => self.should_quit = true,

            Action::OpenVerseSelector => {
                self.selector_open = true;
                self.selector_step = SelectorStep::Book;
                self.selector_search.clear();
                self.selector_index = 0;
                self.selector_selected_book = None;
                self.selector_selected_chapter = None;
            }

            Action::ScrollDown => {
                if let Some(chapter) = &self.current_chapter {
                    if self.current_verse_index < chapter.verses.len().saturating_sub(1) {
                        self.current_verse_index += 1;
                        self.adjust_scroll_for_current_verse();
                    } else {
                        // At end of chapter, try to go to next chapter
                        if let Some(current_chapter) = self.state.current_chapter {
                            let next_chapter_num = current_chapter + 1;
                            if let (Some(loader), Some(book)) = (&self.loader, &self.state.current_book) {
                                if let Ok(next_chapter) = loader.load_chapter(book, next_chapter_num) {
                                    if !next_chapter.verses.is_empty() {
                                        self.state.current_chapter = Some(next_chapter_num);
                                        self.current_chapter = Some(next_chapter);
                                        self.current_verse_index = 0;
                                        self.scroll_offset = 0;
                                    } else {
                                        self.try_next_book()?;
                                    }
                                } else {
                                    self.try_next_book()?;
                                }
                            }
                        }
                    }
                }
            }

            Action::ScrollUp => {
                if self.current_verse_index > 0 {
                    self.current_verse_index -= 1;
                    self.adjust_scroll_for_current_verse();
                } else {
                    // At beginning of chapter, try to go to previous chapter
                    if let Some(current_chapter) = self.state.current_chapter {
                        if current_chapter > 1 {
                            let prev_chapter_num = current_chapter - 1;
                            if let (Some(loader), Some(book)) = (&self.loader, &self.state.current_book) {
                                if let Ok(prev_chapter) = loader.load_chapter(book, prev_chapter_num) {
                                    self.state.current_chapter = Some(prev_chapter_num);
                                    self.current_verse_index = prev_chapter.verses.len().saturating_sub(1);
                                    self.current_chapter = Some(prev_chapter);
                                    // Set scroll to show last verses
                                    self.scroll_offset = self.current_verse_index.saturating_sub(20);
                                }
                            }
                        } else {
                            self.try_previous_book()?;
                        }
                    }
                }
            }

            Action::PageDown => {
                if let Some(chapter) = &self.current_chapter {
                    self.current_verse_index = (self.current_verse_index + 10)
                        .min(chapter.verses.len().saturating_sub(1));
                    self.scroll_offset = self.scroll_offset.saturating_add(10);
                }
            }

            Action::PageUp => {
                self.current_verse_index = self.current_verse_index.saturating_sub(10);
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
            }

            Action::GoToTop => {
                self.current_verse_index = 0;
                self.scroll_offset = 0;
            }

            Action::GoToBottom => {
                if let Some(chapter) = &self.current_chapter {
                    self.current_verse_index = chapter.verses.len().saturating_sub(1);
                    self.scroll_offset = self.current_verse_index.saturating_sub(20);
                }
            }


            Action::OpenSearch => {
                self.view_mode = ViewMode::Search;
                self.vim_normal_mode = false;
            }

            Action::Escape => {
                // Close modals if open
                if self.settings_open {
                    self.settings_open = false;
                } else if self.help_open {
                    self.help_open = false;
                } else {
                    self.view_mode = ViewMode::Reader;
                    self.vim_normal_mode = true;
                }
            }

            Action::OpenBookmarks => {
                self.view_mode = ViewMode::Bookmarks;
            }

            Action::OpenSettings => {
                self.settings_open = true;
            }

            Action::OpenHelp => {
                self.help_open = true;
            }

            Action::DownloadBible => {
                // Only show download modal if using sample data
                if self.using_sample_data {
                    self.download_modal_open = true;
                    self.download_status = DownloadStatus::Ready;
                }
            }

            Action::ToggleBookmark => {
                if let Some(chapter) = &self.current_chapter {
                    if let Some(verse) = chapter.verses.get(self.current_verse_index) {
                        if self.bookmarks.is_bookmarked(&verse.reference) {
                            self.bookmarks.remove(&verse.reference);
                        } else {
                            self.bookmarks
                                .add(crate::config::Bookmark::new(verse.reference.clone()));
                        }
                        self.bookmarks.save()?;
                    }
                }
            }


            Action::Char(c) if self.view_mode == ViewMode::Search => {
                self.search_query.push(c);
            }

            Action::Backspace if self.view_mode == ViewMode::Search => {
                self.search_query.pop();
            }

            _ => {}
        }

        // Persist current reading position to state
        self.state.current_verse_index = self.current_verse_index;

        Ok(())
    }

    /// Save app state
    pub fn save_state(&self) -> Result<()> {
        self.state.save()?;
        self.settings.save()?;
        self.bookmarks.save()?;
        Ok(())
    }

    /// Adjust scroll offset to keep current verse visible
    /// Uses a viewport height of approximately 30 lines (typical terminal)
    fn adjust_scroll_for_current_verse(&mut self) {
        const VIEWPORT_HEIGHT: usize = 30;

        // If current verse is above visible area, scroll up
        if self.current_verse_index < self.scroll_offset {
            self.scroll_offset = self.current_verse_index;
        }
        // If current verse is below visible area, scroll down
        else if self.current_verse_index >= self.scroll_offset + VIEWPORT_HEIGHT {
            self.scroll_offset = self.current_verse_index.saturating_sub(VIEWPORT_HEIGHT - 1);
        }
        // Otherwise, don't scroll - verse stays in place
    }

    /// Try to move to the next book in the Bible
    fn try_next_book(&mut self) -> Result<()> {
        use crate::bible::BOOK_ORDER;

        if let Some(current_book) = &self.state.current_book {
            // Find current book index
            if let Some(current_idx) = BOOK_ORDER.iter().position(|(name, _, _, _)| name == current_book) {
                // Check if there's a next book
                if current_idx + 1 < BOOK_ORDER.len() {
                    let (next_book, _, _, _) = BOOK_ORDER[current_idx + 1];
                    self.state.current_book = Some(next_book.to_string());
                    self.state.current_chapter = Some(1);
                    self.load_current_chapter()?;
                    self.current_verse_index = 0;
                }
            }
        }
        Ok(())
    }

    /// Try to move to the previous book in the Bible
    fn try_previous_book(&mut self) -> Result<()> {
        use crate::bible::BOOK_ORDER;

        if let Some(current_book) = &self.state.current_book {
            // Find current book index
            if let Some(current_idx) = BOOK_ORDER.iter().position(|(name, _, _, _)| name == current_book) {
                // Check if there's a previous book
                if current_idx > 0 {
                    let (prev_book, _, _, chapter_count) = BOOK_ORDER[current_idx - 1];
                    self.state.current_book = Some(prev_book.to_string());
                    self.state.current_chapter = Some(chapter_count);
                    self.load_current_chapter()?;
                    // Go to last verse of the last chapter
                    if let Some(chapter) = &self.current_chapter {
                        self.current_verse_index = chapter.verses.len().saturating_sub(1);
                    }
                }
            }
        }
        Ok(())
    }

    /// Handle selector navigation down (j key)
    fn handle_selector_down(&mut self) -> Result<()> {
        match self.selector_step {
            SelectorStep::Book => {
                let filtered_books = self.get_filtered_books();
                if !filtered_books.is_empty() {
                    self.selector_index = (self.selector_index + 1).min(filtered_books.len() - 1);
                }
            }
            SelectorStep::Chapter => {
                if let Some(book) = &self.selector_selected_book {
                    let chapter_count = crate::bible::get_chapter_count(book);
                    if self.selector_index + 1 < chapter_count as usize {
                        self.selector_index += 1;
                    }
                }
            }
            SelectorStep::Verse => {
                if let Some(chapter) = &self.current_chapter {
                    if self.selector_index + 1 < chapter.verses.len() {
                        self.selector_index += 1;
                    }
                }
            }
        }
        Ok(())
    }

    /// Handle selector navigation up (k key)
    fn handle_selector_up(&mut self) -> Result<()> {
        if self.selector_index > 0 {
            self.selector_index -= 1;
        }
        Ok(())
    }

    /// Handle selector back (Escape key)
    fn handle_selector_back(&mut self) -> Result<()> {
        match self.selector_step {
            SelectorStep::Book => {
                // Close the selector
                self.selector_open = false;
                self.selector_search.clear();
            }
            SelectorStep::Chapter => {
                // Go back to book selection
                self.selector_step = SelectorStep::Book;
                self.selector_index = 0;
                self.selector_selected_book = None;
            }
            SelectorStep::Verse => {
                // Go back to chapter selection
                self.selector_step = SelectorStep::Chapter;
                self.selector_index = 0;
                self.selector_selected_chapter = None;
            }
        }
        Ok(())
    }

    /// Handle selector selection (Enter key)
    fn handle_selector_select(&mut self) -> Result<()> {
        match self.selector_step {
            SelectorStep::Book => {
                // Select book and move to chapter selection
                let filtered_books = self.get_filtered_books();
                if let Some((short_name, _, _, _)) = filtered_books.get(self.selector_index) {
                    self.selector_selected_book = Some(short_name.to_string());
                    self.selector_step = SelectorStep::Chapter;
                    self.selector_index = 0;
                    self.selector_search.clear();
                }
            }
            SelectorStep::Chapter => {
                // Select chapter and move to verse selection
                let selected_chapter = (self.selector_index + 1) as u32;
                self.selector_selected_chapter = Some(selected_chapter);

                // Load the chapter so we can show verse count
                if let (Some(loader), Some(book)) = (&self.loader, &self.selector_selected_book) {
                    if let Ok(chapter) = loader.load_chapter(book, selected_chapter) {
                        self.current_chapter = Some(chapter);
                    }
                }

                self.selector_step = SelectorStep::Verse;
                self.selector_index = 0;
            }
            SelectorStep::Verse => {
                // Jump to the selected verse and close selector
                if let (Some(book), Some(chapter)) = (
                    &self.selector_selected_book,
                    self.selector_selected_chapter,
                ) {
                    self.state.current_book = Some(book.clone());
                    self.state.current_chapter = Some(chapter);
                    self.current_verse_index = self.selector_index;

                    // Ensure chapter is loaded
                    self.load_current_chapter()?;

                    // Reset scroll to top of chapter, then adjust for selected verse
                    self.scroll_offset = 0;
                    self.adjust_scroll_for_current_verse();

                    // Close selector
                    self.selector_open = false;
                    self.selector_search.clear();
                    self.selector_selected_book = None;
                    self.selector_selected_chapter = None;
                }
            }
        }
        Ok(())
    }

    /// Get filtered list of books based on search query
    fn get_filtered_books(&self) -> Vec<(&'static str, &'static str, &'static str, u32)> {
        use crate::bible::{BOOK_ORDER, Testament};

        let search_lower = self.selector_search.to_lowercase();

        BOOK_ORDER
            .iter()
            .filter(|(short, full, _, _)| {
                if self.selector_search.is_empty() {
                    true
                } else {
                    full.to_lowercase().contains(&search_lower)
                        || short.to_lowercase().contains(&search_lower)
                }
            })
            .map(|(short, full, testament, chapter_count)| {
                let testament_str = match testament {
                    Testament::Old => "Old Testament",
                    Testament::New => "New Testament",
                };
                (*short, *full, testament_str, *chapter_count)
            })
            .collect()
    }

    /// Start downloading the Bible for a specific translation
    fn start_bible_download_for(&mut self, translation: &crate::bible::translations::TranslationInfo) -> Result<()> {
        self.download_status = DownloadStatus::Downloading(format!("Downloading {}...", translation.name));

        // Perform the download synchronously (blocking)
        match download_bible(translation) {
            Ok(_) => {
                self.download_status = DownloadStatus::Complete;
            }
            Err(e) => {
                self.download_status = DownloadStatus::Failed(e.to_string());
            }
        }

        Ok(())
    }

    /// Reload the Bible after download
    fn reload_bible(&mut self) -> Result<()> {
        let translations = crate::bible::translations::AVAILABLE_TRANSLATIONS;
        if let Some(translation) = translations.get(self.download_selected_index) {
            let data_dir = crate::config::data_dir()?;
            let translations_dir = data_dir.join("translations");
            let db_path = translations_dir.join(format!("{}.sqlite", translation.id.to_lowercase()));

            if db_path.exists() {
                let loader = BibleLoader::new(db_path.to_string_lossy().as_ref())?;
                self.loader = Some(loader);
                self.using_sample_data = false;
                self.settings.translation = translation.id.to_string();
                self.settings.save()?;
                self.load_current_chapter()?;
            }
        }

        Ok(())
    }
}

/// Download a Bible translation from the internet
fn download_bible(translation: &crate::bible::translations::TranslationInfo) -> Result<()> {
    use crate::bible::loader::init_database;
    use std::io::Read;

    let response = ureq::get(translation.url)
        .call()
        .map_err(|e| anyhow::anyhow!("Download failed: {}", e))?;

    // Check response status
    let status = response.status();
    if status != 200 {
        anyhow::bail!("Server returned status {}", status);
    }

    let mut json_content = String::new();
    response.into_reader().read_to_string(&mut json_content)?;

    // Check we got some content
    if json_content.is_empty() {
        anyhow::bail!("Empty response from server");
    }

    // Parse the JSON with better error message
    let raw_data: serde_json::Value = serde_json::from_str(&json_content)
        .map_err(|e| {
            let preview: String = json_content.chars().take(100).collect();
            anyhow::anyhow!("Invalid JSON: {}. Response starts with: {}", e, preview)
        })?;

    // Set up database path - use translation ID as filename
    let data_dir = crate::config::data_dir()?;
    let translations_dir = data_dir.join("translations");
    std::fs::create_dir_all(&translations_dir)?;
    let db_path = translations_dir.join(format!("{}.sqlite", translation.id.to_lowercase()));

    // Remove old database if exists
    if db_path.exists() {
        std::fs::remove_file(&db_path)?;
    }

    // Initialize database schema
    init_database(&db_path)?;

    // Open database and import
    let conn = rusqlite::Connection::open(&db_path)?;

    // Insert translation metadata
    conn.execute(
        "INSERT INTO translations (id, name, abbreviation, language, description)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            translation.id,
            translation.name,
            translation.id,
            translation.language,
            translation.description
        ],
    )?;

    // Import books and verses - handle different JSON formats
    // Format 1 (thiagobodruk): [{"book": "Genesis", "chapters": [[verses...], ...]}]
    // Format 2 (scrollmapper): {"resultset": {"row": [{"field": [book_id, chapter, verse, text]}]}}

    if let Some(books) = raw_data.as_array() {
        // Format 1: Array of books with chapters
        import_thiagobodruk_format(&conn, books)?;
    } else if let Some(resultset) = raw_data.get("resultset") {
        // Format 2: Scrollmapper format
        import_scrollmapper_format(&conn, resultset)?;
    } else {
        anyhow::bail!("Unknown Bible JSON format");
    }

    Ok(())
}

/// Import from thiagobodruk JSON format
fn import_thiagobodruk_format(conn: &rusqlite::Connection, books: &[serde_json::Value]) -> Result<()> {
    for book in books {
        let book_name = book.get("book")
            .or_else(|| book.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        if let Some(chapters) = book.get("chapters").and_then(|c| c.as_array()) {
            for (chapter_idx, chapter) in chapters.iter().enumerate() {
                let chapter_num = (chapter_idx + 1) as u32;

                if let Some(verses) = chapter.as_array() {
                    for (verse_idx, verse_text) in verses.iter().enumerate() {
                        let verse_num = (verse_idx + 1) as u32;
                        let text = verse_text.as_str().unwrap_or("");

                        conn.execute(
                            "INSERT INTO verses (book, chapter, verse, text)
                             VALUES (?1, ?2, ?3, ?4)",
                            rusqlite::params![book_name, chapter_num, verse_num, text],
                        )?;
                    }
                }
            }
        }
    }
    Ok(())
}

/// Import from scrollmapper JSON format
fn import_scrollmapper_format(conn: &rusqlite::Connection, resultset: &serde_json::Value) -> Result<()> {
    use crate::bible::BOOK_ORDER;

    if let Some(rows) = resultset.get("row").and_then(|r| r.as_array()) {
        for row in rows {
            if let Some(fields) = row.get("field").and_then(|f| f.as_array()) {
                // Fields: [book_id, chapter, verse, text]
                if fields.len() >= 4 {
                    let book_id = fields[0].as_i64().or_else(|| fields[0].as_str().and_then(|s| s.parse().ok())).unwrap_or(1) as usize;
                    let chapter = fields[1].as_i64().or_else(|| fields[1].as_str().and_then(|s| s.parse().ok())).unwrap_or(1) as u32;
                    let verse = fields[2].as_i64().or_else(|| fields[2].as_str().and_then(|s| s.parse().ok())).unwrap_or(1) as u32;
                    let text = fields[3].as_str().unwrap_or("");

                    // Convert book_id to book name using BOOK_ORDER
                    let book_name = if book_id > 0 && book_id <= BOOK_ORDER.len() {
                        BOOK_ORDER[book_id - 1].0
                    } else {
                        "Unknown"
                    };

                    conn.execute(
                        "INSERT OR IGNORE INTO verses (book, chapter, verse, text)
                         VALUES (?1, ?2, ?3, ?4)",
                        rusqlite::params![book_name, chapter, verse, text],
                    )?;
                }
            }
        }
    }
    Ok(())
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.save_state();
    }
}
