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
//! All types are re-exported from this module.

// Module declarations
mod column;
mod config;
mod core;
mod typed;

// Re-export all types
pub use column::TableColumn;
pub use config::DataTableConfig;
pub use core::{
    ColumnDataType, ColumnWidth, SortDirection, SortState, TableDensity, TextAlignment,
};
pub use typed::{
    DisplayTableConfig, InteractiveTableConfig, ReadOnlyTableConfig, TypedDataTableConfig,
    VirtualTableConfig,
};
