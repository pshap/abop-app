//! Focused component tests with modern testing practices
//!
//! This module organizes tests into focused files per component, following Rust testing best practices:
//! - One test file per component for better organization
//! - Shared test utilities to reduce duplication
//! - Clear test naming conventions
//! - Comprehensive edge case coverage
//! - Property-based testing where appropriate

// Import shared test utilities
pub use crate::test_utils::*;

// Individual component test modules
pub mod about;
pub mod audio_controls;
pub mod audio_toolbar;
pub mod status;
pub mod table;

// Advanced testing utilities and configurations
pub mod test_config;

// Re-export common test dependencies to reduce import duplication
pub use crate::state::TableState;
pub use crate::styling::material::MaterialTokens;
pub use crate::theme::ThemeMode;
pub use abop_core::PlayerState;
pub use abop_core::models::audiobook::Audiobook;
pub use abop_core::scanner::ScanProgress;
pub use std::collections::HashSet;
pub use std::path::PathBuf;
