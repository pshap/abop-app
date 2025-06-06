//! Tests for database operations: saving, updating, retrieving audiobooks, and error handling.

// TODO: Import actual database functions and models when available.

#[cfg(test)]
mod db_operations_tests {
    use abop_core::config::Config;
    use abop_core::db::Database;
    use abop_core::models::Audiobook;
    // External crates for tests
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::{Connection, params};

    use std::fs;
    use std::thread;
    use tempfile::tempdir;
    #[test]
    fn test_save_single_audiobook() {
        use chrono::Utc;
        use std::path::PathBuf;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();

        // Create a library first
        let library = db.add_library("Test Library", "/tmp/library").unwrap();

        let ab = Audiobook {
            id: "test1".to_string(),
            library_id: library.id.clone(),
            path: PathBuf::from("/tmp/test1.mp3"),
            title: Some("Test Book".to_string()),
            author: Some("Test Author".to_string()),
            narrator: Some("Test Narrator".to_string()),
            description: Some("A test audiobook".to_string()),
            duration_seconds: Some(123),
            size_bytes: Some(1024),
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        };
        db.add_audiobook(&ab).unwrap();
        let loaded = db
            .get_audiobook("test1")
            .unwrap()
            .expect("Audiobook not found");
        assert_eq!(loaded.title, ab.title);
        assert_eq!(loaded.author, ab.author);
        assert_eq!(loaded.duration_seconds, ab.duration_seconds);
        assert_eq!(loaded.path, ab.path);
        assert!(!loaded.selected);
    }
    #[test]
    fn test_save_multiple_audiobooks() {
        use chrono::Utc;
        use std::path::PathBuf;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_multi.db");
        let db = Database::open(&db_path).unwrap();

        // Create a library first
        let library = db
            .add_library("Multi Test Library", "/tmp/multi_library")
            .unwrap();

        let ab1 = Audiobook {
            id: "ab1".to_string(),
            library_id: library.id.clone(),
            path: PathBuf::from("/tmp/ab1.mp3"),
            title: Some("Book One".to_string()),
            author: Some("Author A".to_string()),
            narrator: Some("Test Narrator".to_string()),
            description: Some("First book".to_string()),
            duration_seconds: Some(100),
            size_bytes: Some(2048),
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        };
        let ab2 = Audiobook {
            id: "ab2".to_string(),
            library_id: library.id.clone(),
            path: PathBuf::from("/tmp/ab2.mp3"),
            title: Some("Book Two".to_string()),
            author: Some("Author B".to_string()),
            narrator: Some("Test Narrator".to_string()),
            description: Some("Second book".to_string()),
            duration_seconds: Some(200),
            size_bytes: Some(4096),
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        };
        db.add_audiobook(&ab1).unwrap();
        db.add_audiobook(&ab2).unwrap();
        let all = db.audiobooks().find_all().unwrap();
        assert!(all.iter().any(|a| a.id == ab1.id && a.title == ab1.title));
        assert!(all.iter().any(|a| a.id == ab2.id && a.title == ab2.title));
    }
    #[test]
    fn test_update_existing_audiobook() {
        use chrono::Utc;
        use std::path::PathBuf;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_update.db");
        let db = Database::open(&db_path).unwrap();

        // Create a library first
        let library = db
            .add_library("Update Test Library", "/tmp/update_library")
            .unwrap();

        let mut ab = Audiobook {
            id: "ab1".to_string(),
            library_id: library.id.clone(),
            path: PathBuf::from("/tmp/ab1.mp3"),
            title: Some("Original Title".to_string()),
            author: Some("Original Author".to_string()),
            narrator: Some("Test Narrator".to_string()),
            description: Some("Original description".to_string()),
            duration_seconds: Some(100),
            size_bytes: Some(2048),
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        };
        db.add_audiobook(&ab).unwrap();
        // Update fields
        ab.title = Some("Updated Title".to_string());
        ab.author = Some("Updated Author".to_string());
        ab.updated_at = Utc::now();
        db.add_audiobook(&ab).unwrap();
        let loaded = db
            .get_audiobook("ab1")
            .unwrap()
            .expect("Audiobook not found");
        assert_eq!(loaded.title, Some("Updated Title".to_string()));
        assert_eq!(loaded.author, Some("Updated Author".to_string()));
    }
    #[test]
    fn test_retrieve_audiobooks_with_filters() {
        use chrono::Utc;
        use std::path::PathBuf;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_filter.db");
        let db = Database::open(&db_path).unwrap();

        // Create a library first
        let library = db
            .add_library("Filter Test Library", "/tmp/filter_library")
            .unwrap();

        let ab1 = Audiobook {
            id: "ab1".to_string(),
            library_id: library.id.clone(),
            path: PathBuf::from("/tmp/ab1.mp3"),
            title: Some("Book One".to_string()),
            author: Some("Author A".to_string()),
            narrator: Some("Test Narrator".to_string()),
            description: Some("First book".to_string()),
            duration_seconds: Some(100),
            size_bytes: Some(2048),
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        };
        let ab2 = Audiobook {
            id: "ab2".to_string(),
            library_id: library.id.clone(),
            path: PathBuf::from("/tmp/ab2.m4b"),
            title: Some("Book Two".to_string()),
            author: Some("Author B".to_string()),
            narrator: Some("Test Narrator".to_string()),
            description: Some("Second book".to_string()),
            duration_seconds: Some(200),
            size_bytes: Some(4096),
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        };
        db.add_audiobook(&ab1).unwrap();
        db.add_audiobook(&ab2).unwrap();
        let all = db.audiobooks().find_all().unwrap();
        let by_author_a: Vec<_> = all
            .iter()
            .filter(|a| a.author == Some("Author A".to_string()))
            .collect();
        assert_eq!(by_author_a.len(), 1);
        assert_eq!(by_author_a[0].id, ab1.id);
        // Example: filter by duration
        let by_duration: Vec<_> = all
            .iter()
            .filter(|a| a.duration_seconds == Some(200))
            .collect();
        assert_eq!(by_duration.len(), 1);
        assert_eq!(by_duration[0].id, ab2.id);
    }
    #[test]
    fn test_special_characters_in_db_entries() {
        use chrono::Utc;
        use std::path::PathBuf;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_special.db");
        let db = Database::open(&db_path).unwrap();

        // Create a library first
        let library = db
            .add_library("Special Chars Library", "/tmp/special_library")
            .unwrap();

        let ab = Audiobook {
            id: "ab1".to_string(),
            library_id: library.id.clone(),
            path: PathBuf::from("/tmp/ab1.mp3"),
            title: Some("Böök: 漢字 & Emoji 🚀".to_string()),
            author: Some("Äuthor/作者".to_string()),
            narrator: Some("Test Narrator".to_string()),
            description: Some("Special chars".to_string()),
            duration_seconds: Some(100),
            size_bytes: Some(2048),
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        };
        db.add_audiobook(&ab).unwrap();
        let loaded = db
            .get_audiobook("ab1")
            .unwrap()
            .expect("Audiobook not found");
        assert_eq!(loaded.title, ab.title);
        assert_eq!(loaded.author, ab.author);
    }

