//! Tests for progress repository operations

pub mod basic_operations;
pub mod read_operations;
pub mod update_delete_operations;

use super::ProgressRepository;
use crate::db::repositories::AudiobookRepository;
use crate::test_utils::{TestDataFactory, TestDatabase};

/// Create a test progress repository with necessary dependencies
pub(super) fn create_test_repo_with_deps(audiobook_ids: &[&str]) -> (ProgressRepository, TestDatabase) {
    let test_db = TestDatabase::new();

    // Create repositories
    let audiobook_repo = AudiobookRepository::new(test_db.connection.clone());
    let progress_repo = ProgressRepository::new(test_db.connection.clone());

    // Create a test library
    let library_id = "test-library-1";
    test_db.insert_test_library(library_id, "Test Library", "/test/path");

    // Create test audiobooks for each ID
    for (i, audiobook_id) in audiobook_ids.iter().enumerate() {
        let audiobook = TestDataFactory::audiobook_with_path(
            audiobook_id,
            library_id,
            std::path::Path::new(&format!("/test/audiobook-{}.mp3", i + 1)),
            &format!("Test Audiobook {}", i + 1),
            "Test Author",
        );
        audiobook_repo
            .upsert(&audiobook)
            .expect("Failed to create test audiobook");
    }

    (progress_repo, test_db)
}
