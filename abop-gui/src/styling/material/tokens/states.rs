//! State tokens for interactive element states
//!
//! This module provides tokens for managing interactive states like hover,
//! focus, pressed, and disabled states according to Material Design specifications.

use iced::Color;

/// State tokens for interactive element states
///
/// This struct contains all the state-related tokens needed for creating
/// consistent interactive behaviors across Material Design components.
#[derive(Debug, Clone)]
pub struct MaterialStates {
    /// Opacity values for different states
    pub opacity: StateOpacity,
    /// Overlay colors for different states
    pub overlay: StateOverlay,
}

impl Default for MaterialStates {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialStates {
    /// Create new Material state tokens
    ///
    /// Returns a new instance with Material Design 3 standard state values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            opacity: StateOpacity::new(),
            overlay: StateOverlay::new(),
        }
    }
}

/// Opacity values for interactive states
///
/// These opacity values follow Material Design 3 specifications for
/// creating consistent state feedback across all interactive elements.
#[derive(Debug, Clone)]
pub struct StateOpacity {
    /// Disabled state opacity (0.38)
    ///
    /// Applied to elements that are not currently interactive,
    /// providing clear visual indication of their disabled state.
    pub disabled: f32,

    /// Hover state opacity (0.08)
    ///
    /// Applied as an overlay when users hover over interactive elements,
    /// providing immediate visual feedback.
    pub hover: f32,

    /// Focus state opacity (0.12)
    ///
    /// Applied to elements that have keyboard focus or are selected,
    /// ensuring accessibility and clear focus indication.
    pub focus: f32,

    /// Pressed state opacity (0.12)
    ///
    /// Applied when users press or tap interactive elements,
    /// providing immediate tactile feedback.
    pub pressed: f32,

    /// Dragged state opacity (0.16)
    ///
    /// Applied to elements being dragged or moved,
    /// providing clear visual indication of the interaction state.
    pub dragged: f32,
}

impl StateOpacity {
    /// Creates a new `StateOpacity` with Material Design 3 standard opacity values
    ///
    /// Returns opacity values for different interactive states according to
    /// Material Design specifications.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            disabled: 0.38,
            hover: 0.08,
            focus: 0.12,
            pressed: 0.12,
            dragged: 0.16,
        }
    }
}

impl Default for StateOpacity {
    fn default() -> Self {
        Self::new()
    }
}

/// Overlay colors for interactive states
///
/// These overlay colors are applied on top of base colors to create
/// state feedback effects according to Material Design specifications.
#[derive(Debug, Clone)]
pub struct StateOverlay {
    /// Hover overlay color
    ///
    /// Semi-transparent overlay applied during hover interactions.
    pub hover: Color,

    /// Focus overlay color
    ///
    /// Semi-transparent overlay applied for focus and selection states.
    pub focus: Color,

    /// Pressed overlay color
    ///
    /// Semi-transparent overlay applied during press interactions.
    pub pressed: Color,

    /// Dragged overlay color
    ///
    /// Semi-transparent overlay applied during drag interactions.
    pub dragged: Color,
}

impl StateOverlay {
    /// Creates a new `StateOverlay` with Material Design 3 standard overlay colors
    ///
    /// Returns semi-transparent black overlay colors for different interactive states
    /// according to Material Design specifications.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hover: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            focus: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            pressed: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            dragged: Color::from_rgba(0.0, 0.0, 0.0, 0.16),
        }
    }
}

impl Default for StateOverlay {
    fn default() -> Self {
        Self::new()
    }
}
