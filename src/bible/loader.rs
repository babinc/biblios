use super::{Bible, Book, Chapter, Translation, Verse, VerseReference, BOOK_ORDER};
use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::Path;

/// Loads Bible data from SQLite database
pub struct BibleLoader {
    conn: Connection,
}

impl BibleLoader {
    /// Create a new loader from a database file
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(db_path)
            .context("Failed to open Bible database")?;
        Ok(Self { conn })
    }

    /// Load translation metadata
    pub fn load_translation(&self) -> Result<Translation> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, abbreviation, language, description FROM translations LIMIT 1"
        )?;

        let translation = stmt.query_row([], |row| {
            Ok(Translation {
                id: row.get(0)?,
                name: row.get(1)?,
                abbreviation: row.get(2)?,
                language: row.get(3)?,
                description: row.get(4)?,
            })
        }).context("Failed to load translation metadata")?;

        Ok(translation)
    }

    /// Load all books
    pub fn load_books(&self) -> Result<Vec<Book>> {
        let books = BOOK_ORDER
            .iter()
            .map(|(name, full_name, testament, chapter_count)| Book {
                name: name.to_string(),
                full_name: full_name.to_string(),
                testament: *testament,
                chapter_count: *chapter_count,
            })
            .collect();

        Ok(books)
    }

    /// Load a complete Bible
    pub fn load(&self) -> Result<Bible> {
        let translation = self.load_translation()?;
        let books = self.load_books()?;

        Ok(Bible {
            translation,
            books,
        })
    }

    /// Load a specific chapter
    pub fn load_chapter(&self, book: &str, chapter: u32) -> Result<Chapter> {
        let mut stmt = self.conn.prepare(
            "SELECT verse, text FROM verses WHERE book = ?1 AND chapter = ?2 ORDER BY verse"
        )?;

        let verses = stmt.query_map(params![book, chapter], |row| {
            let verse_num: u32 = row.get(0)?;
            let text: String = row.get(1)?;

            Ok(Verse {
                reference: VerseReference::new(book, chapter, verse_num),
                text,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(Chapter {
            book: book.to_string(),
            chapter_number: chapter,
            verses,
        })
    }

    /// Load a specific verse
    pub fn load_verse(&self, reference: &VerseReference) -> Result<Option<Verse>> {
        let mut stmt = self.conn.prepare(
            "SELECT text FROM verses WHERE book = ?1 AND chapter = ?2 AND verse = ?3"
        )?;

        let result = stmt.query_row(
            params![&reference.book, reference.chapter, reference.verse],
            |row| {
                let text: String = row.get(0)?;
                Ok(Verse {
                    reference: reference.clone(),
                    text,
                })
            }
        );

        match result {
            Ok(verse) => Ok(Some(verse)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Search for verses containing a query string
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<Verse>> {
        let search_query = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT book, chapter, verse, text FROM verses
             WHERE text LIKE ?1
             ORDER BY book, chapter, verse
             LIMIT ?2"
        )?;

        let verses = stmt.query_map(params![search_query, limit as i64], |row| {
            let book: String = row.get(0)?;
            let chapter: u32 = row.get(1)?;
            let verse: u32 = row.get(2)?;
            let text: String = row.get(3)?;

            Ok(Verse {
                reference: VerseReference::new(book, chapter, verse),
                text,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(verses)
    }
}

/// Initialize a new Bible database with the correct schema
pub fn init_database(db_path: impl AsRef<Path>) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS translations (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            abbreviation TEXT NOT NULL,
            language TEXT NOT NULL,
            description TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS verses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            book TEXT NOT NULL,
            chapter INTEGER NOT NULL,
            verse INTEGER NOT NULL,
            text TEXT NOT NULL,
            UNIQUE(book, chapter, verse)
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_verses_book_chapter
         ON verses(book, chapter)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_verses_search
         ON verses(text)",
        [],
    )?;

    Ok(())
}
