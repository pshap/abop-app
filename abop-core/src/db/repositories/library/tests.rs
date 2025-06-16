#[cfg(test)]
mod tests {
    use super::super::LibraryRepository;
    use crate::db::{connection::EnhancedConnection, migrations::run_migrations};
    use rusqlite::Connection;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use tempfile::NamedTempFile;    /// Set up test database with migrations
    fn setup_test_db() -> Arc<EnhancedConnection> {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let db_path = temp_file.path();
        
        let enhanced_conn = Arc::new(EnhancedConnection::new(db_path));
        
        // Connect to the database first
        enhanced_conn.connect().expect("Failed to connect to database");
        
        // Set up database schema using migrations
        let mut conn = Connection::open(db_path).expect("Failed to open database");
        run_migrations(&mut conn).expect("Failed to run migrations");
        
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

        let library = result.unwrap();
        assert_eq!(library.name, name);
        assert_eq!(library.path, PathBuf::from(path));
        assert!(!library.id.is_empty());
    }    #[test]
    fn test_create_library_duplicate_name() {
        let repo = create_test_repo();
        let name = "Duplicate Library";
        let path1 = "/test/path1";
        let path2 = "/test/path2";

        // Create first library
        let result1 = repo.create(name, path1);
        assert!(result1.is_ok());

        // Try to create second library with same name
        let result2 = repo.create(name, path2);
        assert!(result2.is_err());
        
        let error = result2.unwrap_err();
        // Check for constraint or duplicate error (the library repository handles this case)
        let error_msg = error.to_string();
        assert!(error_msg.contains("duplicate") || error_msg.contains("already exists") || error_msg.contains("UNIQUE"));
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
    }

    #[test]
    fn test_find_all_multiple() {
        let repo = create_test_repo();

        // Create multiple libraries
        let _lib1 = repo.create("Alpha Library", "/alpha").unwrap();
        let _lib2 = repo.create("Beta Library", "/beta").unwrap();
        let _lib3 = repo.create("Gamma Library", "/gamma").unwrap();

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
    }

    #[test]
    fn test_create_with_special_characters() {
        let repo = create_test_repo();
        let name = "Library with Special Characters: éñ & 日本語";
        let path = "/path/with spaces & special chars";

        let result = repo.create(name, path);
        assert!(result.is_ok());

        let library = result.unwrap();
        assert_eq!(library.name, name);
        assert_eq!(library.path, PathBuf::from(path));
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
}
