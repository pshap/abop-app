//! Common builder functionality to reduce DRY violations
//!
//! This module provides shared functionality for all selection builders
//! without changing the public API structure.

use super::super::common::prelude::*;
use super::super::common::{env_has_reduced_motion, validate_props};

// ============================================================================
// Common Builder Fields (Composition over Inheritance)
// ============================================================================

/// Common builder state shared across all selection components
#[derive(Debug, Clone)]
pub struct CommonBuilderState {
    /// Common component properties (label, disabled, size, metadata)
    pub props: ComponentProps,
    /// Whether the component is in an error state
    pub error_state: bool,
    /// Validation configuration for this component
    pub validation_config: ValidationConfig,
    /// Animation configuration for state transitions
    pub animation_config: AnimationConfig,
}

impl CommonBuilderState {
    /// Create new common builder state with default configuration
    #[must_use]
    pub fn new(animation_config: AnimationConfig) -> Self {
        Self {
            props: ComponentProps::new(),
            error_state: false,
            validation_config: super::super::defaults::default_validation_config(),
            animation_config,
        }
    }
}

// ============================================================================
// Common Builder Methods (Trait for shared functionality)
// ============================================================================

/// Trait providing common builder methods for all selection components
///
/// This trait provides a consistent API for building Material Design 3 selection
/// components (checkboxes, switches, radios) while eliminating code duplication.
/// All builders that implement this trait gain access to common functionality
/// like label setting, size configuration, animation controls, and validation.
///
/// # Examples
///
/// ```rust
/// use abop_gui::styling::material::components::selection::*;
///
/// // All selection builders support the same common methods
/// let checkbox = CheckboxBuilder::unchecked()
///     .label("Enable notifications")
///     .size(ComponentSize::Large)
///     .animations_enabled(true)
///     .build()
///     .expect("Valid checkbox");
///
/// let switch = SwitchBuilder::off()
///     .label("Dark mode")
///     .size(ComponentSize::Medium)
///     .with_metadata("theme", "primary")
///     .build()
///     .expect("Valid switch");
/// ```
pub trait CommonSelectionBuilder {
    /// Get mutable access to the common builder state
    fn common_state_mut(&mut self) -> &mut CommonBuilderState;

    /// Get immutable access to the common builder state
    fn common_state(&self) -> &CommonBuilderState;

    /// Set the component label
    #[must_use]
    fn label<T: Into<String>>(mut self, label: T) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut().props.label = Some(label.into());
        self
    }

    /// Set the disabled state
    #[must_use]
    fn disabled(mut self, disabled: bool) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut().props.disabled = disabled;
        self
    }

    /// Set the component size
    #[must_use]
    fn size(mut self, size: ComponentSize) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut().props.size = size;
        self
    }

    /// Add metadata key-value pair
    #[must_use]
    fn with_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self
    where
        Self: Sized,
    {
        // Use efficient mutable insert instead of cloning entire props
        self.common_state_mut().props.insert_metadata(key, value);
        self
    }

    /// Set error state
    #[must_use]
    fn error(mut self, error: bool) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut().error_state = error;
        self
    }

    /// Set validation configuration
    #[must_use]
    fn validation_config(mut self, config: ValidationConfig) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut().validation_config = config;
        self
    }

    /// Set animation configuration
    #[must_use]
    fn animation_config(mut self, config: AnimationConfig) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut().animation_config = config;
        self
    }

    /// Enable/disable animations
    #[must_use]
    fn animations_enabled(mut self, enabled: bool) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut().animation_config.enabled = enabled;
        self
    }

    /// Configure system reduced motion respect
    #[must_use]
    fn respect_reduced_motion(mut self, respect: bool) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut()
            .animation_config
            .respect_reduced_motion = respect;
        self
    }

    /// Set easing curve for animations
    #[must_use]
    fn easing(mut self, easing: EasingCurve) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut().animation_config.easing = easing;
        self
    }

    /// Configure animation duration
    #[must_use]
    fn animation_duration(mut self, duration: std::time::Duration) -> Self
    where
        Self: Sized,
    {
        self.common_state_mut().animation_config.duration = duration;
        self
    }

    /// Apply system preferences for animations
    #[must_use]
    fn with_system_preferences(mut self) -> Self
    where
        Self: Sized,
    {
        if env_has_reduced_motion() {
            self.common_state_mut().animation_config.enabled = false;
        }
        self
    }

    /// Get a reference to the component properties
    #[must_use]
    fn props(&self) -> &ComponentProps {
        &self.common_state().props
    }

    /// Check if component is in error state
    #[must_use]
    fn has_error(&self) -> bool {
        self.common_state().error_state
    }

    /// Validate the common properties
    fn validate_common(&self) -> Result<(), SelectionError> {
        validate_props(
            &self.common_state().props,
            &self.common_state().validation_config,
        )
    }

    /// Check if the component should animate
    #[must_use]
    fn should_animate(&self) -> bool {
        let config = &self.common_state().animation_config;
        config.enabled && (!config.respect_reduced_motion || !env_has_reduced_motion())
    }
}

