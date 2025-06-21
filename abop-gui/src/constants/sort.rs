//! Sorting constants used across the GUI
//!
//! This module contains constants related to table sorting functionality,
//! including valid column identifiers that can be used for sorting audiobooks.

/// Valid column identifiers for sorting audiobooks in tables
/// 
/// These constants define the complete set of columns that can be used
/// for sorting audiobooks in the GUI tables. Used for validation in sort
/// handlers and utilities.
/// 
/// **Sorting Behavior:**
/// - `title`, `author`, `format`, `path`, `library_id`: Lexicographic (alphabetical) sorting
/// - `duration`: Numeric sorting by duration in seconds (unknown/missing duration sorts as 0, appearing first in ascending order)
/// - `size`: Numeric sorting by file size in bytes (unknown/missing size sorts as 0, appearing first in ascending order)
/// 
/// **Unknown Value Placement:**
/// - Ascending sort: Unknown values (0) appear at the beginning
/// - Descending sort: Unknown values (0) appear at the end
/// - This ensures consistent, predictable sorting behavior for incomplete metadata
pub const VALID_SORT_COLUMNS: &[&str] = &[
    "title", 
    "author", 
    "duration",  // Numeric: sorts by duration_seconds field
    "size",      // Numeric: sorts by size_bytes field
    "format", 
    "path", 
    "library_id"
];

/// Default sort column when an invalid column is specified
pub const DEFAULT_SORT_COLUMN: &str = "title";
