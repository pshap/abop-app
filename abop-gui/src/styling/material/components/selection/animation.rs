//! Animation configuration for Material Design 3 selection components
//!
//! This module provides animation settings and easing curves for consistent
//! motion design across all selection components.

use std::time::Duration;

use super::constants;

// ============================================================================
// Animation Configuration
// ============================================================================

/// Animation configuration for selection components
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationConfig {
    /// Animation duration
    pub duration: Duration,
    /// Whether animations are enabled
    pub enabled: bool,
    /// Respect system reduced motion preferences
    pub respect_reduced_motion: bool,
    /// Easing curve type
    pub easing: EasingCurve,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(constants::animation::DEFAULT_DURATION_MS),
            enabled: true,
            respect_reduced_motion: true,
            easing: EasingCurve::Standard,
        }
    }
}

/// Material Design 3 easing curves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingCurve {
    /// Standard easing for common transitions
    Standard,
    /// Emphasized easing for important transitions
    Emphasized,
    /// Decelerated easing for entering elements
    Decelerated,
    /// Accelerated easing for exiting elements
    Accelerated,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_config_default() {
        let config = AnimationConfig::default();
        assert!(config.enabled);
        assert!(config.respect_reduced_motion);
        assert_eq!(config.easing, EasingCurve::Standard);
        assert_eq!(config.duration.as_millis(), 200);
    }
}
