//! Modular container styling system for ABOP GUI
//!
//! This module provides a comprehensive container styling system organized into logical
//! categories for better maintainability and code organization. Each submodule focuses
//! on a specific category of container styles with consistent design token usage.
//!
//! ## Usage
//!
//! Import the specific container style module you need:
//!
//! ```
//! // Example styles for containers - this is a documentation illustration only
//! // In real code, these would be used with proper Iced container instances
//! use abop_gui::styling::container::{LayoutContainerStyles, DialogContainerStyles};
//! use abop_gui::theme::ThemeMode;
//!
//! let theme_mode = ThemeMode::Light;
//!
//! // Create the styles (this part works independently of containers)
//! let content_style = LayoutContainerStyles::content(theme_mode);
//! let modal_style = DialogContainerStyles::modal(theme_mode);
//! ```
//!
//! ## Available Modules
//!
//! - `base` - Basic container styles (card, panel, primary, etc.)
//! - `dialog` - Modal and overlay styles (modal, dropdown, tooltip, etc.)
//! - `feedback` - Status feedback styles (success, warning, error, info)
//! - `layout` - Layout container styles (header, content, sidebar)
//! - `table` - Table-specific styles (table, headers, rows)

pub mod base;
pub mod dialog;
pub mod feedback;
pub mod layout;

// Re-export all container styling functionality
pub use base::{BaseContainerStyles, BaseContainerType, ContainerStyle};
pub use dialog::{DialogContainerStyles, DialogContainerType};
pub use feedback::{FeedbackContainerStyles, FeedbackContainerType};
pub use layout::{LayoutContainerStyles, LayoutContainerType};
