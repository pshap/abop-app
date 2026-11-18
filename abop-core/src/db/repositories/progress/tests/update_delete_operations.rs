//! Tests for progress repository update and delete operations

use super::super::ProgressRepository;
use crate::models::Progress;
use crate::test_utils::TestDatabase;
use chrono::Utc;

/// Create a test progress repository with a fresh database
/// The database contains a library and an `audiobook-1` row, so progress
/// records for `audiobook-1` satisfy the foreign key constraint.
fn create_test_repo() -> (ProgressRepository, TestDatabase) {
    super::create_test_repo_with_deps(&["audiobook-1"])
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
fn test_delete_progress_success() {
    let (repo, _test_db) = create_test_repo();

    let progress = create_test_progress("audiobook-1", 450, false);
    repo.upsert(&progress).unwrap();

    // Delete it
    let result = repo.delete(&progress.id).unwrap();
    assert!(result, "Deleting existing progress should return true");

    // Verify it's gone
    let found = repo.find_by_id(&progress.id).unwrap();
    assert!(found.is_none(), "Deleted progress should not be found");
}

#[test]
fn test_delete_progress_not_found() {
    let (repo, _test_db) = create_test_repo();

    let result = repo.delete("non-existent-id").unwrap();
    assert!(!result, "Deleting nonexistent progress should return false");
}

#[test]
fn test_delete_by_audiobook() {
    let (repo, _test_db) = create_test_repo();

    // Create multiple progress records for same audiobook
    let progress1 = create_test_progress("audiobook-1", 300, false);
    let progress2 = create_test_progress("audiobook-1", 600, true);
    repo.upsert(&progress1).unwrap();
    repo.upsert(&progress2).unwrap();

    // Delete all progress for audiobook
    let result = repo.delete_by_audiobook("audiobook-1").unwrap();
    assert!(result, "Should delete all progress records");

    // Verify they're gone
    let found = repo.find_by_audiobook("audiobook-1").unwrap();
    assert!(found.is_none(), "All progress should be deleted");
}
