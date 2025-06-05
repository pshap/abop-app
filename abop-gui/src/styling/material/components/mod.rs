//! Material Design 3 Components
//!
//! This module provides Material Design 3 component implementations that integrate
//! seamlessly with the Iced framework and ABOP's existing architecture.

/// Material Design 3 button styling system for centralized button styling
pub mod button_style;
/// Material Design 3 container components including cards, surfaces, dividers, and layout containers
pub mod containers;
/// Material Design 3 data components including tables, lists, and tree views
pub mod data;
/// Material Design 3 feedback components including progress indicators, badges, dialogs, and notifications
pub mod feedback;
/// Material Design 3 input components including text fields and form elements
pub mod inputs;
/// Material Design 3 menu constants for centralized menu component values
pub mod menu_constants;
/// Material Design 3 menu container styling system for centralized menu container styling
pub mod menu_container_style;
/// Material Design 3 menu item styling system for consistent menu button styling
pub mod menu_item_style;
/// Material Design 3 menu components including dropdown menus, context menus, and autocomplete
pub mod menus;
/// Material Design 3 navigation components - modularized with tab bars and breadcrumbs for audiobook apps
pub mod navigation;
/// Material Design 3 selection components including checkboxes, radio buttons, switches, and chips
pub mod selection;
/// Material Design 3 selection component styling system for centralized selection styling
pub mod selection_style;
/// Phase 3: Complete Material Design 3 widget implementations as proper Iced widgets
pub mod widgets;

// Re-export specific items from each module to avoid ambiguity
pub use button_style::{
    ButtonColors, ButtonSizeVariant, ButtonStyleVariant, ButtonStyling, create_button_icon,
    create_button_style, get_button_size_properties, get_button_styling, get_icon_size_for_button,
};
// ButtonSize is now exported from widgets module
pub use containers::*;
pub use data::*;
pub use feedback::*;
pub use inputs::*;
pub use menu_constants::*;
pub use menu_container_style::*;
pub use menu_item_style::*;
pub use menus::*;
pub use navigation::*;
pub use selection::*;
pub use selection_style::*;
pub use widgets::MaterialButton;
pub use widgets::{ButtonSize, IconPosition, MaterialButtonVariant};

// Note: feedback::* is already re-exported above via `pub use feedback::*;`
// Individual re-exports are redundant and have been removed to eliminate warnings
