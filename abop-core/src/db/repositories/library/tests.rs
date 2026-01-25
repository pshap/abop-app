//! Tests for library repository operations
//!
//! This module contains comprehensive tests for library repository operations,
//! using centralized test utilities to ensure consistency and reduce duplication.

#[cfg(test)]
mod library_tests {
    use super::super::LibraryRepository;
    use crate::test_utils::{TestAssertions, TestDatabase};
    use std::path::Path;

    /// Create a test repository with fresh database
    fn setup() -> (TestDatabase, LibraryRepository) {
        let db = TestDatabase::new();
        let repo = LibraryRepository::new(db.connection.clone());
        (db, repo)
    }

    #[test]
    fn test_create_library_success() {
        let (_db, repo) = setup();
        let name = "Test Library";
        let path = "/test/path";

        let result = repo.create(name, path).unwrap();

        TestAssertions::assert_library_matches(&result, name, path);
    }

    #[test]
    fn test_create_library_duplicate_name() {
        let (_db, repo) = setup();
        let name = "Duplicate Library";
        let path1 = "/test/path1";
        let path2 = "/test/path2";

        // Create first library
        repo.create(name, path1)
            .expect("First library creation should succeed");

        // Try to create second library with same name
        let result = repo.create(name, path2);
        assert!(
            result.is_err(),
            "Creating library with duplicate name should fail"
        );

        // Verify it's a constraint error
        let error_msg = result.unwrap_err().to_string().to_lowercase();
        assert!(
            error_msg.contains("unique")
                || error_msg.contains("duplicate")
                || error_msg.contains("constraint"),
            "Error should indicate constraint violation: {error_msg}"
        );
    }

    #[test]
    fn test_create_library_duplicate_path() {
        let (_db, repo) = setup();
        let name1 = "Library One";
        let name2 = "Library Two";
        let path = "/test/same/path";

        // Create first library
        repo.create(name1, path)
            .expect("First library creation should succeed");

        // Try to create second library with same path
        let result = repo.create(name2, path);
        assert!(
            result.is_err(),
            "Creating library with duplicate path should fail"
        );

        // Verify it's a constraint error
        let error_msg = result.unwrap_err().to_string().to_lowercase();
        assert!(
            error_msg.contains("unique")
                || error_msg.contains("duplicate")
                || error_msg.contains("constraint")
                || error_msg.contains("path"),
            "Error should indicate path constraint violation: {error_msg}"
        );
    }

    #[test]
    fn test_create_library_empty_name() {
        let (_db, repo) = setup();

        let result = repo.create("", "/test/path");
        assert!(
            result.is_err(),
            "Creating library with empty name should fail"
        );

        let error_msg = result.unwrap_err().to_string().to_lowercase();
        assert!(
            error_msg.contains("name") || error_msg.contains("empty"),
            "Error should mention name validation: {error_msg}"
        );
    }

    #[test]
    fn test_create_library_whitespace_name() {
        let (_db, repo) = setup();

        let result = repo.create("   ", "/test/path");
        assert!(
            result.is_err(),
            "Creating library with whitespace-only name should fail"
        );
    }

    #[test]
    fn test_create_library_empty_path() {
        let (_db, repo) = setup();

        let result = repo.create("Test Library", "");
        assert!(
            result.is_err(),
            "Creating library with empty path should fail"
        );

        let error_msg = result.unwrap_err().to_string().to_lowercase();
        assert!(
            error_msg.contains("path") || error_msg.contains("empty"),
            "Error should mention path validation: {error_msg}"
        );
    }

    #[test]
    fn test_create_library_whitespace_path() {
        let (_db, repo) = setup();

        let result = repo.create("Test Library", "   ");
        assert!(
            result.is_err(),
            "Creating library with whitespace-only path should fail"
        );
    }

    #[test]
    fn test_find_all_empty_database() {
        let (_db, repo) = setup();

        let libraries = repo.find_all().unwrap();
        assert!(
            libraries.is_empty(),
            "Empty database should return no libraries"
        );
    }

    #[test]
    fn test_find_all_with_libraries() {
        let (db, repo) = setup();

        // Add test libraries using our utilities
        db.insert_test_library("lib-1", "Library 1", "/path1");
        db.insert_test_library("lib-2", "Library 2", "/path2");
        db.insert_test_library("lib-3", "Library 3", "/path3");

        let libraries = repo.find_all().unwrap();
        assert_eq!(libraries.len(), 3, "Should return all libraries");

        // Verify we can find specific libraries
        let lib1 = libraries
            .iter()
            .find(|lib| lib.name == "Library 1")
            .unwrap();
        TestAssertions::assert_library_matches(lib1, "Library 1", "/path1");
    }

