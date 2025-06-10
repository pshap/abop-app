//! Test fixtures for comprehensive chip testing
//!
//! This module provides organized test utilities, factories, and assertion helpers
//! for creating and validating chip components and collections.

pub mod chip_factory;
pub mod collection_factory;
pub mod assertion_helpers;
pub mod test_data;

// Re-export commonly used items
pub use chip_factory::*;
pub use collection_factory::*;
pub use assertion_helpers::*;
pub use test_data::*;
