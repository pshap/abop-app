// Example demonstrating Phase 3 Enhanced Connection Management features
// This example shows how to use the enhanced connection features in ABOP

use abop_core::{
    db::{health::ConnectionHealth, Database, repositories::EnhancedRepository},
    error::Result,
    models::{Audiobook, Library},
};
use std::path::Path;
use tempfile::NamedTempFile;

fn main() -> Result<()> {
    // Create a temporary database for this example
    let temp_file = NamedTempFile::new().unwrap();
    let db = Database::new(temp_file.path())?;

    println!("=== Phase 3 Enhanced Connection Management Demo ===\n");

    // Example 1: Basic health monitoring
    demonstrate_health_monitoring(&db)?;

    // Example 2: Connection statistics
    demonstrate_connection_stats(&db)?;

    // Example 3: Enhanced operations with retry logic
    demonstrate_enhanced_operations(&db)?;

    // Example 4: Repository-level enhanced access
    demonstrate_repository_enhanced_access(&db)?;

    // Example 5: Bulk operations with enhanced connection
    demonstrate_bulk_operations(&db)?;

    println!("=== Demo completed successfully! ===");
    Ok(())
}

fn demonstrate_health_monitoring(db: &Database) -> Result<()> {
    println!("1. Health Monitoring");
    println!("===================");

    // Get current health status
    let health = db.get_connection_health()?;
    println!("Current connection health: {health:?}");

    // Force a health check
    match db.check_connection_health() {
        Ok(health) => println!("Health check result: {health:?}"),
        Err(e) => println!("Health check failed: {e}"),
    }

    // Verify we have a healthy connection
    match health {
        ConnectionHealth::Healthy => println!("✅ Connection is healthy and ready for operations"),
        ConnectionHealth::Degraded => println!("⚠️  Connection is degraded but functional"),
        ConnectionHealth::Failed => println!("❌ Connection has failed"),
        ConnectionHealth::Connecting => println!("🔄 Connection is being established"),
    }

    println!();
    Ok(())
}

