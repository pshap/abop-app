//! Constants used throughout the GUI application
//!
//! This module organizes various constants used across the GUI components
//! to maintain consistency and avoid magic values.

pub mod sort;

// Re-export commonly used constants
pub use sort::{VALID_SORT_COLUMNS, DEFAULT_SORT_COLUMN};
