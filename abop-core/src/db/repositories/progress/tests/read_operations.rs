//! Tests for progress repository read operations

use super::super::ProgressRepository;
use crate::models::Progress;
use crate::test_utils::TestDatabase;
use chrono::Utc;

/// Create a test progress repository with a fresh database
fn create_test_repo() -> (ProgressRepository, TestDatabase) {
    super::create_test_repo_with_deps(&["audiobook-1"])
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
        super::create_test_repo_with_deps(&["audiobook-1", "audiobook-2", "audiobook-3"]);

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
