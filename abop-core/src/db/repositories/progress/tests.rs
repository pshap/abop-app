#[cfg(test)]
mod tests {
    use super::super::ProgressRepository;
    use crate::db::repositories::{AudiobookRepository, LibraryRepository};
    use crate::db::{connection::EnhancedConnection, migrations::run_migrations};
    use crate::models::{Audiobook, Progress};
    use chrono::Utc;
    use rusqlite::Connection;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tempfile::NamedTempFile;
    /// Set up test database with migrations
    fn setup_test_db() -> Arc<EnhancedConnection> {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let db_path = temp_file.path();

        let enhanced_conn = Arc::new(EnhancedConnection::new(db_path));

        // Connect to the database first
        enhanced_conn
            .connect()
            .expect("Failed to connect to database");

        // Set up database schema using migrations
        let mut conn = Connection::open(db_path).expect("Failed to open database");
        run_migrations(&mut conn).expect("Failed to run migrations");
        enhanced_conn
    }

    /// Create a test audiobook
    fn create_test_audiobook(id: &str, library_id: &str, path: &str, title: &str) -> Audiobook {
        Audiobook {
            id: id.to_string(),
            library_id: library_id.to_string(),
            path: PathBuf::from(path),
            title: Some(title.to_string()),
            author: Some("Test Author".to_string()),
            narrator: None,
            description: None,
            duration_seconds: Some(3600),
            size_bytes: Some(1024 * 1024),
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        }
    }

    /// Create a test progress repository with necessary dependencies
    fn create_test_repo_with_deps(audiobook_ids: &[&str]) -> (ProgressRepository, Vec<String>) {
        let enhanced_conn = setup_test_db();

        // Create repositories for setting up dependencies
        let library_repo = LibraryRepository::new(Arc::clone(&enhanced_conn));
        let audiobook_repo = AudiobookRepository::new(Arc::clone(&enhanced_conn));
        let progress_repo = ProgressRepository::new(enhanced_conn);

        // Create a test library
        let _library = library_repo
            .create("Test Library", "/test/path")
            .expect("Failed to create test library");

        let mut created_audiobook_ids = Vec::new();

        // Create test audiobooks for each ID
        for (i, &audiobook_id) in audiobook_ids.iter().enumerate() {
            let audiobook = create_test_audiobook(
                audiobook_id,
                &_library.id, // Use the actual library ID from creation
                &format!("/test/audiobook-{}.mp3", i + 1),
                &format!("Test Audiobook {}", i + 1),
            );
            audiobook_repo
                .upsert(&audiobook)
                .expect("Failed to create test audiobook");
            created_audiobook_ids.push(audiobook_id.to_string());
        }

        (progress_repo, created_audiobook_ids)
    }

    /// Create a test progress repository with a fresh database
    fn create_test_repo() -> ProgressRepository {
        let (repo, _) = create_test_repo_with_deps(&["audiobook-1"]);
        repo
    }

