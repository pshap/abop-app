#[cfg(test)]
mod library_tests {
    use super::super::LibraryRepository;
    use crate::db::{connection::EnhancedConnection, migrations::run_migrations};
    use rusqlite::Connection;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use tempfile::NamedTempFile;

    /// Create a temporary database file for testing
    ///
    /// # Returns
    /// A `NamedTempFile` that will be cleaned up when dropped
    fn create_temp_db_file() -> NamedTempFile {
        NamedTempFile::new().expect("Failed to create temporary file for test database")
    }

    /// Create and connect to an enhanced database connection
    ///
    /// # Arguments
    /// * `db_path` - Path to the database file
    ///
    /// # Returns
    /// A connected `Arc<EnhancedConnection>`
    fn create_enhanced_connection(db_path: &Path) -> Arc<EnhancedConnection> {
        let enhanced_conn = Arc::new(EnhancedConnection::new(db_path));
        enhanced_conn
            .connect()
            .expect("Failed to connect to test database - check database setup");
        enhanced_conn
    }

    /// Initialize the database schema using migrations
    ///
    /// # Arguments
    /// * `db_path` - Path to the database file
    fn initialize_schema(db_path: &Path) {
        let mut conn =
            Connection::open(db_path).expect("Failed to open database connection for migrations");
        run_migrations(&mut conn).expect("Failed to run database migrations during test setup");
    }

    /// Set up a fresh test database with migrations and proper error handling.
    ///
    /// # Returns
    /// A new `Arc<EnhancedConnection>` connected to a temporary database with all migrations applied.
    ///
    /// # Panics
    /// Panics if any step of the database setup fails, with a descriptive error message.
    fn setup_test_db() -> Arc<EnhancedConnection> {
        let temp_file = create_temp_db_file();
        let db_path = temp_file.path();
        
        let enhanced_conn = create_enhanced_connection(db_path);
        initialize_schema(db_path);
        
        enhanced_conn
    }

