use crate::ui::themes::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Wrap},
    Frame,
};

/// Configuration for a filterable select component
pub struct FilteredSelectConfig<'a> {
    pub title: String,
    pub items: Vec<SelectItem>,
    pub search_query: &'a str,
    pub selected_index: usize,
    pub show_search: bool,
    pub search_placeholder: &'a str,
    pub help_text: Option<String>,
}

/// An item in the select list
#[derive(Clone)]
pub struct SelectItem {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

impl SelectItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
            icon: None,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Reusable filtered select dropdown component
pub struct FilteredSelect;

impl FilteredSelect {
    /// Render a modal filtered select dropdown
    pub fn render(f: &mut Frame, config: FilteredSelectConfig, theme: &Theme, area: Rect) {
        // Create centered modal (70% width, 80% height)
        let modal_area = Self::centered_rect(70, 80, area);

        // Clear the area behind the modal
        f.render_widget(Clear, modal_area);

        // Main layout
        let chunks = if config.show_search {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Search box
                    Constraint::Min(0),    // Items list
                    Constraint::Length(3), // Help text
                ])
                .split(modal_area)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),    // Items list
                    Constraint::Length(3), // Help text
                ])
                .split(modal_area)
        };

        let (items_area, help_area) = if config.show_search {
            // Render search box
            Self::render_search_box(f, &config, theme, chunks[0]);
            (chunks[1], chunks[2])
        } else {
            (chunks[0], chunks[1])
        };

        // Render items list
        Self::render_items_list(f, &config, theme, items_area);

        // Render help text
        Self::render_help(f, &config, theme, help_area);
    }

    /// Render search input box
    fn render_search_box(f: &mut Frame, config: &FilteredSelectConfig, theme: &Theme, area: Rect) {
        let search_text = if config.search_query.is_empty() {
            config.search_placeholder
        } else {
            config.search_query
        };

        let search_block = theme.block_active_with_title(&config.title);

        let search_widget = Paragraph::new(search_text)
            .block(search_block)
            .style(if config.search_query.is_empty() {
                theme.text_muted()
            } else {
                theme.current()
            });

        f.render_widget(search_widget, area);
    }

    /// Render the items list
    fn render_items_list(f: &mut Frame, config: &FilteredSelectConfig, theme: &Theme, area: Rect) {
        let mut lines = Vec::new();

        for (idx, item) in config.items.iter().enumerate() {
            let is_selected = idx == config.selected_index;

            // Build the line
            let mut text = String::new();

            // Add selection indicator
            if is_selected {
                text.push_str("▶ ");
            } else {
                text.push_str("  ");
            }

            // Add icon if present
            if let Some(icon) = &item.icon {
                text.push_str(icon);
                text.push(' ');
            }

            // Add label
            text.push_str(&item.label);

            // Add description if present
            if let Some(desc) = &item.description {
                text.push_str(&format!(" ({})", desc));
            }

            let style = if is_selected {
                theme.selected()
            } else {
                theme.text_secondary()
            };

            lines.push(Line::styled(text, style));
        }

        // Show "no results" if empty
        if lines.is_empty() {
            lines.push(Line::styled(
                "No results found",
                theme.text_muted(),
            ));
        }

        let block = if config.show_search {
            theme.block_with_title(format!("{} items", config.items.len()))
        } else {
            theme.block_modal_with_title(&config.title)
        };

        let items_widget = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false });

        f.render_widget(items_widget, area);
    }

    /// Render help text at the bottom
    fn render_help(f: &mut Frame, config: &FilteredSelectConfig, theme: &Theme, area: Rect) {
        let help_line = if let Some(custom_help) = &config.help_text {
            Line::styled(custom_help, theme.text_muted())
        } else {
            Line::from(vec![
                Span::styled(" j/k ", theme.accent()),
                Span::styled("navigate ", theme.text_muted()),
                Span::styled(" Enter ", theme.accent()),
                Span::styled("select ", theme.text_muted()),
                Span::styled(" Esc ", theme.error()),
                Span::styled("cancel", theme.text_muted()),
            ])
        };

        let help_widget = Paragraph::new(help_line)
            .block(theme.block_secondary())
            .alignment(Alignment::Center);

        f.render_widget(help_widget, area);
    }

    /// Helper function to create a centered rectangle
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

/// Number grid component for chapter/verse selection
pub struct NumberGrid;

impl NumberGrid {
    /// Render a number grid (for chapters or verses)
    pub fn render(
        f: &mut Frame,
        title: &str,
        max_number: usize,
        selected: usize,
        theme: &Theme,
        area: Rect,
    ) {
        // Create centered modal
        let modal_area = FilteredSelect::centered_rect(60, 70, area);
        f.render_widget(Clear, modal_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(0),    // Grid
                Constraint::Length(3), // Help
            ])
            .split(modal_area);

        // Title
        let title_widget = Paragraph::new(title)
            .block(theme.block_modal_with_title(format!("Select (1-{})", max_number)))
            .alignment(Alignment::Center)
            .style(theme.text_secondary());

        f.render_widget(title_widget, chunks[0]);

        // Number grid (10 per row)
        let mut lines = Vec::new();
        let mut current_line = Vec::new();

        for num in 1..=max_number {
            let is_selected = num == selected + 1;

            let num_str = format!("{:>3}", num);
            let style = if is_selected {
                theme.selected()
            } else {
                theme.text_secondary()
            };

            current_line.push(Span::styled(num_str, style));
            current_line.push(Span::raw("  "));

            if num % 10 == 0 || num == max_number {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
            }
        }

        let grid_widget = Paragraph::new(lines)
            .block(theme.block())
            .alignment(Alignment::Center);

        f.render_widget(grid_widget, chunks[1]);

        // Help
        let help_line = Line::from(vec![
            Span::styled(" j/k ", theme.accent()),
            Span::styled("navigate ", theme.text_muted()),
            Span::styled(" 1-9 ", theme.accent()),
            Span::styled("jump ", theme.text_muted()),
            Span::styled(" Enter ", theme.accent()),
            Span::styled("select ", theme.text_muted()),
            Span::styled(" Esc ", theme.error()),
            Span::styled("cancel", theme.text_muted()),
        ]);

        let help_widget = Paragraph::new(help_line)
            .block(theme.block_secondary())
            .alignment(Alignment::Center);

        f.render_widget(help_widget, chunks[2]);
    }
}
