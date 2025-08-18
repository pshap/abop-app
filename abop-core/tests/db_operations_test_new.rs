use abop_core::db::{
    DatabaseError,
    connection::EnhancedConnection,
    repositories::{AudiobookRepository, LibraryRepository},
};
use abop_core::models::Audiobook;
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tempfile::NamedTempFile;

/// Test data factory for integration tests
struct TestDataFactory;

impl TestDataFactory {
    /// Create an Audiobook with customizable metadata
    fn custom_audiobook(
        id: &str,
        library_id: &str,
        title: Option<&str>,
        author: Option<&str>,
        path: Option<&Path>,
        duration_seconds: Option<u64>,
        size_bytes: Option<u64>,
    ) -> Audiobook {
        let now = Utc::now();
        Audiobook {
            id: id.to_string(),
            library_id: library_id.to_string(),
            path: path
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("/test/path/audiobook.mp3")),
            title: title.map(|s| s.to_string()),
            author: author.map(|s| s.to_string()),
            narrator: None,
            description: None,
            duration_seconds,
            size_bytes,
            cover_art: None,
            created_at: now,
            updated_at: now,
            selected: false,
        }
    }
}

/// Set up a test database with all migrations applied
fn setup_test_db() -> Arc<EnhancedConnection> {
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let db_path = temp_file.into_temp_path();

    // Create and connect to the database
    let enhanced_conn = EnhancedConnection::new(&db_path);
    enhanced_conn
        .connect()
        .expect("Failed to connect to database");

    // Run migrations using the connection's with_connection method
    enhanced_conn
        .with_connection_mut(|conn| -> Result<(), DatabaseError> {
            // Create the migrations table if it doesn't exist
            conn.execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                applied INTEGER NOT NULL DEFAULT 1,
                applied_at TIMESTAMP
            )"#,
            )?;

            // Execute the initial schema (includes all necessary columns and migrations)
            conn.execute_batch(include_str!("../src/db/migrations/001_initial_schema.sql"))?;

            Ok(())
        })
        .expect("Failed to run migrations");

    Arc::new(enhanced_conn)
}

/// Helper function to create a test library
fn create_test_library(repo: &LibraryRepository, name: &str, path: &'static str) -> String {
    let library = repo
        .create(name, Path::new(path))
        .expect("Failed to create test library");
    library.id
}

#[test]
fn test_basic_database_connection() {
    let conn = setup_test_db();

    // Simple test to verify the database connection works
    let result = conn.with_connection(|conn| {
        let mut stmt = conn.prepare("SELECT 1")?;
        let result: i32 = stmt.query_row([], |row| row.get(0))?;
        assert_eq!(result, 1);
        Ok::<_, DatabaseError>(())
    });
    assert!(
        result.is_ok(),
        "Should be able to execute SQL on the database"
    );
}

#[test]
fn test_library_repository() {
    let conn = setup_test_db();
    let repo = LibraryRepository::new(conn);

    // Test creating a library
    let library_name = "Test Library";
    let library_path = "/test/path";
    let library_id = create_test_library(&repo, library_name, library_path);

    // Verify library was created
    let library = repo
        .find_by_id(&library_id)
        .expect("Failed to find library")
        .expect("Library should exist");

    assert_eq!(library.name, library_name);
    assert_eq!(library.path, PathBuf::from(library_path));

    // Test finding all libraries
    let libraries = repo.find_all().expect("Failed to find all libraries");
    assert!(!libraries.is_empty(), "Should find at least one library");
    assert!(
        libraries.iter().any(|l| l.id == library_id),
        "Should find the created library"
    );

    // Clean up
    repo.delete(&library_id)
        .expect("Failed to clean up test library");
}