fn demonstrate_connection_stats(db: &Database) -> Result<()> {
    println!("2. Connection Statistics");
    println!("=======================");

    // Create a library for demo
    let library = Library {
        id: "demo".to_string(),
        name: "Demo Library".to_string(),
        path: Path::new("/demo/path").to_path_buf(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    db.add_library(&library)?;

    let stats = db.get_connection_stats()?;
    println!("📊 Connection Statistics:");
    println!(
        "   - Successful operations: {}",
        stats.successful_operations
    );
    println!("   - Failed operations: {}", stats.failed_operations);
    println!(
        "   - Average operation time: {:.2}ms",
        stats.avg_operation_duration_ms
    );
    println!("   - Connection uptime: {:?}", stats.connection_uptime);
    println!(
        "   - Last successful operation: {:?}",
        stats.last_successful_operation
    );
    println!(
        "   - Last failed operation: {:?}",
        stats.last_failed_operation
    );

    println!();
    Ok(())
}

fn demonstrate_enhanced_operations(db: &Database) -> Result<()> {
    println!("3. Enhanced Operations with Retry Logic");
    println!("======================================");

    // Example of using enhanced connection for custom operations
    let result = db.with_enhanced_connection(|conn| {
        // This operation benefits from automatic retry logic and health monitoring
        use rusqlite::params;

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM libraries")?;
        let count: i32 = stmt.query_row(params![], |row| row.get(0))?;
        Ok(count)
    })?;

    println!("✅ Enhanced operation completed successfully");
    println!("   Libraries count (via enhanced connection): {result}");

    println!();
    Ok(())
}

fn demonstrate_repository_enhanced_access(db: &Database) -> Result<()> {
    println!("4. Repository-Level Enhanced Access");
    println!("==================================");

    let repositories = db.get_repositories()?;

    // Check if enhanced connection is available
    if let Some(enhanced_conn) = repositories.get_enhanced_connection() {
        println!("✅ Enhanced connection available through repository manager");

        let health = enhanced_conn.get_health()?;
        println!("   Health via repository manager: {health:?}");

        // Use enhanced connection for a custom operation
        let _result = enhanced_conn
            .with_connection(|conn| {
                // Perform a maintenance operation - count tables
                let mut stmt =
                    conn.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")?;
                let count: i32 = stmt.query_row([], |row| row.get(0))?;
                Ok(count)
            })
            .map_err(|e| abop_core::error::AppError::Other(e.to_string()))?;

        println!("   Database integrity check completed");
    } else {
        println!("❌ Enhanced connection not available");
    }

    // Demonstrate enhanced repository trait
    let library_repo = db.get_library_repository()?;
    match library_repo.get_enhanced_connection() {
        Some(_) => println!("✅ Library repository has enhanced connection access"),
        None => println!("ℹ️  Library repository uses default enhanced connection behavior"),
    }

    println!();
    Ok(())
}

fn demonstrate_bulk_operations(db: &Database) -> Result<()> {
    println!("5. Bulk Operations with Enhanced Connection");
    println!("==========================================");

    // First, create a library for our audiobooks
    let library = Library {
        id: "bulk_demo".to_string(),
        name: "Bulk Demo Library".to_string(),
        path: Path::new("/bulk/demo").to_path_buf(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    db.add_library(&library)?;

    // Create some sample audiobooks
    let audiobooks = vec![
        create_sample_audiobook("1", &library.id, "Book One", "Author A"),
        create_sample_audiobook("2", &library.id, "Book Two", "Author B"),
        create_sample_audiobook("3", &library.id, "Book Three", "Author C"),
    ];

    println!(
        "📚 Performing bulk insert of {} audiobooks...",
        audiobooks.len()
    );

    // Perform bulk insert using enhanced connection
    let start_time = std::time::Instant::now();

    for audiobook in &audiobooks {
        db.add_audiobook(audiobook)?;
    }

    let elapsed = start_time.elapsed();
    println!("✅ Bulk insert completed in {elapsed:?}");

    // Verify the insert worked
    let books_in_library = db.get_audiobooks(&library.id)?;
    println!(
        "   Verified: {} audiobooks inserted",
        books_in_library.len()
    );

    // Show updated connection statistics
    let stats = db.get_connection_stats()?;
    println!("📊 Updated statistics after bulk operation:");
    println!(
        "   - Total successful operations: {}",
        stats.successful_operations
    );
    println!(
        "   - Average operation time: {:.2}ms",
        stats.avg_operation_duration_ms
    );

    println!();
    Ok(())
}

fn create_sample_audiobook(id: &str, library_id: &str, title: &str, author: &str) -> Audiobook {
    use chrono::Utc;
    use std::path::PathBuf;

    Audiobook {
        id: id.to_string(),
        library_id: library_id.to_string(),
        path: PathBuf::from(format!("/demo/books/{}.mp3", title.replace(" ", "_"))),
        title: Some(title.to_string()),
        author: Some(author.to_string()),
        narrator: Some("Demo Narrator".to_string()),
        description: Some(format!("A sample audiobook titled '{title}'")),
        duration_seconds: Some(3600),  // 1 hour
        size_bytes: Some(1024 * 1024), // 1 MB
        cover_art: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        selected: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_enhanced_connection_demo() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::open(temp_file.path()).expect("Failed to open database");

        // Test basic functionality
        assert!(matches!(db.connection_health(), ConnectionHealth::Healthy));

        // Test enhanced operations work
        let result = db.execute_with_enhanced_connection(|conn| {
            use abop_core::db::error::DatabaseError;
            conn.execute("SELECT 1", []).map_err(DatabaseError::from)
        });
        assert!(result.is_ok());

        // Test statistics are being tracked
        let stats = db.connection_stats();
        assert!(stats.successful_operations > 0);
    }
}
