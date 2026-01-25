//! Material Design 3 Data Components
//!
//! This module implements Material Design 3 data display components including:
//! - Data tables with sorting and filtering
//! - Lists with selection support
//! - Tree views for hierarchical data
//! 
//! These components are built on top of Iced and follow Material Design 3 guidelines.

pub mod builders;
pub mod constants;
pub mod helpers;
pub mod list;
pub mod table;
pub mod tree;
pub mod types;

// Re-export commonly used types for convenience
pub use builders::*;
pub use constants::*;
pub use helpers::*;
pub use list::MaterialList;
pub use table::MaterialDataTable;
pub use tree::MaterialTreeView;
pub use types::*;