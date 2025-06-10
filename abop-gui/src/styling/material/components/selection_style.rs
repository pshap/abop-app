//! Material Design 3 Selection Component Styling System
//!
//! This module provides a centralized styling system for all selection components,
//! eliminating code duplication across MaterialCheckbox, MaterialRadio, MaterialSwitch, and MaterialChip.
//!
//! ## Design Goals
//! - Centralized color logic for all selection states
//! - Consistent Material Design 3 styling patterns  
//! - Reduced code duplication
//! - Type-safe styling with clear state management
//! - Builder pattern support for fluent construction
//! - Theme integration with Material Design 3 specifications

use iced::{Background, Color, Theme};
use serde::{Deserialize, Serialize};

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::colors::MaterialColors;

/// Selection component states for styling purposes
///
/// This enum represents all possible interaction states across different
/// selection components (checkbox, radio, chip) in a unified way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionState {
    /// Component is active and selected/checked
    ActiveSelected,
    /// Component is active but not selected/checked  
    ActiveUnselected,
    /// Component is being hovered and is selected/checked
    HoveredSelected,
    /// Component is being hovered but not selected/checked
    HoveredUnselected,
    /// Component is pressed and selected/checked
    PressedSelected,
    /// Component is pressed but not selected/checked
    PressedUnselected,
    /// Component is disabled and selected/checked
    DisabledSelected,
    /// Component is disabled and not selected/checked
    DisabledUnselected,
}

/// Selection component variants for different styling approaches
///
/// Different selection components need slightly different visual treatments
/// while sharing the same underlying color logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionVariant {
    /// Checkbox component styling
    Checkbox,
    /// Radio button component styling  
    Radio,
    /// Chip component styling (filter chips)
    Chip,
    /// Switch component styling
    Switch,
}

/// Size variants for selection components
///
/// Provides consistent sizing across all selection components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionSize {
    /// Small size (16px) - for dense layouts
    Small,
    /// Medium size (20px) - default size
    Medium,
    /// Large size (24px) - for accessibility
    Large,
}

impl SelectionSize {
    /// Get the touch target size in pixels
    pub fn touch_target_size(&self) -> f32 {
        match self {
            Self::Small => 32.0,
            Self::Medium => 40.0,
            Self::Large => 48.0,
        }
    }
}

impl Default for SelectionSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl SelectionSize {
    /// Get the pixel size for the selection component
    #[must_use]
    pub const fn size_px(&self) -> f32 {
        match self {
            Self::Small => 16.0,
            Self::Medium => 20.0,
            Self::Large => 24.0,
        }
    }

    /// Get the appropriate border width for the size
    #[must_use]
    pub const fn border_width(&self) -> f32 {
        match self {
            Self::Small => 1.5,
            Self::Medium => 2.0,
            Self::Large => 2.5,
        }
    }
}

/// Centralized color calculation for selection components
///
/// This structure encapsulates all the color logic that was previously
/// duplicated across the three `appearance()` methods.
#[derive(Debug, Clone)]
pub struct SelectionColors {
    /// The material color scheme to use
    pub colors: MaterialColors,
    /// Whether the component is in an error state
    pub error_state: bool,
}

impl SelectionColors {
    /// Create new selection colors with the given color scheme
    #[must_use]
    pub const fn new(colors: MaterialColors) -> Self {
        Self {
            colors,
            error_state: false,
        }
    }

    /// Set error state for form validation
    #[must_use]
    pub const fn with_error(mut self, error: bool) -> Self {
        self.error_state = error;
        self
    }