// ============================================================================
// Macro for implementing the trait (reduces boilerplate)
// ============================================================================

/// Macro to implement CommonSelectionBuilder for a builder struct
#[macro_export]
macro_rules! impl_common_selection_builder {
    ($builder_type:ty, $common_field:ident) => {
        impl CommonSelectionBuilder for $builder_type {
            fn common_state_mut(&mut self) -> &mut CommonBuilderState {
                &mut self.$common_field
            }

            fn common_state(&self) -> &CommonBuilderState {
                &self.$common_field
            }
        }
    };
    // Generic version for builders with type parameters
    ($builder_type:ident<$($generic:ident),+>, $common_field:ident, where $($bounds:tt)+) => {
        impl<$($generic),+> CommonSelectionBuilder for $builder_type<$($generic),+>
        where
            $($bounds)+
        {
            fn common_state_mut(&mut self) -> &mut CommonBuilderState {
                &mut self.$common_field
            }

            fn common_state(&self) -> &CommonBuilderState {
                &self.$common_field
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // Test struct to verify the trait works
    #[derive(Debug, Clone)]
    struct TestBuilder {
        common: CommonBuilderState,
    }

    impl TestBuilder {
        fn new() -> Self {
            Self {
                common: CommonBuilderState::new(AnimationConfig {
                    duration: Duration::from_millis(200),
                    enabled: true,
                    respect_reduced_motion: true,
                    easing: EasingCurve::Standard,
                }),
            }
        }
    }

    impl_common_selection_builder!(TestBuilder, common);

    #[test]
    fn test_common_builder_functionality() {
        let builder = TestBuilder::new()
            .label("Test Label")
            .disabled(true)
            .size(ComponentSize::Large)
            .error(true);

        assert_eq!(builder.props().label, Some("Test Label".to_string()));
        assert!(builder.props().disabled);
        assert_eq!(builder.props().size, ComponentSize::Large);
        assert!(builder.has_error());
    }

    #[test]
    fn test_animation_configuration() {
        let builder = TestBuilder::new()
            .animations_enabled(false)
            .respect_reduced_motion(false);

        assert!(!builder.common_state().animation_config.enabled);
        assert!(
            !builder
                .common_state()
                .animation_config
                .respect_reduced_motion
        );
    }

    #[test]
    fn test_metadata() {
        let builder = TestBuilder::new()
            .with_metadata("icon", "check")
            .with_metadata("color", "primary");

        assert_eq!(builder.props().get_metadata("icon"), Some("check"));
        assert_eq!(builder.props().get_metadata("color"), Some("primary"));
    }

    #[test]
    fn test_validation() {
        let valid_builder = TestBuilder::new().label("Valid Label");
        assert!(valid_builder.validate_common().is_ok());

        let invalid_builder = TestBuilder::new().label("x".repeat(300));
        assert!(invalid_builder.validate_common().is_err());
    }
}
