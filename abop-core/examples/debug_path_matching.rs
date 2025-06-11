//! Debug path matching issues in library database operations
use abop_core::db::Database;
use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("🔍 Debug Path Matching");
    println!("=====================");

    // Remove existing database
    let db_path = Database::get_app_database_path()?;
    if db_path.exists() {
        std::fs::remove_file(&db_path)?;
        println!("🗑️  Removed existing database file");
    }
    // Open fresh database
    let db = Database::open_app_database()?;
    println!("✅ Opened fresh database");

    // Test database schema
    let conn = db.connect()?;
    let table_exists: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='libraries'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if table_exists == 0 {
        println!("❌ Libraries table doesn't exist! Database schema not initialized.");
        return Ok(());
    }

    println!("✅ Database schema is properly initialized");

    // Test path
    let test_path = PathBuf::from("C:\\temp\\test_validation_library");
    println!("🎯 Test path: {}", test_path.display());

    // Create library
    let library_id = db.add_library_with_path("Test Library", test_path.clone())?;
    println!("✅ Created library with ID: {library_id}");

    // Query all libraries to see what's stored
    let all_libraries = db.get_libraries()?;
    println!("📚 All libraries in database:");
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
        println!(
            "      Path components: {:?}",
            lib.path.components().collect::<Vec<_>>()
        );
    }

    // Try to find the library by path
    println!("\n🔍 Searching for library by path...");
    match db.find_library_by_path(&test_path)? {
        Some(found) => {
            println!("✅ Found library: {} (ID: {})", found.name, found.id);
        }
        None => {
            println!("❌ Library not found by path!");

            // Try different path variations
            println!("\n🔄 Trying path variations:");

            let variations = [
                test_path.clone(),
                test_path
                    .canonicalize()
                    .unwrap_or_else(|_| test_path.clone()),
                PathBuf::from("C:/temp/test_validation_library"), // Forward slashes
                PathBuf::from("c:\\temp\\test_validation_library"), // Lowercase drive
            ];

            for (i, variation) in variations.iter().enumerate() {
                println!("  {}. Trying: '{}'", i + 1, variation.display());
                match db.find_library_by_path(variation)? {
                    Some(found) => {
                        println!(
                            "    ✅ FOUND with this variation! {} (ID: {})",
                            found.name, found.id
                        );
                        break;
                    }
                    None => {
                        println!("    ❌ Not found");
                    }
                }
            }
        }
    }

    // Test direct SQL query
    println!("\n🔍 Direct SQL test:");
    let conn = db.connect()?;

    // Get all paths from database
    let mut stmt = conn.prepare("SELECT id, name, path FROM libraries")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    for row in rows {
        let (id, name, path_str) = row?;
        println!("  DB: ID={id}, Name={name}, Path='{path_str}'");

        // Compare with our test path
        let our_path_str = test_path.to_string_lossy();
        println!("  Our path string: '{our_path_str}'");
        println!("  Paths equal: {}", path_str == our_path_str);
        println!(
            "  Paths equal (case insensitive): {}",
            path_str.to_lowercase() == our_path_str.to_lowercase()
        );
    }

    Ok(())
}
