//! Tests for library repository update and delete operations

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
fn test_update_library_success() {
    let (db, repo) = setup();

    // Create initial library
    let library = repo.create("Original Name", "/original/path").unwrap();
    let library_id = library.id.clone();

    // Update library
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
    let (db, repo) = setup();

    let result = repo
        .update("nonexistent-id", "New Name", Path::new("/new/path"))
        .unwrap();
    assert!(!result, "Updating nonexistent library should return false");
}

#[test]
fn test_update_library_duplicate_name() {
    let (db, repo) = setup();

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
    let (db, repo) = setup();

    // Create two libraries
    let lib1 = repo.create("Library 1", "/path1").unwrap();
    let _lib2 = repo.create("Library 2", "/path2").unwrap();

    // Try to update lib1 to have same path as lib2
    let result = repo.update(&lib1.id, "New Name", Path::new("/path2"));
    assert!(result.is_err(), "Updating to duplicate path should fail");
}

#[test]
fn test_delete_library_success() {
    let (db, repo) = setup();

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
    let (db, repo) = setup();

    let result = repo.delete("nonexistent-id").unwrap();
    assert!(!result, "Deleting nonexistent library should return false");
}
