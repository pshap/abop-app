//! Tests for library repository search and validation operations

use super::super::LibraryRepository;
use crate::test_utils::TestDatabase;

/// Create a test repository with fresh database
fn setup() -> (TestDatabase, LibraryRepository) {
    let db = TestDatabase::new();
    let repo = LibraryRepository::new(db.connection.clone());
    (db, repo)
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