    /// Calculate the primary component color (background, border, or fill)
    ///
    /// This method centralizes the color logic that was repeated across
    /// all three `appearance()` methods.
    #[must_use]
    pub fn primary_color(&self, state: SelectionState, variant: SelectionVariant) -> Color {
        match (state, self.error_state, variant) {
            // Error states take precedence
            (SelectionState::ActiveSelected | SelectionState::HoveredSelected, true, _) => {
                self.colors.error.base
            }
            (SelectionState::ActiveUnselected | SelectionState::HoveredUnselected, true, _) => {
                Color::TRANSPARENT
            }

            // Normal selected states
            (
                SelectionState::ActiveSelected | SelectionState::HoveredSelected,
                false,
                SelectionVariant::Checkbox,
            ) => self.colors.primary.base,
            (
                SelectionState::ActiveSelected | SelectionState::HoveredSelected,
                false,
                SelectionVariant::Radio,
            ) => Color::TRANSPARENT, // Radio buttons have transparent background
            (
                SelectionState::ActiveSelected | SelectionState::HoveredSelected,
                false,
                SelectionVariant::Chip,
            ) => self.colors.secondary_container,

            // Normal unselected states
            (SelectionState::ActiveUnselected | SelectionState::HoveredUnselected, false, _) => {
                Color::TRANSPARENT
            }

            // Pressed states
            (SelectionState::PressedSelected, false, SelectionVariant::Chip) => {
                ColorUtils::darken(self.colors.secondary_container, 0.1)
            }
            (SelectionState::PressedUnselected, false, _) => {
                ColorUtils::with_alpha(self.colors.on_surface, 0.12)
            }

            // Disabled states
            (SelectionState::DisabledSelected, _, _) => {
                if matches!(variant, SelectionVariant::Checkbox) {
                    ColorUtils::with_alpha(self.colors.on_surface, 0.38)
                } else {
                    Color::TRANSPARENT
                }
            }
            (SelectionState::DisabledUnselected, _, _) => Color::TRANSPARENT,

            // Fallback
            _ => Color::TRANSPARENT,
        }
    }

    /// Calculate the border color for the selection component
    #[must_use]
    pub const fn border_color(&self, state: SelectionState, variant: SelectionVariant) -> Color {
        match (state, self.error_state) {
            // Error states
            (_, true) => self.colors.error.base, // Selected states - Material Design 3 spec: checkbox, radio, and switch share primary.base for selected states
            (SelectionState::ActiveSelected | SelectionState::HoveredSelected, false) => {
                match variant {
                    SelectionVariant::Checkbox
                    | SelectionVariant::Radio
                    | SelectionVariant::Switch => self.colors.primary.base,
                    SelectionVariant::Chip => self.colors.secondary_container,
                }
            }

            // Unselected states
            (SelectionState::ActiveUnselected | SelectionState::HoveredUnselected, false) => {
                self.colors.outline
            }

            // Disabled states
            (SelectionState::DisabledSelected | SelectionState::DisabledUnselected, _) => {
                ColorUtils::with_alpha(self.colors.on_surface, 0.38)
            }

            // Pressed states
            (SelectionState::PressedSelected, false) => match variant {
                SelectionVariant::Chip => self.colors.secondary_container,
                _ => self.colors.primary.base,
            },
            (SelectionState::PressedUnselected, false) => self.colors.outline,
        }
    }

    /// Calculate the foreground color (text, icon, or dot)
    #[must_use]
    pub const fn foreground_color(
        &self,
        state: SelectionState,
        variant: SelectionVariant,
    ) -> Color {
        match (state, self.error_state, variant) {
            // Error states
            (
                SelectionState::ActiveSelected | SelectionState::HoveredSelected,
                true,
                SelectionVariant::Checkbox,
            ) => self.colors.on_error,
            (
                SelectionState::ActiveSelected | SelectionState::HoveredSelected,
                true,
                SelectionVariant::Radio,
            ) => self.colors.error.base,
            (SelectionState::ActiveUnselected | SelectionState::HoveredUnselected, true, _) => {
                Color::TRANSPARENT
            }

            // Normal selected states
            (
                SelectionState::ActiveSelected | SelectionState::HoveredSelected,
                false,
                SelectionVariant::Checkbox,
            ) => self.colors.on_primary,
            (
                SelectionState::ActiveSelected | SelectionState::HoveredSelected,
                false,
                SelectionVariant::Radio,
            ) => self.colors.primary.base,
            (
                SelectionState::ActiveSelected | SelectionState::HoveredSelected,
                false,
                SelectionVariant::Chip,
            ) => self.colors.on_secondary_container,

            // Normal unselected states
            (SelectionState::ActiveUnselected | SelectionState::HoveredUnselected, false, _) => {
                Color::TRANSPARENT
            }

            // Disabled states
            (SelectionState::DisabledSelected, _, SelectionVariant::Checkbox) => {
                self.colors.surface
            }
            (SelectionState::DisabledSelected | SelectionState::DisabledUnselected, _, _) => {
                Color::TRANSPARENT
            }

            // Text color for chips and labels
            _ => self.colors.on_surface,
        }
    }

