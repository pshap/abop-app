//! Material Design 3 Data Components
//!
//! This module implements Material Design 3 data display components including:
//! - Data tables with sorting and filtering
//! - Lists with selection support
//! - Tree views for hierarchical data
//!
//! These components are built on top of Iced and follow Material Design 3 guidelines.
//!
//! # Usage Examples
//!
//! ## Basic Table with Builder Pattern
//!
//! ```rust
//! use abop_gui::styling::material::components::data::*;
//!
//! // Create a simple readonly table
//! let table = TableLayout::readonly()
//!     .text_column("name", "Name")
//!     .number_column("age", "Age")
//!     .date_column("created", "Created At")
//!     .build();
//!
//! // Create an interactive table with custom configuration
//! let interactive_table = TableLayout::interactive()
//!     .text_column("title", "Title")
//!     .number_column("price", "Price")
//!     .actions_column("Actions")
//!     .configure(|config| config.comfortable().with_stripes())
//!     .build();
//! ```

pub mod builders;
pub mod constants;
pub mod helpers;
pub mod list;
pub mod table;
pub mod tree;
pub mod types;

// Re-export commonly used types for convenience
pub use builders::*;
pub use helpers::*;
pub use list::MaterialList;
pub use table::MaterialDataTable;
pub use tree::MaterialTreeView;
pub use types::*;

// Legacy re-exports to maintain backward compatibility
pub use table::DataTable;
