//! Sorting constants used across the GUI
//!
//! This module contains constants related to table sorting functionality,
//! including valid column identifiers that can be used for sorting audiobooks.

/// Valid column identifiers for sorting audiobooks in tables
/// 
/// These constants define the complete set of columns that can be used
/// for sorting audiobooks in the GUI tables. Used for validation in sort
/// handlers and utilities.
pub const VALID_SORT_COLUMNS: &[&str] = &[
    "title", 
    "author", 
    "duration", 
    "size", 
    "format", 
    "path", 
    "library_id"
];

/// Default sort column when an invalid column is specified
pub const DEFAULT_SORT_COLUMN: &str = "title";
