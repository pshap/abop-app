//! Material Design 3 Menu Constants
//!
//! This module provides centralized constants for menu component dimensions,
//! spacing, and other hardcoded values to ensure consistency across all menu components.

use crate::styling::material::MaterialTokens;
use iced::{Color, border::Radius};

/// Trait for creating consistent `StyleValues` across menu components
pub trait StyleValuesProvider {
    /// Create style values from material tokens
    fn style_values(&self, tokens: &MaterialTokens) -> MenuStyleValues;
}

/// Centralized style values for menu components
#[derive(Debug, Clone)]
pub struct MenuStyleValues {
    /// Color for text and icons on surface variants, used for secondary content
    pub on_surface_variant: Color,
    /// Primary text color for content displayed on surface backgrounds
    pub on_surface: Color,
    /// Highest elevation surface container color in the Material Design color system
    pub surface_container_highest: Color,
    /// Outline color for borders, dividers, and component boundaries
    pub outline: Color,
    /// Primary brand color used for key interactive elements
    pub primary: Color,
    /// Background color for secondary containers and selected states
    pub secondary_container: Color,
    /// Text and icon color for content on secondary container backgrounds
    pub on_secondary_container: Color,
    /// Border radius value for rounded corners on menu components
    pub corner_radius: Radius,
}

impl MenuStyleValues {
    /// Create style values from material tokens
    #[must_use]
    pub const fn from_tokens(tokens: &MaterialTokens) -> Self {
        let colors = &tokens.colors;
        let shapes = &tokens.shapes;

        Self {
            on_surface_variant: colors.on_surface_variant,
            on_surface: colors.on_surface,
            surface_container_highest: colors.surface_container_highest,
            outline: colors.outline,
            primary: colors.primary.base,
            secondary_container: colors.secondary.container,
            on_secondary_container: colors.secondary.on_container,
            corner_radius: shapes.corner_extra_small.to_radius(),
        }
    }
}

/// Default dimensions for menu components
pub mod dimensions {
    /// Default minimum width for select menus and autocomplete
    pub const DEFAULT_SELECT_WIDTH: f32 = 200.0;

    /// Default minimum width for dropdown menus
    pub const DEFAULT_MENU_MIN_WIDTH: f32 = 112.0;

    /// Default maximum height for scrollable menus
    pub const DEFAULT_MENU_MAX_HEIGHT: f32 = 280.0;

    /// Standard height for scrollable content in select menus
    pub const SELECT_MENU_SCROLL_HEIGHT: f32 = 280.0;

    /// Fixed height for divider elements
    pub const DIVIDER_HEIGHT: f32 = 1.0;
}

/// Padding values for menu components following Material Design spacing
pub mod padding {
    use iced::Padding;

    /// Standard padding for menu items (12px top/bottom, 16px left/right)
    pub const MENU_ITEM: Padding = Padding {
        top: 12.0,
        right: 16.0,
        bottom: 12.0,
        left: 16.0,
    };

    /// Container padding for menu containers (8px top/bottom, 0px left/right)
    pub const MENU_CONTAINER: Padding = Padding {
        top: 8.0,
        right: 0.0,
        bottom: 8.0,
        left: 0.0,
    };

    /// Padding for select button content (12px top/bottom, 16px left/right)
    pub const SELECT_BUTTON: Padding = Padding {
        top: 12.0,
        right: 16.0,
        bottom: 12.0,
        left: 16.0,
    };

    /// Padding for menu headers (8px top/bottom, 16px left/right)
    pub const MENU_HEADER: Padding = Padding {
        top: 8.0,
        right: 16.0,
        bottom: 8.0,
        left: 16.0,
    };

    /// Padding for autocomplete suggestions container (4px top/bottom, 0px left/right)
    pub const AUTOCOMPLETE_SUGGESTIONS: Padding = Padding {
        top: 4.0,
        right: 0.0,
        bottom: 4.0,
        left: 0.0,
    };
}

/// Spacing values for component layouts
pub mod spacing {
    /// Standard spacing between elements in menu items with icons
    pub const MENU_ITEM_ICON_TEXT: u16 = 12;

    /// Standard spacing between select button elements
    pub const SELECT_BUTTON_ELEMENTS: u16 = 8;

    /// Standard spacing between label and input in form components
    pub const LABEL_INPUT: u16 = 4;
}

/// Typography sizes for menu components
pub mod typography {
    /// Standard text size for menu item labels
    pub const MENU_ITEM_TEXT_SIZE: u16 = 14;

    /// Text size for menu headers
    pub const MENU_HEADER_TEXT_SIZE: u16 = 11;

    /// Text size for form labels
    pub const LABEL_TEXT_SIZE: u16 = 12;

    /// Text size for dropdown arrow and small icons
    pub const SMALL_ICON_SIZE: u16 = 12;

    /// Text size for standard icons in menu items
    pub const MENU_ICON_SIZE: u16 = 18;
}

/// Default elevation levels for menu components
pub mod elevation {
    /// Default elevation level for dropdown menus
    pub const DROPDOWN_MENU: u8 = 2;

    /// Maximum allowed elevation level
    pub const MAX_ELEVATION: u8 = 5;
}

/// Default limits for component behavior
pub mod limits {
    /// Default maximum number of autocomplete suggestions to display
    pub const DEFAULT_MAX_SUGGESTIONS: usize = 5;
}
