//! Tests for audiobook repository search and validation operations

use crate::db::repositories::AudiobookRepository;
use crate::test_utils::{TestDataFactory, TestDatabase};

#[test]
fn test_find_by_library() {
    let test_db = TestDatabase::new();
    let repo = AudiobookRepository::new(test_db.connection.clone());

    // Create test data with libraries
    let lib1_id = "lib1";
    let lib2_id = "lib2";
    test_db.insert_test_library(lib1_id, "Library 1", "/path/to/library1");
    test_db.insert_test_library(lib2_id, "Library 2", "/path/to/library2");

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

    // Test find_by_library with lib1
    let lib1_books = repo
        .find_by_library(lib1_id)
        .expect("find_by_library should not fail");

    // Verify only books from lib1 are returned
    assert_eq!(lib1_books.len(), 2, "Should return 2 books from lib1");
    let lib1_book_ids: Vec<_> = lib1_books.iter().map(|b| &b.id).collect();
    assert!(
        lib1_book_ids.contains(&&book1.id),
        "Book 1 should be in lib1 results"
    );
    assert!(
        lib1_book_ids.contains(&&book2.id),
        "Book 2 should be in lib1 results"
    );
}

#[test]
fn test_find_by_author() {
    let test_db = TestDatabase::new();
    let repo = AudiobookRepository::new(test_db.connection.clone());

    let lib1_id = "test-library-1";
    test_db.insert_test_library(lib1_id, "Test Library 1", "/path/to/library1");

    let book1_id = uuid::Uuid::new_v4().to_string();
    let mut book1 = TestDataFactory::custom_audiobook(
        &book1_id,
        lib1_id,
        None,
        None,
        Some(std::path::Path::new("/test/path/to/audiobook1")),
        Some(3600),
        Some(1024),
    );

    book1.author = Some("Test Author".to_string());
    repo.upsert(&book1).expect("Failed to insert book1");

    // Test find_by_author
    let author_books = repo
        .find_by_author("Test Author")
        .expect("find_by_author should not fail");

    assert_eq!(author_books.len(), 1, "Should find 1 book by author");
    assert_eq!(author_books[0].id, book1.id, "Should find correct book");
}

#[test]
fn test_delete_audiobook() {
    let test_db = TestDatabase::new();
    let repo = AudiobookRepository::new(test_db.connection.clone());

    let library_id = "test-library-1";
    test_db.insert_test_library(library_id, "Test Library", "/test/path");

    let audiobook_id = uuid::Uuid::new_v4().to_string();
    let audiobook = TestDataFactory::custom_audiobook(
        &audiobook_id,
        library_id,
        None,
        None,
        Some(std::path::Path::new("/test/path/to/audiobook")),
        Some(3600),
        Some(1024),
    );

    // Insert audiobook
    repo.upsert(&audiobook)
        .expect("Failed to insert test audiobook");

    // Test deleting audiobook
    let deleted = repo.delete(&audiobook.id).expect("delete should not fail");
    assert!(deleted, "Delete should return true for successful deletion");

    // Verify it no longer exists
    let found = repo
        .find_by_id(&audiobook.id)
        .expect("find_by_id should not fail after delete");
    assert!(
        found.is_none(),
        "Audiobook should not be found after deletion"
    );
}