#[test]
fn test_library_operations() {
    let conn = setup_test_db();
    let repo = LibraryRepository::new(conn);

    // Test creating a library
    let library_id = create_test_library(&repo, "Test Library", "/test/path");

    // Verify library was created
    let library = repo
        .find_by_id(&library_id)
        .expect("Failed to find library")
        .unwrap();
    assert_eq!(library.name, "Test Library");
    assert_eq!(library.path, PathBuf::from("/test/path"));

    // Test finding all libraries
    let libraries = repo.find_all().expect("Failed to find all libraries");
    assert!(!libraries.is_empty(), "Should find at least one library");
    assert!(
        libraries.iter().any(|l| l.id == library_id),
        "Should find the created library"
    );

    // Test updating the library
    let new_name = "Updated Library";
    let new_path = "/updated/path";

    // Update the library
    let update_result = repo.update(&library_id, new_name, Path::new(new_path));
    assert!(update_result.is_ok(), "Update operation should succeed");

    // Verify the update
    let updated_lib = repo
        .find_by_id(&library_id)
        .expect("Failed to find library after update")
        .expect("Library should exist after update");

    assert_eq!(updated_lib.name, new_name);
    assert_eq!(updated_lib.path, PathBuf::from(new_path));

    // Test deleting the library
    let delete_result = repo.delete(&library_id);
    assert!(delete_result.is_ok(), "Delete operation should succeed");

    // Verify deletion
    let deleted_lib = repo
        .find_by_id(&library_id)
        .expect("Failed to query library after deletion");
    assert!(deleted_lib.is_none(), "Library should be deleted");

    assert_eq!(updated_lib.name, new_name, "Library name should be updated");
    assert_eq!(
        updated_lib.path,
        PathBuf::from(new_path),
        "Library path should be updated"
    );

    // Test deleting the library
    let delete_result = repo.delete(&library_id);
    assert!(delete_result.is_ok(), "Delete operation should succeed");

    // Verify the library was deleted
    let found = repo
        .find_by_id(&library_id)
        .expect("Failed to query library after delete");
    assert!(found.is_none(), "Library should be deleted");
}

/// Helper function to create a test audiobook
fn create_test_audiobook(
    repo: &AudiobookRepository,
    library_id: &str,
    id: &str,
    title: &str,
    path: &str,
) -> String {
    let audiobook = TestDataFactory::custom_audiobook(
        id,
        library_id,
        Some(title),
        Some("Test Author"),
        Some(Path::new(path)),
        Some(3600),
        Some(1024 * 1024),
    );

    repo.upsert(&audiobook)
        .expect("Failed to create test audiobook");
    id.to_string()
}

#[test]
fn test_audiobook_operations() {
    // Setup
    let conn = setup_test_db();
    let lib_repo = LibraryRepository::new(Arc::clone(&conn));
    let audio_repo = AudiobookRepository::new(Arc::clone(&conn));

    // Use a path that works on both Unix and Windows
    let test_library_path = if cfg!(windows) {
        "C:\\temp\\abop_test_lib"
    } else {
        "/tmp/abop_test_lib"
    };

    // Ensure the directory exists
    std::fs::create_dir_all(test_library_path).expect("Failed to create test directory");

    let library_id = create_test_library(&lib_repo, "Audiobook Test Library", test_library_path);

    // Create a test audio file path
    let audio_path = Path::new(test_library_path).join("test_audio.mp3");
    std::fs::File::create(&audio_path).expect("Failed to create test audio file");
    let audio_path = audio_path
        .to_str()
        .expect("Path not valid UTF-8")
        .to_string();

    // Test creating an audiobook
    let audio_id = create_test_audiobook(
        &audio_repo,
        &library_id,
        "test-audio-1",
        "Test Audiobook",
        &audio_path,
    );

    // Verify audiobook was created
    let audiobook = audio_repo
        .find_by_id(&audio_id)
        .expect("Failed to find audiobook")
        .expect("Audiobook not found");

    assert_eq!(audiobook.title, Some("Test Audiobook".to_string()));
    assert_eq!(audiobook.library_id, library_id);

    // Test finding by library
    let library_books = audio_repo
        .find_by_library(&library_id)
        .expect("Failed to find audiobooks by library");

    assert!(!library_books.is_empty());
    assert!(library_books.iter().any(|b| b.id == audio_id));

    // Test updating the audiobook
    let mut updated = audiobook;
    updated.title = Some("Updated Title".to_string());
    audio_repo
        .upsert(&updated)
        .expect("Failed to update audiobook");

    // Verify update
    let updated_audio = audio_repo.find_by_id(&audio_id).unwrap().unwrap();
    assert_eq!(updated_audio.title, Some("Updated Title".to_string()));

    // Test finding by author
    let author_books = audio_repo
        .find_by_author("Test Author")
        .expect("Failed to find audiobooks by author");
    assert!(!author_books.is_empty());
    assert!(author_books.iter().any(|b| b.id == audio_id));

    // Test finding by path
    let found_by_path = audio_repo
        .find_by_path(&audio_path)
        .expect("Failed to find audiobook by path");
    assert!(found_by_path.is_some());
    assert_eq!(found_by_path.unwrap().id, audio_id);

    // Test counting by library
    let count = audio_repo
        .count_by_library(&library_id)
        .expect("Failed to count audiobooks by library");
    assert!(count > 0, "Should count at least one audiobook");

    // Test deleting the audiobook
    audio_repo
        .delete(&audio_id)
        .expect("Failed to delete audiobook");
    let deleted = audio_repo.find_by_id(&audio_id).unwrap();
    assert!(deleted.is_none(), "Audiobook should be deleted");
}