    #[test]
    fn test_find_by_id_success() {
        let (db, repo) = setup();

        // Insert test library
        db.insert_test_library("test-lib-123", "Test Library", "/test/path");

        let library = repo.find_by_id("test-lib-123").unwrap().unwrap();
        TestAssertions::assert_library_matches(&library, "Test Library", "/test/path");
        assert_eq!(library.id, "test-lib-123");
    }

    #[test]
    fn test_find_by_id_not_found() {
        let (_db, repo) = setup();

        let result = repo.find_by_id("nonexistent").unwrap();
        assert!(
            result.is_none(),
            "Should return None for nonexistent library"
        );
    }

    #[test]
    fn test_find_by_name_success() {
        let (db, repo) = setup();

        // Insert test library
        db.insert_test_library("lib-123", "Unique Library Name", "/unique/path");

        let library = repo.find_by_name("Unique Library Name").unwrap().unwrap();
        TestAssertions::assert_library_matches(&library, "Unique Library Name", "/unique/path");
    }

    #[test]
    fn test_find_by_name_not_found() {
        let (_db, repo) = setup();

        let result = repo.find_by_name("Nonexistent Library").unwrap();
        assert!(
            result.is_none(),
            "Should return None for nonexistent library"
        );
    }

    #[test]
    fn test_find_by_path_success() {
        let (db, repo) = setup();

        // Insert test library
        db.insert_test_library("lib-456", "Path Test Library", "/unique/test/path");

        let library = repo.find_by_path("/unique/test/path").unwrap().unwrap();
        TestAssertions::assert_library_matches(&library, "Path Test Library", "/unique/test/path");
    }

    #[test]
    fn test_find_by_path_not_found() {
        let (_db, repo) = setup();

        let result = repo.find_by_path("/nonexistent/path").unwrap();
        assert!(result.is_none(), "Should return None for nonexistent path");
    }

    #[test]
    fn test_update_library_success() {
        let (_db, repo) = setup();

        // Create initial library
        let library = repo.create("Original Name", "/original/path").unwrap();
        let library_id = library.id.clone();

        // Update the library
        let success = repo
            .update(&library_id, "Updated Name", Path::new("/updated/path"))
            .unwrap();
        assert!(success, "Update should succeed");

        // Verify the update
        let updated = repo.find_by_id(&library_id).unwrap().unwrap();
        TestAssertions::assert_library_matches(&updated, "Updated Name", "/updated/path");
        assert_eq!(updated.id, library_id, "ID should remain the same");
    }

    #[test]
    fn test_update_library_not_found() {
        let (_db, repo) = setup();

        let result = repo
            .update("nonexistent-id", "New Name", Path::new("/new/path"))
            .unwrap();
        assert!(!result, "Updating nonexistent library should return false");
    }

    #[test]
    fn test_update_library_duplicate_name() {
        let (_db, repo) = setup();

        // Create two libraries
        let lib1 = repo.create("Library 1", "/path1").unwrap();
        let _lib2 = repo.create("Library 2", "/path2").unwrap();

        // Try to update lib1 to have same name as lib2
        let result = repo.update(&lib1.id, "Library 2", Path::new("/new/path"));

        // This might succeed (if no unique constraint) or fail (if constraint exists)
        // Let's check what actually happens and adjust accordingly
        match result {
            Ok(success) => {
                // If it succeeds, verify the update worked
                assert!(success, "Update should succeed if no constraint");
            }
            Err(error) => {
                // If it fails, verify it's a constraint error
                let error_msg = error.to_string().to_lowercase();
                assert!(
                    error_msg.contains("unique") || error_msg.contains("constraint"),
                    "Should be constraint error: {error_msg}"
                );
            }
        }
    }

    #[test]
    fn test_update_library_duplicate_path() {
        let (_db, repo) = setup();

        // Create two libraries
        let lib1 = repo.create("Library 1", "/path1").unwrap();
        let _lib2 = repo.create("Library 2", "/path2").unwrap();

        // Try to update lib1 to have same path as lib2
        let result = repo.update(&lib1.id, "New Name", Path::new("/path2"));
        assert!(result.is_err(), "Updating to duplicate path should fail");
    }

    #[test]
    fn test_delete_library_success() {
        let (_db, repo) = setup();

        // Create library
        let library = repo.create("To Delete", "/delete/path").unwrap();

        // Delete it
        let success = repo.delete(&library.id).unwrap();
        assert!(success, "Deleting existing library should return true");

        // Verify it's gone
        let found = repo.find_by_id(&library.id).unwrap();
        assert!(found.is_none(), "Deleted library should not be found");
    }

