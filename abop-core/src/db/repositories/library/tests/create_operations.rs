//! Tests for library repository create operations

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
