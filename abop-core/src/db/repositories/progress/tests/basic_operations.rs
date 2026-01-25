//! Tests for progress repository basic operations

use super::super::ProgressRepository;
use crate::models::Progress;
use crate::test_utils::TestDatabase;
use chrono::Utc;

/// Create a test progress repository with a fresh database
fn create_test_repo() -> (ProgressRepository, TestDatabase) {
    let test_db = TestDatabase::new();
    let repo = ProgressRepository::new(test_db.connection.clone());
    (repo, test_db)
}

/// Create a test progress record
fn create_test_progress(audiobook_id: &str, position: u64, completed: bool) -> Progress {
    Progress {
        id: format!("progress-{audiobook_id}"),
        audiobook_id: audiobook_id.to_string(),
        position_seconds: position,
        completed,
        last_played: Some(Utc::now()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn test_upsert_new_progress() {
    let (repo, test_db) = create_test_repo();

    // Create a test library first
    let library_id = "test-library-1";
    test_db.insert_test_library(library_id, "Test Library", "/test/path");

    let progress = create_test_progress("audiobook-1", 300, false);

    let result = repo.upsert(&progress);
    assert!(result.is_ok());

    // Verify progress was saved
    let saved = repo.find_by_audiobook("audiobook-1").unwrap();
    assert!(saved.is_some());

    let saved_progress = saved.unwrap();
    assert_eq!(saved_progress.audiobook_id, "audiobook-1");
    assert_eq!(saved_progress.position_seconds, 300);
    assert!(!saved_progress.completed);
}

#[test]
fn test_upsert_update_existing_progress() {
    let (repo, test_db) = create_test_repo();

    // Create a test library first
    let library_id = "test-library-1";
    test_db.insert_test_library(library_id, "Test Library", "/test/path");

    let initial_progress = create_test_progress("audiobook-1", 300, false);

    // Insert initial progress
    repo.upsert(&initial_progress).unwrap();

    // Update the progress
    let updated_progress = Progress {
        id: initial_progress.id.clone(),
        audiobook_id: "audiobook-1".to_string(),
        position_seconds: 600,
        completed: true,
        last_played: Some(Utc::now()),
        created_at: initial_progress.created_at,
        updated_at: Utc::now(),
    };

    let result = repo.upsert(&updated_progress);
    assert!(result.is_ok());

    // Verify progress was updated
    let saved = repo.find_by_audiobook("audiobook-1").unwrap().unwrap();
    assert_eq!(saved.position_seconds, 600);
    assert!(saved.completed);
}

#[test]
fn test_find_by_audiobook_existing() {
    let (repo, test_db) = create_test_repo();

    // Create a test library first
    let library_id = "test-library-1";
    test_db.insert_test_library(library_id, "Test Library", "/test/path");

    let progress = create_test_progress("audiobook-1", 450, false);
    repo.upsert(&progress).unwrap();

    let result = repo.find_by_audiobook("audiobook-1");
    assert!(result.is_ok());

    let found = result.unwrap();
    assert!(found.is_some());

    let found_progress = found.unwrap();
    assert_eq!(found_progress.audiobook_id, "audiobook-1");
    assert_eq!(found_progress.position_seconds, 450);
    assert!(!found_progress.completed);
}

#[test]
fn test_find_by_audiobook_non_existing() {
    let (repo, _test_db) = create_test_repo();

    let result = repo.find_by_audiobook("non-existent-audiobook");
    assert!(result.is_ok());

    let found = result.unwrap();
    assert!(found.is_none());
}

#[test]
fn test_find_by_id_existing() {
    let (repo, test_db) = create_test_repo();

    // Create a test library first
    let library_id = "test-library-1";
    test_db.insert_test_library(library_id, "Test Library", "/test/path");

    let progress = create_test_progress("audiobook-1", 450, false);
    repo.upsert(&progress).unwrap();

    let result = repo.find_by_id(&progress.id);
    assert!(result.is_ok());

    let found = result.unwrap();
    assert!(found.is_some());

    let found_progress = found.unwrap();
    assert_eq!(found_progress.id, progress.id);
    assert_eq!(found_progress.audiobook_id, "audiobook-1");
}

#[test]
fn test_find_by_id_non_existing() {
    let (repo, _test_db) = create_test_repo();

    let result = repo.find_by_id("non-existent-id");
    assert!(result.is_ok());

    let found = result.unwrap();
    assert!(found.is_none());
}
