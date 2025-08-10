//! Component traits for Material Design 3 selection components
//!
//! This module defines the core traits that selection components implement,
//! providing unified interfaces for state management, validation, and animation.

use super::animation::AnimationConfig;
use super::properties::ComponentProps;
use super::validation::SelectionError;

// ============================================================================
// Core Selection Component Traits (Simplified and Focused)
// ============================================================================

/// Core interface for selection components with validation
///
/// This trait focuses on the essential operations that all selection components need.
/// It avoids complex generics and provides clear, focused functionality.
pub trait SelectionComponent {
    /// The state type for this component
    type State: Copy + PartialEq;

    /// The message type produced by this component
    type Message;

    /// Get the current state
    fn state(&self) -> Self::State;

    /// Get the component properties
    fn props(&self) -> &ComponentProps;

    /// Validate the current widget state
    fn validate(&self) -> Result<(), SelectionError>;

    /// Check if the component has validation errors
    fn has_error(&self) -> bool;
}

/// Trait for components that can change state
pub trait StatefulComponent: SelectionComponent {
    /// Update the component state with validation
    fn set_state(&mut self, new_state: Self::State) -> Result<(), SelectionError>;

    /// Set error state
    fn set_error(&mut self, error: bool);
}

/// Trait for components that support animation
pub trait AnimatedComponent {
    /// Get animation configuration
    fn animation_config(&self) -> &AnimationConfig;

    /// Set animation configuration
    fn set_animation_config(&mut self, config: AnimationConfig);

    /// Check if animations should be used
    #[must_use]
    fn should_animate(&self) -> bool {
        let config = self.animation_config();
        config.enabled && (!config.respect_reduced_motion || !env_has_reduced_motion())
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Check if reduced motion is requested via environment variables
///
/// This function checks environment variables and provides a simple cross-platform
/// reduced motion detection. It does **not** check OS-level accessibility settings.
///
/// # Environment Variables
/// - `ABOP_REDUCE_MOTION`: Application-specific setting ("1" or "true")
/// - `PREFER_REDUCED_MOTION`: General accessibility setting ("1" or "true")
#[must_use]
pub fn env_has_reduced_motion() -> bool {
    /// Check if an environment variable indicates reduced motion preference
    fn env_var_is_enabled(var_name: &str) -> bool {
        std::env::var(var_name)
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    env_var_is_enabled("ABOP_REDUCE_MOTION") || env_var_is_enabled("PREFER_REDUCED_MOTION")
}
