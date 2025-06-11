//! Selection component state system for Material Design 3
//!
//! This module provides comprehensive state management for selection components
//! following Material Design 3 interaction patterns.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

use super::constants;

/// Comprehensive selection component states following Material Design 3 interaction patterns
///
/// This enum represents all possible interaction states across different
/// selection components (checkbox, radio, chip, switch) in a unified way.
/// Based on Material Design 3 state layer system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionState {
    /// Component is in default state and selected/checked
    DefaultSelected,
    /// Component is in default state but not selected/checked  
    DefaultUnselected,
    /// Component is being hovered and is selected/checked
    HoveredSelected,
    /// Component is being hovered but not selected/checked
    HoveredUnselected,
    /// Component is pressed and selected/checked
    PressedSelected,
    /// Component is pressed but not selected/checked
    PressedUnselected,
    /// Component has focus and is selected/checked
    FocusedSelected,
    /// Component has focus but not selected/checked
    FocusedUnselected,
    /// Component is disabled and selected/checked
    DisabledSelected,
    /// Component is disabled and not selected/checked
    DisabledUnselected,
}

impl SelectionState {
    /// Check if the component is in a selected state
    #[must_use]
    pub const fn is_selected(self) -> bool {
        matches!(
            self,
            Self::DefaultSelected
                | Self::HoveredSelected
                | Self::PressedSelected
                | Self::FocusedSelected
                | Self::DisabledSelected
        )
    }

    /// Check if the component is in an interactive state (not disabled)
    #[must_use]
    pub const fn is_interactive(self) -> bool {
        !matches!(self, Self::DisabledSelected | Self::DisabledUnselected)
    }

    /// Check if the component is in a hover state
    #[must_use]
    pub const fn is_hovered(self) -> bool {
        matches!(self, Self::HoveredSelected | Self::HoveredUnselected)
    }

    /// Check if the component is in a pressed state
    #[must_use]
    pub const fn is_pressed(self) -> bool {
        matches!(self, Self::PressedSelected | Self::PressedUnselected)
    }

    /// Check if the component is in a focused state
    #[must_use]
    pub const fn is_focused(self) -> bool {
        matches!(self, Self::FocusedSelected | Self::FocusedUnselected)
    }

    /// Check if the component is disabled
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::DisabledSelected | Self::DisabledUnselected)
    }

    /// Get the base state (without interaction modifiers)
    #[must_use]
    pub const fn base_state(self) -> BaseSelectionState {
        if self.is_selected() {
            BaseSelectionState::Selected
        } else {
            BaseSelectionState::Unselected
        }
    }

    /// Get the interaction state
    #[must_use]
    pub const fn interaction_state(self) -> InteractionState {
        if self.is_disabled() {
            InteractionState::Disabled
        } else if self.is_pressed() {
            InteractionState::Pressed
        } else if self.is_focused() {
            InteractionState::Focused
        } else if self.is_hovered() {
            InteractionState::Hovered
        } else {
            InteractionState::Default
        }
    }
}

impl fmt::Display for SelectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state_str = match self {
            Self::DefaultSelected => "default-selected",
            Self::DefaultUnselected => "default-unselected",
            Self::HoveredSelected => "hovered-selected",
            Self::HoveredUnselected => "hovered-unselected",
            Self::PressedSelected => "pressed-selected",
            Self::PressedUnselected => "pressed-unselected",
            Self::FocusedSelected => "focused-selected",
            Self::FocusedUnselected => "focused-unselected",
            Self::DisabledSelected => "disabled-selected",
            Self::DisabledUnselected => "disabled-unselected",
        };
        write!(f, "{state_str}")
    }
}

/// Base selection state (selected or unselected)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseSelectionState {
    /// Component is selected/checked
    Selected,
    /// Component is unselected/unchecked
    Unselected,
}

/// Interaction state of the component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionState {
    /// Default state (no interaction)
    Default,
    /// Component is being hovered
    Hovered,
    /// Component is being pressed
    Pressed,
    /// Component has focus
    Focused,
    /// Component is disabled
    Disabled,
}

/// Selection component variants with comprehensive Material Design 3 support
///
/// Different selection components need specific visual treatments while
/// sharing the same underlying color logic and state management.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionVariant {
    /// Checkbox component styling with Material Design 3 specifications
    Checkbox,
    /// Radio button component styling with circular design patterns
    Radio,
    /// Chip component styling with multiple chip variants support
    Chip,
    /// Switch component styling with Material Design 3 switch patterns
    Switch,
}