    /// Calculate the text color for component labels
    #[must_use]
    pub const fn text_color(&self, state: SelectionState) -> Color {
        match state {
            SelectionState::DisabledSelected | SelectionState::DisabledUnselected => {
                ColorUtils::with_alpha(self.colors.on_surface, 0.38)
            }
            _ => self.colors.on_surface,
        }
    }
}

/// Builder for creating selection component styling
///
/// Provides a fluent interface for creating Material Design selection styles
/// with all the appropriate state handling.
#[derive(Debug, Clone)]
pub struct SelectionStyleBuilder {
    /// The color scheme to use for styling
    colors: MaterialColors,
    /// The component variant being styled
    variant: SelectionVariant,
    /// The size of the component
    size: SelectionSize,
    /// Whether the component is in an error state
    error: bool,
    /// Whether the component is in a hover state
    hover: bool,
    /// Whether the component is in a pressed state
    pressed: bool,
    /// Whether the component is focused
    focused: bool,
}

impl SelectionStyleBuilder {
    /// Create a new selection style builder
    #[must_use]
    pub fn new(colors: MaterialColors, variant: SelectionVariant) -> Self {
        Self {
            colors,
            variant,
            size: SelectionSize::default(),
            error: false,
            hover: false,
            pressed: false,
            focused: false,
        }
    }

    /// Set the component size
    #[must_use]
    pub const fn size(mut self, size: SelectionSize) -> Self {
        self.size = size;
        self
    }

    /// Enable error state for validation
    #[must_use]
    pub const fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Enable hover state
    #[must_use]
    pub const fn hover(mut self, hover: bool) -> Self {
        self.hover = hover;
        self
    }

    /// Enable pressed state
    #[must_use]
    pub const fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    /// Enable focused state
    #[must_use]
    pub const fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Create checkbox styling function
    pub fn checkbox_style(
        self,
    ) -> impl Fn(&iced::Theme, iced::widget::checkbox::Status) -> iced::widget::checkbox::Style
    {
        let colors = self.colors;
        let size = self.size;
        let error = self.error;

        move |_theme: &Theme, status: iced::widget::checkbox::Status| {
            let state = match status {
                iced::widget::checkbox::Status::Active { is_checked: true } => {
                    SelectionState::ActiveSelected
                }
                iced::widget::checkbox::Status::Active { is_checked: false } => {
                    SelectionState::ActiveUnselected
                }
                iced::widget::checkbox::Status::Hovered { is_checked: true } => {
                    SelectionState::HoveredSelected
                }
                iced::widget::checkbox::Status::Hovered { is_checked: false } => {
                    SelectionState::HoveredUnselected
                }
                iced::widget::checkbox::Status::Disabled { is_checked: true } => {
                    SelectionState::DisabledSelected
                }
                iced::widget::checkbox::Status::Disabled { is_checked: false } => {
                    SelectionState::DisabledUnselected
                }
            };

            let colors = SelectionColors::new(colors.clone()).with_error(error);

            iced::widget::checkbox::Style {
                background: colors
                    .primary_color(state, SelectionVariant::Checkbox)
                    .into(),
                icon_color: colors.foreground_color(state, SelectionVariant::Checkbox),
                border: iced::Border {
                    radius: 2.0.into(),
                    width: size.border_width(),
                    color: colors.border_color(state, SelectionVariant::Checkbox),
                },
                text_color: Some(colors.text_color(state)),
            }
        }
    }

