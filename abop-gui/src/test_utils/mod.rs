//! Test utilities for ABOP GUI
//!
//! This module provides centralized test utilities and helpers for testing
//! GUI components, eliminating code duplication and providing consistent patterns.

pub mod components;

// Re-export commonly used utilities
pub use components::*;

// Legacy compatibility functions for existing tests
// TODO: These should be removed when tests are migrated to use the new utilities

/// Legacy function for backward compatibility
pub fn create_test_audiobook(id: &str, title: &str) -> abop_core::models::audiobook::Audiobook {
    TestDataFactory::audiobook(id, title)
}

/// Legacy function for backward compatibility
pub fn create_custom_test_audiobook(
    id: &str,
    title: &str,
    author: &str,
    duration: Option<u64>,
) -> abop_core::models::audiobook::Audiobook {
    TestDataFactory::custom_audiobook(id, title, author, duration, Some(1024000))
}
