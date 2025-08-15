//! Animation state management and progress calculation
//!
//! Provides efficient animation state tracking with accurate progress calculation
//! and integration helpers for Iced animations.

use super::{
    AnimationPattern, DurationCategory, DurationLevel, EasingCurve, EasingType, MotionTokens,
};
use std::time::{Duration, Instant};

/// Animation state enumeration
///
/// Represents the current lifecycle state of an animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationState {
    /// Animation is ready to start but hasn't been started yet
    #[default]
    Ready,
    /// Animation is currently running
    Running,
    /// Animation has been paused (future feature)
    Paused,
    /// Animation has completed successfully
    Completed,
    /// Animation was stopped/cancelled before completion
    Stopped,
}

impl AnimationState {
    /// Check if the animation is in an active state (running or paused)
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self, Self::Running | Self::Paused)
    }

    /// Check if the animation is finished (completed or stopped)
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        matches!(self, Self::Completed | Self::Stopped)
    }

    /// Check if the animation can be started
    #[must_use]
    pub const fn can_start(&self) -> bool {
        matches!(self, Self::Ready | Self::Stopped)
    }
}

/// Animation specification and state management
///
/// Represents a complete animation with duration, easing, and progress calculation.
/// This replaces the old `AnimationSpec` with improved state management.
#[derive(Debug, Clone)]
pub struct Animation {
    /// Duration of the animation
    duration: Duration,
    /// Easing curve for the animation
    easing: &'static EasingCurve,
    /// Animation start time (None if not started)
    start_time: Option<Instant>,
    /// Current state of the animation
    state: AnimationState,
    /// Speed factor for this animation (1.0 = normal speed)
    speed_factor: f32,
    /// Whether this animation should respect reduced motion preferences
    respect_reduced_motion: bool,
}

impl Animation {
    /// Create a new animation with duration and easing
    #[must_use]
    pub const fn new(duration: Duration, easing: &'static EasingCurve) -> Self {
        Self {
            duration,
            easing,
            start_time: None,
            state: AnimationState::Ready,
            speed_factor: 1.0,
            respect_reduced_motion: true,
        }
    }

    /// Create animation from a predefined pattern
    ///    /// This is the most convenient way to create animations for common UI interactions.
    #[must_use]
    pub fn from_pattern(pattern: AnimationPattern) -> Self {
        let config = pattern.config();
        Self::new(
            MotionTokens::duration(config.duration_category, config.duration_level),
            MotionTokens::easing(config.easing_type),
        )
    }

    /// Create a custom animation with explicit parameters
    #[must_use]
    pub fn custom(
        category: DurationCategory,
        level: DurationLevel,
        easing_type: EasingType,
    ) -> Self {
        Self::new(
            MotionTokens::duration(category, level),
            MotionTokens::easing(easing_type),
        )
    }

    /// Set speed factor for this animation
    ///
    /// Values > 1.0 make the animation faster, values < 1.0 make it slower.
    /// This is an alternative to global duration scaling.
    #[must_use]
    pub const fn with_speed_factor(mut self, factor: f32) -> Self {
        self.speed_factor = factor.max(0.1); // Minimum 10% speed
        self
    }

    /// Set whether this animation should respect reduced motion preferences
    #[must_use]
    pub const fn with_reduced_motion_respect(mut self, respect: bool) -> Self {
        self.respect_reduced_motion = respect;
        self
    }

    /// Start the animation
    ///
    /// Records the current time as the animation start time and updates state.
    pub fn start(&mut self) {
        if self.state.can_start() {
            self.start_time = Some(Instant::now());
            self.state = AnimationState::Running;
        }
    }

    /// Stop the animation
    ///
    /// Stops the animation and sets state to Stopped.
    pub const fn stop(&mut self) {
        self.state = AnimationState::Stopped;
    }

    /// Reset the animation to unstarted state
    pub const fn reset(&mut self) {
        self.start_time = None;
        self.state = AnimationState::Ready;
    }

    /// Get the current animation state
    #[must_use]
    pub fn state(&self) -> AnimationState {
        // Update state based on actual progress if running
        if self.state == AnimationState::Running && self.is_complete() {
            AnimationState::Completed
        } else {
            self.state
        }
    }

