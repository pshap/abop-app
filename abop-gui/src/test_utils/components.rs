//! Component test utilities for GUI testing
//!
//! This module provides centralized utilities for testing GUI components,
//! eliminating code duplication and providing consistent test patterns.

use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;
use abop_core::models::audiobook::Audiobook;
use abop_core::models::library::Library;
use abop_core::models::progress::Progress;
use abop_core::PlayerState;
use std::collections::HashSet;
use std::path::PathBuf;

/// Factory for creating test data for GUI components
pub struct TestDataFactory;

impl TestDataFactory {
    /// Create a test audiobook with default values
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the audiobook
    /// * `title` - Title of the audiobook
    ///
    /// # Returns
    /// A configured `Audiobook` instance with test data
    pub fn audiobook(id: &str, title: &str) -> Audiobook {
        let path = PathBuf::from(format!("/test/path/{title}.mp3"));
        let mut audiobook = Audiobook::new("test-library-id", &path);
        audiobook.id = id.to_string();
        audiobook.title = Some(title.to_string());
        audiobook.author = Some("Test Author".to_string());
        audiobook.duration_seconds = Some(3600); // 1 hour
        audiobook.size_bytes = Some(1024000); // ~1MB
        audiobook
    }

    /// Create a test audiobook with custom metadata
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the audiobook
    /// * `title` - Title of the audiobook
    /// * `author` - Author of the audiobook
    /// * `duration_seconds` - Duration in seconds
    /// * `size_bytes` - File size in bytes
    ///
    /// # Returns
    /// A configured `Audiobook` instance with the specified metadata
    pub fn custom_audiobook(
        id: &str,
        title: &str,
        author: &str,
        duration_seconds: Option<u64>,
        size_bytes: Option<u64>,
    ) -> Audiobook {
        let path = PathBuf::from(format!("/test/path/{title}.mp3"));
        let mut audiobook = Audiobook::new("test-library-id", &path);
        audiobook.id = id.to_string();
        audiobook.title = Some(title.to_string());
        audiobook.author = Some(author.to_string());
        audiobook.duration_seconds = duration_seconds;
        audiobook.size_bytes = size_bytes;
        audiobook
    }

    /// Create an Audiobook using an explicit path along with id/title/author
    pub fn audiobook_with_path<P: AsRef<std::path::Path>>(
        id: &str,
        title: &str,
        author: &str,
        path: P,
    ) -> Audiobook {
        let mut audiobook = Audiobook::new("test-library-id", path);
        audiobook.id = id.to_string();
        audiobook.title = Some(title.to_string());
        audiobook.author = Some(author.to_string());
        audiobook
    }

    /// Create a collection of test audiobooks for bulk testing
    ///
    /// # Arguments
    /// * `count` - Number of audiobooks to create
    ///
    /// # Returns
    /// A vector of `Audiobook` instances with varied test data
    pub fn audiobook_collection(count: usize) -> Vec<Audiobook> {
        (0..count)
            .map(|i| {
                Self::custom_audiobook(
                    &format!("test-{i}"),
                    &format!("Test Book {}", i + 1),
                    &format!("Author {}", (i % 5) + 1), // Cycle through 5 authors
                    Some(3600 + (i as u64 * 300)), // Vary duration
                    Some(1024000 + (i as u64 * 50000)), // Vary size
                )
            })
            .collect()
    }

    /// Create a test library
    ///
    /// # Arguments
    /// * `id` - Library ID
    /// * `name` - Library name
    /// * `path` - Library path
    ///
    /// # Returns
    /// A test `Library` instance
    pub fn library(id: &str, name: &str, path: &str) -> Library {
        Library {
            id: id.to_string(),
            name: name.to_string(),
            path: PathBuf::from(path),
        }
    }

