//! Helper Functions for Material Design 3 Navigation Components
//!
//! This module provides convenient helper functions and utilities for creating
//! and working with Material Design 3 navigation components in audiobook applications.

use super::{MaterialBreadcrumbs, MaterialTabBar};
use crate::styling::material::MaterialTokens;

/// Helper functions for creating Material navigation components
impl MaterialTokens {
    /// Create a Material tab bar with the current tokens
    ///
    /// # Examples
    /// ```
    /// use abop_gui::styling::MaterialTokens;
    /// use abop_gui::styling::material::Tab;
    ///
    /// let tokens = MaterialTokens::default();
    /// let tab_bar = tokens.tab_bar()
    ///     .tab(Tab::new("Library").icon("📚"))
    ///     .tab(Tab::new("Playing").icon("▶️"))
    ///     .tab(Tab::new("Settings").icon("⚙️"))
    ///     .selected(0);
    /// ```
    #[must_use]
    pub fn tab_bar(&self) -> MaterialTabBar {
        MaterialTabBar::new()
    }

    /// Create Material breadcrumbs with the current tokens
    ///
    /// # Examples
    /// ```
    /// use abop_gui::styling::MaterialTokens;
    /// use abop_gui::styling::material::BreadcrumbItem;
    ///
    /// let tokens = MaterialTokens::default();
    /// let breadcrumbs = tokens.breadcrumbs()
    ///     .item(BreadcrumbItem::new("Library").icon("📚"))
    ///     .item(BreadcrumbItem::new("Fiction"))
    ///     .item(BreadcrumbItem::new("Mystery").clickable(false))
    ///     .separator(" > ");
    /// ```
    #[must_use]
    pub fn breadcrumbs(&self) -> MaterialBreadcrumbs {
        MaterialBreadcrumbs::new()
    }
}

/// Common navigation patterns for audiobook applications
pub mod audiobook_patterns {
    use super::super::{BreadcrumbItem, Tab};

    /// Create standard audiobook app tabs
    #[must_use]
    pub fn create_audiobook_tabs() -> Vec<Tab> {
        vec![
            Tab::new("Library").icon("📚"),
            Tab::new("Currently Playing").icon("▶️"),
            Tab::new("Queue").icon("📋"),
            Tab::new("Settings").icon("⚙️"),
        ]
    }

    /// Create breadcrumbs for audiobook folder navigation
    #[must_use]
    pub fn create_folder_breadcrumbs(path_components: &[&str]) -> Vec<BreadcrumbItem> {
        let mut items = Vec::new();

        // Add home/root
        items.push(BreadcrumbItem::new("Library").icon("📚"));

        // Add path components
        for (index, component) in path_components.iter().enumerate() {
            let is_last = index == path_components.len() - 1;
            items.push(
                BreadcrumbItem::new(*component).clickable(!is_last), // Last item is current location, not clickable
            );
        }

        items
    }

    /// Create breadcrumbs for audiobook series navigation
    #[must_use]
    pub fn create_series_breadcrumbs(
        author: &str,
        series: Option<&str>,
        book: Option<&str>,
    ) -> Vec<BreadcrumbItem> {
        let mut items = Vec::new();

        // Add library root
        items.push(BreadcrumbItem::new("Library").icon("📚"));

        // Add author
        items.push(BreadcrumbItem::new(author).icon("👤"));

        // Add series if present
        if let Some(series_name) = series {
            items.push(BreadcrumbItem::new(series_name).icon("📖"));
        }

        // Add book if present (current location, not clickable)
        if let Some(book_name) = book {
            items.push(BreadcrumbItem::new(book_name).icon("🎧").clickable(false));
        }

        items
    }
}
