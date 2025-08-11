//! Test utilities for GUI components
//!
//! This module provides shared test fixtures and utilities to reduce code duplication
//! across component tests.

use abop_core::models::audiobook::Audiobook;
use std::path::PathBuf;

/// Creates a test audiobook with default values and the specified ID and title.
///
/// # Arguments
/// * `id` - The ID to assign to the audiobook
/// * `title` - The title to assign to the audiobook
///
/// # Returns
/// A fully configured `Audiobook` instance suitable for testing
pub fn create_test_audiobook(id: &str, title: &str) -> Audiobook {
    let path = PathBuf::from(format!("/test/path/{title}.mp3"));
    let mut audiobook = Audiobook::new("test-library-id", &path);
    audiobook.id = id.to_string();
    audiobook.title = Some(title.to_string());
    audiobook.author = Some("Test Author".to_string());
    audiobook.duration_seconds = Some(3600);
    audiobook.size_bytes = Some(1024000);
    audiobook
}

/// Creates a test audiobook with customizable metadata.
///
/// # Arguments
/// * `id` - The ID to assign to the audiobook
/// * `title` - The title to assign to the audiobook
/// * `author` - The author to assign to the audiobook
/// * `duration` - Duration in seconds
///
/// # Returns
/// A configured `Audiobook` instance with the specified metadata
pub fn create_custom_test_audiobook(
    id: &str,
    title: &str,
    author: &str,
    duration: Option<u64>,
) -> Audiobook {
    let path = PathBuf::from(format!("/test/path/{title}.mp3"));
    let mut audiobook = Audiobook::new("test-library-id", &path);
    audiobook.id = id.to_string();
    audiobook.title = Some(title.to_string());
    audiobook.author = Some(author.to_string());
    audiobook.duration_seconds = duration;
    audiobook.size_bytes = Some(1024000);
    audiobook
}