    /// Check if the animation has been started
    #[must_use]
    pub const fn is_started(&self) -> bool {
        self.start_time.is_some()
    }

    /// Get the effective duration considering speed factor and reduced motion
    #[must_use]
    pub fn effective_duration(&self) -> Duration {
        let mut duration = self.duration;

        // Apply speed factor with safe conversion
        let duration_ms = duration.as_millis() as f64; // u128 to f64 conversion
        let speed_adjusted_ms = (duration_ms / f64::from(self.speed_factor))
            .round()
            .clamp(0.0, u64::MAX as f64);
        duration = Duration::from_millis(speed_adjusted_ms as u64);

        // Apply reduced motion if enabled
        if self.respect_reduced_motion && Self::should_reduce_motion() {
            let duration_ms = duration.as_millis() as f64;
            let reduced_ms = (duration_ms * 0.1).round().clamp(0.0, u64::MAX as f64);
            duration = Duration::from_millis(reduced_ms as u64);
        }

        duration
    }

    /// Get current progress (0.0 to 1.0) based on elapsed time
    ///
    /// Returns None if the animation hasn't been started.
    /// Returns Some(progress) where progress is the eased value.
    #[must_use]
    pub fn progress(&self) -> Option<f32> {
        let start_time = self.start_time?;
        let elapsed = start_time.elapsed();
        let effective_duration = self.effective_duration();

        if effective_duration.is_zero() {
            return Some(1.0);
        }

        // Safe conversion using f64 to avoid precision loss
        let elapsed_ms = elapsed.as_millis() as f64;
        let effective_duration_ms = effective_duration.as_millis() as f64;

        let linear_progress = if effective_duration_ms > 0.0 {
            (elapsed_ms / effective_duration_ms).min(1.0) as f32
        } else {
            1.0
        };
        Some(self.easing.sample(linear_progress))
    }

    /// Get linear progress (0.0 to 1.0) without easing applied
    #[must_use]
    pub fn linear_progress(&self) -> Option<f32> {
        let start_time = self.start_time?;
        let elapsed = start_time.elapsed();
        let effective_duration = self.effective_duration();

        if effective_duration.is_zero() {
            return Some(1.0);
        }

        // Safe conversion using f64 to avoid precision loss
        let elapsed_ms = elapsed.as_millis() as f64;
        let duration_ms = effective_duration.as_millis() as f64;

        if duration_ms > 0.0 {
            Some((elapsed_ms / duration_ms).min(1.0) as f32)
        } else {
            Some(1.0)
        }
    }

    /// Check if animation is complete
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.start_time
            .is_some_and(|start_time| start_time.elapsed() >= self.effective_duration())
    }

    /// Get remaining time in the animation
    #[must_use]
    pub fn remaining_time(&self) -> Option<Duration> {
        let start_time = self.start_time?;
        let elapsed = start_time.elapsed();
        let effective_duration = self.effective_duration();

        if elapsed >= effective_duration {
            Some(Duration::ZERO)
        } else {
            Some(effective_duration - elapsed)
        }
    }

    /// Get elapsed time since animation started
    #[must_use]
    pub fn elapsed_time(&self) -> Option<Duration> {
        Some(self.start_time?.elapsed())
    }

    /// Get the original (unscaled) duration
    #[must_use]
    pub const fn original_duration(&self) -> Duration {
        self.duration
    }

    /// Get the easing curve
    #[must_use]
    pub const fn easing(&self) -> &EasingCurve {
        self.easing
    }

    /// Simple check for reduced motion preference
    ///
    /// Uses environment variables for cross-platform reduced motion detection.
    /// Can be enhanced with OS-specific APIs in the future.
    fn should_reduce_motion() -> bool {
        // Check common environment variables for reduced motion preference
        std::env::var("ABOP_REDUCE_MOTION").is_ok_and(|v| v == "1" || v.to_lowercase() == "true")
            || std::env::var("PREFER_REDUCED_MOTION")
                .is_ok_and(|v| v == "1" || v.to_lowercase() == "true")
    }
}

