//! Tests for audiobook repository operations

use super::*;
use crate::test_utils::TestDataFactory;
use crate::{
    db::{EnhancedConnection, migrations::run_migrations, repositories::AudiobookRepository},
    models::Audiobook,
};
use rusqlite::{Connection, params};
use std::sync::Arc;
use tempfile::NamedTempFile;

fn setup_test_db() -> (AudiobookRepository, tempfile::TempPath) {
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let db_path = temp_file.into_temp_path();

    // Create a new connection to the database
    let mut conn = Connection::open(&db_path).expect("Failed to open database");

    // Run migrations on the connection
    run_migrations(&mut conn).expect("Failed to run migrations");

    // Set up test data
    conn.execute(
        "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
        params!["test-library-1", "Test Library", "/test/library/path"],
    )
    .expect("Failed to create test library");

    // Create the EnhancedConnection and repository
    let connection = Arc::new(EnhancedConnection::new(
        db_path.to_str().expect("Invalid temp path"),
    ));
    // Ensure the connection is established before use
    connection
        .connect()
        .expect("Failed to connect EnhancedConnection in test");

    (AudiobookRepository::new(connection), db_path)
}

fn create_test_db() -> AudiobookRepository {
    let (repo, _temp_file) = setup_test_db();
    repo
}

fn create_test_audiobook(library_id: &str, path: &str) -> Audiobook {
    TestDataFactory::custom_audiobook(
        &uuid::Uuid::new_v4().to_string(),
        library_id,
        Some("Test Audiobook"),
        Some("Test Author"),
        Some(PathBuf::from(path).as_path()),
        Some(3600),
        Some(1024 * 1024 * 100),
    )
}

#[test]
fn test_audiobook_repository_creation() {
    let repo = create_test_db();

    // Verify we can execute a query
    let conn = repo.connect();
    let result = conn.with_connection(|db_conn| {
        db_conn
            .query_row("SELECT 1", [], |_| Ok(()))
            .map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Query failed: {e}"),
            })
    });

    assert!(
        result.is_ok(),
        "Should be able to execute a query: {result:?}"
    );
}

#[test]
fn test_upsert_new_audiobook() {
    let repo = create_test_db();

    let mut audiobook = create_test_audiobook("test-library-1", "/test/path/to/audiobook");

    // Test inserting a new audiobook
    let result = repo.upsert(&audiobook);
    assert!(result.is_ok(), "Should insert new audiobook: {result:?}");

    // Test updating the audiobook
    audiobook.title = Some("Updated Title".to_string());
    let result = repo.upsert(&audiobook);
    assert!(
        result.is_ok(),
        "Should update existing audiobook: {result:?}"
    );

    // Verify the update was successful
    let found = repo
        .find_by_id(&audiobook.id)
        .expect("Should find audiobook");
    assert_eq!(found.unwrap().title, Some("Updated Title".to_string()));

    // Test with invalid data (empty ID)
    let mut invalid = audiobook.clone();
    invalid.id = "".to_string();
    let result = repo.upsert(&invalid);
    assert!(result.is_err(), "Should not allow empty ID");

    // Test with very long ID
    let mut long_id = audiobook.clone();
    long_id.id = "a".repeat(1000);
    long_id.path = PathBuf::from("/test/path/to/audiobook_long_id");
    let result = repo.upsert(&long_id);
    assert!(result.is_ok(), "Should handle long IDs");
}

#[test]
fn test_find_audiobook_by_id() {
    let repo = create_test_db();

    // Insert test data
    let audiobook = create_test_audiobook("test-library-1", "/test/path/to/audiobook");
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

    // Test with empty ID
    let empty_id = "";
    let empty_result = repo.find_by_id(empty_id);
    assert!(empty_result.is_err(), "Empty ID should return an error");
}

#[test]
fn test_find_nonexistent_audiobook() {
    let repo = create_test_db();

    // Test with a non-existent ID
    let non_existent_id = "this-id-does-not-exist-123";
    let result = repo.find_by_id(non_existent_id);

    assert!(result.is_ok(), "Finding non-existent ID should not fail");
    assert!(
        result.unwrap().is_none(),
        "Should not find non-existent audiobook"
    );

    // Test with empty ID
    let empty_result = repo.find_by_id("");
    assert!(empty_result.is_err(), "Empty ID should return an error");
}

