//! Tests for library repository read operations

use super::super::LibraryRepository;
use crate::test_utils::{TestAssertions, TestDatabase};

/// Create a test repository with fresh database
fn setup() -> (TestDatabase, LibraryRepository) {
    let db = TestDatabase::new();
    let repo = LibraryRepository::new(db.connection.clone());
    (db, repo)
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