    /// Create a test progress record
    ///
    /// # Arguments
    /// * `audiobook_id` - ID of the audiobook
    /// * `position_seconds` - Current position in seconds
    /// * `completed` - Whether the audiobook is completed
    ///
    /// # Returns
    /// A test `Progress` instance
    pub fn progress(audiobook_id: &str, position_seconds: u64, completed: bool) -> Progress {
        let now = chrono::Utc::now();
        Progress {
            id: format!("progress-{}", uuid::Uuid::new_v4()),
            audiobook_id: audiobook_id.to_string(),
            position_seconds,
            completed,
            last_played: Some(now),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Utilities for creating GUI state and test scenarios
pub struct TestStateFactory;

impl TestStateFactory {
    /// Create Material Design tokens for testing
    ///
    /// # Arguments
    /// * `theme_mode` - The theme mode to use
    ///
    /// # Returns
    /// Material tokens for the specified theme
    pub fn material_tokens(theme_mode: ThemeMode) -> MaterialTokens {
        match theme_mode {
            ThemeMode::Light | ThemeMode::MaterialLight => MaterialTokens::light(),
            ThemeMode::Dark | ThemeMode::MaterialDark | ThemeMode::System => MaterialTokens::dark(),
            ThemeMode::MaterialDynamic => {
                let seed_color = iced::Color::from_rgb(0.4, 0.2, 0.8);
                MaterialTokens::from_seed_color(seed_color, true)
            }
        }
    }

    /// Create a selection state for testing
    ///
    /// # Arguments
    /// * `selected_ids` - IDs of selected audiobooks
    ///
    /// # Returns
    /// A HashSet containing the selected IDs
    pub fn selection_state(selected_ids: &[&str]) -> HashSet<String> {
        selected_ids.iter().map(|&id| id.to_string()).collect()
    }

    /// Create different player states for testing
    pub fn player_states() -> Vec<(PlayerState, &'static str)> {
        vec![
            (PlayerState::Stopped, "stopped state"),
            (PlayerState::Playing, "playing state"),
            (PlayerState::Paused, "paused state"),
        ]
    }

    /// Create different theme modes for testing
    pub fn theme_modes() -> Vec<ThemeMode> {
        vec![
            ThemeMode::Light,
            ThemeMode::Dark,
            ThemeMode::MaterialLight,
            ThemeMode::MaterialDark,
            ThemeMode::MaterialDynamic,
            ThemeMode::System,
        ]
    }
}

/// Common test scenarios and edge cases
pub struct TestScenarios;

impl TestScenarios {
    /// Test scenario: Empty audiobook collection
    pub fn empty_collection() -> Vec<Audiobook> {
        vec![]
    }

    /// Test scenario: Single audiobook
    pub fn single_audiobook() -> Vec<Audiobook> {
        vec![TestDataFactory::audiobook("single", "Single Book")]
    }

    /// Test scenario: Multiple audiobooks with varied metadata
    pub fn varied_audiobooks() -> Vec<Audiobook> {
        vec![
            TestDataFactory::custom_audiobook("1", "Short Book", "Alice", Some(1800), Some(500000)),
            TestDataFactory::custom_audiobook("2", "Long Book", "Bob", Some(7200), Some(2000000)),
            TestDataFactory::custom_audiobook("3", "No Duration", "Charlie", None, Some(1000000)),
            TestDataFactory::custom_audiobook("4", "No Size", "Diana", Some(3600), None),
            TestDataFactory::custom_audiobook("5", "Minimal Data", "Eve", None, None),
        ]
    }

    /// Test scenario: Audiobooks with edge case titles
    pub fn edge_case_titles() -> Vec<Audiobook> {
        vec![
            TestDataFactory::audiobook("empty", ""),
            TestDataFactory::audiobook("long", &"Very ".repeat(50)),
            TestDataFactory::audiobook("special", "Book with Special Characters: !@#$%^&*()"),
            TestDataFactory::audiobook("unicode", "Book with Unicode: 📚 测试 🎧"),
            TestDataFactory::audiobook("newlines", "Book\nWith\nNewlines"),
        ]
    }

    /// Test scenario: Large audiobook collection for performance testing
    pub fn large_collection(size: usize) -> Vec<Audiobook> {
        TestDataFactory::audiobook_collection(size)
    }

    /// Test scenario: Selection state variations
    pub fn selection_scenarios() -> Vec<(HashSet<String>, &'static str)> {
        vec![
            (HashSet::new(), "no selection"),
            (TestStateFactory::selection_state(&["1"]), "single selection"),
            (TestStateFactory::selection_state(&["1", "2"]), "multiple selection"),
            (TestStateFactory::selection_state(&["1", "2", "3", "4", "5"]), "all selected"),
        ]
    }
}

/// Assertion utilities for component testing
pub struct TestAssertions;

impl TestAssertions {
    /// Assert that an element was created without panicking
    ///
    /// This is a basic test that ensures component creation doesn't crash
    pub fn assert_element_creation<T>(_element: T) {
        // The fact that we can call this function means the element was created successfully
        // This is a common pattern in the existing tests
    }

    /// Assert that a component handles all theme modes correctly
    ///
    /// # Arguments
    /// * `component_factory` - A function that creates the component given a theme mode
    pub fn assert_all_themes_supported<F, T>(component_factory: F)
    where
        F: Fn(ThemeMode) -> T,
    {
        for theme_mode in TestStateFactory::theme_modes() {
            let element = component_factory(theme_mode);
            Self::assert_element_creation(element);
        }
    }

    /// Assert that a component handles various audiobook collections
    ///
    /// # Arguments
    /// * `component_factory` - A function that creates the component given audiobooks
    pub fn assert_audiobook_collections_supported<F, T>(component_factory: F)
    where
        F: Fn(Vec<Audiobook>) -> T,
    {
        let scenarios = vec![
            TestScenarios::empty_collection(),
            TestScenarios::single_audiobook(),
            TestScenarios::varied_audiobooks(),
            TestScenarios::edge_case_titles(),
        ];

        for audiobooks in scenarios {
            let element = component_factory(audiobooks);
            Self::assert_element_creation(element);
        }
    }

    /// Assert that a component handles various selection states
    ///
    /// # Arguments
    /// * `audiobooks` - The audiobooks to test with
    /// * `component_factory` - A function that creates the component given selection state
    pub fn assert_selection_states_supported<F, T>(
        audiobooks: Vec<Audiobook>,
        component_factory: F,
    )
    where
        F: Fn(Vec<Audiobook>, HashSet<String>) -> T,
    {
        for (selection, _description) in TestScenarios::selection_scenarios() {
            let element = component_factory(audiobooks.clone(), selection);
            Self::assert_element_creation(element);
        }
    }
}

/// Performance testing utilities
pub struct PerformanceTestUtils;

impl PerformanceTestUtils {
    /// Measure component creation time
    ///
    /// # Arguments
    /// * `component_factory` - Function that creates the component
    ///
    /// # Returns
    /// Duration taken to create the component
    pub fn measure_creation_time<F, T>(component_factory: F) -> std::time::Duration
    where
        F: FnOnce() -> T,
    {
        let start = std::time::Instant::now();
        let _element = component_factory();
        start.elapsed()
    }

    /// Test component creation with large datasets
    ///
    /// # Arguments
    /// * `component_factory` - Function that creates component from audiobooks
    /// * `max_size` - Maximum collection size to test
    pub fn test_scalability<F, T>(component_factory: F, max_size: usize)
    where
        F: Fn(Vec<Audiobook>) -> T,
    {
        let sizes = [10, 100, 500, 1000, max_size];
        
        for &size in &sizes {
            let audiobooks = TestScenarios::large_collection(size);
            let duration = Self::measure_creation_time(|| component_factory(audiobooks));
            
            // Log performance for manual verification
            // In a real test, you might want to assert performance thresholds
            println!("Size {}: {:?}", size, duration);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audiobook_factory() {
        let audiobook = TestDataFactory::audiobook("test-1", "Test Book");
        assert_eq!(audiobook.id, "test-1");
        assert_eq!(audiobook.title, Some("Test Book".to_string()));
        assert_eq!(audiobook.author, Some("Test Author".to_string()));
    }

    #[test]
    fn test_custom_audiobook_factory() {
        let audiobook = TestDataFactory::custom_audiobook(
            "custom-1", 
            "Custom Book", 
            "Custom Author", 
            Some(1800), 
            Some(2000000)
        );
        assert_eq!(audiobook.id, "custom-1");
        assert_eq!(audiobook.author, Some("Custom Author".to_string()));
        assert_eq!(audiobook.duration_seconds, Some(1800));
        assert_eq!(audiobook.size_bytes, Some(2000000));
    }

    #[test]
    fn test_audiobook_collection() {
        let collection = TestDataFactory::audiobook_collection(5);
        assert_eq!(collection.len(), 5);
        
        // Check that each audiobook has unique data
        for (i, audiobook) in collection.iter().enumerate() {
            assert_eq!(audiobook.id, format!("test-{i}"));
            assert_eq!(audiobook.title, Some(format!("Test Book {}", i + 1)));
        }
    }

    #[test]
    fn test_material_tokens_factory() {
        for theme_mode in TestStateFactory::theme_modes() {
            let tokens = TestStateFactory::material_tokens(theme_mode);
            // Just verify creation doesn't panic
            let _ = tokens;
        }
    }

    #[test]
    fn test_selection_state() {
        let selection = TestStateFactory::selection_state(&["1", "2", "3"]);
        assert_eq!(selection.len(), 3);
        assert!(selection.contains("1"));
        assert!(selection.contains("2"));
        assert!(selection.contains("3"));
    }

    #[test]
    fn test_test_scenarios() {
        assert!(TestScenarios::empty_collection().is_empty());
        assert_eq!(TestScenarios::single_audiobook().len(), 1);
        assert_eq!(TestScenarios::varied_audiobooks().len(), 5);
        assert_eq!(TestScenarios::edge_case_titles().len(), 5);
    }

    #[test]
    fn test_library_factory() {
        let library = TestDataFactory::library("lib-1", "Test Library", "/test/path");
        assert_eq!(library.id, "lib-1");
        assert_eq!(library.name, "Test Library");
        assert_eq!(library.path, PathBuf::from("/test/path"));
    }

    #[test]
    fn test_progress_factory() {
        let progress = TestDataFactory::progress("book-1", 1800, false);
        assert_eq!(progress.audiobook_id, "book-1");
        assert_eq!(progress.position_seconds, 1800);
        assert!(!progress.completed);
    }
}