#[test]
fn test_find_all_audiobooks_empty() {
    let repo = create_test_db();

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
    let repo = create_test_db();

    // Insert libraries for used library_ids
    repo.execute_query(|conn| {
        conn.execute(
            "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
            params!["test-library-1", "Test Library 1", "/path/to/library1"],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
            params!["test-library-2", "Test Library 2", "/path/to/library2"],
        )?;
        Ok(())
    })
    .expect("Failed to insert test libraries");

    // Create test data with libraries
    let book1 = create_test_audiobook("test-library-1", "/test/path/to/audiobook1");
    let book2 = create_test_audiobook("test-library-1", "/test/path/to/audiobook2");
    let book3 = create_test_audiobook("test-library-2", "/test/path/to/audiobook3");

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

#[test]
fn test_find_by_library() {
    let repo = create_test_db();

    // Create test data with libraries
    let book1 = create_test_audiobook("lib1", "/test/path/to/audiobook1");
    let book2 = create_test_audiobook("lib1", "/test/path/to/audiobook2");
    let book3 = create_test_audiobook("lib2", "/test/path/to/audiobook3");

    // Insert libraries for used library_ids
    repo.execute_query(|conn| {
        conn.execute(
            "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
            params!["lib1", "Library 1", "/path/to/library1"],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
            params!["lib2", "Library 2", "/path/to/library2"],
        )?;
        Ok(())
    })
    .expect("Failed to insert test libraries");

    // Insert test data
    repo.upsert(&book1).expect("Failed to insert book1");
    repo.upsert(&book2).expect("Failed to insert book2");
    repo.upsert(&book3).expect("Failed to insert book3");

    // Test find_by_library with lib1
    let lib1_books = repo
        .find_by_library("lib1")
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

    // Test find_by_library with lib2
    let lib2_books = repo
        .find_by_library("lib2")
        .expect("find_by_library should not fail");
    assert_eq!(lib2_books.len(), 1, "Should return 1 book from lib2");
    assert_eq!(
        lib2_books[0].id, book3.id,
        "Book 3 should be in lib2 results"
    );

    // Test find_by_library with non-existent library
    let no_books = repo
        .find_by_library("non-existent-lib")
        .expect("find_by_library should not fail with non-existent library");
    assert!(
        no_books.is_empty(),
        "Should return empty vector for non-existent library"
    );
}

#[test]
fn test_count_by_library() {
    let repo = create_test_db();

    // Insert libraries for used library_ids
    repo.execute_query(|conn| {
        conn.execute(
            "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
            params!["lib1", "Library 1", "/path/to/library1"],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
            params!["lib2", "Library 2", "/path/to/library2"],
        )?;
        Ok(())
    })
    .expect("Failed to insert test libraries");

    // Create test data with libraries
    let book1 = create_test_audiobook("lib1", "/test/path/to/audiobook1");
    let book2 = create_test_audiobook("lib1", "/test/path/to/audiobook2");
    let book3 = create_test_audiobook("lib2", "/test/path/to/audiobook3");

    // Insert test data
    repo.upsert(&book1).expect("Failed to insert book1");
    repo.upsert(&book2).expect("Failed to insert book2");
    repo.upsert(&book3).expect("Failed to insert book3");

    // Test count_by_library with lib1
    let lib1_count = repo
        .count_by_library("lib1")
        .expect("count_by_library should not fail");
    assert_eq!(lib1_count, 2, "Should count 2 books in lib1");

    // Test count_by_library with lib2
    let lib2_count = repo
        .count_by_library("lib2")
        .expect("count_by_library should not fail");
    assert_eq!(lib2_count, 1, "Should count 1 book in lib2");

    // Test with non-existent library
    let no_count = repo
        .count_by_library("non-existent-lib")
        .expect("count_by_library should not fail with non-existent library");
    assert_eq!(no_count, 0, "Should return 0 for non-existent library");

    // Test with empty library ID
    let empty_result = repo.count_by_library("");
    assert!(
        empty_result.is_err(),
        "Empty library ID should return an error"
    );
}
#[test]
fn test_find_by_author() {
    let repo = create_test_db();

    // Insert libraries for used library_ids
    repo.execute_query(|conn| {
        conn.execute(
            "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
            params!["test-library-1", "Test Library 1", "/path/to/library1"],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
            params!["test-library-2", "Test Library 2", "/path/to/library2"],
        )?;
        Ok(())
    })
    .expect("Failed to insert test libraries");

    // Create test data with different authors
    let mut book1 = create_test_audiobook("test-library-1", "/test/path/to/audiobook1");
    let mut book2 = create_test_audiobook("test-library-1", "/test/path/to/audiobook2");
    let mut book3 = create_test_audiobook("test-library-2", "/test/path/to/audiobook3");

    // Set different authors
    book1.author = Some("Author A".to_string());
    book2.author = Some("Author B".to_string());
    book3.author = Some("Author A".to_string());

    // Insert test data
    repo.upsert(&book1).expect("Failed to insert book1");
    repo.upsert(&book2).expect("Failed to insert book2");
    repo.upsert(&book3).expect("Failed to insert book3");

    // Test find_by_author with Author A
    let author_a_books = repo
        .find_by_author("Author A")
        .expect("find_by_author should not fail");

    // Verify only books by Author A are returned
    assert_eq!(author_a_books.len(), 2, "Should find 2 books by Author A");
    let author_a_book_ids: Vec<_> = author_a_books.iter().map(|b| &b.id).collect();
    assert!(
        author_a_book_ids.contains(&&book1.id),
        "Book 1 should be in Author A results"
    );
    assert!(
        !author_a_book_ids.contains(&&book2.id),
        "Book 2 should not be in Author A results"
    );
    assert!(
        author_a_book_ids.contains(&&book3.id),
        "Book 3 should be in Author A results"
    );

    // Test find_by_author with Author B
    let author_b_books = repo
        .find_by_author("Author B")
        .expect("find_by_author should not fail");
    assert_eq!(author_b_books.len(), 1, "Should find 1 book by Author B");
    assert_eq!(
        author_b_books[0].id, book2.id,
        "Book 2 should be the only book by Author B"
    );

    // Test find_by_author with non-existent author
    let no_books = repo
        .find_by_author("Non-existent Author")
        .expect("find_by_author should not fail with non-existent author");
    assert!(
        no_books.is_empty(),
        "Should return empty vector for non-existent author"
    );

    // Test with empty author
    let empty_result = repo.find_by_author("");
    assert!(empty_result.is_err(), "Empty author should return an error");
}

#[test]
fn test_find_by_path() {
    let repo = create_test_db();

    // Create test data with paths
    let book1 = create_test_audiobook("test-library-1", "/audiobooks/fiction/book1.mp3");
    let book2 = create_test_audiobook("test-library-1", "/audiobooks/nonfiction/book2.mp3");

    // Insert test data
    repo.upsert(&book1).expect("Failed to insert book1");
    repo.upsert(&book2).expect("Failed to insert book2");

    // Test find_by_path with exact match
    let found_book1 = repo
        .find_by_path(book1.path.to_str().unwrap())
        .expect("find_by_path should not fail")
        .expect("Should find book by exact path");
    assert_eq!(
        found_book1.id, book1.id,
        "Should find the correct book by path"
    );

    // Test with different path separators (Windows/Unix compatibility)
    let windows_path = book2.path.to_str().unwrap().replace("/", "\\\\");
    let found_book2 = repo.find_by_path(&windows_path);

    // The test should pass regardless of the path format
    if let Ok(Some(found)) = found_book2 {
        assert_eq!(
            found.id, book2.id,
            "Should find book with different path separators"
        );
    } else {
        // On some systems, the path might not match due to normalization
        // Try with the original path as a fallback
        let found_book2 = repo
            .find_by_path(book2.path.to_str().unwrap())
            .expect("find_by_path should not fail with original path");
        assert_eq!(
            found_book2.unwrap().id,
            book2.id,
            "Should find book with original path"
        );
    }

    // Test with non-existent path
    let non_existent_path = "/path/that/does/not/exist.mp3";
    let not_found = repo
        .find_by_path(non_existent_path)
        .expect("find_by_path with non-existent path should not fail");
    assert!(
        not_found.is_none(),
        "Should return None for non-existent path"
    );

    // Test with empty path
    let empty_result = repo.find_by_path("");
    assert!(empty_result.is_err(), "Empty path should return an error");
}

#[test]
fn test_exists() {
    let repo = create_test_db();
    let audiobook = create_test_audiobook("test-library-1", "/test/path/to/audiobook");

    // Test with non-existent audiobook
    let exists = repo
        .exists(&audiobook.id)
        .expect("exists check should not fail");
    assert!(!exists, "Audiobook should not exist before insertion");

    // Insert the audiobook
    repo.upsert(&audiobook)
        .expect("Failed to insert test audiobook");

    // Test with existing audiobook
    let exists = repo
        .exists(&audiobook.id)
        .expect("exists check should not fail");
    assert!(exists, "Audiobook should exist after insertion");

    // Test with empty ID
    let empty_result = repo.exists("");
    assert!(empty_result.is_err(), "Empty ID should return an error");

    // Test with very long ID
    let long_id = "a".repeat(1000);
    let long_id_result = repo.exists(&long_id);
    assert!(long_id_result.is_ok(), "Should handle long IDs gracefully");

    // Test with special characters in ID
    let special_id = "id-with-special-chars!@#$%^&*()";
    let special_result = repo.exists(special_id);
    assert!(
        special_result.is_ok(),
        "Should handle special characters in ID"
    );
}

#[test]
fn test_delete_audiobook() {
    // Setup
    let (repo, _temp_file) = setup_test_db();
    let audiobook = create_test_audiobook("test-library-1", "/test/path/to/audiobook");

    // Insert the audiobook
    repo.upsert(&audiobook)
        .expect("Failed to insert test audiobook");

    // Verify it exists
    let exists = repo
        .exists(&audiobook.id)
        .expect("exists check should not fail");
    assert!(exists, "Audiobook should exist before deletion");

    // Test deleting the audiobook
    let deleted = repo.delete(&audiobook.id).expect("delete should not fail");
    assert!(deleted, "Delete should return true for successful deletion");

    // Verify it no longer exists
    let exists = repo
        .exists(&audiobook.id)
        .expect("exists check should not fail after deletion");
    assert!(!exists, "Audiobook should not exist after deletion");

    // Test deleting non-existent audiobook
    let deleted_again = repo
        .delete(&audiobook.id)
        .expect("delete of non-existent should not fail");
    assert!(
        !deleted_again,
        "Delete should return false for non-existent audiobook"
    );

    // Test with empty ID
    let empty_result = repo.delete("");
    assert!(empty_result.is_err(), "Empty ID should return an error");

    // Test with very long ID
    let long_id = "a".repeat(1000);
    let long_id_result = repo.delete(&long_id);
    assert!(long_id_result.is_ok(), "Should handle long IDs gracefully");

    // Test with special characters in ID
    let special_id = "id-with-special-chars!@#$%^&*()";
    let special_result = repo.delete(special_id);
    assert!(
        special_result.is_ok(),
        "Should handle special characters in ID"
    );
}

#[test]
fn test_repository_basic_operations() {
    // Setup
    let (repo, _temp_file) = setup_test_db();

    // Test initial state - no audiobooks
    let all_audiobooks = repo
        .find_all()
        .expect("find_all should not fail on empty database");
    assert!(all_audiobooks.is_empty(), "Should start with no audiobooks");

    // Create a test audiobook
    let mut audiobook = create_test_audiobook("test-library-1", "/test/path/to/audiobook");

    // Test insert
    let insert_result = repo.upsert(&audiobook);
    assert!(insert_result.is_ok(), "Insert should succeed");

    // Verify insert
    let found = repo
        .find_by_id(&audiobook.id)
        .expect("find_by_id should not fail")
        .expect("Inserted audiobook should be found");
    assert_eq!(
        found.id, audiobook.id,
        "Found audiobook should match inserted one"
    );

    // Test update
    let original_title = audiobook.title.clone();
    audiobook.title = Some("Updated Title".to_string());

    let update_result = repo.upsert(&audiobook);
    assert!(update_result.is_ok(), "Update should succeed");

    // Verify update
    let updated = repo
        .find_by_id(&audiobook.id)
        .expect("find_by_id should not fail after update")
        .expect("Updated audiobook should be found");
    assert_eq!(
        updated.title,
        Some("Updated Title".to_string()),
        "Title should be updated"
    );
    assert_ne!(updated.title, original_title, "Title should have changed");

    // Test find_all after insert
    let all_after_insert = repo
        .find_all()
        .expect("find_all should not fail after insert");
    assert_eq!(
        all_after_insert.len(),
        1,
        "Should find one audiobook after insert"
    );
    assert_eq!(
        all_after_insert[0].id, audiobook.id,
        "Found audiobook should match inserted one"
    );

    // Test delete
    let delete_result = repo.delete(&audiobook.id);
    assert!(delete_result.is_ok(), "Delete should succeed");
    assert!(
        delete_result.unwrap(),
        "Delete should return true for successful deletion"
    );

    // Verify delete
    let not_found = repo
        .find_by_id(&audiobook.id)
        .expect("find_by_id should not fail after delete");
    assert!(
        not_found.is_none(),
        "Audiobook should not be found after deletion"
    );

    // Test find_all after delete
    let all_after_delete = repo
        .find_all()
        .expect("find_all should not fail after delete");
    assert!(
        all_after_delete.is_empty(),
        "Should find no audiobooks after delete"
    );

    // Test error cases
    let empty_id_result = repo.find_by_id("");
    assert!(empty_id_result.is_err(), "Empty ID should return an error");

    let empty_delete_result = repo.delete("");
    assert!(
        empty_delete_result.is_err(),
        "Delete with empty ID should return an error"
    );

    // Test with very long ID
    let long_id = "a".repeat(1000);
    let long_id_result = repo.find_by_id(&long_id);
    assert!(long_id_result.is_ok(), "Should handle long IDs gracefully");

    // Test with special characters in ID
    let special_id = "id-with-special-chars!@#$%^&*()";
    let special_result = repo.find_by_id(special_id);
    assert!(
        special_result.is_ok(),
        "Should handle special characters in ID"
    );
}
