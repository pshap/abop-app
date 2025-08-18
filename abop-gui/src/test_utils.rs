//! Test utilities for GUI components
//!
//! This module provides shared test fixtures and utilities to reduce code duplication
//! across component tests.

use abop_core::models::audiobook::Audiobook;
use std::path::{Path, PathBuf};

/// Default test path prefix for audiobook files
const TEST_AUDIOBOOK_PATH_PREFIX: &str = "/test/path";

/// Default test library ID used across GUI tests
const TEST_LIBRARY_ID: &str = "test-library-id";

/// Creates a test audiobook with default values and the specified ID and title.
///
/// # Arguments
/// * `id` - The ID to assign to the audiobook
/// * `title` - The title to assign to the audiobook
///
/// # Returns
/// A fully configured `Audiobook` instance suitable for testing
pub fn create_test_audiobook(id: &str, title: &str) -> Audiobook {
    let path = PathBuf::from(format!("{TEST_AUDIOBOOK_PATH_PREFIX}/{title}.mp3"));
    let mut audiobook = Audiobook::new(TEST_LIBRARY_ID, &path);
    audiobook.id = id.to_string();
    audiobook.title = Some(title.to_string());
    audiobook.author = Some("Test Author".to_string());
    audiobook.duration_seconds = Some(3600);
    audiobook.size_bytes = Some(1024000);
    audiobook
}

/// Centralized factory for creating test data objects used across GUI tests
///
/// This factory provides simplified audiobook creation for GUI component testing.
/// Unlike the core TestDataFactory, this version:
/// - Uses concrete strings for `title` and `author` (no Option types)
/// - Automatically generates paths based on title for consistency
/// - Uses a fixed "test-library-id" for simplified UI testing
/// - Focuses on GUI component testing scenarios
///
/// Use this factory when:
/// - Testing UI components that display audiobook information
/// - Creating test data for view rendering and interaction tests
/// - Building consistent test datasets for GUI component validation
pub struct TestDataFactory;

impl TestDataFactory {
    /// Create an Audiobook with customizable metadata, including duration and size
    pub fn custom_audiobook(
        id: &str,
        title: &str,
        author: &str,
        duration_seconds: Option<u64>,
        size_bytes: Option<u64>,
    ) -> Audiobook {
        let path = PathBuf::from(format!("{TEST_AUDIOBOOK_PATH_PREFIX}/{title}.mp3"));
        let mut audiobook = Audiobook::new(TEST_LIBRARY_ID, &path);
        audiobook.id = id.to_string();
        audiobook.title = Some(title.to_string());
        audiobook.author = Some(author.to_string());
        audiobook.duration_seconds = duration_seconds;
        audiobook.size_bytes = size_bytes;
        audiobook
    }

    /// Create an Audiobook using an explicit path along with id/title/author
    pub fn audiobook_with_path<P: AsRef<Path>>(
        id: &str,
        title: &str,
        author: &str,
        path: P,
    ) -> Audiobook {
        let mut audiobook = Audiobook::new(TEST_LIBRARY_ID, path);
        audiobook.id = id.to_string();
        audiobook.title = Some(title.to_string());
        audiobook.author = Some(author.to_string());
        audiobook
    }
}
