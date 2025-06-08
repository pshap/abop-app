//! Dialog container styles for interactive overlay elements
//!
//! Provides container styling for modal dialogs, dropdowns, tooltips
//! and other interactive overlay components. These styles ensure
//! proper visual hierarchy and user interaction feedback.

use crate::styling::material::helpers::ElevationHelpers;
use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;
use iced::widget::container;
use iced::{Background, Border, Color};

/// Dialog-specific container style types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogContainerType {
    /// Modal container with strong elevation and rounded corners
    /// Used for dialog boxes and overlay content
    Modal,
    /// Dropdown container with elevated styling
    /// Used for menu items and selection lists
    Dropdown,
    /// Tooltip container with compact styling
    /// Used for contextual help and information display
    Tooltip,
}

/// Dialog container styling utilities
pub struct DialogContainerStyles;

impl DialogContainerStyles {
    /// Get dialog container style based on type and theme
    #[must_use]
    pub fn get_style(style_type: DialogContainerType, theme_mode: ThemeMode) -> container::Style {
        let material_tokens = MaterialTokens::new();
        let shapes = material_tokens.shapes();
        let ui = material_tokens.ui();
        match style_type {
            DialogContainerType::Modal => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: shapes.dialog().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(5),
            },
            DialogContainerType::Dropdown => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_color())),
                border: Border {
                    color: theme_mode.border_color(),
                    width: ui.border_width_standard,
                    radius: shapes.card().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(3),
            },
            DialogContainerType::Tooltip => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.9))),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: shapes.text_field().to_radius(),
                },
                shadow: material_tokens.elevation_shadow(2),
            },
        }
    }

    /// Creates a modal container style with strong elevation and rounded corners
    ///
    /// Modal containers provide prominent overlay styling for dialog boxes
    /// and important interactions that require user attention.
    ///
    /// # Returns
    /// A style function that creates modal container appearance with strong shadows
    pub fn modal(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(DialogContainerType::Modal, theme_mode);
        move |_| style
    }

    /// Creates a dropdown container style with elevated styling
    ///
    /// Dropdown containers provide floating appearance for menu items
    /// and selection lists with appropriate elevation and borders.
    ///
    /// # Returns
    /// A style function that creates dropdown container appearance
    pub fn dropdown(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(DialogContainerType::Dropdown, theme_mode);
        move |_| style
    }
    /// Creates a tooltip container style with compact styling
    ///
    /// Tooltip containers provide minimal, floating appearance for
    /// contextual help and information display elements.
    ///
    /// # Returns
    /// A style function that creates tooltip container appearance
    pub fn tooltip(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(DialogContainerType::Tooltip, theme_mode);
        move |_| style
    }
}
