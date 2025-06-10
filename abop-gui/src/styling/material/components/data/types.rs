//! Core types and data structures for Material Design 3 data components
//!
//! This module has been refactored into a modular structure for better organization
//! and maintainability. The types are now split across focused modules:
//!
//! - `core`: Basic enums and fundamental types
//! - `column`: Table column definitions and builders
//! - `config`: Data table configuration and builder methods
//! - `typed`: Type-safe configurations with compile-time guarantees
//!
//! All types are re-exported from this module for backward compatibility.

// Module declarations
mod core;
mod column;
mod config;
mod typed;

// Re-export all types for backward compatibility
pub use core::{SortDirection, TableDensity, TextAlignment, ColumnDataType, ColumnWidth, SortState};
pub use column::TableColumn;
pub use config::DataTableConfig;
pub use typed::{
    TypedDataTableConfig,
    ReadOnlyTableConfig,
    InteractiveTableConfig,
    VirtualTableConfig,
    DisplayTableConfig,
};
