//! Search functionality for audiobooks
//!
//! This module provides search capabilities for finding audiobooks
//! based on various criteria including title, author, narrator, and more.

pub mod engine;

// Re-export commonly used types
pub use engine::SearchEngine;
