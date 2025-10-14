//! Search bar component for audiobook library
//!
//! This module provides a search bar component with real-time filtering
//! and Material Design 3 styling.

use iced::widget::{container, row, text, text_input};
use iced::{Element, Length, Padding};
use std::sync::Arc;

use crate::messages::Message;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::inputs::MaterialSearchField;

/// Search bar component for filtering audiobooks
#[derive(Debug, Clone)]
pub struct SearchBar {
    /// Current search query
    query: String,
    /// Whether the search bar is focused
    focused: bool,
    /// Placeholder text
    placeholder: String,
}

impl SearchBar {
    /// Create a new search bar
    pub fn new() -> Self {
        Self {
            query: String::new(),
            focused: false,
            placeholder: "Search audiobooks...".to_string(),
        }
    }

    /// Create a new search bar with custom placeholder
    pub fn with_placeholder(placeholder: impl Into<String>) -> Self {
        Self {
            query: String::new(),
            focused: false,
            placeholder: placeholder.into(),
        }
    }

    /// Update the search query
    pub fn update_query(&mut self, query: String) {
        self.query = query;
    }

    /// Get the current search query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Set the focused state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Check if the search bar is focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Clear the search query
    pub fn clear(&mut self) {
        self.query.clear();
    }

    /// Check if the search bar has a query
    pub fn has_query(&self) -> bool {
        !self.query.trim().is_empty()
    }

    /// Create the view for the search bar
    pub fn view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        let search_field = MaterialSearchField::new()
            .full_width()
            .view(&self.query, |query| Message::SearchQuery(query));

        let search_container = container(search_field)
            .width(Length::Fill)
            .padding(Padding::from([8.0, 16.0]));

        // Add search icon and clear button if needed
        let mut search_row = row![search_container];

        if self.has_query() {
            // Add clear button
            let clear_button = text("✕")
                .size(16.0)
                .color(tokens.colors.on_surface_variant);
            
            let clear_container = container(clear_button)
                .padding(Padding::from([8.0, 8.0]))
                .on_press(Message::SearchQuery(String::new()));

            search_row = search_row.push(clear_container);
        }

        container(search_row)
            .width(Length::Fill)
            .style(|_theme| {
                crate::styling::material::MaterialSurface::new()
                    .variant(crate::styling::material::SurfaceVariant::SurfaceContainer)
                    .style(tokens)
            })
            .into()
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Search bar with advanced filters
#[derive(Debug, Clone)]
pub struct AdvancedSearchBar {
    /// Basic search bar
    search_bar: SearchBar,
    /// Whether advanced filters are visible
    filters_visible: bool,
    /// Current author filter
    author_filter: String,
    /// Current narrator filter
    narrator_filter: String,
    /// Minimum duration filter (in seconds)
    min_duration: Option<u64>,
    /// Maximum duration filter (in seconds)
    max_duration: Option<u64>,
}

impl AdvancedSearchBar {
    /// Create a new advanced search bar
    pub fn new() -> Self {
        Self {
            search_bar: SearchBar::new(),
            filters_visible: false,
            author_filter: String::new(),
            narrator_filter: String::new(),
            min_duration: None,
            max_duration: None,
        }
    }

    /// Toggle the visibility of advanced filters
    pub fn toggle_filters(&mut self) {
        self.filters_visible = !self.filters_visible;
    }

    /// Update the search query
    pub fn update_query(&mut self, query: String) {
        self.search_bar.update_query(query);
    }

    /// Update the author filter
    pub fn update_author_filter(&mut self, author: String) {
        self.author_filter = author;
    }

    /// Update the narrator filter
    pub fn update_narrator_filter(&mut self, narrator: String) {
        self.narrator_filter = narrator;
    }

    /// Update duration filters
    pub fn update_duration_filters(&mut self, min: Option<u64>, max: Option<u64>) {
        self.min_duration = min;
        self.max_duration = max;
    }

    /// Get the current search query
    pub fn query(&self) -> &str {
        self.search_bar.query()
    }

    /// Get the current author filter
    pub fn author_filter(&self) -> &str {
        &self.author_filter
    }

    /// Get the current narrator filter
    pub fn narrator_filter(&self) -> &str {
        &self.narrator_filter
    }

    /// Get duration filters
    pub fn duration_filters(&self) -> (Option<u64>, Option<u64>) {
        (self.min_duration, self.max_duration)
    }

    /// Check if any filters are active
    pub fn has_filters(&self) -> bool {
        !self.author_filter.trim().is_empty()
            || !self.narrator_filter.trim().is_empty()
            || self.min_duration.is_some()
            || self.max_duration.is_some()
    }

    /// Clear all filters
    pub fn clear_filters(&mut self) {
        self.author_filter.clear();
        self.narrator_filter.clear();
        self.min_duration = None;
        self.max_duration = None;
    }

    /// Clear all search and filters
    pub fn clear_all(&mut self) {
        self.search_bar.clear();
        self.clear_filters();
    }

    /// Create the view for the advanced search bar
    pub fn view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        let mut search_content = column![self.search_bar.view(tokens)];

        if self.filters_visible {
            // Add filter controls
            let filters = self.create_filter_controls(tokens);
            search_content = search_content.push(filters);
        }

        container(search_content)
            .width(Length::Fill)
            .style(|_theme| {
                crate::styling::material::MaterialSurface::new()
                    .variant(crate::styling::material::SurfaceVariant::SurfaceContainer)
                    .style(tokens)
            })
            .into()
    }

    /// Create filter controls
    fn create_filter_controls<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        // For now, just show a simple text indicating filters are available
        // In a full implementation, this would include input fields for filters
        let filter_text = if self.has_filters() {
            "Filters active"
        } else {
            "No filters"
        };

        container(
            text(filter_text)
                .size(tokens.typography().body_small.size)
                .color(tokens.colors.on_surface_variant),
        )
        .padding(Padding::from([4.0, 16.0]))
        .into()
    }
}

impl Default for AdvancedSearchBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_bar_creation() {
        let search_bar = SearchBar::new();
        assert!(search_bar.query().is_empty());
        assert!(!search_bar.is_focused());
        assert!(!search_bar.has_query());
    }

    #[test]
    fn test_search_bar_with_query() {
        let mut search_bar = SearchBar::new();
        search_bar.update_query("test query".to_string());
        
        assert_eq!(search_bar.query(), "test query");
        assert!(search_bar.has_query());
    }

    #[test]
    fn test_search_bar_clear() {
        let mut search_bar = SearchBar::new();
        search_bar.update_query("test query".to_string());
        search_bar.clear();
        
        assert!(search_bar.query().is_empty());
        assert!(!search_bar.has_query());
    }

    #[test]
    fn test_advanced_search_bar_creation() {
        let advanced_search = AdvancedSearchBar::new();
        assert!(advanced_search.query().is_empty());
        assert!(!advanced_search.has_filters());
    }

    #[test]
    fn test_advanced_search_bar_filters() {
        let mut advanced_search = AdvancedSearchBar::new();
        advanced_search.update_author_filter("Test Author".to_string());
        advanced_search.update_duration_filters(Some(3600), Some(7200));
        
        assert!(advanced_search.has_filters());
        assert_eq!(advanced_search.author_filter(), "Test Author");
        assert_eq!(advanced_search.duration_filters(), (Some(3600), Some(7200)));
    }
}
