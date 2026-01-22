use crate::app::{App, DownloadStatus};
use crate::bible::translations::AVAILABLE_TRANSLATIONS;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

/// Render the download modal
pub fn render(f: &mut Frame, app: &App) {
    // Create centered modal - make it taller for the list
    let area = centered_rect(60, 70, f.area());

    // Clear background
    f.render_widget(Clear, area);

    // Modal block
    let block = Block::default()
        .title(" Download Bible Translation ")
        .title_style(Style::default().fg(app.theme.text_heading).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.border_active))
        .style(Style::default().bg(app.theme.bg_modal));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Layout for content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),  // Header text
            Constraint::Min(0),     // Translation list or status
            Constraint::Length(2),  // Actions hint
        ])
        .split(inner);

    // Header based on status
    let header = match &app.download_status {
        DownloadStatus::Ready => Line::from(vec![
            Span::styled("Select a translation to download:", Style::default().fg(app.theme.text_secondary)),
        ]),
        DownloadStatus::Downloading(msg) => Line::from(vec![
            Span::styled(msg.clone(), Style::default().fg(app.theme.accent_primary)),
        ]),
        DownloadStatus::Complete => Line::from(vec![
            Span::styled("Download complete!", Style::default().fg(app.theme.accent_success).add_modifier(Modifier::BOLD)),
        ]),
        DownloadStatus::Failed(err) => Line::from(vec![
            Span::styled(format!("Error: {}", err), Style::default().fg(app.theme.accent_error)),
        ]),
    };

    let header_paragraph = Paragraph::new(header).alignment(Alignment::Center);
    f.render_widget(header_paragraph, chunks[0]);

    // Main content area
    match &app.download_status {
        DownloadStatus::Ready => {
            render_translation_list(f, app, chunks[1]);
        }
        DownloadStatus::Downloading(_) => {
            render_progress(f, app, chunks[1]);
        }
        DownloadStatus::Complete => {
            render_complete(f, app, chunks[1]);
        }
        DownloadStatus::Failed(_) => {
            render_failed(f, app, chunks[1]);
        }
    }

    // Actions hint at bottom
    let hint = match &app.download_status {
        DownloadStatus::Ready => Line::from(vec![
            Span::styled("j/k", Style::default().fg(app.theme.accent_primary).add_modifier(Modifier::BOLD)),
            Span::styled(" navigate  ", Style::default().fg(app.theme.text_muted)),
            Span::styled("Enter", Style::default().fg(app.theme.accent_primary).add_modifier(Modifier::BOLD)),
            Span::styled(" download  ", Style::default().fg(app.theme.text_muted)),
            Span::styled("Esc", Style::default().fg(app.theme.accent_primary).add_modifier(Modifier::BOLD)),
            Span::styled(" cancel", Style::default().fg(app.theme.text_muted)),
        ]),
        DownloadStatus::Downloading(_) => Line::from(vec![
            Span::styled("Please wait...", Style::default().fg(app.theme.text_muted)),
        ]),
        DownloadStatus::Complete => Line::from(vec![
            Span::styled("Enter", Style::default().fg(app.theme.accent_primary).add_modifier(Modifier::BOLD)),
            Span::styled(" to start reading", Style::default().fg(app.theme.text_muted)),
        ]),
        DownloadStatus::Failed(_) => Line::from(vec![
            Span::styled("Enter", Style::default().fg(app.theme.accent_primary).add_modifier(Modifier::BOLD)),
            Span::styled(" retry  ", Style::default().fg(app.theme.text_muted)),
            Span::styled("Esc", Style::default().fg(app.theme.accent_primary).add_modifier(Modifier::BOLD)),
            Span::styled(" cancel", Style::default().fg(app.theme.text_muted)),
        ]),
    };

    let hint_paragraph = Paragraph::new(hint).alignment(Alignment::Center);
    f.render_widget(hint_paragraph, chunks[2]);
}

fn render_translation_list(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();
    let mut current_language = "";

    for (idx, translation) in AVAILABLE_TRANSLATIONS.iter().enumerate() {
        // Add language header if changed
        if translation.language != current_language {
            if !current_language.is_empty() {
                lines.push(Line::from(""));
            }
            let lang_name = match translation.language {
                "en" => "English",
                "es" => "Spanish",
                "pt" => "Portuguese",
                "de" => "German",
                "fr" => "French",
                "zh" => "Chinese",
                "ru" => "Russian",
                "ko" => "Korean",
                _ => translation.language,
            };
            lines.push(Line::from(Span::styled(
                format!("  {}", lang_name),
                Style::default().fg(app.theme.text_muted).add_modifier(Modifier::BOLD),
            )));
            current_language = translation.language;
        }

        let is_selected = idx == app.download_selected_index;

        // Build the line - show index for debugging
        let prefix = if is_selected { " > " } else { "   " };
        let style = if is_selected {
            Style::default().fg(app.theme.accent_primary).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.text_secondary)
        };

        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{:<5}", translation.id), style),
            Span::styled(" ", Style::default()),
            Span::styled(translation.name, style),
            Span::styled(format!(" [{}]", idx), Style::default().fg(app.theme.text_muted)),
        ]));

        // Show description for selected item
        if is_selected {
            lines.push(Line::from(vec![
                Span::styled("      ", Style::default()),
                Span::styled(translation.description, Style::default().fg(app.theme.text_muted)),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

fn render_progress(f: &mut Frame, app: &App, area: Rect) {
    let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let frame_idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() / 100) as usize % spinner_frames.len();

    let lines = vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}  ", spinner_frames[frame_idx]),
            Style::default().fg(app.theme.accent_primary).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "This may take a moment...",
            Style::default().fg(app.theme.text_muted),
        )),
    ];

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_complete(f: &mut Frame, app: &App, area: Rect) {
    let translation = AVAILABLE_TRANSLATIONS.get(app.download_selected_index);
    let name = translation.map(|t| t.name).unwrap_or("Bible");

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("{} has been downloaded!", name),
            Style::default().fg(app.theme.text_secondary),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to start reading.",
            Style::default().fg(app.theme.text_muted),
        )),
    ];

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_failed(f: &mut Frame, app: &App, area: Rect) {
    let error_msg = if let DownloadStatus::Failed(err) = &app.download_status {
        err.clone()
    } else {
        "Unknown error".to_string()
    };

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            error_msg,
            Style::default().fg(app.theme.accent_error),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to retry or Esc to cancel.",
            Style::default().fg(app.theme.text_muted),
        )),
    ];

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

/// Create a centered rect of given percentage size
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
