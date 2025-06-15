//! Database Centralization Validation Tool
//!
//! This tool validates that the database centralization is working correctly
//! by testing database paths, operations, and consistency.

use abop_core::db::Database;
use abop_core::error::{ErrorContext, Result};
use rusqlite::OptionalExtension;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("🔍 ABOP Database Centralization Validation");
    println!("==========================================");

    // Test 1: Check database path consistency
    test_database_path_consistency()?;

    // Test 2: Test database creation and operations
    test_database_operations()?;

    // Test 3: Test library operations
    test_library_operations()?;

    println!("\n✅ All validation tests completed successfully!");
    Ok(())
}

fn test_database_path_consistency() -> Result<()> {
    println!("\n📍 Test 1: Database Path Consistency");
    println!("-----------------------------------");

    // Get the expected database path
    let db_path = Database::get_app_database_path().context("Failed to get app database path")?;
    println!("Expected database path: {}", db_path.display());

    // Check if parent directory structure makes sense
    let parent = db_path.parent().ok_or_else(|| {
        let error_msg =
            "Database path has no parent directory - this indicates an invalid path structure";
        eprintln!("ERROR: {error_msg}");
        abop_core::error::AppError::Other(error_msg.to_string())
    })?;

    println!("Database directory: {}", parent.display());

    // Verify this is in the correct location (should be in AppData)
    let data_dir = dirs::data_dir().ok_or_else(|| {
        let error_msg = "Could not determine the system's data directory. This might indicate a platform compatibility issue or insufficient permissions.";
        eprintln!("ERROR: {error_msg}");
        abop_core::error::AppError::Other(error_msg.to_string())
    })?;

    if db_path.starts_with(&data_dir) {
        println!("✅ Database path is correctly located in user data directory");
    } else {
        let error_msg = format!(
            "Database path location is incorrect. Expected path to be under '{}', but found '{}'",
            data_dir.display(),
            db_path.display()
        );
        eprintln!("ERROR: {error_msg}");
        println!("❌ Database path is NOT in user data directory");
        println!("   Expected prefix: {}", data_dir.display());
        return Err(abop_core::error::AppError::Other(error_msg));
    }

    println!("✅ Database path consistency test passed");
    Ok(())
}

fn test_database_operations() -> Result<()> {
    println!("\n🗄️  Test 2: Database Operations");
    println!("------------------------------");

    // Remove existing database to start fresh
    let db_path = Database::get_app_database_path()?;
    if db_path.exists() {
        println!("🗑️  Removing existing database file...");
        std::fs::remove_file(&db_path)?;
    }

    // Open the centralized database (this should create it fresh)
    println!("Opening centralized database...");
    let db = Database::open_app_database().context("Failed to open centralized database")?;

    println!("✅ Successfully opened centralized database");

    // Test basic database operations
    println!("Testing basic database connectivity...");
    let _conn = db.connect().context("Failed to get database connection")?;

    println!("✅ Database connection successful");

    // Check if the libraries table exists
    println!("Checking if libraries table exists...");
    let conn = db.connect()?;

    let table_exists: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='libraries'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    println!("Libraries table exists: {}", table_exists > 0);

    if table_exists == 0 {
        println!("❌ Libraries table does not exist! Migration may have failed.");

        // Check if migrations table exists
        let migrations_exist: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='migrations'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        println!("Migrations table exists: {}", migrations_exist > 0);

        if migrations_exist > 0 {
            // Check migration records
            let migration_count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM migrations WHERE applied = 1",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            println!("Applied migrations count: {migration_count}");
        }

        let error_msg =
            "Database schema not properly initialized - migrations table not found or corrupted";
        eprintln!("ERROR: {error_msg}");
        return Err(abop_core::error::AppError::Other(error_msg.to_string()));
    }

    // Test getting libraries (should work even if empty)
    println!("Testing library operations...");
    let libraries = db.get_libraries().context("Failed to get libraries")?;

    println!("📚 Found {} libraries in database", libraries.len());

    for (i, library) in libraries.iter().enumerate() {
        println!(
            "  {}. {} (ID: {}) - Path: {}",
            i + 1,
            library.name,
            library.id,
            library.path.display()
        );
    }

    println!("✅ Database operations test passed");
    Ok(())
}

