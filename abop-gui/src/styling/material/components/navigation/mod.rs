//! Material Design 3 Navigation Components
//!
//! This module provides Material Design 3 navigation components optimized for
//! audiobook organizing and processing applications, including tab bars and
//! breadcrumbs for content organization and hierarchical navigation.
//!
//! ## Components
//! - **TabBar**: Content organization tabs for switching between different views
//! - **Breadcrumbs**: Hierarchical navigation for folder structures and collections
//!
//! ## Design Goals
//! - Streamlined navigation focused on audiobook app use cases
//! - Material Design 3 compliance with proper elevation and styling
//! - Efficient navigation through audiobook collections and hierarchies
//! - Clean separation of concerns with modular component structure

/// Tab bar component for content organization and view switching
pub mod tab_bar;

/// Breadcrumbs component for hierarchical navigation
pub mod breadcrumbs;

/// Helper functions and utilities for navigation components
pub mod helpers;

// Re-export main components for convenience
pub use breadcrumbs::{BreadcrumbItem, MaterialBreadcrumbs};
pub use helpers::*;
pub use tab_bar::{MaterialTabBar, Tab};
