//! Tests for progress repository read operations

use super::super::ProgressRepository;
use crate::db::repositories::AudiobookRepository;
use crate::models::Progress;
use crate::test_utils::{TestDataFactory, TestDatabase};
use chrono::Utc;
use std::path::Path;

/// Create a test progress repository with necessary dependencies
fn create_test_repo_with_deps(audiobook_ids: &[&str]) -> (ProgressRepository, TestDatabase) {
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
            Path::new(&format!("/test/audiobook-{}.mp3", i + 1)),
            &format!("Test Audiobook {}", i + 1),
            "Test Author",
        );
        audiobook_repo
            .upsert(&audiobook)
            .expect("Failed to create test audiobook");
    }

    (progress_repo, test_db)
}

/// Create a test progress repository with a fresh database
fn create_test_repo() -> (ProgressRepository, TestDatabase) {
    create_test_repo_with_deps(&["audiobook-1"])
}

#[test]
fn test_find_all_empty() {
    let (repo, _test_db) = create_test_repo();

    let result = repo.find_all();
    assert!(result.is_ok());

    let progress_list = result.unwrap();
    assert!(progress_list.is_empty());
}

#[test]
fn test_find_all_multiple() {
    let (repo, _test_db) =
        create_test_repo_with_deps(&["audiobook-1", "audiobook-2", "audiobook-3"]);

    // Create multiple progress records
    let progress1 = Progress {
        id: "progress-1".to_string(),
        audiobook_id: "audiobook-1".to_string(),
        position_seconds: 300,
        completed: false,
        last_played: Some(Utc::now()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let progress2 = Progress {
        id: "progress-2".to_string(),
        audiobook_id: "audiobook-2".to_string(),
        position_seconds: 600,
        completed: true,
        last_played: Some(Utc::now()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let progress3 = Progress {
        id: "progress-3".to_string(),
        audiobook_id: "audiobook-3".to_string(),
        position_seconds: 150,
        completed: false,
        last_played: Some(Utc::now()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    repo.upsert(&progress1).unwrap();
    repo.upsert(&progress2).unwrap();
    repo.upsert(&progress3).unwrap();

    let result = repo.find_all();
    assert!(result.is_ok());

    let progress_list = result.unwrap();
    assert_eq!(progress_list.len(), 3);

    // Results should be ordered by updated_at DESC
    // Since we created them in sequence, they should maintain that order
    let audiobook_ids: Vec<String> = progress_list
        .iter()
        .map(|p| p.audiobook_id.clone())
        .collect();
    assert!(audiobook_ids.contains(&"audiobook-1".to_string()));
    assert!(audiobook_ids.contains(&"audiobook-2".to_string()));
    assert!(audiobook_ids.contains(&"audiobook-3".to_string()));
}
