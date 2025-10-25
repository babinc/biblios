use anyhow::{Context, Result};
use rusqlite::Connection;
use serde_json::Value;
use std::path::Path;

/// Parse JSON Bible format and import into database
/// Expected format: { "translation": {...}, "books": [...] }
pub fn import_json_bible(json_path: impl AsRef<Path>, db_path: impl AsRef<Path>) -> Result<()> {
    let json_content = std::fs::read_to_string(&json_path)
        .context("Failed to read JSON Bible file")?;

    let data: Value = serde_json::from_str(&json_content)
        .context("Failed to parse JSON")?;

    let conn = Connection::open(db_path)?;

    // Import translation metadata
    if let Some(translation) = data.get("translation") {
        conn.execute(
            "INSERT OR REPLACE INTO translations (id, name, abbreviation, language, description)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                translation["id"].as_str().unwrap_or("unknown"),
                translation["name"].as_str().unwrap_or("Unknown"),
                translation["abbreviation"].as_str().unwrap_or("UNK"),
                translation["language"].as_str().unwrap_or("en"),
                translation["description"].as_str().unwrap_or(""),
            ],
        )?;
    }

    // Import verses
    if let Some(books) = data.get("books").and_then(|b| b.as_array()) {
        for book in books {
            let book_name = book["name"].as_str().context("Missing book name")?;

            if let Some(chapters) = book.get("chapters").and_then(|c| c.as_array()) {
                for (chapter_idx, chapter) in chapters.iter().enumerate() {
                    let chapter_num = (chapter_idx + 1) as u32;

                    if let Some(verses) = chapter.as_array() {
                        for (verse_idx, verse_text) in verses.iter().enumerate() {
                            let verse_num = (verse_idx + 1) as u32;
                            let text = verse_text.as_str().unwrap_or("");

                            conn.execute(
                                "INSERT OR REPLACE INTO verses (book, chapter, verse, text)
                                 VALUES (?1, ?2, ?3, ?4)",
                                rusqlite::params![book_name, chapter_num, verse_num, text],
                            )?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Parse reference string like "John 3:16" or "Genesis 1:1-3"
pub fn parse_reference(input: &str) -> Option<(String, u32, u32)> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    // Book name might be multiple words (e.g., "1 John")
    let chapter_verse_idx = parts.iter().position(|p| p.contains(':'))?;
    let book = parts[..chapter_verse_idx].join(" ");

    let chapter_verse = parts[chapter_verse_idx];
    let cv_parts: Vec<&str> = chapter_verse.split(':').collect();

    if cv_parts.len() != 2 {
        return None;
    }

    let chapter = cv_parts[0].parse::<u32>().ok()?;

    // Handle ranges like "1-3", just take the first verse for now
    let verse_str = cv_parts[1].split('-').next()?;
    let verse = verse_str.parse::<u32>().ok()?;

    Some((book, chapter, verse))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reference() {
        assert_eq!(
            parse_reference("John 3:16"),
            Some(("John".to_string(), 3, 16))
        );

        assert_eq!(
            parse_reference("1 John 2:5"),
            Some(("1 John".to_string(), 2, 5))
        );

        assert_eq!(
            parse_reference("Genesis 1:1"),
            Some(("Genesis".to_string(), 1, 1))
        );
    }
}
