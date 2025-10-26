pub mod loader;
pub mod parser;
pub mod search;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a Bible verse reference
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VerseReference {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
}

impl VerseReference {
    pub fn new(book: impl Into<String>, chapter: u32, verse: u32) -> Self {
        Self {
            book: book.into(),
            chapter,
            verse,
        }
    }
}

impl fmt::Display for VerseReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}:{}", self.book, self.chapter, self.verse)
    }
}

/// Represents a single verse with its content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verse {
    pub reference: VerseReference,
    pub text: String,
}

/// Represents a chapter containing multiple verses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub book: String,
    pub chapter_number: u32,
    pub verses: Vec<Verse>,
}

/// Represents a book of the Bible
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub full_name: String,
    pub testament: Testament,
    pub chapter_count: u32,
}

/// Testament classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Testament {
    Old,
    New,
}

/// Represents a Bible translation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub id: String,
    pub name: String,
    pub abbreviation: String,
    pub language: String,
    pub description: String,
}

/// Main Bible structure holding all data
pub struct Bible {
    pub translation: Translation,
    pub books: Vec<Book>,
}

impl Bible {
    /// Get a specific verse
    pub fn get_verse(&self, _reference: &VerseReference) -> Option<Verse> {
        // This will be implemented when we add the database layer
        None
    }

    /// Get all verses in a chapter
    pub fn get_chapter(&self, book: &str, chapter: u32) -> Option<Chapter> {
        // This will be implemented when we add the database layer
        None
    }

    /// Get list of all books
    pub fn get_books(&self) -> &[Book] {
        &self.books
    }

    /// Get a book by name
    pub fn get_book(&self, name: &str) -> Option<&Book> {
        self.books.iter().find(|b| b.name == name || b.full_name == name)
    }
}

/// Get the book ID for Bible SuperSearch databases (1-66)
pub fn book_name_to_id(name: &str) -> Option<u32> {
    BOOK_ORDER.iter()
        .position(|(short_name, _, _, _)| *short_name == name)
        .map(|idx| (idx + 1) as u32)
}

/// Get the book name from ID (1-66)
pub fn book_id_to_name(id: u32) -> Option<&'static str> {
    if id < 1 || id as usize > BOOK_ORDER.len() {
        return None;
    }
    Some(BOOK_ORDER[(id - 1) as usize].0)
}

/// Get the number of chapters in a book
pub fn get_chapter_count(book_name: &str) -> u32 {
    BOOK_ORDER
        .iter()
        .find(|(short, full, _, _)| *short == book_name || *full == book_name)
        .map(|(_, _, _, count)| *count)
        .unwrap_or(0)
}

/// Standard book order for easy reference
pub const BOOK_ORDER: &[(&str, &str, Testament, u32)] = &[
    // Old Testament
    ("Gen", "Genesis", Testament::Old, 50),
    ("Exod", "Exodus", Testament::Old, 40),
    ("Lev", "Leviticus", Testament::Old, 27),
    ("Num", "Numbers", Testament::Old, 36),
    ("Deut", "Deuteronomy", Testament::Old, 34),
    ("Josh", "Joshua", Testament::Old, 24),
    ("Judg", "Judges", Testament::Old, 21),
    ("Ruth", "Ruth", Testament::Old, 4),
    ("1Sam", "1 Samuel", Testament::Old, 31),
    ("2Sam", "2 Samuel", Testament::Old, 24),
    ("1Kgs", "1 Kings", Testament::Old, 22),
    ("2Kgs", "2 Kings", Testament::Old, 25),
    ("1Chr", "1 Chronicles", Testament::Old, 29),
    ("2Chr", "2 Chronicles", Testament::Old, 36),
    ("Ezra", "Ezra", Testament::Old, 10),
    ("Neh", "Nehemiah", Testament::Old, 13),
    ("Esth", "Esther", Testament::Old, 10),
    ("Job", "Job", Testament::Old, 42),
    ("Ps", "Psalms", Testament::Old, 150),
    ("Prov", "Proverbs", Testament::Old, 31),
    ("Eccl", "Ecclesiastes", Testament::Old, 12),
    ("Song", "Song of Solomon", Testament::Old, 8),
    ("Isa", "Isaiah", Testament::Old, 66),
    ("Jer", "Jeremiah", Testament::Old, 52),
    ("Lam", "Lamentations", Testament::Old, 5),
    ("Ezek", "Ezekiel", Testament::Old, 48),
    ("Dan", "Daniel", Testament::Old, 12),
    ("Hos", "Hosea", Testament::Old, 14),
    ("Joel", "Joel", Testament::Old, 3),
    ("Amos", "Amos", Testament::Old, 9),
    ("Obad", "Obadiah", Testament::Old, 1),
    ("Jonah", "Jonah", Testament::Old, 4),
    ("Mic", "Micah", Testament::Old, 7),
    ("Nah", "Nahum", Testament::Old, 3),
    ("Hab", "Habakkuk", Testament::Old, 3),
    ("Zeph", "Zephaniah", Testament::Old, 3),
    ("Hag", "Haggai", Testament::Old, 2),
    ("Zech", "Zechariah", Testament::Old, 14),
    ("Mal", "Malachi", Testament::Old, 4),
    // New Testament
    ("Matt", "Matthew", Testament::New, 28),
    ("Mark", "Mark", Testament::New, 16),
    ("Luke", "Luke", Testament::New, 24),
    ("John", "John", Testament::New, 21),
    ("Acts", "Acts", Testament::New, 28),
    ("Rom", "Romans", Testament::New, 16),
    ("1Cor", "1 Corinthians", Testament::New, 16),
    ("2Cor", "2 Corinthians", Testament::New, 13),
    ("Gal", "Galatians", Testament::New, 6),
    ("Eph", "Ephesians", Testament::New, 6),
    ("Phil", "Philippians", Testament::New, 4),
    ("Col", "Colossians", Testament::New, 4),
    ("1Thess", "1 Thessalonians", Testament::New, 5),
    ("2Thess", "2 Thessalonians", Testament::New, 3),
    ("1Tim", "1 Timothy", Testament::New, 6),
    ("2Tim", "2 Timothy", Testament::New, 4),
    ("Titus", "Titus", Testament::New, 3),
    ("Phlm", "Philemon", Testament::New, 1),
    ("Heb", "Hebrews", Testament::New, 13),
    ("Jas", "James", Testament::New, 5),
    ("1Pet", "1 Peter", Testament::New, 5),
    ("2Pet", "2 Peter", Testament::New, 3),
    ("1John", "1 John", Testament::New, 5),
    ("2John", "2 John", Testament::New, 1),
    ("3John", "3 John", Testament::New, 1),
    ("Jude", "Jude", Testament::New, 1),
    ("Rev", "Revelation", Testament::New, 22),
];
