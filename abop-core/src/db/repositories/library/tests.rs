#[cfg(test)]
mod tests {
    use super::super::LibraryRepository;
    use crate::db::{connection::EnhancedConnection, migrations::run_migrations};
    use rusqlite::Connection;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use tempfile::NamedTempFile;/// Set up test database with migrations and proper error handling
    fn setup_test_db() -> Arc<EnhancedConnection> {
        let temp_file = NamedTempFile::new()
            .expect("Failed to create temp file for test database");
        let db_path = temp_file.path();
        
        let enhanced_conn = Arc::new(EnhancedConnection::new(db_path));
        
        // Connect to the database first
        enhanced_conn.connect()
            .expect("Failed to connect to test database - check database setup");
        
        // Set up database schema using migrations
        let mut conn = Connection::open(db_path)
            .expect("Failed to open database connection for migrations");
        
        run_migrations(&mut conn)
            .expect("Failed to run database migrations during test setup");
        
        enhanced_conn
    }

    /// Create a test library repository with a fresh database
    fn create_test_repo() -> LibraryRepository {
        let enhanced_conn = setup_test_db();
        LibraryRepository::new(enhanced_conn)
    }

    #[test]
    fn test_create_library_success() {
        let repo = create_test_repo();
        let name = "Test Library";
        let path = "/test/path";

        let result = repo.create(name, path);
        assert!(result.is_ok());

        let library = result.unwrap();        assert_eq!(library.name, name);
        assert_eq!(library.path, PathBuf::from(path));
        assert!(!library.id.is_empty());
    }

    #[test]
    fn test_create_library_duplicate_name() {
        let repo = create_test_repo();
        let name = "Duplicate Library";
        let path1 = "/test/path1";
        let path2 = "/test/path2";

        // Create first library
        let result1 = repo.create(name, path1);
        assert!(result1.is_ok());        // Try to create second library with same name
        let result2 = repo.create(name, path2);
        assert!(result2.is_err());
          let error = result2.unwrap_err();
        // Check for the actual error types returned by the repository
        let error_debug = format!("{:?}", error);
        assert!(
            error_debug.contains("Sqlite") || error_debug.contains("DuplicateEntry") || error_debug.contains("already exists"),
            "Expected duplicate error for library name, got: {}",
            error_debug
        );
    }

    #[test]
    fn test_create_library_duplicate_path() {
        let repo = create_test_repo();
        let name1 = "Library One";
        let name2 = "Library Two";
        let path = "/test/same/path";

        // Create first library
        let result1 = repo.create(name1, path);
        assert!(result1.is_ok());

        // Try to create second library with same path but different name
        let result2 = repo.create(name2, path);
        assert!(result2.is_err());
          let error = result2.unwrap_err();
        // Check for the actual error types returned by the repository
        let error_debug = format!("{:?}", error);
        assert!(
            error_debug.contains("Sqlite") || error_debug.contains("DuplicateEntry") || error_debug.contains("already exists"),
            "Expected duplicate error for library path, got: {}",
            error_debug
        );
    }

    #[test]
    fn test_find_by_id_existing() {
        let repo = create_test_repo();
        let name = "Find Test Library";
        let path = "/test/find/path";

        // Create a library first
        let created = repo.create(name, path).unwrap();

        // Find by ID
        let result = repo.find_by_id(&created.id);
        assert!(result.is_ok());

        let found = result.unwrap();
        assert!(found.is_some());

        let library = found.unwrap();
        assert_eq!(library.id, created.id);
        assert_eq!(library.name, name);
        assert_eq!(library.path, PathBuf::from(path));
    }

