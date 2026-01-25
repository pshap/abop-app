//! Tests for audiobook repository create/read/update operations

use super::*;
use crate::db::repositories::AudiobookRepository;
use crate::test_utils::{TestDataFactory, TestDatabase};

#[test]
fn test_upsert_new_audiobook() {
    let test_db = TestDatabase::new();
    let repo = AudiobookRepository::new(test_db.connection.clone());

    let library_id = "test-library-1";
    test_db.insert_test_library(library_id, "Test Library", "/test/path");

    let audiobook_id = uuid::Uuid::new_v4().to_string();
    let mut audiobook = TestDataFactory::custom_audiobook(
        &audiobook_id,
        library_id,
        Some("Test Audiobook"),
        Some("Test Author"),
        Some(std::path::Path::new("/test/path/to/audiobook")),
        Some(3600),
        Some(1024 * 1024 * 100),
    );

    // Test inserting a new audiobook
    let result = repo.upsert(&audiobook);
    assert!(result.is_ok(), "Should insert new audiobook: {result:?}");

    // Test updating audiobook
    audiobook.title = Some("Updated Title".to_string());
    let result = repo.upsert(&audiobook);
    assert!(
        result.is_ok(),
        "Should update existing audiobook: {result:?}"
    );

    // Verify update was successful
    let found = repo
        .find_by_id(&audiobook.id)
        .expect("Should find audiobook");
    assert_eq!(found.unwrap().title, Some("Updated Title".to_string()));
}

#[test]
fn test_find_audiobook_by_id() {
    let test_db = TestDatabase::new();
    let repo = AudiobookRepository::new(test_db.connection.clone());

    let library_id = "test-library-1";
    test_db.insert_test_library(library_id, "Test Library", "/test/path");

    let audiobook_id = uuid::Uuid::new_v4().to_string();
    let audiobook = TestDataFactory::custom_audiobook(
        &audiobook_id,
        library_id,
        Some("Test Audiobook"),
        Some("Test Author"),
        Some(std::path::Path::new("/test/path/to/audiobook")),
        Some(3600),
        Some(1024 * 1024),
    );

    // Insert test data
    repo.upsert(&audiobook)
        .expect("Failed to insert test audiobook");

    // Test finding existing audiobook
    let found = repo
        .find_by_id(&audiobook.id)
        .expect("Failed to find audiobook");
    assert!(found.is_some(), "Should find existing audiobook");
    assert_eq!(found.unwrap().id, audiobook.id);

    // Test finding non-existent audiobook
    let not_found = repo
        .find_by_id("non-existent-id")
        .expect("Query should succeed");
    assert!(
        not_found.is_none(),
        "Should not find non-existent audiobook"
    );
}

#[test]
fn test_find_all_audiobooks_empty() {
    let test_db = TestDatabase::new();
    let repo = AudiobookRepository::new(test_db.connection.clone());

    // Test find_all on empty database
    let result = repo
        .find_all()
        .expect("find_all should not fail on empty database");

    assert!(
        result.is_empty(),
        "Should return empty vector for empty database"
    );
}

#[test]
fn test_find_all_audiobooks_multiple() {
    let test_db = TestDatabase::new();
    let repo = AudiobookRepository::new(test_db.connection.clone());

    // Insert libraries for used library_ids
    let lib1_id = "test-library-1";
    let lib2_id = "test-library-2";
    test_db.insert_test_library(lib1_id, "Test Library 1", "/path/to/library1");
    test_db.insert_test_library(lib2_id, "Test Library 2", "/path/to/library2");

    // Create test data with libraries
    let book1_id = uuid::Uuid::new_v4().to_string();
    let book2_id = uuid::Uuid::new_v4().to_string();
    let book3_id = uuid::Uuid::new_v4().to_string();

    let book1 = TestDataFactory::custom_audiobook(
        &book1_id,
        lib1_id,
        None,
        None,
        Some(std::path::Path::new("/test/path/to/audiobook1")),
        Some(3600),
        Some(1024),
    );
    let book2 = TestDataFactory::custom_audiobook(
        &book2_id,
        lib1_id,
        None,
        None,
        Some(std::path::Path::new("/test/path/to/audiobook2")),
        Some(3600),
        Some(1024),
    );
    let book3 = TestDataFactory::custom_audiobook(
        &book3_id,
        lib2_id,
        None,
        None,
        Some(std::path::Path::new("/test/path/to/audiobook3")),
        Some(3600),
        Some(1024),
    );

    // Insert test data
    repo.upsert(&book1).expect("Failed to insert book1");
    repo.upsert(&book2).expect("Failed to insert book2");
    repo.upsert(&book3).expect("Failed to insert book3");

    // Test find_all
    let all_books = repo.find_all().expect("find_all should not fail");

    // Verify all books are returned
    assert_eq!(all_books.len(), 3, "Should return all 3 books");
    let book_ids: Vec<_> = all_books.iter().map(|b| &b.id).collect();
    assert!(book_ids.contains(&&book1.id), "Book 1 should be in results");
    assert!(book_ids.contains(&&book2.id), "Book 2 should be in results");
    assert!(book_ids.contains(&&book3.id), "Book 3 should be in results");
}