    #[test]
    fn test_load_application_settings() {
        // TODO: Implement test for loading settings
    }

    #[test]
    fn test_default_settings_when_none_exist() {
        // TODO: Implement test for default settings
    }

    #[test]
    fn test_handling_corrupted_settings_data() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        fs::write(&config_path, b"not valid json").unwrap();
        let result = serde_json::from_str::<Config>(&fs::read_to_string(&config_path).unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_database_connection_failures() {
        let dir = tempdir().unwrap();
        let result = Connection::open(dir.path());
        assert!(
            result.is_err(),
            "Expected error when opening a directory as a database"
        );
    }

    #[test]
    fn test_transaction_rollbacks() {
        // Create in-memory DB and schema
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT NOT NULL)",
            [],
        )
        .unwrap();
        // Start transaction
        let tx = conn.transaction().unwrap();
        tx.execute("INSERT INTO test (value) VALUES (?1)", params!["ok"])
            .unwrap();
        // Simulate error (e.g., NOT NULL violation)
        let result = tx.execute(
            "INSERT INTO test (value) VALUES (?1)",
            params![Option::<String>::None],
        );
        assert!(result.is_err());
        // Rollback
        drop(tx); // Not committed, so should rollback
        // Check that no rows were inserted
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_concurrent_database_access() {
        // Setup connection pool
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).unwrap();
        // Create schema
        {
            let conn = pool.get().unwrap();
            conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT)", [])
                .unwrap();
        }
        // Spawn threads to insert data concurrently
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let pool = pool.clone();
                thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    conn.execute("INSERT INTO test (value) VALUES (?1)", [format!("val{i}")])
                        .unwrap();
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        // Check row count
        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 5);
    }
}