    #[test]
    fn test_find_by_id_non_existing() {
        let repo = create_test_repo();
        let fake_id = "non-existent-id";

        let result = repo.find_by_id(fake_id);
        assert!(result.is_ok());

        let found = result.unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_find_by_name_existing() {
        let repo = create_test_repo();
        let name = "Named Library";
        let path = "/test/named/path";

        // Create a library first
        repo.create(name, path).unwrap();

        // Find by name
        let result = repo.find_by_name(name);
        assert!(result.is_ok());

        let found = result.unwrap();
        assert!(found.is_some());

        let library = found.unwrap();
        assert_eq!(library.name, name);
        assert_eq!(library.path, PathBuf::from(path));
    }

    #[test]
    fn test_find_by_name_non_existing() {
        let repo = create_test_repo();
        let fake_name = "Non-existent Library";

        let result = repo.find_by_name(fake_name);
        assert!(result.is_ok());

        let found = result.unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_find_by_path_existing() {
        let repo = create_test_repo();
        let name = "Path Library";
        let path = "/unique/test/path";

        // Create a library first
        repo.create(name, path).unwrap();

        // Find by path
        let result = repo.find_by_path(path);
        assert!(result.is_ok());

        let found = result.unwrap();
        assert!(found.is_some());

        let library = found.unwrap();
        assert_eq!(library.name, name);
        assert_eq!(library.path, PathBuf::from(path));
    }

    #[test]
    fn test_find_by_path_non_existing() {
        let repo = create_test_repo();
        let fake_path = "/non/existent/path";

        let result = repo.find_by_path(fake_path);
        assert!(result.is_ok());

        let found = result.unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_find_all_empty() {
        let repo = create_test_repo();

        let result = repo.find_all();
        assert!(result.is_ok());

        let libraries = result.unwrap();
        assert!(libraries.is_empty());
    }    #[test]
    fn test_find_all_multiple() {
        let repo = create_test_repo();

        // Create multiple libraries using batch operations for better performance
        let libraries_to_create = vec![
            ("Alpha Library", "/alpha"),
            ("Beta Library", "/beta"),
            ("Gamma Library", "/gamma"),
        ];
        
        // Use transactions for bulk operations
        let mut created_libraries = Vec::new();
        for (name, path) in libraries_to_create {
            let library = repo.create(name, path).unwrap();
            created_libraries.push(library);
        }

        let result = repo.find_all();
        assert!(result.is_ok());

        let libraries = result.unwrap();
        assert_eq!(libraries.len(), 3);

        // Should be ordered by name (Alpha, Beta, Gamma)
        assert_eq!(libraries[0].name, "Alpha Library");
        assert_eq!(libraries[1].name, "Beta Library");
        assert_eq!(libraries[2].name, "Gamma Library");
    }

    #[test]
    fn test_update_library_success() {
        let repo = create_test_repo();
        let original_name = "Original Library";
        let original_path = "/original/path";

        // Create a library first
        let created = repo.create(original_name, original_path).unwrap();

        // Update the library
        let new_name = "Updated Library";
        let new_path = Path::new("/updated/path");
        let result = repo.update(&created.id, new_name, new_path);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true for successful update

        // Verify the update
        let updated = repo.find_by_id(&created.id).unwrap().unwrap();
        assert_eq!(updated.name, new_name);
        assert_eq!(updated.path, PathBuf::from(new_path));
    }

    #[test]
    fn test_update_library_non_existing() {
        let repo = create_test_repo();
        let fake_id = "non-existent-id";
        let new_name = "New Name";
        let new_path = Path::new("/new/path");

        let result = repo.update(fake_id, new_name, new_path);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for non-existent library
    }

    #[test]
    fn test_delete_library_success() {
        let repo = create_test_repo();
        let name = "Delete Test Library";
        let path = "/delete/test/path";

        // Create a library first
        let created = repo.create(name, path).unwrap();

        // Delete the library
        let result = repo.delete(&created.id);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true for successful deletion

        // Verify the deletion
        let found = repo.find_by_id(&created.id).unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_library_non_existing() {
        let repo = create_test_repo();
        let fake_id = "non-existent-id";

        let result = repo.delete(fake_id);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for non-existent library
    }

    #[test]
    fn test_exists_library_true() {
        let repo = create_test_repo();
        let name = "Exists Test Library";
        let path = "/exists/test/path";

        // Create a library first
        let created = repo.create(name, path).unwrap();

        // Check if it exists
        let result = repo.exists(&created.id);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_exists_library_false() {
        let repo = create_test_repo();
        let fake_id = "non-existent-id";

        let result = repo.exists(fake_id);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }    #[test]
    fn test_create_with_special_characters() {
        let repo = create_test_repo();
        let name = "Library with Special Characters: éñ & 日本語";
        let path = "/path/with spaces & special chars";

        let result = repo.create(name, path);
        assert!(result.is_ok());

        let library = result.unwrap();
        assert_eq!(library.name, name);
        assert_eq!(library.path, PathBuf::from(path));
        
        // Test that special characters are properly normalized and stored
        let retrieved = repo.find_by_id(&library.id).unwrap().unwrap();
        assert_eq!(retrieved.name, name);
        assert_eq!(retrieved.path, PathBuf::from(path));
        
        // Test filesystem path normalization (basic check)
        assert!(retrieved.path.to_string_lossy().contains("spaces"));
        assert!(retrieved.path.to_string_lossy().contains("&"));
        
        // Test Unicode handling
        assert!(retrieved.name.contains("éñ"));
        assert!(retrieved.name.contains("日本語"));
        
        // Test that the library can be found by name with special characters
        let found_by_name = repo.find_by_name(name).unwrap().unwrap();
        assert_eq!(found_by_name.id, library.id);
    }

    #[test]
    fn test_workflow_create_update_delete() {
        let repo = create_test_repo();
        
        // Create
        let original_name = "Workflow Test Library";
        let original_path = "/workflow/original";
        let created = repo.create(original_name, original_path).unwrap();

        // Update
        let updated_name = "Updated Workflow Library";
        let updated_path = Path::new("/workflow/updated");
        let update_result = repo.update(&created.id, updated_name, updated_path);
        assert!(update_result.is_ok());
        assert!(update_result.unwrap());

        // Verify update
        let found = repo.find_by_id(&created.id).unwrap().unwrap();
        assert_eq!(found.name, updated_name);
        assert_eq!(found.path, PathBuf::from(updated_path));

        // Delete
        let delete_result = repo.delete(&created.id);
        assert!(delete_result.is_ok());
        assert!(delete_result.unwrap());

        // Verify deletion
        let not_found = repo.find_by_id(&created.id).unwrap();
        assert!(not_found.is_none());
    }

    #[test]
    fn test_bulk_operations_performance() {
        let repo = create_test_repo();
          // Test creating and finding a larger number of libraries efficiently
        const NUM_LIBRARIES: usize = 50;
        let mut created_ids = Vec::with_capacity(NUM_LIBRARIES);
          // Prepare test data first to avoid lifetime issues
        let test_data: Vec<(String, std::path::PathBuf)> = (0..NUM_LIBRARIES)
            .map(|i| (format!("Bulk Library {}", i), std::path::PathBuf::from(format!("/bulk/library/{}", i))))
            .collect();
        
        // Batch creation - use owned PathBuf values
        for (name, path) in test_data {
            let library = repo.create(&name, path).unwrap();
            created_ids.push(library.id);
        }
        
        // Verify all were created
        assert_eq!(created_ids.len(), NUM_LIBRARIES);
        
        // Batch verification using find_all (more efficient than individual lookups)
        let all_libraries = repo.find_all().unwrap();
        assert_eq!(all_libraries.len(), NUM_LIBRARIES);
        
        // Verify ordering is consistent
        for i in 1..all_libraries.len() {
            assert!(all_libraries[i-1].name <= all_libraries[i].name, 
                    "Libraries should be ordered by name");
        }
        
        // Test that find operations are working correctly on bulk data
        let first_library = repo.find_by_name("Bulk Library 0").unwrap().unwrap();
        assert_eq!(first_library.name, "Bulk Library 0");
        assert_eq!(first_library.path, PathBuf::from("/bulk/library/0"));
    }
}
