//! Material Design 3 Motion System - Modular Implementation
//!
//! This module provides a comprehensive and efficient implementation of the Material Design 3
//! motion system. It's been refactored for better performance, maintainability, and API clarity.
//!
//! # Architecture
//!
//! The motion system is split into focused modules:
//! - `tokens`: Duration constants and lookup functions
//! - `easing`: Cubic Bezier easing curves with accurate sampling
//! - `patterns`: High-level animation patterns for common UI interactions
//! - `animation`: Animation state management and progress calculation
//!
//! # Key Improvements
//!
//! - **Memory Efficient**: Static curve storage, no redundant data
//! - **Performance**: O(1) lookups, accurate cubic Bezier sampling
//! - **Type Safe**: Strong enum types prevent invalid configurations
//! - **Clean API**: Focused, single-responsibility interfaces
//!
//! # Public API
//!
//! The module provides specific exports instead of glob imports for better clarity:
//! - Animation types: [`Animation`], [`AnimationBuilder`], [`AnimationState`]
//! - Easing: [`EasingType`], [`EasingCurve`], [`CubicBezier`]
//! - Patterns: [`AnimationPattern`], [`PatternConfig`], [`PatternsByUseCase`], [`PatternSelector`]
//! - Tokens: [`MotionTokens`], [`DurationCategory`], [`DurationLevel`]

mod animation;
mod easing;
mod patterns;
mod tokens;

// Re-export specific public API items instead of glob imports
pub use animation::{Animation, AnimationBuilder, AnimationState};
pub use easing::{CubicBezier, EasingCurve, EasingType};
pub use patterns::{AnimationPattern, PatternConfig, PatternSelector, PatternsByUseCase};
pub use tokens::{DurationCategory, DurationLevel, MotionTokens};

// Convenience functions for quick access
impl MotionTokens {
    /// Create an animation from a pattern (convenience function)
    #[must_use]
    pub fn animation_from_pattern(pattern: AnimationPattern) -> Animation {
        Animation::from_pattern(pattern)
    }

    /// Create a custom animation
    #[must_use]
    pub fn custom_animation(
        category: DurationCategory,
        level: DurationLevel,
        easing_type: EasingType,
    ) -> Animation {
        Animation::new(Self::duration(category, level), Self::easing(easing_type))
    }
}
