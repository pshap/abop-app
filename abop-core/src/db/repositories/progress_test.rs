//! Tests for the progress repository

use super::*;
use crate::{db::test_utils::create_test_database, models::Progress};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

/// Helper function to create a test progress record
fn create_test_progress(audiobook_id: &str) -> Progress {
    Progress {
        id: Uuid::new_v4().to_string(),
        audiobook_id: audiobook_id.to_string(),
        position_seconds: 300, // 5 minutes
        completed: false,
        last_played: Some(Utc::now()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn test_upsert_and_find() {
    // Setup
    let conn = Arc::new(create_test_database());
    let repo = ProgressRepository::new(conn);
    let audiobook_id = Uuid::new_v4().to_string();
    
    // Create and insert a progress record
    let mut progress = create_test_progress(&audiobook_id);
    repo.upsert(&progress).unwrap();
    
    // Test find_by_audiobook
    let found = repo.find_by_audiobook(&audiobook_id).unwrap().unwrap();
    assert_eq!(found.audiobook_id, audiobook_id);
    assert_eq!(found.position_seconds, 300);
    
    // Test find_by_id
    let found_by_id = repo.find_by_id(&progress.id).unwrap().unwrap();
    assert_eq!(found_by_id.id, progress.id);
    
    // Test update
    progress.position_seconds = 600; // 10 minutes
    repo.upsert(&progress).unwrap();
    
    let updated = repo.find_by_id(&progress.id).unwrap().unwrap();
    assert_eq!(updated.position_seconds, 600);
}

#[test]
fn test_find_all() {
    // Setup
    let conn = Arc::new(create_test_database());
    let repo = ProgressRepository::new(conn);
    
    // Insert multiple progress records
    for i in 0..3 {
        let mut progress = create_test_progress(&Uuid::new_v4().to_string());
        progress.position_seconds = 100 * (i + 1) as i64;
        repo.upsert(&progress).unwrap();
    }
    
    // Test find_all
    let all_progress = repo.find_all().unwrap();
    assert_eq!(all_progress.len(), 3);
    
    // Verify positions are correct
    let positions: Vec<i64> = all_progress.iter()
        .map(|p| p.position_seconds)
        .collect();
    assert!(positions.contains(&100));
    assert!(positions.contains(&200));
    assert!(positions.contains(&300));
}

#[test]
fn test_recently_played() {
    // Setup
    let conn = Arc::new(create_test_database());
    let repo = ProgressRepository::new(conn);
    
    // Create progress records with different last_played dates
    let mut progress1 = create_test_progress(&Uuid::new_v4().to_string());
    progress1.last_played = Some(Utc::now() - chrono::Duration::days(2));
    repo.upsert(&progress1).unwrap();
    
    let mut progress2 = create_test_progress(&Uuid::new_v4().to_string());
    progress2.last_played = Some(Utc::now() - chrono::Duration::hours(12));
    repo.upsert(&progress2).unwrap();
    
    // Test get_recently_played
    let recent = repo.get_recently_played(1).unwrap(); // Last 1 day
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].id, progress2.id);
    
    let recent = repo.get_recently_played(3).unwrap(); // Last 3 days
    assert_eq!(recent.len(), 2);
}

#[test]
fn test_completed_and_in_progress() {
    // Setup
    let conn = Arc::new(create_test_database());
    let repo = ProgressRepository::new(conn);
    
    // Create completed and in-progress records
    let mut completed = create_test_progress(&Uuid::new_v4().to_string());
    completed.completed = true;
    repo.upsert(&completed).unwrap();
    
    let in_progress = create_test_progress(&Uuid::new_v4().to_string());
    repo.upsert(&in_progress).unwrap();
    
    // Test get_completed
    let completed_books = repo.get_completed().unwrap();
    assert_eq!(completed_books.len(), 1);
    assert_eq!(completed_books[0].id, completed.id);
    
    // Test get_in_progress
    let in_progress_books = repo.get_in_progress().unwrap();
    assert_eq!(in_progress_books.len(), 1);
    assert_eq!(in_progress_books[0].id, in_progress.id);
}

#[test]
fn test_update_position() {
    // Setup
    let conn = Arc::new(create_test_database());
    let repo = ProgressRepository::new(conn);
    let audiobook_id = Uuid::new_v4().to_string();
    
    // Insert initial progress
    let mut progress = create_test_progress(&audiobook_id);
    repo.upsert(&progress).unwrap();
    
    // Test update_position
    let updated = repo.update_position(&audiobook_id, 900).unwrap();
    assert!(updated);
    
    let updated_progress = repo.find_by_audiobook(&audiobook_id).unwrap().unwrap();
    assert_eq!(updated_progress.position_seconds, 900);
    assert!(!updated_progress.completed);
    
    // Test with non-existent audiobook
    let not_found = repo.update_position("nonexistent", 1000).unwrap();
    assert!(!not_found);
}

#[test]
fn test_mark_completed() {
    // Setup
    let conn = Arc::new(create_test_database());
    let repo = ProgressRepository::new(conn);
    let audiobook_id = Uuid::new_v4().to_string();
    
    // Insert initial progress
    let progress = create_test_progress(&audiobook_id);
    repo.upsert(&progress).unwrap();
    
    // Test mark as completed
    let marked = repo.mark_completed(&audiobook_id, true).unwrap();
    assert!(marked);
    
    let updated = repo.find_by_audiobook(&audiobook_id).unwrap().unwrap();
    assert!(updated.completed);
    
    // Test mark as not completed
    repo.mark_completed(&audiobook_id, false).unwrap();
    let updated = repo.find_by_audiobook(&audiobook_id).unwrap().unwrap();
    assert!(!updated.completed);
    
    // Test with non-existent audiobook
    let not_found = repo.mark_completed("nonexistent", true).unwrap();
    assert!(!not_found);
}

#[test]
fn test_delete() {
    // Setup
    let conn = Arc::new(create_test_database());
    let repo = ProgressRepository::new(conn);
    let audiobook_id = Uuid::new_v4().to_string();
    
    // Insert test data
    let progress = create_test_progress(&audiobook_id);
    repo.upsert(&progress).unwrap();
    
    // Test delete_by_audiobook
    let deleted = repo.delete_by_audiobook(&audiobook_id).unwrap();
    assert!(deleted);
    
    let found = repo.find_by_audiobook(&audiobook_id).unwrap();
    assert!(found.is_none());
    
    // Test delete by id
    let progress = create_test_progress(&Uuid::new_v4().to_string());
    repo.upsert(&progress).unwrap();
    
    let deleted = repo.delete(&progress.id).unwrap();
    assert!(deleted);
    
    let found = repo.find_by_id(&progress.id).unwrap();
    assert!(found.is_none());
    
    // Test delete non-existent
    let not_found = repo.delete("nonexistent").unwrap();
    assert!(!not_found);
}

#[test]
fn test_statistics() {
    // Setup
    let conn = Arc::new(create_test_database());
    let repo = ProgressRepository::new(conn);
    
    // Insert test data
    let mut completed = create_test_progress(&Uuid::new_v4().to_string());
    completed.completed = true;
    repo.upsert(&completed).unwrap();
    
    let in_progress = create_test_progress(&Uuid::new_v4().to_string());
    repo.upsert(&in_progress).unwrap();
    
    // Test get_statistics
    let (total, completed_count, in_progress_count) = repo.get_statistics().unwrap();
    assert_eq!(total, 2);
    assert_eq!(completed_count, 1);
    assert_eq!(in_progress_count, 1);
}

#[test]
fn test_exists_for_audiobook() {
    // Setup
    let conn = Arc::new(create_test_database());
    let repo = ProgressRepository::new(conn);
    let audiobook_id = Uuid::new_v4().to_string();
    
    // Test non-existent
    let exists = repo.exists_for_audiobook(&audiobook_id).unwrap();
    assert!(!exists);
    
    // Insert and test exists
    let progress = create_test_progress(&audiobook_id);
    repo.upsert(&progress).unwrap();
    
    let exists = repo.exists_for_audiobook(&audiobook_id).unwrap();
    assert!(exists);
}
