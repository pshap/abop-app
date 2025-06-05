//! Search-related models and utilities

use crate::models::audiobook::Audiobook;
use serde::{Deserialize, Serialize};

/// Represents a search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// The audiobook that matched the search
    pub audiobook: Audiobook,
    /// Relevance score (higher is more relevant)
    pub score: f32,
    /// Snippets of matching text with highlights
    pub highlights: Vec<String>,
}

impl SearchResult {
    /// Creates a new search result
    #[must_use]
    pub const fn new(audiobook: Audiobook, score: f32) -> Self {
        Self {
            audiobook,
            score,
            highlights: Vec::new(),
        }
    }

    /// Creates a search result with highlights
    #[must_use]
    pub const fn with_highlights(
        audiobook: Audiobook,
        score: f32,
        highlights: Vec<String>,
    ) -> Self {
        Self {
            audiobook,
            score,
            highlights,
        }
    }

    /// Adds a highlight snippet to the search result
    pub fn add_highlight(&mut self, highlight: String) {
        self.highlights.push(highlight);
    }

    /// Gets the best highlight snippet (first one if available)
    #[must_use]
    pub fn best_highlight(&self) -> Option<&String> {
        self.highlights.first()
    }

    /// Gets a formatted summary of the search result
    #[must_use]
    pub fn summary(&self) -> String {
        let title = self.audiobook.display_title();
        let author = self.audiobook.display_author();

        self.best_highlight().map_or_else(
            || format!("{title} by {author}"),
            |highlight| format!("{title} by {author} - {highlight}"),
        )
    }

    /// Returns true if this is a high-relevance result (score > 0.8)
    #[must_use]
    pub fn is_high_relevance(&self) -> bool {
        self.score > 0.8
    }

    /// Returns a relevance category string
    #[must_use]
    pub fn relevance_category(&self) -> &'static str {
        if self.score > 0.9 {
            "Excellent Match"
        } else if self.score > 0.7 {
            "Good Match"
        } else if self.score > 0.5 {
            "Fair Match"
        } else {
            "Weak Match"
        }
    }
}

/// Search query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The search term or phrase
    pub query: String,
    /// Filter by specific library (optional)
    pub library_id: Option<String>,
    /// Filter by author (optional)
    pub author: Option<String>,
    /// Filter by narrator (optional)
    pub narrator: Option<String>,
    /// Minimum duration in seconds (optional)
    pub min_duration: Option<u64>,
    /// Maximum duration in seconds (optional)
    pub max_duration: Option<u64>,
    /// Include completed audiobooks in results
    pub include_completed: bool,
    /// Maximum number of results to return
    pub limit: Option<usize>,
}