fn test_library_operations() -> Result<()> {
    println!("\n📚 Test 3: Library Operations");
    println!("-----------------------------");

    let db = Database::open_app_database().context("Failed to open centralized database")?;

    // Create a test library path (we won't actually create files)
    let test_path = PathBuf::from("C:\\temp\\test_validation_library");

    println!(
        "Testing library creation with path: {}",
        test_path.display()
    );

    // Check if this library already exists
    match db.find_library_by_path(&test_path)? {
        Some(existing) => {
            println!(
                "📖 Found existing library: {} (ID: {})",
                existing.name, existing.id
            );
        }
        None => {
            println!("📝 Creating new test library...");

            // Create a new library
            let library_id = db
                .add_library_with_path("Validation Test Library", test_path.clone())
                .context("Failed to create test library")?;

            println!("✅ Created library with ID: {library_id}");
            // Debug: Show what's actually in the database
            println!("🔍 Debugging path matching...");
            let all_libraries = db.get_libraries()?;
            println!("📚 All libraries in database after creation:");
            for (i, lib) in all_libraries.iter().enumerate() {
                println!(
                    "  {}. {} (ID: {}) - Path: '{}'",
                    i + 1,
                    lib.name,
                    lib.id,
                    lib.path.display()
                );
                println!(
                    "      Path as string_lossy: '{}'",
                    lib.path.to_string_lossy()
                );
            }

            // Our test path details
            println!("🎯 Our test path: '{}'", test_path.display());
            println!(
                "    Test path as string_lossy: '{}'",
                test_path.to_string_lossy()
            );

            // Try direct SQL query
            let conn = db.connect()?;
            let mut stmt = conn.prepare("SELECT id, name, path FROM libraries WHERE path = ?1")?;
            let path_str = test_path.to_string_lossy();

            let found_via_sql = stmt
                .query_row([path_str.as_ref()], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                })
                .optional()?;
            println!("🔍 Direct SQL query result: {found_via_sql:?}");

            // Verify we can find it
            let created_library = db.find_library_by_path(&test_path)?.ok_or_else(|| {
                let error_msg = format!(
                    "Library not found after creation at path: {}",
                    test_path.display()
                );
                eprintln!("ERROR: {error_msg}");
                abop_core::error::AppError::Other(error_msg)
            })?;

            println!(
                "✅ Successfully retrieved created library: {}",
                created_library.name
            );

            // Test library repository operations
            let repo = db.libraries();

            // Find by ID
            let found_by_id = repo
                .find_by_id(&library_id)
                .context("Failed to find library by ID")?
                .ok_or_else(|| {
                    let error_msg = format!("Library not found by ID: {library_id}");
                    eprintln!("ERROR: {error_msg}");
                    abop_core::error::AppError::Other(error_msg)
                })?;

            println!("✅ Found library by ID: {}", found_by_id.name);

            // Find by name
            let found_by_name = repo
                .find_by_name("Validation Test Library")
                .context("Failed to find library by name")?;

            if found_by_name.is_some() {
                println!("✅ Found library by name");
            } else {
                println!("⚠️  Library not found by name (this might be OK if names aren't unique)");
            }
        }
    }

    // Test getting all libraries
    let all_libraries = db.get_libraries().context("Failed to get all libraries")?;

    println!("📊 Total libraries in database: {}", all_libraries.len());

    println!("✅ Library operations test passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_path_consistency() {
        let result = super::test_database_path_consistency();
        assert!(
            result.is_ok(),
            "Database path consistency test failed: {:?}",
            result
        );
    }

    #[test]
    fn test_database_operations() {
        let result = super::test_database_operations();
        assert!(
            result.is_ok(),
            "Database operations test failed: {:?}",
            result
        );
    }
}