    /// Create a test progress record
    fn create_test_progress(audiobook_id: &str, position: u64, completed: bool) -> Progress {
        Progress {
            id: format!("progress-{}", audiobook_id),
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
        let repo = create_test_repo();
        let progress = create_test_progress("audiobook-1", 300, false);

        let result = repo.upsert(&progress);
        assert!(result.is_ok());

        // Verify the progress was saved
        let saved = repo.find_by_audiobook("audiobook-1").unwrap();
        assert!(saved.is_some());

        let saved_progress = saved.unwrap();
        assert_eq!(saved_progress.audiobook_id, "audiobook-1");
        assert_eq!(saved_progress.position_seconds, 300);
        assert!(!saved_progress.completed);
    }

    #[test]
    fn test_upsert_update_existing_progress() {
        let repo = create_test_repo();
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

        // Verify the progress was updated
        let saved = repo.find_by_audiobook("audiobook-1").unwrap().unwrap();
        assert_eq!(saved.position_seconds, 600);
        assert!(saved.completed);
    }

    #[test]
    fn test_find_by_audiobook_existing() {
        let repo = create_test_repo();
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
        let repo = create_test_repo();

        let result = repo.find_by_audiobook("non-existent-audiobook");
        assert!(result.is_ok());

        let found = result.unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_find_by_id_existing() {
        let repo = create_test_repo();
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
        let repo = create_test_repo();

        let result = repo.find_by_id("non-existent-id");
        assert!(result.is_ok());

        let found = result.unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_find_all_empty() {
        let repo = create_test_repo();

        let result = repo.find_all();
        assert!(result.is_ok());

        let progress_list = result.unwrap();
        assert!(progress_list.is_empty());
    }
    #[test]
    fn test_find_all_multiple() {
        let (repo, _) = create_test_repo_with_deps(&["audiobook-1", "audiobook-2", "audiobook-3"]);

        // Create multiple progress records
        let progress1 = create_test_progress("audiobook-1", 300, false);
        let progress2 = create_test_progress("audiobook-2", 600, true);
        let progress3 = create_test_progress("audiobook-3", 150, false);

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
    #[test]
    fn test_get_recently_played() {
        let (repo, _) = create_test_repo_with_deps(&["audiobook-1", "audiobook-2", "audiobook-3"]);

        // Create progress records with different last_played times
        let mut progress1 = create_test_progress("audiobook-1", 300, false);
        progress1.last_played = Some(Utc::now()); // Recent

        let mut progress2 = create_test_progress("audiobook-2", 600, true);
        progress2.last_played = Some(Utc::now() - chrono::Duration::days(5)); // Old

        let mut progress3 = create_test_progress("audiobook-3", 150, false);
        progress3.last_played = None; // Never played

        repo.upsert(&progress1).unwrap();
        repo.upsert(&progress2).unwrap();
        repo.upsert(&progress3).unwrap();

        // Get recently played within 7 days
        let result = repo.get_recently_played(7);
        assert!(result.is_ok());

        let recent_list = result.unwrap();
        // Should include progress1 and progress2 (within 7 days), but not progress3 (never played)
        assert_eq!(recent_list.len(), 2);

        let audiobook_ids: Vec<String> =
            recent_list.iter().map(|p| p.audiobook_id.clone()).collect();
        assert!(audiobook_ids.contains(&"audiobook-1".to_string()));
        assert!(audiobook_ids.contains(&"audiobook-2".to_string()));
    }
    #[test]
    fn test_get_completed() {
        let (repo, _) = create_test_repo_with_deps(&["audiobook-1", "audiobook-2", "audiobook-3"]);

        // Create progress records with different completion status
        let progress1 = create_test_progress("audiobook-1", 300, false);
        let progress2 = create_test_progress("audiobook-2", 600, true);
        let progress3 = create_test_progress("audiobook-3", 150, true);

        repo.upsert(&progress1).unwrap();
        repo.upsert(&progress2).unwrap();
        repo.upsert(&progress3).unwrap();

        let result = repo.get_completed();
        assert!(result.is_ok());

        let completed_list = result.unwrap();
        assert_eq!(completed_list.len(), 2);

        let audiobook_ids: Vec<String> = completed_list
            .iter()
            .map(|p| p.audiobook_id.clone())
            .collect();
        assert!(audiobook_ids.contains(&"audiobook-2".to_string()));
        assert!(audiobook_ids.contains(&"audiobook-3".to_string()));
        assert!(!audiobook_ids.contains(&"audiobook-1".to_string()));
    }
    #[test]
    fn test_get_in_progress() {
        let (repo, _) = create_test_repo_with_deps(&["audiobook-1", "audiobook-2", "audiobook-3"]);

        // Create progress records with different states
        let progress1 = create_test_progress("audiobook-1", 0, false); // Not started
        let progress2 = create_test_progress("audiobook-2", 300, false); // In progress
        let progress3 = create_test_progress("audiobook-3", 600, true); // Completed

        repo.upsert(&progress1).unwrap();
        repo.upsert(&progress2).unwrap();
        repo.upsert(&progress3).unwrap();

        let result = repo.get_in_progress();
        assert!(result.is_ok());

        let in_progress_list = result.unwrap();
        assert_eq!(in_progress_list.len(), 1);

        let found_progress = &in_progress_list[0];
        assert_eq!(found_progress.audiobook_id, "audiobook-2");
        assert_eq!(found_progress.position_seconds, 300);
        assert!(!found_progress.completed);
    }

    #[test]
    fn test_update_position_existing() {
        let repo = create_test_repo();
        let progress = create_test_progress("audiobook-1", 300, false);

        repo.upsert(&progress).unwrap();

        let result = repo.update_position("audiobook-1", 500);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true for successful update

        // Verify the position was updated
        let updated = repo.find_by_audiobook("audiobook-1").unwrap().unwrap();
        assert_eq!(updated.position_seconds, 500);
    }

    #[test]
    fn test_update_position_non_existing() {
        let repo = create_test_repo();

        let result = repo.update_position("non-existent-audiobook", 500);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for non-existent audiobook
    }

    #[test]
    fn test_mark_completed_existing() {
        let repo = create_test_repo();
        let progress = create_test_progress("audiobook-1", 300, false);

        repo.upsert(&progress).unwrap();

        let result = repo.mark_completed("audiobook-1", true);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true for successful update

        // Verify the completion status was updated
        let updated = repo.find_by_audiobook("audiobook-1").unwrap().unwrap();
        assert!(updated.completed);
    }

    #[test]
    fn test_mark_completed_non_existing() {
        let repo = create_test_repo();

        let result = repo.mark_completed("non-existent-audiobook", true);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for non-existent audiobook
    }

    #[test]
    fn test_delete_by_audiobook_existing() {
        let repo = create_test_repo();
        let progress = create_test_progress("audiobook-1", 300, false);

        repo.upsert(&progress).unwrap();

        let result = repo.delete_by_audiobook("audiobook-1");
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true for successful deletion

        // Verify the progress was deleted
        let found = repo.find_by_audiobook("audiobook-1").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_by_audiobook_non_existing() {
        let repo = create_test_repo();

        let result = repo.delete_by_audiobook("non-existent-audiobook");
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for non-existent audiobook
    }

    #[test]
    fn test_delete_by_id_existing() {
        let repo = create_test_repo();
        let progress = create_test_progress("audiobook-1", 300, false);

        repo.upsert(&progress).unwrap();

        let result = repo.delete(&progress.id);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true for successful deletion

        // Verify the progress was deleted
        let found = repo.find_by_id(&progress.id).unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_by_id_non_existing() {
        let repo = create_test_repo();

        let result = repo.delete("non-existent-id");
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for non-existent progress
    }
    #[test]
    fn test_get_statistics() {
        let (repo, _) = create_test_repo_with_deps(&[
            "audiobook-1",
            "audiobook-2",
            "audiobook-3",
            "audiobook-4",
        ]);

        // Create progress records with different states
        let progress1 = create_test_progress("audiobook-1", 0, false); // Not started
        let progress2 = create_test_progress("audiobook-2", 300, false); // In progress
        let progress3 = create_test_progress("audiobook-3", 600, true); // Completed
        let progress4 = create_test_progress("audiobook-4", 150, true); // Completed

        repo.upsert(&progress1).unwrap();
        repo.upsert(&progress2).unwrap();
        repo.upsert(&progress3).unwrap();
        repo.upsert(&progress4).unwrap();

        let result = repo.get_statistics();
        assert!(result.is_ok());

        let (total, completed, in_progress) = result.unwrap();
        assert_eq!(total, 4); // Total progress records
        assert_eq!(completed, 2); // Two completed
        assert_eq!(in_progress, 1); // One in progress (position > 0 and not completed)
    }

    #[test]
    fn test_exists_for_audiobook_true() {
        let repo = create_test_repo();
        let progress = create_test_progress("audiobook-1", 300, false);

        repo.upsert(&progress).unwrap();

        let result = repo.exists_for_audiobook("audiobook-1");
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_exists_for_audiobook_false() {
        let repo = create_test_repo();

        let result = repo.exists_for_audiobook("non-existent-audiobook");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
    #[test]
    fn test_progress_workflow() {
        let (repo, _) = create_test_repo_with_deps(&["audiobook-workflow"]);
        let audiobook_id = "audiobook-workflow";

        // Check that progress doesn't exist initially
        assert!(!repo.exists_for_audiobook(audiobook_id).unwrap());

        // Create initial progress
        let progress = create_test_progress(audiobook_id, 100, false);
        repo.upsert(&progress).unwrap();

        // Verify progress exists
        assert!(repo.exists_for_audiobook(audiobook_id).unwrap());

        // Update position
        repo.update_position(audiobook_id, 500).unwrap();
        let updated = repo.find_by_audiobook(audiobook_id).unwrap().unwrap();
        assert_eq!(updated.position_seconds, 500);

        // Mark as completed
        repo.mark_completed(audiobook_id, true).unwrap();
        let completed = repo.find_by_audiobook(audiobook_id).unwrap().unwrap();
        assert!(completed.completed);

        // Delete progress
        repo.delete_by_audiobook(audiobook_id).unwrap();
        assert!(!repo.exists_for_audiobook(audiobook_id).unwrap());
    }
}
