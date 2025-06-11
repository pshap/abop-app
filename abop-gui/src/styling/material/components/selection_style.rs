//! Material Design 3 Selection Component Styling System
//!
//! This module provides a sophisticated styling system for all selection components,
//! implementing the strategy pattern used throughout the Material Design system.
//! Eliminates code duplication across MaterialCheckbox, MaterialRadio, MaterialSwitch, and MaterialChip.
//!
//! ## Phase 1 Architectural Improvements
//! - Strategy pattern implementation for consistent architecture
//! - Complete Material Design 3 state system integration
//! - Enhanced error handling and validation
//! - Proper integration with MaterialTokens
//! - Type-safe styling with comprehensive state management
//! - Builder pattern support with fluent construction
//! - Full theme integration with Material Design 3 specifications
//!
//! ## Integration with Material Strategy System
//! This implementation follows the same architectural patterns as the button styling
//! system, ensuring consistency across all Material components.

use iced::{Background, Border, Color, Shadow, Theme};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::tokens::core::MaterialTokens;

// ============================================================================
// Component State System (Material Design 3 Compliant)
// ============================================================================

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
            Self::Checkbox => 2.0,
            Self::Radio => 12.0, // Circle
            Self::Chip => 8.0,
            Self::Switch => 16.0,
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
            Self::Small => 16.0,
            Self::Medium => 20.0,
            Self::Large => 24.0,
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
            Self::Small => 1.5,
            Self::Medium => 2.0,
            Self::Large => 2.5,
        }
    }

    /// Get the appropriate text size for labels
    #[must_use]
    pub const fn text_size(self) -> f32 {
        match self {
            Self::Small => 12.0,
            Self::Medium => 14.0,
            Self::Large => 16.0,
        }
    }

    /// Get padding for the component
    #[must_use]
    pub const fn padding(self) -> f32 {
        match self {
            Self::Small => 4.0,
            Self::Medium => 8.0,
            Self::Large => 12.0,
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

// ============================================================================
// Error Handling System
// ============================================================================

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

// ============================================================================
// Strategy Pattern Implementation
// ============================================================================

/// Comprehensive styling properties for a selection component state
#[derive(Debug, Clone)]
pub struct SelectionStyling {
    /// Background color or gradient for the component
    pub background: Background,
    /// Text color for component labels
    pub text_color: Color,
    /// Border styling including color, width, and radius
    pub border: Border,
    /// Optional shadow for elevation effects
    pub shadow: Option<Shadow>,
    /// Foreground color (icon, checkmark, etc.)
    pub foreground_color: Color,
    /// Optional state layer color for interactions
    pub state_layer: Option<Color>,
}

/// Strategy trait for selection component styling following Material Design 3
pub trait SelectionStyleStrategy {
    /// Get styling for a specific selection state
    ///
    /// # Arguments
    /// * `state` - The current selection state
    /// * `tokens` - Material Design tokens for consistent styling
    /// * `size` - Component size variant
    /// * `error_state` - Whether the component is in error state
    ///
    /// # Returns
    /// Complete styling configuration for the given state
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError>;

    /// Get the variant name for debugging and logging
    fn variant_name(&self) -> &'static str;

    /// Whether this variant supports error states
    fn supports_error_state(&self) -> bool {
        true
    }

    /// Whether this variant supports custom icons
    fn supports_icons(&self) -> bool {
        false
    }

    /// Whether this variant supports indeterminate state
    fn supports_indeterminate(&self) -> bool {
        false
    }

    /// Get the default size for this variant
    fn default_size(&self) -> SelectionSize {
        SelectionSize::Medium
    }
}

/// Context information for selection styling operations
#[derive(Debug, Clone, Default)]
pub struct SelectionStyleContext {
    /// Whether this component represents a primary selection
    pub is_primary: bool,
    /// Whether the component is in an error state
    pub error_state: bool,
    /// Whether the component has custom content (icons, etc.)
    pub has_custom_content: bool,
    /// Whether the component is part of a group
    pub is_part_of_group: bool,
}

// ============================================================================
// Centralized Color Calculation System
// ============================================================================

/// Enhanced color calculation for selection components using Material Design 3 tokens
///
/// This structure provides comprehensive color logic while leveraging
/// the centralized `MaterialTokens` system for consistency and maintainability.
#[derive(Debug, Clone)]
pub struct SelectionColors {
    /// The material token system to use
    tokens: MaterialTokens,
    /// Current component variant
    variant: SelectionVariant,
    /// Component size
    size: SelectionSize,
    /// Whether the component is in an error state
    error_state: bool,
}

impl SelectionColors {
    /// Create new selection colors with the given token system
    #[must_use]
    pub fn new(tokens: MaterialTokens, variant: SelectionVariant) -> Self {
        Self {
            tokens,
            variant,
            size: SelectionSize::Medium,
            error_state: false,
        }
    }

    /// Set component size
    #[must_use]
    pub const fn with_size(mut self, size: SelectionSize) -> Self {
        self.size = size;
        self
    }

    /// Set error state for form validation
    #[must_use]
    pub const fn with_error(mut self, error: bool) -> Self {
        self.error_state = error;
        self
    }

    /// Calculate the primary component color (background, border, or fill)
    ///
    /// This method centralizes the color logic using Material Design 3 token system.
    #[must_use]
    pub fn primary_color(&self, state: SelectionState) -> Color {
        let colors = &self.tokens.colors;

        match (state, self.error_state, self.variant) {
            // Error states take precedence
            (state, true, _) if state.is_selected() => colors.error.base,
            (state, true, _) if !state.is_selected() => Color::TRANSPARENT,

            // Normal selected states
            (state, false, SelectionVariant::Checkbox) if state.is_selected() => {
                if state.is_disabled() {
                    ColorUtils::with_alpha(colors.on_surface, 0.38)
                } else {
                    colors.primary.base
                }
            }
            (state, false, SelectionVariant::Radio) if state.is_selected() => {
                // Radio buttons use transparent background with colored dot
                Color::TRANSPARENT
            }
            (state, false, SelectionVariant::Chip) if state.is_selected() => {
                if state.is_disabled() {
                    ColorUtils::with_alpha(colors.on_surface, 0.12)
                } else if state.is_pressed() {
                    ColorUtils::darken(colors.secondary_container, 0.1)
                } else {
                    colors.secondary_container
                }
            }
            (state, false, SelectionVariant::Switch) if state.is_selected() => {
                if state.is_disabled() {
                    ColorUtils::with_alpha(colors.on_surface, 0.12)
                } else {
                    colors.primary.base
                }
            }

            // Normal unselected states
            (state, false, _) if !state.is_selected() => {
                if state.is_pressed() {
                    ColorUtils::with_alpha(colors.on_surface, 0.12)
                } else {
                    Color::TRANSPARENT
                }
            }

            // Fallback
            _ => Color::TRANSPARENT,
        }
    }

    /// Calculate the border color for the selection component
    #[must_use]
    pub fn border_color(&self, state: SelectionState) -> Color {
        let colors = &self.tokens.colors;

        match (state, self.error_state) {
            // Error states
            (_, true) => colors.error.base,

            // Selected states
            (state, false) if state.is_selected() => {
                if state.is_disabled() {
                    ColorUtils::with_alpha(colors.on_surface, 0.38)
                } else {
                    match self.variant {
                        SelectionVariant::Checkbox
                        | SelectionVariant::Radio
                        | SelectionVariant::Switch => colors.primary.base,
                        SelectionVariant::Chip => colors.secondary_container,
                    }
                }
            }

            // Unselected states
            (state, false) if !state.is_selected() => {
                if state.is_disabled() {
                    ColorUtils::with_alpha(colors.on_surface, 0.38)
                } else {
                    colors.outline
                }
            }

            // Fallback
            _ => colors.outline,
        }
    }

    /// Calculate the foreground color (text, icon, or dot)
    #[must_use]
    pub fn foreground_color(&self, state: SelectionState) -> Color {
        let colors = &self.tokens.colors;

        match (state, self.error_state, self.variant) {
            // Error states
            (state, true, SelectionVariant::Checkbox) if state.is_selected() => colors.on_error,
            (state, true, SelectionVariant::Radio) if state.is_selected() => colors.error.base,
            (_, true, _) => Color::TRANSPARENT,

            // Normal selected states
            (state, false, SelectionVariant::Checkbox) if state.is_selected() => {
                if state.is_disabled() {
                    colors.surface
                } else {
                    colors.on_primary
                }
            }
            (state, false, SelectionVariant::Radio) if state.is_selected() => {
                if state.is_disabled() {
                    ColorUtils::with_alpha(colors.on_surface, 0.38)
                } else {
                    colors.primary.base
                }
            }
            (state, false, SelectionVariant::Chip) if state.is_selected() => {
                if state.is_disabled() {
                    ColorUtils::with_alpha(colors.on_surface, 0.38)
                } else {
                    colors.on_secondary_container
                }
            }
            (state, false, SelectionVariant::Switch) if state.is_selected() => {
                if state.is_disabled() {
                    colors.surface
                } else {
                    colors.on_primary
                }
            }

            // Normal unselected states
            (state, false, _) if !state.is_selected() => Color::TRANSPARENT,

            // Fallback
            _ => colors.on_surface,
        }
    }

    /// Calculate the text color for component labels
    #[must_use]
    pub fn text_color(&self, state: SelectionState) -> Color {
        let colors = &self.tokens.colors;

        if state.is_disabled() {
            ColorUtils::with_alpha(colors.on_surface, 0.38)
        } else {
            colors.on_surface
        }
    }

    /// Calculate state layer color for interactions
    #[must_use]
    pub fn state_layer_color(&self, state: SelectionState) -> Option<Color> {
        let colors = &self.tokens.colors;
        let opacity = &self.tokens.states.opacity;

        if state.is_disabled() {
            return None;
        }

        let base_color = if self.error_state {
            colors.error.base
        } else if state.is_selected() {
            colors.primary.base
        } else {
            colors.on_surface
        };

        let layer_opacity = if state.is_pressed() {
            opacity.pressed
        } else if state.is_focused() {
            opacity.focus
        } else if state.is_hovered() {
            opacity.hover
        } else {
            return None;
        };

        Some(ColorUtils::with_alpha(base_color, layer_opacity))
    }

    /// Get border configuration for the component
    #[must_use]
    pub fn border(&self, state: SelectionState) -> Border {
        Border {
            color: self.border_color(state),
            width: self.size.border_width(),
            radius: self.variant.default_border_radius().into(),
        }
    }

    /// Create complete styling for the given state
    #[must_use]
    pub fn create_styling(&self, state: SelectionState) -> SelectionStyling {
        SelectionStyling {
            background: Background::Color(self.primary_color(state)),
            text_color: self.text_color(state),
            border: self.border(state),
            shadow: None, // Selection components typically don't use shadows
            foreground_color: self.foreground_color(state),
            state_layer: self.state_layer_color(state),
        }
    }
}

// ============================================================================
// Builder for Selection Component Styling
// ============================================================================

/// Builder for creating selection component styling with enhanced capabilities
///
/// Provides a fluent interface for creating Material Design selection styles
/// with comprehensive state handling and strategy pattern integration.
#[derive(Debug, Clone)]
pub struct SelectionStyleBuilder {
    /// The token system to use for styling
    tokens: MaterialTokens,
    /// The component variant being styled
    variant: SelectionVariant,
    /// The size of the component
    size: SelectionSize,
    /// Whether the component is in an error state
    error: bool,
    /// Additional styling context
    context: SelectionStyleContext,
}

impl SelectionStyleBuilder {
    /// Create a new selection style builder
    #[must_use]
    pub fn new(tokens: MaterialTokens, variant: SelectionVariant) -> Self {
        Self {
            tokens,
            variant,
            size: SelectionSize::default(),
            error: false,
            context: SelectionStyleContext::default(),
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

    /// Set styling context
    #[must_use]
    pub fn context(mut self, context: SelectionStyleContext) -> Self {
        self.context = context;
        self
    }

    /// Create a color calculator for this configuration
    #[must_use]
    pub fn colors(&self) -> SelectionColors {
        SelectionColors::new(self.tokens.clone(), self.variant)
            .with_size(self.size)
            .with_error(self.error)
    }

    /// Create checkbox styling function
    ///
    /// Returns a function that can be used with Iced's checkbox styling system.
    pub fn checkbox_style(
        self,
    ) -> impl Fn(&Theme, iced::widget::checkbox::Status) -> iced::widget::checkbox::Style {
        move |_theme: &Theme, status: iced::widget::checkbox::Status| {
            let state = match status {
                iced::widget::checkbox::Status::Active { is_checked: true } => {
                    SelectionState::DefaultSelected
                }
                iced::widget::checkbox::Status::Active { is_checked: false } => {
                    SelectionState::DefaultUnselected
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

            let colors = self.colors();
            let styling = colors.create_styling(state);

            iced::widget::checkbox::Style {
                background: styling.background,
                icon_color: styling.foreground_color,
                border: styling.border,
                text_color: Some(styling.text_color),
            }
        }
    }

    /// Create radio button styling function
    ///
    /// Returns a function that can be used with Iced's radio styling system.
    pub fn radio_style(
        self,
    ) -> impl Fn(&Theme, iced::widget::radio::Status) -> iced::widget::radio::Style {
        move |_theme: &Theme, status: iced::widget::radio::Status| {
            let state = match status {
                iced::widget::radio::Status::Active { is_selected: true } => {
                    SelectionState::DefaultSelected
                }
                iced::widget::radio::Status::Active { is_selected: false } => {
                    SelectionState::DefaultUnselected
                }
                iced::widget::radio::Status::Hovered { is_selected: true } => {
                    SelectionState::HoveredSelected
                }
                iced::widget::radio::Status::Hovered { is_selected: false } => {
                    SelectionState::HoveredUnselected
                }
            };

            let colors = self.colors();
            let styling = colors.create_styling(state);

            iced::widget::radio::Style {
                background: styling.background,
                dot_color: styling.foreground_color,
                border_width: styling.border.width,
                border_color: styling.border.color,
                text_color: Some(styling.text_color),
            }
        }
    }

    /// Create chip button styling function
    ///
    /// Returns a function that can be used with Iced's button styling system for chips.
    pub fn chip_style(
        self,
        is_selected: bool,
    ) -> impl Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        move |_theme: &Theme, status: iced::widget::button::Status| {
            let state = if is_selected {
                match status {
                    iced::widget::button::Status::Active => SelectionState::DefaultSelected,
                    iced::widget::button::Status::Hovered => SelectionState::HoveredSelected,
                    iced::widget::button::Status::Pressed => SelectionState::PressedSelected,
                    iced::widget::button::Status::Disabled => SelectionState::DisabledSelected,
                }
            } else {
                match status {
                    iced::widget::button::Status::Active => SelectionState::DefaultUnselected,
                    iced::widget::button::Status::Hovered => SelectionState::HoveredUnselected,
                    iced::widget::button::Status::Pressed => SelectionState::PressedUnselected,
                    iced::widget::button::Status::Disabled => SelectionState::DisabledUnselected,
                }
            };

            let colors = self.colors();
            let styling = colors.create_styling(state);

            iced::widget::button::Style {
                background: Some(styling.background),
                text_color: styling.text_color,
                border: styling.border,
                shadow: styling.shadow.unwrap_or_default(),
            }
        }
    }

    /// Create switch styling function
    ///
    /// Returns a function that can be used with Iced's button styling system for switches.
    pub fn switch_style(
        self,
        is_enabled: bool,
    ) -> impl Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        move |_theme: &Theme, status: iced::widget::button::Status| {
            let state = if is_enabled {
                match status {
                    iced::widget::button::Status::Active => SelectionState::DefaultSelected,
                    iced::widget::button::Status::Hovered => SelectionState::HoveredSelected,
                    iced::widget::button::Status::Pressed => SelectionState::PressedSelected,
                    iced::widget::button::Status::Disabled => SelectionState::DisabledSelected,
                }
            } else {
                match status {
                    iced::widget::button::Status::Active => SelectionState::DefaultUnselected,
                    iced::widget::button::Status::Hovered => SelectionState::HoveredUnselected,
                    iced::widget::button::Status::Pressed => SelectionState::PressedUnselected,
                    iced::widget::button::Status::Disabled => SelectionState::DisabledUnselected,
                }
            };

            let colors = self.colors();
            let styling = colors.create_styling(state);

            iced::widget::button::Style {
                background: Some(styling.background),
                text_color: styling.text_color,
                border: Border {
                    radius: 16.0.into(), // Switch-specific border radius
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: styling.shadow.unwrap_or_default(),
            }
        }
    }
}

// ============================================================================
// Convenience Factory Functions
// ============================================================================

/// Create a selection style builder for checkbox components
#[must_use]
pub fn checkbox_style(tokens: &MaterialTokens) -> SelectionStyleBuilder {
    SelectionStyleBuilder::new(tokens.clone(), SelectionVariant::Checkbox)
}

/// Create a selection style builder for radio button components
#[must_use]
pub fn radio_style(tokens: &MaterialTokens) -> SelectionStyleBuilder {
    SelectionStyleBuilder::new(tokens.clone(), SelectionVariant::Radio)
}

/// Create a selection style builder for chip components
#[must_use]
pub fn chip_style(tokens: &MaterialTokens) -> SelectionStyleBuilder {
    SelectionStyleBuilder::new(tokens.clone(), SelectionVariant::Chip)
}

/// Create a selection style builder for switch components
#[must_use]
pub fn switch_style(tokens: &MaterialTokens) -> SelectionStyleBuilder {
    SelectionStyleBuilder::new(tokens.clone(), SelectionVariant::Switch)
}

// ============================================================================
// Strategy Pattern Implementations (Placeholder for Phase 2)
// ============================================================================

/// Checkbox strategy implementation
pub struct CheckboxStrategy;

impl SelectionStyleStrategy for CheckboxStrategy {
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError> {
        let colors = SelectionColors::new(tokens.clone(), SelectionVariant::Checkbox)
            .with_size(size)
            .with_error(error_state);
        Ok(colors.create_styling(state))
    }

    fn variant_name(&self) -> &'static str {
        "Checkbox"
    }

    fn supports_indeterminate(&self) -> bool {
        true
    }
}

/// Radio button strategy implementation  
pub struct RadioStrategy;

impl SelectionStyleStrategy for RadioStrategy {
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError> {
        let colors = SelectionColors::new(tokens.clone(), SelectionVariant::Radio)
            .with_size(size)
            .with_error(error_state);
        Ok(colors.create_styling(state))
    }

    fn variant_name(&self) -> &'static str {
        "Radio"
    }
}

/// Chip strategy implementation
pub struct ChipStrategy;

impl SelectionStyleStrategy for ChipStrategy {
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError> {
        let colors = SelectionColors::new(tokens.clone(), SelectionVariant::Chip)
            .with_size(size)
            .with_error(error_state);
        Ok(colors.create_styling(state))
    }

    fn variant_name(&self) -> &'static str {
        "Chip"
    }

    fn supports_icons(&self) -> bool {
        true
    }
}

/// Switch strategy implementation
pub struct SwitchStrategy;

impl SelectionStyleStrategy for SwitchStrategy {
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError> {
        let colors = SelectionColors::new(tokens.clone(), SelectionVariant::Switch)
            .with_size(size)
            .with_error(error_state);
        Ok(colors.create_styling(state))
    }

    fn variant_name(&self) -> &'static str {
        "Switch"
    }
}