    /// Create a test library repository with a fresh database
    ///
    /// # Returns
    /// A new `LibraryRepository` instance connected to a fresh test database.
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
    }

    #[test]
    fn test_create_library_duplicate_name() {
        let repo = create_test_repo();
        let name = "Duplicate Library";
        let path1 = "/test/path1";
        let path2 = "/test/path2";

        // Create first library
        let result1 = repo.create(name, path1);
        assert!(result1.is_ok(), "First library creation should succeed");

        // Try to create second library with same name
        let result2 = repo.create(name, path2);

        // Check for error
        assert!(
            result2.is_err(),
            "Creating library with duplicate name should fail"
        );

        // Get the error and check if it's the expected constraint error
        let error = result2.unwrap_err();

        // Check if it's a DatabaseError containing an SQLite constraint error
        let error_msg = error.to_string().to_lowercase();
        assert!(
            error_msg.contains("unique")
                || error_msg.contains("duplicate")
                || error_msg.contains("constraint"),
            "Error message should indicate a constraint violation, got: {error_msg}",
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
        assert!(result1.is_ok(), "First library creation should succeed");

        // Try to create second library with same path but different name
        let result2 = repo.create(name2, path);

        // Check for error
        assert!(
            result2.is_err(),
            "Creating library with duplicate path should fail"
        );

        // Get the error and check if it's the expected constraint error
        let error = result2.unwrap_err();

        // Check if it's a DatabaseError containing an SQLite constraint error
        let error_msg = error.to_string().to_lowercase();
        assert!(
            error_msg.contains("unique")
                || error_msg.contains("duplicate")
                || error_msg.contains("constraint")
                || error_msg.contains("path"),
            "Error message should indicate a path constraint violation, got: {error_msg}",
        );
    }

    /// Test that creating a library with an empty name or path fails
    #[test]
    fn test_create_library_invalid_input() {
        let repo = create_test_repo();

        // Test empty name
        let result = repo.create("", "/valid/path");
        assert!(
            result.is_err(),
            "Creating library with empty name should fail"
        );

        // Test empty path
        let result = repo.create("Valid Name", "");
        assert!(
            result.is_err(),
            "Creating library with empty path should fail"
        );

        // Test whitespace-only name
        let result = repo.create("   ", "/valid/path");
        assert!(
            result.is_err(),
            "Creating library with whitespace-only name should fail"
        );

        // Test whitespace-only path
        let result = repo.create("Valid Name", "   ");
        assert!(
            result.is_err(),
            "Creating library with whitespace-only path should fail"
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
    }
    #[test]
    fn test_find_all_multiple() {
        let repo = create_test_repo();

        // Create multiple libraries using batch operations for better performance
        let libraries_to_create = vec![
            ("Alpha Library", "/alpha"),
            ("Beta Library", "/beta"),
            ("Gamma Library", "/gamma"),
        ];

        // Use batched create_many for efficient bulk operations
        let created_count = repo.create_many(&libraries_to_create).unwrap();
        assert_eq!(created_count, 3, "All libraries should be created");

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

        // Test various special characters and edge cases
        let test_cases = [
            ("Standard ASCII", "Test Library", "/test/path"),
            (
                "Unicode characters",
                "Bibliothèque avec accents: éèàçù",
                "/chemin/avec/caractères",
            ),
            (
                "Non-Latin scripts",
                "図書館 ライブラリ 图书馆 도서관",
                "/путь/к/библиотеке",
            ),
            (
                "Special symbols",
                "Library !@#$%^&*()_+-=[]{}|;:'\",.<>/?",
                "/path/with/symbols/!@#$%^&*()",
            ),
            (
                "Whitespace in name",
                "  Library with  extra   spaces  ",
                "/path/with/spaces",
            ),
            ("Mixed case", "LiBrArY wItH mIxEd CaSe", "/PaTh/To/LiBrArY"),
        ];

        for (test_case, name, path) in test_cases {
            // Test creation
            let result = repo.create(name, path);
            assert!(
                result.is_ok(),
                "Failed to create library for case '{test_case}': {result:?}",
            );

            let library = result.unwrap();
            assert_eq!(library.name, name, "Name mismatch for case '{test_case}'");
            assert_eq!(
                library.path,
                PathBuf::from(path),
                "Path mismatch for case '{test_case}'",
            );

            // Test retrieval
            let found = repo.find_by_id(&library.id);
            assert!(
                found.is_ok(),
                "Failed to find library by ID for case '{test_case}'",
            );
            let found_lib = found.unwrap();
            assert!(
                found_lib.is_some(),
                "Library not found by ID for case '{test_case}'",
            );
            assert_eq!(
                found_lib.unwrap().id,
                library.id,
                "Found library ID mismatch for case '{test_case}'",
            );

            // Test that the library can be found by name with special characters
            let found_by_name = repo.find_by_name(name);
            assert!(
                found_by_name.is_ok(),
                "Failed to find library by name for case '{test_case}'",
            );
            assert_eq!(
                found_by_name.unwrap().unwrap().id,
                library.id,
                "Found library by name ID mismatch for case '{test_case}'",
            );

            // Clean up
            assert!(
                repo.delete(&library.id).is_ok(),
                "Failed to clean up library for case '{test_case}'",
            );
        }
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
    fn test_bulk_operations() {
        // This test verifies that bulk operations work correctly with a moderate number of items
        // Note: For actual performance testing, use criterion or another benchmarking framework
        const NUM_LIBRARIES: usize = 50; // Reasonable number for functional testing

        let repo = create_test_repo();

        // Create libraries using batch operations (more efficient)
        // Use a smaller dataset that we can define statically for bulk operations test
        const BULK_TEST_LIBRARIES: [(&str, &str); 5] = [
            ("Bulk Library 1", "/bulk/path/1"),
            ("Bulk Library 2", "/bulk/path/2"),
            ("Bulk Library 3", "/bulk/path/3"),
            ("Bulk Library 4", "/bulk/path/4"),
            ("Bulk Library 5", "/bulk/path/5"),
        ];

        let created_count = repo.create_many(&BULK_TEST_LIBRARIES).expect("Failed to create libraries in batch");
        assert_eq!(created_count, BULK_TEST_LIBRARIES.len(), "Not all libraries were created in batch");

        // Create remaining libraries using individual operations for simplicity
        // Design rationale: While batch operations are more efficient, using individual
        // creates for the remaining items keeps the test code simple and readable.
        // The bulk operation above already tests the performance-critical path.
        // For test code, maintainability is prioritized over micro-optimization.
        let remaining_count = NUM_LIBRARIES - BULK_TEST_LIBRARIES.len();
        for i in 0..remaining_count {
            let name = format!("Individual Library {i}");
            let path = std::path::PathBuf::from(format!("/individual/path/{i}"));
            repo.create(&name, path).expect("Failed to create individual library");
        }

        // Verify all libraries were created
        let all_libraries = repo.find_all().expect("Failed to fetch all libraries");
        assert_eq!(
            all_libraries.len(),
            NUM_LIBRARIES,
            "Incorrect number of libraries found"
        );

        // Verify each library can be found by ID and collect IDs for cleanup
        let created_ids: Vec<String> = all_libraries.iter().map(|lib| lib.id.clone()).collect();
        for id in &created_ids {
            assert!(
                repo.find_by_id(id).is_ok(),
                "Failed to find library with ID: {id}",
            );
        }

        // Clean up
        for id in created_ids {
            assert!(
                repo.delete(&id).is_ok(),
                "Failed to delete library with ID: {id}",
            );
        }

        // Verify all libraries were deleted
        assert!(
            repo.find_all()
                .expect("Failed to fetch all libraries")
                .is_empty(),
            "Not all libraries were deleted"
        );
    }

    /// Test transaction behavior in bulk operations
    /// 
    /// This test verifies database transaction isolation and rollback behavior
    /// in bulk operations that contain both valid and invalid entries.
    /// 
    /// **Test Scenarios:**
    /// 1. Creates valid individual libraries (separate transactions)
    /// 2. Attempts bulk creation with duplicate paths (should fail atomically)
    /// 3. Verifies no partial commits occurred (transaction rollback)
    /// 
    /// **Expected Behavior:**
    /// - Individual valid libraries should be created successfully
    /// - Bulk operation with constraint violations should fail entirely
    /// - No partial state should be left in the database after failure
    /// - Transaction boundaries should be properly maintained
    #[test]
    fn test_bulk_operations_transaction() {
        let repo = create_test_repo();

        // Define test cases with a duplicate path using static lifetime
        const VALID_LIBS: [(&str, &str); 2] =
            [("Valid 1", "/valid/path/1"), ("Valid 2", "/valid/path/2")];

        // This should fail because of the duplicate path
        const DUPLICATE_CASES: [(&str, &str); 2] = [
            ("Should Fail", "/duplicate/path"),
            ("Should Also Fail", "/duplicate/path"),
        ];

        // Try to create all libraries in separate transactions (should succeed)
        for (name, path) in &VALID_LIBS {
            assert!(
                repo.create(name, *path).is_ok(),
                "Failed to create valid library: {name} at {path}",
            );
        }

        // Try to create libraries with a duplicate path in a transaction (should fail)
        let result = repo.create_many(&DUPLICATE_CASES);

        // The entire batch should fail due to the duplicate path
        assert!(
            result.is_err(),
            "Expected batch operation with duplicate paths to fail"
        );

        // Verify no partial changes were committed
        let all_libraries = repo.find_all().expect("Failed to fetch all libraries");
        assert_eq!(
            all_libraries.len(),
            VALID_LIBS.len(),
            "Number of libraries should match the valid ones created"
        );
    }

    /// Test bulk operations functionality with a moderate number of records
    /// 
    /// This is a functional test that verifies bulk operations work correctly,
    /// not a performance benchmark. For actual performance benchmarking,
    /// see the benches/ directory with criterion-based benchmarks.
    #[test]
    fn test_bulk_operations_performance() {
        let repo = create_test_repo();
        // Test creating and finding a larger number of libraries efficiently
        // Using 50 libraries to test bulk operations performance without making tests too slow
        const NUM_LIBRARIES: usize = 50;
        let mut created_ids = Vec::with_capacity(NUM_LIBRARIES);
        // Prepare test data first to avoid lifetime issues
        let test_data: Vec<(String, std::path::PathBuf)> = (0..NUM_LIBRARIES)
            .map(|i| {
                (
                    format!("Bulk Library {i}"),
                    std::path::PathBuf::from(format!("/bulk/library/{i}")),
                )
            })
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
            assert!(
                all_libraries[i - 1].name <= all_libraries[i].name,
                "Libraries should be ordered by name"
            );
        }

        // Test that find operations are working correctly on bulk data
        let first_library = repo.find_by_name("Bulk Library 0").unwrap().unwrap();
        assert_eq!(first_library.name, "Bulk Library 0");
        assert_eq!(first_library.path, PathBuf::from("/bulk/library/0"));
    }
}
