//! Windows-specific platform implementations
//!
//! This module provides Windows-specific functionality including:
//! - Environment variable utilities for safe manipulation
//! - Path handling that respects Windows path conventions
//! - OS-specific behavior implementations

pub mod env_utils;
pub mod path_utils;

// Re-export for convenience
pub use env_utils::*;
pub use path_utils::*;
