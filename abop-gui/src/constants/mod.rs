//! Constants used throughout the GUI application
//!
//! This module organizes various constants used across the GUI components
//! to maintain consistency and avoid magic values.

pub mod sort;

// Re-export commonly used constants
pub use sort::{DEFAULT_SORT_COLUMN, VALID_SORT_COLUMNS};
