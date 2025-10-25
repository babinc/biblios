use super::Verse;

/// Search engine for Bible verses using fuzzy matching
pub struct SearchEngine {
    // Placeholder for future nucleo integration
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {}
    }

    /// Perform fuzzy search on verses
    pub fn search<'a>(&self, verses: &'a [Verse], query: &str) -> Vec<&'a Verse> {
        let query_lower = query.to_lowercase();

        verses
            .iter()
            .filter(|verse| {
                verse.text.to_lowercase().contains(&query_lower) ||
                verse.reference.to_string().to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Search with ranking (more relevant results first)
    pub fn ranked_search<'a>(&self, verses: &'a [Verse], query: &str) -> Vec<(f64, &'a Verse)> {
        let query_lower = query.to_lowercase();

        let mut results: Vec<(f64, &Verse)> = verses
            .iter()
            .filter_map(|verse| {
                let text_lower = verse.text.to_lowercase();
                let ref_lower = verse.reference.to_string().to_lowercase();

                // Simple scoring: exact word match > partial match
                let score = if text_lower.split_whitespace().any(|w| w == query_lower) {
                    1.0
                } else if text_lower.contains(&query_lower) {
                    0.5
                } else if ref_lower.contains(&query_lower) {
                    0.3
                } else {
                    return None;
                };

                Some((score, verse))
            })
            .collect();

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        results
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}