/// Animation builder for complex configurations
#[derive(Debug)]
pub struct AnimationBuilder {
    duration: Option<Duration>,
    easing: Option<&'static EasingCurve>,
    speed_factor: f32,
    respect_reduced_motion: bool,
}

impl Default for AnimationBuilder {
    fn default() -> Self {
        Self {
            duration: None,
            easing: None,
            speed_factor: 1.0,
            respect_reduced_motion: true,
        }
    }
}

impl AnimationBuilder {
    /// Create a new animation builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set duration using category and level
    #[must_use]
    pub const fn duration(mut self, category: DurationCategory, level: DurationLevel) -> Self {
        self.duration = Some(MotionTokens::duration(category, level));
        self
    }

    /// Set explicit duration
    #[must_use]
    pub const fn duration_millis(mut self, millis: u64) -> Self {
        self.duration = Some(Duration::from_millis(millis));
        self
    }

    /// Set easing curve
    #[must_use]
    pub fn easing(mut self, easing_type: EasingType) -> Self {
        self.easing = Some(MotionTokens::easing(easing_type));
        self
    }

    /// Set speed factor
    #[must_use]
    pub const fn speed_factor(mut self, factor: f32) -> Self {
        self.speed_factor = factor.max(0.1);
        self
    }

    /// Set reduced motion respect
    #[must_use]
    pub const fn respect_reduced_motion(mut self, respect: bool) -> Self {
        self.respect_reduced_motion = respect;
        self
    }

    /// Build the animation
    #[must_use]
    pub fn build(self) -> Animation {
        let duration = self.duration.unwrap_or(crate::styling::design_tokens::animation::SLOW_DURATION);
        let easing = self
            .easing
            .unwrap_or_else(|| MotionTokens::easing(EasingType::Standard));

        Animation {
            duration,
            easing,
            start_time: None,
            state: AnimationState::Ready,
            speed_factor: self.speed_factor,
            respect_reduced_motion: self.respect_reduced_motion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_animation_creation() {
        let animation = Animation::from_pattern(AnimationPattern::SimpleStateChange);
        assert!(!animation.is_started());
        assert!(animation.progress().is_none());
    }

    #[test]
    fn test_animation_progress() {
        let mut animation = Animation::from_pattern(AnimationPattern::FadeInOut);

        // Animation not started
        assert!(animation.progress().is_none());
        assert!(!animation.is_complete());

        // Start animation
        animation.start();
        assert!(animation.is_started());

        // Progress should be available
        let progress = animation.progress().unwrap();
        assert!((0.0..=1.0).contains(&progress));
    }

    #[test]
    fn test_animation_builder() {
        let animation = AnimationBuilder::new()
            .duration(DurationCategory::Short, DurationLevel::Level1)
            .easing(EasingType::Standard)
            .speed_factor(2.0)
            .build();

        assert_eq!(animation.original_duration(), Duration::from_millis(50));
        assert_eq!(animation.easing().name, "standard");
    }

    #[test]
    fn test_speed_factor() {
        let normal = Animation::from_pattern(AnimationPattern::SimpleStateChange);
        let fast =
            Animation::from_pattern(AnimationPattern::SimpleStateChange).with_speed_factor(2.0);

        assert_eq!(fast.effective_duration(), normal.effective_duration() / 2);
    }

    #[test]
    fn test_custom_animation() {
        let animation = Animation::custom(
            DurationCategory::Medium,
            DurationLevel::Level3,
            EasingType::Emphasized,
        );

        assert_eq!(animation.original_duration(), Duration::from_millis(350));
        assert_eq!(animation.easing().name, "emphasized");
    }

    #[test]
    fn test_animation_timing() {
        let mut animation = Animation::from_pattern(AnimationPattern::FadeInOut);
        animation.start();

        // Small delay to test timing
        thread::sleep(Duration::from_millis(1));

        let elapsed = animation.elapsed_time().unwrap();
        assert!(elapsed.as_millis() >= 1);

        let remaining = animation.remaining_time().unwrap();
        assert!(remaining < animation.effective_duration());
    }
}
