//! Common types, traits, and utilities for Material Design 3 selection components
//!
//! This module serves as the central coordinator for all selection component modules,
//! providing organized re-exports and a convenient prelude for easy access to all types.
//!
//! ## Module Organization
//! - [`states`] - Component state enums and implementations
//! - [`properties`] - Shared properties and configuration types
//! - [`validation`] - Validation system and error handling
//! - [`animation`] - Animation configuration and easing curves
//! - [`traits`] - Component behavior traits
//!
//! ## Usage
//! Most users should import from the [`prelude`] module for convenient access:
//! ```rust,no_run
//! use abop_gui::styling::material::components::selection::common::prelude::*;
//! ```

// Re-exports are handled below - no direct imports needed at the top level

// Re-export all public items from separate modules
pub use crate::styling::material::components::selection::animation::*;
pub use crate::styling::material::components::selection::properties::*;
pub use crate::styling::material::components::selection::states::*;
pub use crate::styling::material::components::selection::traits::*;
pub use crate::styling::material::components::selection::validation::*;

// ============================================================================
// Prelude - Import this for convenient access to all traits
// ============================================================================

/// Convenient re-exports for component traits and types
///
/// This prelude provides easy access to all commonly used types from the selection
/// component system. Import this to get everything you need in most use cases.
///
/// # Examples
/// ```rust,no_run
/// use abop_gui::styling::material::components::selection::common::prelude::*;
///
/// let props = ComponentProps::new()
///     .with_label("Toggle me")
///     .size(ComponentSize::Large);
///
/// let checkbox = CheckboxState::Unchecked.toggle();
/// assert_eq!(checkbox, CheckboxState::Checked);
/// ```
pub mod prelude {
    // Re-export everything from our submodules
    pub use super::super::animation::*;
    pub use super::super::properties::*;
    pub use super::super::states::*;
    pub use super::super::traits::*;
    pub use super::super::validation::*;

    // Unified state traits from parent module
    pub use super::super::state_traits::{
        AnimatableState, ComponentState, InteractiveState, MultiLevelState,
    };

    // Constants access
    pub use super::super::constants;
}

// Note: Metadata keys are now centralized in the constants module
// Access them via: constants::metadata_keys::LEADING_ICON, etc.

// Constants are now centralized in the `constants` module.
// Use: `use super::constants` to access all organized constants.

// ============================================================================
// Integration Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_prelude_imports() {
        // Test that all expected types are available through prelude
        let _state = CheckboxState::default();
        let _props = ComponentProps::new();
        let _size = ComponentSize::Medium;
        let _variant = ChipVariant::Filter;
        let _config = ValidationConfig::default();
        let _animation = AnimationConfig::default();
    }

    #[test]
    fn test_constants_access() {
        // Test that constants are properly accessible
        assert_eq!(constants::animation::DEFAULT_DURATION_MS, 200);
        assert_eq!(constants::ui::MIN_TOUCH_TARGET_SIZE, 48.0);
        assert_eq!(constants::ui::MAX_LABEL_LENGTH, 200);
        assert_eq!(constants::chips::MAX_LABEL_LENGTH, 100);

        // Test that component sizes use constants correctly
        assert_eq!(
            ComponentSize::Large.touch_target_size(),
            constants::sizes::touch_targets::LARGE
        );
        assert_eq!(
            ComponentSize::Medium.size_px(),
            constants::sizes::MEDIUM_SIZE_PX
        );
    }

    #[test]
    fn test_modular_refactoring_maintains_api() {
        // Test that the refactored module structure maintains the same public API
        let props = ComponentProps::new().with_label("Test");
        let checkbox = CheckboxState::Unchecked;
        let switch = SwitchState::Off;
        let chip = ChipState::Unselected;

        // Validation functions should still work
        assert!(validate_checkbox_state(checkbox, &props).is_ok());
        assert!(validate_switch_state(switch, &props).is_ok());

        // State methods should still work
        assert_eq!(checkbox.toggle(), CheckboxState::Checked);
        assert_eq!(switch.toggle(), SwitchState::On);
        assert_eq!(chip.toggle(), ChipState::Selected);
    }
}
