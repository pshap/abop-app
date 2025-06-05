//! Animation helper trait for Material Design token system
//!
//! This trait provides helper methods for creating Material Design 3 compliant animations
//! with proper timing, easing, and reduced motion support.

use crate::styling::material::motion::{Animation, AnimationPattern};

/// Helper trait for animation-related functionality
///
/// This trait provides convenient methods for creating common animation patterns
/// used throughout Material Design interfaces. All animations follow Material Design 3
/// motion specifications with proper timing and easing curves.
pub trait AnimationHelpers {
    /// Create a fade in/out animation
    ///
    /// Returns a pre-configured animation for fade transitions, commonly used
    /// for content appearing and disappearing in the interface.
    ///
    /// # Returns
    /// Animation configured for smooth fade in/out transitions
    fn fade_animation(&self) -> Animation {
        Animation::from_pattern(AnimationPattern::FadeInOut)
    }

    /// Create a button press animation
    ///
    /// Returns a pre-configured animation for button interactions, providing
    /// immediate visual feedback for user actions.
    ///
    /// # Returns
    /// Animation configured for button state changes and press feedback
    fn button_animation(&self) -> Animation {
        Animation::from_pattern(AnimationPattern::SimpleStateChange)
    }

    /// Create a modal/dialog animation
    ///
    /// Returns a pre-configured animation for modal presentations, following
    /// Material Design container transform patterns.
    ///
    /// # Returns
    /// Animation configured for modal appearance and dismissal
    fn modal_animation(&self) -> Animation {
        Animation::from_pattern(AnimationPattern::ContainerTransform)
    }

    /// Create a slide animation
    ///
    /// Returns a pre-configured animation for slide transitions, commonly used
    /// in navigation and content transitions.
    ///
    /// # Returns
    /// Animation configured for smooth slide transitions
    fn slide_animation(&self) -> Animation {
        Animation::from_pattern(AnimationPattern::Slide)
    }

    /// Create a scale animation
    ///
    /// Returns a pre-configured animation for scale effects, used for emphasis
    /// and focus changes in the interface.
    ///
    /// # Returns
    /// Animation configured for scale transformations
    fn scale_animation(&self) -> Animation {
        Animation::from_pattern(AnimationPattern::Scale)
    }

    /// Create a dismiss animation
    ///
    /// Returns a pre-configured animation for dismissing elements, providing
    /// clear visual feedback when removing interface components.
    ///
    /// # Returns
    /// Animation configured for element dismissal
    fn dismiss_animation(&self) -> Animation {
        Animation::from_pattern(AnimationPattern::Dismiss)
    }

    /// Create a loading animation
    ///
    /// Returns a pre-configured animation for loading states with proper
    /// reduced motion support for accessibility.
    ///
    /// # Returns
    /// Animation configured for loading indicators with reduced motion respect
    fn loading_animation(&self) -> Animation {
        Animation::from_pattern(AnimationPattern::Loading).with_reduced_motion_respect(false)
    }

    /// Create a hover state animation
    ///
    /// Returns a pre-configured animation for hover effects with enhanced speed
    /// for immediate user feedback.
    ///
    /// # Returns
    /// Animation configured for hover state transitions with faster timing
    fn hover_animation(&self) -> Animation {
        Animation::from_pattern(AnimationPattern::SimpleStateChange).with_speed_factor(1.2)
    }
}
