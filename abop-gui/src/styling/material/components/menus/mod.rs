//! Material Design 3 Menu Components
//!
//! This module provides Material Design 3 menu components including dropdown menus,
//! context menus, select menus, and autocomplete that integrate with the Iced framework.
//!
//! The module is organized into separate files for each menu component type:
//! - `base_menu.rs`: Common traits and shared functionality
//! - `dropdown_menu.rs`: Standard dropdown menus
//! - `context_menu.rs`: Context menus for right-click actions
//! - `select_menu.rs`: Select/dropdown menus with a single selection
//! - `autocomplete.rs`: Text input with filtered suggestions
//! - `menu_item.rs`: Menu item components and types

mod autocomplete;
mod base_menu;
mod context_menu;
mod dropdown_menu;
mod menu_item;
mod select_menu;

// Re-export all public items
pub use autocomplete::*;
pub use base_menu::*;
pub use context_menu::*;
pub use dropdown_menu::*;
pub use menu_item::*;
pub use select_menu::*;

// Re-export helper type for backward compatibility
pub use crate::styling::material::components::menu_constants::MenuStyleValues as StyleValues;

// Helper functions for creating Material menu components
use crate::styling::material::MaterialTokens;

/// Helper functions for creating Material menu components
///
/// This implementation provides convenient factory methods for creating
/// Material Design menu components that are automatically configured
/// with the current Material Design tokens.
impl MaterialTokens {
    /// Create a Material menu with the current tokens
    #[must_use]
    pub fn menu(&self) -> MaterialMenu {
        MaterialMenu::new()
    }

    /// Create a Material context menu with the current tokens
    #[must_use]
    pub fn context_menu(&self) -> MaterialContextMenu {
        MaterialContextMenu::new()
    }

    /// Create a Material select menu with the current tokens
    #[must_use]
    pub fn select_menu<T: Clone>(&self) -> MaterialSelectMenu<T> {
        MaterialSelectMenu::new()
    }

    /// Create a Material autocomplete with the current tokens
    #[must_use]
    pub fn autocomplete<T: Clone>(&self) -> MaterialAutocomplete<T> {
        MaterialAutocomplete::new()
    }
}
