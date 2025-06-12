//! Advanced builder patterns and composition utilities
//!
//! This module provides sophisticated builder patterns including:
//! - Conditional building with validation
//! - Builder composition for complex configurations
//! - State validation at compile time
//! - Performance-optimized builder method chaining

use super::super::common::prelude::*;

// Type aliases for complex function types
/// Type alias for configuration functions used in builder patterns
///
/// This reduces complexity and improves readability of function signatures
/// that accept configuration callbacks for builder customization.
pub type ConfigurationFn<B> = Box<dyn Fn(B) -> Result<B, SelectionError>>;

// ============================================================================
// Core Builder Trait System
// ============================================================================

/// Core builder trait for all selection components
pub trait ComponentBuilder<T> {
    /// The component type being built
    type Component;

    /// The error type for validation failures
    type Error;

    /// Build the component with validation
    fn build(self) -> Result<Self::Component, Self::Error>;

    /// Build the component without validation (for internal use)
    fn build_unchecked(self) -> Self::Component;

    /// Validate the current builder state
    fn validate(&self) -> Result<(), Self::Error>;
}

/// Trait for builders that support conditional configuration
pub trait ConditionalBuilder<T>: ComponentBuilder<T> {
    /// Apply configuration conditionally
    fn when(self, condition: bool, f: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        if condition { f(self) } else { self }
    }

    /// Apply configuration if Some value is provided
    fn when_some<U>(self, value: Option<U>, f: impl FnOnce(Self, U) -> Self) -> Self
    where
        Self: Sized,
    {
        if let Some(val) = value {
            f(self, val)
        } else {
            self
        }
    }

    /// Apply fallible configuration conditionally
    fn try_when(
        self,
        condition: bool,
        f: impl FnOnce(Self) -> Result<Self, Self::Error>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if condition { f(self) } else { Ok(self) }
    }
}

/// Trait for builders that support batch configuration
pub trait BatchBuilder<T>: ComponentBuilder<T> {
    /// Apply multiple configurations in sequence
    fn configure(self, configs: &[impl Fn(Self) -> Self]) -> Self
    where
        Self: Sized + Clone,
    {
        configs
            .iter()
            .fold(self, |builder, config| config(builder.clone()))
    }
}

// ============================================================================
// Phase 2: Advanced Builder Patterns
// ============================================================================

/// Extended conditional builder with validation support
pub trait AdvancedConditionalBuilder<T>: ConditionalBuilder<T> {
    /// Apply a configuration when a validation condition passes
    fn when_validated<F, G>(self, validator: F, config: G) -> Result<Self, SelectionError>
    where
        Self: Sized,
        F: FnOnce(&Self) -> Result<(), SelectionError>,
        G: FnOnce(Self) -> Self,
    {
        validator(&self)?;
        Ok(config(self))
    }
}

/// Trait for builders that support state validation at compile time
pub trait StatefulBuilder<State> {
    /// Validate state transitions
    fn validate_state_transition(&self, new_state: State) -> Result<(), SelectionError>;

    /// Apply state with validation
    fn apply_state_validated(self, state: State) -> Result<Self, SelectionError>
    where
        Self: Sized;
}

/// Builder composition utility for complex configurations
pub struct BuilderComposer<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> BuilderComposer<T> {
    /// Create a new builder composer
    #[must_use]
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    /// Compose multiple builder configurations
    pub fn compose<B: ComponentBuilder<T> + Clone>(
        &self,
        builder: B,
        configurations: &[ConfigurationFn<B>],
    ) -> Result<B, SelectionError> {
        configurations
            .iter()
            .try_fold(builder, |acc, config| config(acc.clone()))
    }
}

impl<T> Default for BuilderComposer<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Enhanced Common Builder Methods Macro
// ============================================================================

/// Enhanced macro to generate common builder methods with Phase 2 improvements
/// Includes validation, error handling, performance optimizations, and conditional building
macro_rules! impl_common_builder_methods {
    ($builder:ty) => {
        impl $builder {
            /// Set disabled state
            #[must_use]
            #[inline]
            pub const fn disabled(mut self, disabled: bool) -> Self {
                self.props.disabled = disabled;
                self
            }

            /// Set component size
            #[must_use]
            #[inline]
            pub const fn size(mut self, size: ComponentSize) -> Self {
                self.props.size = size;
                self
            }

            /// Set error state for validation feedback
            #[must_use]
            #[inline]
            pub const fn error(mut self, error: bool) -> Self {
                self.error_state = error;
                self
            }

            /// Set validation configuration
            #[must_use]
            #[inline]
            pub fn validation(mut self, config: ValidationConfig) -> Self {
                self.validation_config = config;
                self
            }

            /// Set animation configuration
            #[must_use]
            #[inline]
            pub const fn animation(mut self, config: AnimationConfig) -> Self {
                self.animation_config = config;
                self
            }

            /// Check if error state is enabled
            #[must_use]
            #[inline]
            pub const fn has_error(&self) -> bool {
                self.error_state
            }

            // Phase 2: Advanced Builder Methods

            /// Set disabled state conditionally
            #[must_use]
            #[inline]
            pub fn disabled_when(self, condition: bool) -> Self {
                if condition { self.disabled(true) } else { self }
            }

            /// Set error state conditionally
            #[must_use]
            #[inline]
            pub fn error_when(self, condition: bool) -> Self {
                if condition { self.error(true) } else { self }
            }

            /// Apply size with validation
            pub fn size_validated(
                self,
                size: ComponentSize,
                min_size: Option<ComponentSize>,
                max_size: Option<ComponentSize>,
            ) -> Result<Self, SelectionError> {
                if let Some(min) = min_size {
                    if size < min {
                        return Err(SelectionError::ValidationError(format!(
                            "Size {:?} is smaller than minimum {:?}",
                            size, min
                        )));
                    }
                }

                if let Some(max) = max_size {
                    if size > max {
                        return Err(SelectionError::ValidationError(format!(
                            "Size {:?} is larger than maximum {:?}",
                            size, max
                        )));
                    }
                }

                Ok(self.size(size))
            }

            /// Apply multiple configurations in a validated chain
            pub fn configure_chain(
                self,
                configurations: &[ConfigurationFn<Self>],
            ) -> Result<Self, SelectionError>
            where
                Self: Clone,
            {
                configurations
                    .iter()
                    .try_fold(self, |builder, config| config(builder.clone()))
            }

            /// Reset to default configuration
            #[must_use]
            pub fn reset_to_defaults(mut self) -> Self {
                self.props = ComponentProps::new();
                self.error_state = false;
                self.validation_config = super::super::defaults::default_validation_config();
                self
            }

            /// Get a summary of current configuration
            #[must_use]
            pub fn configuration_summary(&self) -> ConfigurationSummary {
                ConfigurationSummary {
                    disabled: self.props.disabled,
                    size: self.props.size,
                    has_label: self.props.label.is_some(),
                    label_length: self.props.label.as_ref().map(|l| l.len()).unwrap_or(0),
                    has_error: self.error_state,
                    animation_enabled: self.animation_config.enabled,
                }
            }
        }
    };
}

// Make the macro available to other modules
pub(crate) use impl_common_builder_methods;
