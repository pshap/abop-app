//! Search engine implementation for audiobooks
//!
//! This module provides a simple in-memory search engine with fuzzy matching
//! capabilities for searching through audiobook collections.

use std::collections::HashMap;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::models::{Audiobook, SearchQuery, SearchResult};

/// Search engine for audiobooks with fuzzy matching
pub struct SearchEngine {
    /// Fuzzy matcher instance
    matcher: SkimMatcherV2,
    /// Search index mapping audiobook IDs to searchable text
    index: HashMap<String, SearchableText>,
    /// Configuration for search behavior
    config: SearchConfig,
}

/// Searchable text extracted from an audiobook
#[derive(Debug, Clone)]
struct SearchableText {
    /// Combined searchable text (title + author + narrator)
    combined: String,
    /// Individual fields for targeted searching
    title: String,
    author: String,
    narrator: String,
    /// Duration in seconds for duration-based filtering
    duration_seconds: Option<u64>,
    /// Library ID for library-based filtering
    library_id: String,
}

/// Configuration for search behavior
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// Minimum score threshold for fuzzy matching (0.0 to 1.0)
    pub min_score: f64,
    /// Maximum number of results to return
    pub max_results: usize,
    /// Whether to use case-sensitive matching
    pub case_sensitive: bool,
    /// Whether to include partial matches
    pub include_partial: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            min_score: 0.3,
            max_results: 100,
            case_sensitive: false,
            include_partial: true,
        }
    }
}

impl SearchEngine {
    /// Create a new search engine with default configuration
    pub fn new() -> Self {
        Self::with_config(SearchConfig::default())
    }

    /// Create a new search engine with custom configuration
    pub fn with_config(config: SearchConfig) -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            index: HashMap::new(),
            config,
        }
    }

    /// Add an audiobook to the search index
    pub fn add_audiobook(&mut self, audiobook: &Audiobook) {
        let searchable_text = SearchableText {
            combined: self.create_combined_text(audiobook),
            title: audiobook.title.as_deref().unwrap_or("").to_string(),
            author: audiobook.author.as_deref().unwrap_or("").to_string(),
            narrator: audiobook.narrator.as_deref().unwrap_or("").to_string(),
            duration_seconds: audiobook.duration_seconds,
            library_id: audiobook.library_id.clone(),
        };

        self.index.insert(audiobook.id.clone(), searchable_text);
    }

    /// Remove an audiobook from the search index
    pub fn remove_audiobook(&mut self, audiobook_id: &str) {
        self.index.remove(audiobook_id);
    }

    /// Update an audiobook in the search index
    pub fn update_audiobook(&mut self, audiobook: &Audiobook) {
        self.add_audiobook(audiobook); // Same as add for HashMap
    }

    /// Search for audiobooks matching the given query
    pub fn search(&self, query: &SearchQuery, audiobooks: &[Audiobook]) -> Vec<SearchResult> {
        if query.query.trim().is_empty() {
            return self.apply_filters(query, audiobooks);
        }

        let mut results = Vec::new();

        for audiobook in audiobooks {
            // Check if audiobook is in our index
            let Some(searchable_text) = self.index.get(&audiobook.id) else {
                continue;
            };

            // Apply filters first
            if !self.matches_filters(query, searchable_text) {
                continue;
            }

            // Perform fuzzy search
            let score = self.calculate_search_score(&query.query, searchable_text);

            if score >= self.config.min_score {
                results.push(SearchResult::new(audiobook.clone(), score as f32));
            }
        }

        // Sort by score (highest first)
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        // Limit results
        results.truncate(self.config.max_results);
        results
    }

    /// Apply filters without text search
    fn apply_filters(&self, query: &SearchQuery, audiobooks: &[Audiobook]) -> Vec<SearchResult> {
        let mut results = Vec::new();

        for audiobook in audiobooks {
            let Some(searchable_text) = self.index.get(&audiobook.id) else {
                continue;
            };

            if self.matches_filters(query, searchable_text) {
                results.push(SearchResult::new(audiobook.clone(), 1.0));
            }
        }

        results
    }

    /// Check if an audiobook matches the search filters
    fn matches_filters(&self, query: &SearchQuery, searchable_text: &SearchableText) -> bool {
        // Library filter
        if let Some(ref library_id) = query.library_id {
            if searchable_text.library_id != *library_id {
                return false;
            }
        }

        // Author filter
        if let Some(ref author) = query.author {
            if !self.matches_text(&searchable_text.author, author) {
                return false;
            }
        }

        // Narrator filter
        if let Some(ref narrator) = query.narrator {
            if !self.matches_text(&searchable_text.narrator, narrator) {
                return false;
            }
        }

        // Duration filters
        if let Some(min_duration) = query.min_duration {
            if let Some(duration) = searchable_text.duration_seconds {
                if duration < min_duration {
                    return false;
                }
            }
        }

        if let Some(max_duration) = query.max_duration {
            if let Some(duration) = searchable_text.duration_seconds {
                if duration > max_duration {
                    return false;
                }
            }
        }

        // Completion filter
        if !query.include_completed {
            // For now, we don't have completion status in the search index
            // This would need to be added if we want to filter by completion
        }

        true
    }

    /// Check if text matches a filter (case-insensitive substring match)
    fn matches_text(&self, text: &str, filter: &str) -> bool {
        if self.config.case_sensitive {
            text.contains(filter)
        } else {
            text.to_lowercase().contains(&filter.to_lowercase())
        }
    }

    /// Calculate search score for a query against searchable text
    fn calculate_search_score(&self, query: &str, searchable_text: &SearchableText) -> f64 {
        let query_lower = query.to_lowercase();
        let mut best_score: f64 = 0.0;

        // Search in combined text (highest weight)
        if let Some(score) = self.matcher.fuzzy_match(&searchable_text.combined, &query_lower) {
            best_score = best_score.max(score as f64 / 100.0);
        }

        // Search in individual fields with different weights
        if let Some(score) = self.matcher.fuzzy_match(&searchable_text.title, &query_lower) {
            best_score = best_score.max(score as f64 / 100.0 * 0.9); // Slightly lower weight
        }

        if let Some(score) = self.matcher.fuzzy_match(&searchable_text.author, &query_lower) {
            best_score = best_score.max(score as f64 / 100.0 * 0.8);
        }

        if let Some(score) = self.matcher.fuzzy_match(&searchable_text.narrator, &query_lower) {
            best_score = best_score.max(score as f64 / 100.0 * 0.7);
        }

        best_score
    }

    /// Create combined searchable text from an audiobook
    fn create_combined_text(&self, audiobook: &Audiobook) -> String {
        let mut parts = Vec::new();

        if let Some(ref title) = audiobook.title {
            parts.push(title);
        }

        if let Some(ref author) = audiobook.author {
            parts.push(author);
        }

        if let Some(ref narrator) = audiobook.narrator {
            parts.push(narrator);
        }

        parts.into_iter().collect::<Vec<_>>().join(" ")
    }

    /// Get the number of indexed audiobooks
    pub fn index_size(&self) -> usize {
        self.index.len()
    }

    /// Clear the search index
    pub fn clear(&mut self) {
        self.index.clear();
    }

    /// Update search configuration
    pub fn update_config(&mut self, config: SearchConfig) {
        self.config = config;
    }

    /// Get current search configuration
    pub fn config(&self) -> &SearchConfig {
        &self.config
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SearchEngine {
    fn clone(&self) -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            index: self.index.clone(),
            config: self.config.clone(),
        }
    }
}