    /// Create radio button styling function
    pub fn radio_style(
        self,
    ) -> impl Fn(&iced::Theme, iced::widget::radio::Status) -> iced::widget::radio::Style {
        let colors = self.colors;
        let error = self.error;

        move |_theme: &Theme, status: iced::widget::radio::Status| {
            let state = match status {
                iced::widget::radio::Status::Active { is_selected: true } => {
                    SelectionState::ActiveSelected
                }
                iced::widget::radio::Status::Active { is_selected: false } => {
                    SelectionState::ActiveUnselected
                }
                iced::widget::radio::Status::Hovered { is_selected: true } => {
                    SelectionState::HoveredSelected
                }
                iced::widget::radio::Status::Hovered { is_selected: false } => {
                    SelectionState::HoveredUnselected
                } // Note: radio::Status doesn't have a Disabled variant in current Iced API
            };

            let colors = SelectionColors::new(colors.clone()).with_error(error);

            iced::widget::radio::Style {
                background: colors.primary_color(state, SelectionVariant::Radio).into(),
                dot_color: colors.foreground_color(state, SelectionVariant::Radio),
                border_width: 1.0,
                border_color: colors.border_color(state, SelectionVariant::Radio),
                text_color: Some(colors.text_color(state)),
            }
        }
    }

    /// Create chip button styling function
    pub fn chip_style(
        self,
        is_selected: bool,
    ) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        let colors = self.colors;
        let variant = self.variant;

        move |_theme: &Theme, status: iced::widget::button::Status| {
            let state = if is_selected {
                match status {
                    iced::widget::button::Status::Active => SelectionState::ActiveSelected,
                    iced::widget::button::Status::Hovered => SelectionState::HoveredSelected,
                    iced::widget::button::Status::Pressed => SelectionState::PressedSelected,
                    iced::widget::button::Status::Disabled => SelectionState::DisabledSelected,
                }
            } else {
                match status {
                    iced::widget::button::Status::Active => SelectionState::ActiveUnselected,
                    iced::widget::button::Status::Hovered => SelectionState::HoveredUnselected,
                    iced::widget::button::Status::Pressed => SelectionState::PressedUnselected,
                    iced::widget::button::Status::Disabled => SelectionState::DisabledUnselected,
                }
            };

            let selection_colors = SelectionColors::new(colors.clone());

            iced::widget::button::Style {
                background: Some(Background::from(
                    selection_colors.primary_color(state, variant),
                )),
                text_color: selection_colors.foreground_color(state, variant),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: selection_colors.border_color(state, variant),
                },
                shadow: iced::Shadow::default(),
            }
        }
    }

    /// Create switch styling function
    pub fn switch_style(
        self,
        is_enabled: bool,
    ) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        let colors = self.colors;

        move |_theme: &Theme, status: iced::widget::button::Status| {
            let state = if is_enabled {
                match status {
                    iced::widget::button::Status::Active => SelectionState::ActiveSelected,
                    iced::widget::button::Status::Hovered => SelectionState::HoveredSelected,
                    iced::widget::button::Status::Pressed => SelectionState::PressedSelected,
                    iced::widget::button::Status::Disabled => SelectionState::DisabledSelected,
                }
            } else {
                match status {
                    iced::widget::button::Status::Active => SelectionState::ActiveUnselected,
                    iced::widget::button::Status::Hovered => SelectionState::HoveredUnselected,
                    iced::widget::button::Status::Pressed => SelectionState::PressedUnselected,
                    iced::widget::button::Status::Disabled => SelectionState::DisabledUnselected,
                }
            };

            let selection_colors = SelectionColors::new(colors.clone());

            iced::widget::button::Style {
                background: Some(Background::from(
                    selection_colors.primary_color(state, SelectionVariant::Switch),
                )),
                text_color: selection_colors.foreground_color(state, SelectionVariant::Switch),
                border: iced::Border {
                    radius: 12.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: iced::Shadow::default(),
            }
        }
    }
}
