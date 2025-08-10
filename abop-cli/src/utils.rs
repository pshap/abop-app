//! Shared utilities for ABOP CLI
//!
//! This module contains utility functions used across different command
//! implementations, including database queries and result formatting.

use crate::error::{CliResult, CliResultExt};
use abop_core::{
    db::Database,
    models::audiobook::fallbacks::{UNKNOWN_AUTHOR, UNKNOWN_TITLE},
};
use anyhow::Context;
use log::{info, warn};

/// Get the total count of audiobooks across all libraries
///
/// # Arguments
/// * `db` - Database connection
///
/// # Returns
/// The total number of audiobooks, or an error if the query fails
pub fn get_audiobook_count(db: &Database) -> CliResult<usize> {
    // Get all libraries first
    let libraries = db.get_libraries()
        .with_database_context("fetching libraries")?;

    if libraries.is_empty() {
        return Ok(0);
    }

    // Sum across all libraries for total count
    let mut total_count = 0;
    for library in &libraries {
        let count = db.count_audiobooks_in_library(library.id.as_str())
            .with_database_context("counting audiobooks")?;
        total_count += count;
    }
    
    Ok(total_count)
}

/// Display scan results with audiobook count and examples
///
/// # Arguments
/// * `db` - Database connection
///
/// # Errors
/// Returns an error if database queries fail
pub fn show_scan_results(db: &Database) -> CliResult<()> {
    let count = get_audiobook_count(db)?;

    if count == 0 {
        warn!("No audiobooks found in the library");
        return Ok(());
    }

    info!("📚 Total audiobooks found: {count}");

    // Show sample audiobooks
    show_sample_audiobooks(db)?;
    
    Ok(())
}

/// Display a comprehensive list of all audiobooks in the database
///
/// # Arguments
/// * `db` - Database connection
///
/// # Errors
/// Returns an error if database operations fail
pub fn show_audiobook_list(db: &Database) -> CliResult<()> {
    // Get all libraries first
    let libraries = db.get_libraries()
        .with_database_context("fetching libraries")?;
    
    if libraries.is_empty() {
        info!("No libraries found in database. You may need to scan a library first.");
        return Ok(());
    }

    // Use the first available library (we already checked libraries is not empty)
    let library_id = libraries.first().unwrap().id.as_str();

    let total_count = db
        .count_audiobooks_in_library(library_id)
        .with_database_context("counting audiobooks")?;

    if total_count == 0 {
        info!("No audiobooks found in database. Try scanning a library directory first.");
        return Ok(());
    }

    // Configure pagination to avoid memory issues
    let page_size = get_pagination_size();
    info!("Found {total_count} audiobooks in library {library_id} (page_size={page_size})");

    show_paginated_audiobooks(db, library_id, total_count, page_size)?;
    
    Ok(())
}

/// Show sample audiobooks from the database
fn show_sample_audiobooks(db: &Database) -> CliResult<()> {
    // Get libraries to show audiobook examples
    let libraries = db.get_libraries()
        .with_database_context("fetching libraries for samples")?;
    
    if libraries.is_empty() {
        return Ok(());
    }

    let library_id = libraries
        .first()
        .unwrap() // Safe: we checked !libraries.is_empty() above
        .id
        .as_str();

    // Show first few audiobooks as examples
    let sample_audiobooks = db
        .get_audiobooks_in_library_paginated(library_id, Some(5), 0)
        .with_database_context("fetching sample audiobooks")?;
    
    let total_count = db
        .count_audiobooks_in_library(library_id)
        .with_database_context("counting total audiobooks")?;

    if !sample_audiobooks.is_empty() {
        info!("Sample audiobooks:");
        for (i, book) in sample_audiobooks.iter().enumerate() {
            info!(
                "  {}. {} - {}",
                i + 1,
                book.title.as_deref().unwrap_or(UNKNOWN_TITLE),
                book.author.as_deref().unwrap_or(UNKNOWN_AUTHOR)
            );
        }

        if total_count > 5 {
            info!("  ... and {} more", total_count - 5);
        }
    }

    Ok(())
}

/// Display paginated list of audiobooks
fn show_paginated_audiobooks(
    db: &Database,
    library_id: &str,
    total_count: usize,
    page_size: usize,
) -> CliResult<()> {
    let mut offset = 0;
    let mut displayed = 0;

    while offset < total_count {
        log::debug!("Loading audiobooks with offset: {offset}, limit: {page_size}");
        
        let audiobooks = db
            .get_audiobooks_in_library_paginated(library_id, Some(page_size), offset)
            .context("Failed to get paginated audiobooks")?;

        for book in audiobooks {
            displayed += 1;
            println!(
                "{}. {} - {} ({})",
                displayed,
                book.title.as_deref().unwrap_or(UNKNOWN_TITLE),
                book.author.as_deref().unwrap_or(UNKNOWN_AUTHOR),
                book.path.display()
            );
        }

        offset += page_size;
    }

    Ok(())
}

/// Get pagination size from environment or use default
fn get_pagination_size() -> usize {
    std::env::var("ABOP_PAGE_SIZE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(100)
        .clamp(1, 1000) // Ensure minimum of 1 and cap at reasonable maximum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pagination_size() {
        // Test default value (when env var is not set)
        // We can't safely remove env vars in tests, so we test with known values
        
        // Test clamping logic by directly testing the parsing
        let test_cases = vec![
            ("50", 50),      // Valid value
            ("0", 1),        // Too low, clamped to 1
            ("2000", 1000),  // Too high, clamped to 1000
            ("invalid", 100), // Invalid, falls back to default
        ];
        
        for (input, expected) in test_cases {
            let parsed = input.parse::<usize>().ok().unwrap_or(100).clamp(1, 1000);
            assert_eq!(parsed, expected, "Input '{}' should parse to {}", input, expected);
        }
        
        // Test that function returns a reasonable default
        let default_page_size = get_pagination_size();
        assert!((1..=1000).contains(&default_page_size), "Page size should be between 1 and 1000, got {}", default_page_size);
    }

    #[test]
    fn test_fallback_constants_available() {
        // Test that fallback constants are available and not empty
        assert!(!UNKNOWN_TITLE.is_empty());
        assert!(!UNKNOWN_AUTHOR.is_empty());
        assert_eq!(
            UNKNOWN_TITLE,
            abop_core::models::audiobook::fallbacks::UNKNOWN_TITLE
        );
        assert_eq!(
            UNKNOWN_AUTHOR,
            abop_core::models::audiobook::fallbacks::UNKNOWN_AUTHOR
        );
    }

}