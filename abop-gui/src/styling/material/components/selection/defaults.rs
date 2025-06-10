//! Default configurations for Material Design 3 selection components
//!
//! This module provides default configurations and helper functions used across
//! all selection component builders.

use super::common::*;

/// Create default validation configuration for selection components
#[must_use]
pub fn default_validation_config() -> ValidationConfig {
    ValidationConfig::default()
}

/// Create default animation configuration for selection components
#[must_use]
pub fn default_animation_config() -> AnimationConfig {
    AnimationConfig {
        duration: std::time::Duration::from_millis(200),
        enabled: true,
        respect_reduced_motion: true,
        easing: EasingCurve::Standard,
    }
}

/// Check if the system has reduced motion enabled (placeholder implementation)
#[must_use]
pub fn system_has_reduced_motion() -> bool {
    // In a real implementation, this would check OS accessibility settings
    // For now, return false (no reduced motion)
    false
}

/// Default animation configuration specifically for chips
#[must_use]
pub fn default_chip_animation_config() -> AnimationConfig {
    default_animation_config()
}

/// Default animation configuration specifically for radio buttons
#[must_use]
pub fn default_radio_animation_config() -> AnimationConfig {
    default_animation_config()
}

/// Default animation configuration specifically for switches
#[must_use]
pub fn default_switch_animation_config() -> AnimationConfig {
    default_animation_config()
}