impl SelectionVariant {
    /// Get the display name for the variant
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Checkbox => "Checkbox",
            Self::Radio => "Radio",
            Self::Chip => "Chip",
            Self::Switch => "Switch",
        }
    }

    /// Check if this variant supports indeterminate state
    #[must_use]
    pub const fn supports_indeterminate(self) -> bool {
        matches!(self, Self::Checkbox)
    }

    /// Check if this variant supports custom icons
    #[must_use]
    pub const fn supports_icons(self) -> bool {
        matches!(self, Self::Chip)
    }

    /// Get the default border radius for this variant
    #[must_use]
    pub const fn default_border_radius(self) -> f32 {
        match self {
            Self::Checkbox => constants::border_radius::CHECKBOX,
            Self::Radio => constants::border_radius::RADIO,
            Self::Chip => constants::border_radius::CHIP,
            Self::Switch => constants::border_radius::SWITCH,
        }
    }
}

impl fmt::Display for SelectionVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Size variants for selection components with Material Design 3 specifications
///
/// Provides consistent sizing across all selection components following
/// Material Design 3 touch target and accessibility guidelines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SelectionSize {
    /// Small size (16px) - for dense layouts and compact spaces
    Small,
    /// Medium size (20px) - default size for most use cases
    Medium,
    /// Large size (24px) - for accessibility and prominent placement
    Large,
}

impl Default for SelectionSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl SelectionSize {
    /// Get the pixel size for the selection component
    #[must_use]
    pub const fn size_px(self) -> f32 {
        match self {
            Self::Small => constants::size::SMALL_PX,
            Self::Medium => constants::size::MEDIUM_PX,
            Self::Large => constants::size::LARGE_PX,
        }
    }

    /// Get the touch target size in pixels (Material Design minimum requirements)
    #[must_use]
    pub const fn touch_target_size(self) -> f32 {
        match self {
            Self::Small => 32.0,  // Compact but still accessible
            Self::Medium => 40.0, // Standard touch target
            Self::Large => 48.0,  // Full Material Design touch target
        }
    }

    /// Get the appropriate border width for the size
    #[must_use]
    pub const fn border_width(self) -> f32 {
        match self {
            Self::Small => constants::size::SMALL_BORDER,
            Self::Medium => constants::size::MEDIUM_BORDER,
            Self::Large => constants::size::LARGE_BORDER,
        }
    }

    /// Get the appropriate text size for labels
    #[must_use]
    pub const fn text_size(self) -> f32 {
        match self {
            Self::Small => constants::size::SMALL_TEXT,
            Self::Medium => constants::size::MEDIUM_TEXT,
            Self::Large => constants::size::LARGE_TEXT,
        }
    }

    /// Get padding for the component
    #[must_use]
    pub const fn padding(self) -> f32 {
        match self {
            Self::Small => constants::size::SMALL_PADDING,
            Self::Medium => constants::size::MEDIUM_PADDING,
            Self::Large => constants::size::LARGE_PADDING,
        }
    }
}

impl fmt::Display for SelectionSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size_str = match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        };
        write!(f, "{size_str}")
    }
}

/// Errors that can occur during selection component styling operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SelectionStyleError {
    /// Invalid state combination provided
    #[error("Invalid selection state combination: {details}")]
    InvalidState {
        /// Details about the invalid state
        details: String,
    },

    /// Unsupported variant configuration
    #[error("Unsupported variant configuration for {variant}: {reason}")]
    UnsupportedVariant {
        /// The variant that is unsupported
        variant: SelectionVariant,
        /// Reason why it's unsupported
        reason: String,
    },

    /// Token system integration error
    #[error("Token system error: {message}")]
    TokenError {
        /// Error message from token system
        message: String,
    },

    /// Color calculation error
    #[error("Color calculation failed: {context}")]
    ColorError {
        /// Context about the color calculation failure
        context: String,
    },
}

/// Comprehensive styling properties for a selection component state
#[derive(Debug, Clone)]
pub struct SelectionStyling {
    /// Background color or gradient for the component
    pub background: iced::Background,
    /// Text color for component labels
    pub text_color: iced::Color,
    /// Border styling including color, width, and radius
    pub border: iced::Border,
    /// Optional shadow for elevation effects
    pub shadow: Option<iced::Shadow>,
    /// Foreground color (icon, checkmark, etc.)
    pub foreground_color: iced::Color,
    /// Optional state layer color for interactions
    pub state_layer: Option<iced::Color>,
}
