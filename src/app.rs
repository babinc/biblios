use crate::bible::{loader::BibleLoader, Chapter, VerseReference};
use crate::config::{BookmarkManager, ReadingState, Settings};
use crate::input::Action;
use crate::ui::themes::{get_theme, Theme};
use anyhow::Result;

/// Application state and data
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

    /// Scroll offset
    pub scroll_offset: usize,

    /// Selected bookmark index (when viewing bookmarks)
    pub bookmark_selected: usize,

    /// Vim normal mode (true) or insert mode (false)
    pub vim_normal_mode: bool,

    /// Whether the app should quit
    pub should_quit: bool,
}

/// Different view modes in the app
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Reader,
    Search,
    Bookmarks,
    Settings,
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
            scroll_offset: 0,
            bookmark_selected: 0,
            vim_normal_mode: true,
            should_quit: false,
        })
    }

    /// Initialize with a Bible database
    pub fn with_bible(mut self, db_path: &str) -> Result<Self> {
        let loader = BibleLoader::new(db_path)?;
        self.loader = Some(loader);
        self.load_current_chapter()?;
        Ok(self)
    }

    /// Load the current chapter based on state
    pub fn load_current_chapter(&mut self) -> Result<()> {
        if let (Some(loader), Some(book), Some(chapter)) = (
            &self.loader,
            &self.state.current_book,
            self.state.current_chapter,
        ) {
            self.current_chapter = Some(loader.load_chapter(book, chapter)?);
            self.scroll_offset = self.state.scroll_offset;
        }
        Ok(())
    }

    /// Handle an action
    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Quit => self.should_quit = true,

            Action::ScrollDown => {
                if let Some(chapter) = &self.current_chapter {
                    if self.scroll_offset < chapter.verses.len().saturating_sub(1) {
                        self.scroll_offset += 1;
                    }
                }
            }

            Action::ScrollUp => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }

            Action::PageDown => {
                if let Some(chapter) = &self.current_chapter {
                    self.scroll_offset = (self.scroll_offset + 10)
                        .min(chapter.verses.len().saturating_sub(1));
                }
            }

            Action::PageUp => {
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
            }

            Action::GoToTop => {
                self.scroll_offset = 0;
            }

            Action::GoToBottom => {
                if let Some(chapter) = &self.current_chapter {
                    self.scroll_offset = chapter.verses.len().saturating_sub(1);
                }
            }

            Action::NextChapter => {
                if let Some(current_chapter) = self.state.current_chapter {
                    self.state.current_chapter = Some(current_chapter + 1);
                    self.load_current_chapter()?;
                    self.scroll_offset = 0;
                }
            }

            Action::PreviousChapter => {
                if let Some(current_chapter) = self.state.current_chapter {
                    if current_chapter > 1 {
                        self.state.current_chapter = Some(current_chapter - 1);
                        self.load_current_chapter()?;
                        self.scroll_offset = 0;
                    }
                }
            }

            Action::OpenSearch => {
                self.view_mode = ViewMode::Search;
                self.vim_normal_mode = false;
            }

            Action::CloseSearch | Action::Escape => {
                self.view_mode = ViewMode::Reader;
                self.vim_normal_mode = true;
            }

            Action::OpenBookmarks => {
                self.view_mode = ViewMode::Bookmarks;
            }

            Action::CloseBookmarks => {
                self.view_mode = ViewMode::Reader;
            }

            Action::OpenSettings => {
                self.view_mode = ViewMode::Settings;
            }

            Action::CloseSettings => {
                self.view_mode = ViewMode::Reader;
            }

            Action::OpenHelp => {
                self.view_mode = ViewMode::Help;
            }

            Action::CloseHelp => {
                self.view_mode = ViewMode::Reader;
            }

            Action::ToggleBookmark => {
                if let Some(chapter) = &self.current_chapter {
                    if let Some(verse) = chapter.verses.get(self.scroll_offset) {
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

            Action::ToggleInputMode => {
                self.settings.toggle_input_mode();
                self.settings.save()?;
            }

            Action::ToggleFocusMode => {
                self.settings.toggle_focus_mode();
                self.settings.save()?;
            }

            Action::Char(c) if self.view_mode == ViewMode::Search => {
                self.search_query.push(c);
            }

            Action::Backspace if self.view_mode == ViewMode::Search => {
                self.search_query.pop();
            }

            _ => {}
        }

        // Update state
        self.state.scroll_offset = self.scroll_offset;

        Ok(())
    }

    /// Save app state
    pub fn save_state(&self) -> Result<()> {
        self.state.save()?;
        self.settings.save()?;
        self.bookmarks.save()?;
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.save_state();
    }
}
