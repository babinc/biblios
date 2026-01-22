mod app;
mod bible;
mod config;
mod input;
mod ui;
mod utils;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

fn main() -> Result<()> {
    // Handle CLI arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--import" => {
                if args.len() < 3 {
                    eprintln!("Error: --import requires a JSON file path");
                    eprintln!("Usage: biblios --import <bible.json>");
                    std::process::exit(1);
                }
                return import_bible(&args[2]);
            }
            "--version" | "-v" => {
                println!("biblios {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            arg => {
                eprintln!("Unknown argument: {}", arg);
                print_help();
                std::process::exit(1);
            }
        }
    }

    // Initialize app
    let (db_path, is_sample) = get_or_create_sample_db()?;
    let mut app = App::new()?.with_bible(&db_path, is_sample)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events, not release or repeat
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Check for quit
                if matches!(
                    key.code,
                    KeyCode::Char('q') | KeyCode::Char('c')
                ) && app.view_mode == app::ViewMode::Reader {
                    break;
                }

                // Process key event
                let action = if app.selector_open {
                    // When selector is open, handle text input specially
                    input::process_selector_key_event(key)
                } else if app.download_modal_open {
                    // Download modal allows navigation
                    input::process_modal_key_event(key, true)
                } else if app.theme_picker_open {
                    // Theme picker allows navigation
                    input::process_modal_key_event(key, true)
                } else if app.settings_open {
                    // Settings modal allows navigation to change settings
                    input::process_modal_key_event(key, true)
                } else if app.help_open {
                    // Help modal is read-only, only allow close
                    input::process_modal_key_event(key, false)
                } else {
                    input::process_key_event(key)
                };

                app.handle_action(action)?;

                if app.should_quit {
                    break;
                }
            }
        }
    }

    Ok(())
}

/// Get the KJV Bible database path, returning (path, is_sample_data)
fn get_or_create_sample_db() -> Result<(String, bool)> {
    let data_dir = config::data_dir()?;
    let translations_dir = data_dir.join("translations");
    let kjv_path = translations_dir.join("kjv.sqlite");

    // If KJV doesn't exist, create sample database
    if !kjv_path.exists() {
        let sample_path = data_dir.join("sample.db");
        if !sample_path.exists() {
            create_sample_database(&sample_path)?;
        }
        return Ok((sample_path.to_string_lossy().to_string(), true));
    }

    Ok((kjv_path.to_string_lossy().to_string(), false))
}

/// Create a sample Bible database with John 3:16-17
fn create_sample_database(db_path: &std::path::Path) -> Result<()> {
    use bible::loader::init_database;
    use rusqlite::Connection;

    // Initialize the schema
    init_database(db_path)?;

    let conn = Connection::open(db_path)?;

    // Insert sample translation
    conn.execute(
        "INSERT INTO translations (id, name, abbreviation, language, description)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            "KJV",
            "King James Version",
            "KJV",
            "en",
            "The King James Version (KJV) is an English translation of the Christian Bible."
        ],
    )?;

    // Insert John chapter 1 (first few verses as sample)
    let john_1_verses = vec![
        "In the beginning was the Word, and the Word was with God, and the Word was God.",
        "The same was in the beginning with God.",
        "All things were made by him; and without him was not any thing made that was made.",
        "In him was life; and the life was the light of men.",
        "And the light shineth in darkness; and the darkness comprehended it not.",
    ];

    for (i, verse_text) in john_1_verses.iter().enumerate() {
        conn.execute(
            "INSERT INTO verses (book, chapter, verse, text) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["John", 1, (i + 1) as u32, verse_text],
        )?;
    }

    // Insert John 3:16-17 (famous verses)
    conn.execute(
        "INSERT INTO verses (book, chapter, verse, text) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            "John",
            3,
            16,
            "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life."
        ],
    )?;

    conn.execute(
        "INSERT INTO verses (book, chapter, verse, text) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            "John",
            3,
            17,
            "For God sent not his Son into the world to condemn the world; but that the world through him might be saved."
        ],
    )?;

    Ok(())
}

fn print_help() {
    println!("biblios - A terminal Bible reader");
    println!();
    println!("USAGE:");
    println!("    biblios              Run the Bible reader");
    println!("    biblios --import <file.json>  Import a Bible from JSON");
    println!("    biblios --help       Show this help message");
    println!("    biblios --version    Show version");
    println!();
    println!("IMPORT FORMAT:");
    println!("    The JSON file should have this structure:");
    println!("    {{");
    println!("      \"translation\": {{");
    println!("        \"id\": \"KJV\",");
    println!("        \"name\": \"King James Version\",");
    println!("        \"abbreviation\": \"KJV\",");
    println!("        \"language\": \"en\"");
    println!("      }},");
    println!("      \"books\": [");
    println!("        {{");
    println!("          \"name\": \"Genesis\",");
    println!("          \"chapters\": [[\"verse1\", \"verse2\", ...], ...]");
    println!("        }}");
    println!("      ]");
    println!("    }}");
    println!();
    println!("To download the KJV Bible, run: scripts/download-kjv.sh");
}

fn import_bible(json_path: &str) -> Result<()> {
    use bible::loader::init_database;
    use std::path::Path;

    let json_path = Path::new(json_path);
    if !json_path.exists() {
        anyhow::bail!("File not found: {}", json_path.display());
    }

    // Set up the database path
    let data_dir = config::data_dir()?;
    let translations_dir = data_dir.join("translations");
    std::fs::create_dir_all(&translations_dir)?;
    let db_path = translations_dir.join("kjv.sqlite");

    println!("Importing Bible from: {}", json_path.display());
    println!("Database location: {}", db_path.display());

    // Initialize database schema
    init_database(&db_path)?;

    // Import the JSON Bible
    bible::parser::import_json_bible(json_path, &db_path)?;

    println!("Import complete! Run 'biblios' to start reading.");

    Ok(())
}
