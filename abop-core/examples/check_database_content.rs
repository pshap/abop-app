//! Check Database Content
//!
//! This tool checks the content of the centralized database without modifying it.

use abop_core::db::Database;
use abop_core::error::{ErrorContext, Result};

fn main() -> Result<()> {
    println!("🔍 Checking Centralized Database Content");
    println!("=======================================");
    // Get the database path
    let db_path = Database::get_app_database_path().context("Failed to get app database path")?;

    println!("Database path: {}", db_path.display());

    if !db_path.exists() {
        println!("❌ Database file does not exist!");
        return Ok(());
    }

    // Open the database (without removing it)
    let db = Database::open_app_database().context("Failed to open centralized database")?;

    // Get all libraries
    let libraries = db.get_libraries().context("Failed to get libraries")?;

    println!("📚 Libraries in database: {}", libraries.len());
    for (i, library) in libraries.iter().enumerate() {
        println!("  {}. {} (ID: {})", i + 1, library.name, library.id);
        println!("     Path: {}", library.path.display());

        // Get audiobooks for this library
        let audiobooks = db
            .get_audiobooks_in_library(&library.id)
            .context("Failed to get audiobooks")?;

        println!("     Audiobooks: {}", audiobooks.len());
        for (j, audiobook) in audiobooks.iter().enumerate() {
            println!(
                "       {}. {}",
                j + 1,
                audiobook.title.as_deref().unwrap_or("Unknown Title")
            );
            println!(
                "          Author: {}",
                audiobook.author.as_deref().unwrap_or("Unknown")
            );
            println!("          Path: {}", audiobook.path.display());
        }
        println!();
    } // Get total stats
    let repo = db.audiobook_repository();
    let total_audiobooks = repo.find_all().map_err(|e| {
        abop_core::error::AppError::Other(format!("Failed to get all audiobooks: {}", e))
    })?;

    println!("📊 Total Statistics:");
    println!("   Libraries: {}", libraries.len());
    println!("   Audiobooks: {}", total_audiobooks.len());

    Ok(())
}