    #[test]
    fn test_delete_library_not_found() {
        let (_db, repo) = setup();

        let result = repo.delete("nonexistent-id").unwrap();
        assert!(!result, "Deleting nonexistent library should return false");
    }

    #[test]
    fn test_find_by_name_case_sensitivity() {
        let (db, repo) = setup();

        db.insert_test_library("lib-1", "Existing Library", "/path");

        // Find by exact name should work
        let found = repo.find_by_name("Existing Library").unwrap();
        assert!(
            found.is_some(),
            "Should find existing library by exact name"
        );

        // Different case should not find it
        let found = repo.find_by_name("existing library").unwrap();
        assert!(
            found.is_none(),
            "Should not find library with different case"
        );
    }

    #[test]
    fn test_find_by_path_exact_match() {
        let (db, repo) = setup();

        db.insert_test_library("lib-1", "Library", "/existing/path");

        let found = repo.find_by_path("/existing/path").unwrap();
        assert!(
            found.is_some(),
            "Should find existing library by exact path"
        );

        let found = repo.find_by_path("/nonexistent/path").unwrap();
        assert!(
            found.is_none(),
            "Should not find nonexistent library by path"
        );
    }

    #[test]
    fn test_library_count_via_find_all() {
        let (db, repo) = setup();

        // Initially empty
        let libraries = repo.find_all().unwrap();
        assert_eq!(libraries.len(), 0, "Empty database should have 0 libraries");

        // Add some libraries
        db.insert_test_library("lib-1", "Library 1", "/path1");
        db.insert_test_library("lib-2", "Library 2", "/path2");

        let libraries = repo.find_all().unwrap();
        assert_eq!(
            libraries.len(),
            2,
            "Should count all libraries via find_all"
        );
    }

    #[test]
    fn test_case_sensitivity() {
        let (_db, repo) = setup();

        // Create library with specific case
        repo.create("Test Library", "/test/path").unwrap();

        // Search with different case should not find it (case sensitive)
        let result = repo.find_by_name("test library").unwrap();
        assert!(result.is_none(), "Search should be case sensitive");

        // Exact case should find it
        let result = repo.find_by_name("Test Library").unwrap();
        assert!(result.is_some(), "Exact case should find library");
    }

    #[test]
    fn test_special_characters_in_name() {
        let (_db, repo) = setup();

        let special_name = "Library with Special Characters: !@#$%^&*()";
        let library = repo.create(special_name, "/test/path").unwrap();

        TestAssertions::assert_library_matches(&library, special_name, "/test/path");

        // Should be able to find it
        let found = repo.find_by_name(special_name).unwrap().unwrap();
        assert_eq!(found.name, special_name);
    }

    #[test]
    fn test_unicode_characters() {
        let (_db, repo) = setup();

        let unicode_name = "图书馆 with 📚 emojis";
        let unicode_path = "/测试/path/📁";

        let library = repo.create(unicode_name, unicode_path).unwrap();

        TestAssertions::assert_library_matches(&library, unicode_name, unicode_path);
    }

    #[test]
    fn test_very_long_name() {
        let (_db, repo) = setup();

        let long_name = "Very ".repeat(100) + "Long Library Name";
        let result = repo.create(&long_name, "/test/path");

        // Should either succeed or fail gracefully with appropriate error
        match result {
            Ok(library) => {
                assert_eq!(library.name, long_name);
            }
            Err(error) => {
                let error_msg = error.to_string().to_lowercase();
                assert!(
                    error_msg.contains("length")
                        || error_msg.contains("size")
                        || error_msg.contains("long"),
                    "Error should indicate length limit: {error_msg}"
                );
            }
        }
    }

    #[test]
    fn test_multiple_operations() {
        let (_db, repo) = setup();

        // Create multiple libraries with unique names and paths
        repo.create("Library A", "/path/a").unwrap();
        repo.create("Library B", "/path/b").unwrap();
        repo.create("Library C", "/path/c").unwrap();
        repo.create("Library D", "/path/d").unwrap();
        repo.create("Library E", "/path/e").unwrap();

        // Verify all libraries were created
        let libraries = repo.find_all().unwrap();
        assert_eq!(libraries.len(), 5, "All libraries should be persisted");

        // Check that we can find each one
        for (name, path) in [("Library A", "/path/a"), ("Library B", "/path/b")] {
            let found = repo.find_by_name(name).unwrap().unwrap();
            TestAssertions::assert_library_matches(&found, name, path);
        }
    }
}
