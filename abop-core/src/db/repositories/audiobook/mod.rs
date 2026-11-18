//! Audiobook repository for database operations
//!
//! This module handles all database operations related to audiobooks.

pub mod crud_operations;
pub mod queries;
pub mod search_operations;
pub mod validation_operations;

// Re-export the main repository and key types for backward compatibility
pub use crud_operations::AudiobookRepository;
pub use validation_operations::*;

#[cfg(test)]
mod tests;