impl SearchQuery {
    /// Creates a simple search query with just a search term
    #[must_use]
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            library_id: None,
            author: None,
            narrator: None,
            min_duration: None,
            max_duration: None,
            include_completed: true,
            limit: None,
        }
    }

    /// Sets the library filter
    #[must_use]
    pub fn in_library(mut self, library_id: &str) -> Self {
        self.library_id = Some(library_id.to_string());
        self
    }

    /// Sets the author filter
    #[must_use]
    pub fn by_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    /// Sets the narrator filter
    #[must_use]
    pub fn by_narrator(mut self, narrator: &str) -> Self {
        self.narrator = Some(narrator.to_string());
        self
    }

    /// Sets duration range filters
    #[must_use]
    pub const fn duration_range(
        mut self,
        min_seconds: Option<u64>,
        max_seconds: Option<u64>,
    ) -> Self {
        self.min_duration = min_seconds;
        self.max_duration = max_seconds;
        self
    }

    /// Sets whether to include completed audiobooks
    #[must_use]
    pub const fn include_completed(mut self, include: bool) -> Self {
        self.include_completed = include;
        self
    }

    /// Sets the maximum number of results
    #[must_use]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Returns true if any filters are applied
    #[must_use]
    pub const fn has_filters(&self) -> bool {
        self.library_id.is_some()
            || self.author.is_some()
            || self.narrator.is_some()
            || self.min_duration.is_some()
            || self.max_duration.is_some()
            || !self.include_completed
    }

    /// Gets a human-readable description of the active filters
    #[must_use]
    pub fn filter_description(&self) -> String {
        let mut filters = Vec::new();

        if let Some(library_id) = &self.library_id {
            filters.push(format!("library: {library_id}"));
        }
        if let Some(author) = &self.author {
            filters.push(format!("author: {author}"));
        }
        if let Some(narrator) = &self.narrator {
            filters.push(format!("narrator: {narrator}"));
        }
        if let Some(min) = self.min_duration {
            let hours = min / 3600;
            if hours > 0 {
                filters.push(format!("min duration: {hours}h"));
            } else {
                filters.push(format!("min duration: {}min", min / 60));
            }
        }
        if let Some(max) = self.max_duration {
            let hours = max / 3600;
            if hours > 0 {
                filters.push(format!("max duration: {hours}h"));
            } else {
                filters.push(format!("max duration: {}min", max / 60));
            }
        }
        if !self.include_completed {
            filters.push("incomplete only".to_string());
        }

        if filters.is_empty() {
            "No filters".to_string()
        } else {
            filters.join(", ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::audiobook::*;
    use chrono::Utc;
    use std::path::Path;

    fn create_test_audiobook() -> Audiobook {
        let now = Utc::now();
        Audiobook {
            id: "test-book".to_string(),
            library_id: "test-library".to_string(),
            path: Path::new("/test/book.mp3").to_path_buf(),
            title: Some(TEST_TITLE.to_string()),
            author: Some(TEST_AUTHOR.to_string()),
            narrator: Some(TEST_NARRATOR.to_string()),
            description: None,
            duration_seconds: Some(3600),
            size_bytes: None,
            cover_art: None,
            created_at: now,
            updated_at: now,
            selected: false,
        }
    }

    #[test]
    fn test_search_result_creation() {
        let audiobook = create_test_audiobook();
        let result = SearchResult::new(audiobook.clone(), 0.85);

        assert_eq!(result.audiobook.id, "test-book");
        assert_eq!(result.score, 0.85);
        assert!(result.highlights.is_empty());
        assert!(result.is_high_relevance());
    }

    #[test]
    fn test_search_result_with_highlights() {
        let audiobook = create_test_audiobook();
        let highlights = vec!["Chapter 1".to_string(), "Important scene".to_string()];
        let result = SearchResult::with_highlights(audiobook, 0.75, highlights);

        assert_eq!(result.highlights.len(), 2);
        assert_eq!(result.best_highlight(), Some(&"Chapter 1".to_string()));
    }

    #[test]
    fn test_relevance_categories() {
        let audiobook = create_test_audiobook();

        let excellent = SearchResult::new(audiobook.clone(), 0.95);
        assert_eq!(excellent.relevance_category(), "Excellent Match");

        let good = SearchResult::new(audiobook.clone(), 0.8);
        assert_eq!(good.relevance_category(), "Good Match");

        let fair = SearchResult::new(audiobook.clone(), 0.6);
        assert_eq!(fair.relevance_category(), "Fair Match");

        let weak = SearchResult::new(audiobook, 0.3);
        assert_eq!(weak.relevance_category(), "Weak Match");
    }

    #[test]
    fn test_search_query_creation() {
        let query = SearchQuery::new("fantasy adventure");
        assert_eq!(query.query, "fantasy adventure");
        assert!(query.library_id.is_none());
        assert!(query.include_completed);
        assert!(!query.has_filters());
    }

    #[test]
    fn test_search_query_builder() {
        let query = SearchQuery::new("fantasy")
            .in_library("lib-123")
            .by_author("Tolkien")
            .duration_range(Some(3600), Some(14400))
            .include_completed(false)
            .limit(10);

        assert_eq!(query.library_id, Some("lib-123".to_string()));
        assert_eq!(query.author, Some("Tolkien".to_string()));
        assert_eq!(query.min_duration, Some(3600));
        assert_eq!(query.max_duration, Some(14400));
        assert!(!query.include_completed);
        assert_eq!(query.limit, Some(10));
        assert!(query.has_filters());
    }

    #[test]
    fn test_filter_description() {
        let simple_query = SearchQuery::new("test");
        assert_eq!(simple_query.filter_description(), "No filters");

        let complex_query = SearchQuery::new("test")
            .by_author(TEST_AUTHOR)
            .duration_range(Some(7200), None)
            .include_completed(false);

        let description = complex_query.filter_description();
        assert!(description.contains(&format!("author: {TEST_AUTHOR}")));
        assert!(description.contains("min duration: 2h"));
        assert!(description.contains("incomplete only"));
    }
}