impl std::fmt::Debug for SearchEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchEngine")
            .field("index_size", &self.index.len())
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn create_test_audiobook(id: &str, title: &str, author: &str) -> Audiobook {
        Audiobook {
            id: id.to_string(),
            library_id: "test-library".to_string(),
            path: Path::new("/test/book.mp3").to_path_buf(),
            title: Some(title.to_string()),
            author: Some(author.to_string()),
            narrator: None,
            description: None,
            duration_seconds: Some(3600),
            size_bytes: Some(1024 * 1024),
            cover_art: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            selected: false,
        }
    }

    #[test]
    fn test_search_engine_creation() {
        let engine = SearchEngine::new();
        assert_eq!(engine.index_size(), 0);
    }

    #[test]
    fn test_add_audiobook() {
        let mut engine = SearchEngine::new();
        let audiobook = create_test_audiobook("1", "Test Book", "Test Author");
        
        engine.add_audiobook(&audiobook);
        assert_eq!(engine.index_size(), 1);
    }

    #[test]
    fn test_search_basic() {
        let mut engine = SearchEngine::new();
        let audiobook = create_test_audiobook("1", "The Great Gatsby", "F. Scott Fitzgerald");
        engine.add_audiobook(&audiobook);

        let query = SearchQuery::new("gatsby");
        let results = engine.search(&query, &[audiobook.clone()]);

        assert!(!results.is_empty());
        assert_eq!(results[0].audiobook.id, "1");
    }

    #[test]
    fn test_search_with_filters() {
        let mut engine = SearchEngine::new();
        let audiobook = create_test_audiobook("1", "Test Book", "Test Author");
        engine.add_audiobook(&audiobook);

        let query = SearchQuery::new("test").by_author("Test Author");
        let results = engine.search(&query, &[audiobook.clone()]);

        assert!(!results.is_empty());
    }

    #[test]
    fn test_remove_audiobook() {
        let mut engine = SearchEngine::new();
        let audiobook = create_test_audiobook("1", "Test Book", "Test Author");
        
        engine.add_audiobook(&audiobook);
        assert_eq!(engine.index_size(), 1);

        engine.remove_audiobook("1");
        assert_eq!(engine.index_size(), 0);
    }
}
