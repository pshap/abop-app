//! Test utilities for GUI components
//!
//! This module provides shared test fixtures and utilities to reduce code duplication
//! across component tests. All test utilities follow modern Rust testing practices.
//!
//! **Note**: This module is only compiled during testing to avoid shipping test-only APIs.

use abop_core::models::audiobook::Audiobook;
use std::path::PathBuf;

/// Creates a test audiobook with default realistic values.
///
/// # Arguments
/// * `id` - The ID to assign to the audiobook
/// * `title` - The title to assign to the audiobook
///
/// # Returns
/// A fully configured `Audiobook` instance suitable for testing
/// 
/// # Examples
/// ```rust,no_run
/// use abop_gui::test_utils::create_test_audiobook;
/// let audiobook = create_test_audiobook("test-1", "My Test Book");
/// assert_eq!(audiobook.id, "test-1");
/// ```
pub fn create_test_audiobook(id: &str, title: &str) -> Audiobook {
    let path = PathBuf::from(format!("/test/audiobooks/{}/{}.mp3", id, title));
    let mut audiobook = Audiobook::new("test-library-id", &path);
    audiobook.id = id.to_string();
    audiobook.title = Some(title.to_string());
    audiobook.author = Some("Test Author".to_string());
    audiobook.duration_seconds = Some(3600); // 1 hour
    audiobook.size_bytes = Some(52_428_800); // ~50MB
    audiobook
}

/// Creates a test audiobook with fully customizable metadata.
///
/// # Arguments
/// * `id` - The ID to assign to the audiobook
/// * `title` - The title to assign to the audiobook  
/// * `author` - The author to assign to the audiobook
/// * `duration` - Duration in seconds (None for missing metadata)
///
/// # Returns
/// A configured `Audiobook` instance with the specified metadata
pub fn create_custom_test_audiobook(
    id: &str,
    title: &str,
    author: &str,
    duration: Option<u64>,
) -> Audiobook {
    let path = PathBuf::from(format!("/test/custom/{}/{}.mp3", id, title));
    let mut audiobook = Audiobook::new("test-library-id", &path);
    audiobook.id = id.to_string();
    audiobook.title = Some(title.to_string());
    audiobook.author = Some(author.to_string());
    audiobook.duration_seconds = duration;
    audiobook.size_bytes = duration.map(|d| d.saturating_mul(12_000)); // Rough estimate based on duration
    audiobook
}

/// Creates a batch of test audiobooks for performance/bulk testing.
///
/// # Arguments
/// * `count` - Number of audiobooks to create
/// * `prefix` - Prefix for IDs and titles
///
/// # Returns
/// Vector of audiobooks with sequential IDs and titles
pub fn create_test_audiobook_batch(count: usize, prefix: &str) -> Vec<Audiobook> {
    (0..count)
        .map(|i| create_test_audiobook(&format!("{prefix}_{i:03}"), &format!("{prefix} Book {i:03}")))
        .collect()
}

/// Creates a test audiobook with minimal/missing metadata for edge case testing.
pub fn create_minimal_audiobook(id: &str) -> Audiobook {
    create_custom_test_audiobook(id, "", "", None)
}

/// Creates a test audiobook with extreme values for stress testing.
pub fn create_extreme_audiobook(id: &str) -> Audiobook {
    let mut audiobook = create_custom_test_audiobook(
        id,
        &"Very Long Title That Goes On And On ".repeat(20),
        &"Extremely Long Author Name ".repeat(10), 
        Some(86400) // 24 hours
    );
    audiobook.size_bytes = Some(u64::MAX / 1000); // Very large but not overflowing
    audiobook
}
