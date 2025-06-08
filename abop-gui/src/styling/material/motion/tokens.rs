//! Duration tokens for Material Design 3 motion system
//!
//! Provides efficient access to Material Design 3 duration tokens through
//! compile-time constants and type-safe enums.

use std::time::Duration;

/// Duration token constants for Material Design 3 motion system
mod duration_constants {
    use std::time::Duration;

    // Short durations (50-200ms) for micro-interactions
    pub const SHORT_1: Duration = Duration::from_millis(50);
    pub const SHORT_2: Duration = Duration::from_millis(100);
    pub const SHORT_3: Duration = Duration::from_millis(150);
    pub const SHORT_4: Duration = Duration::from_millis(200);

    // Medium durations (250-400ms) for standard transitions
    pub const MEDIUM_1: Duration = Duration::from_millis(250);
    pub const MEDIUM_2: Duration = Duration::from_millis(300);
    pub const MEDIUM_3: Duration = Duration::from_millis(350);
    pub const MEDIUM_4: Duration = Duration::from_millis(400);

    // Long durations (450-600ms) for complex transitions
    pub const LONG_1: Duration = Duration::from_millis(450);
    pub const LONG_2: Duration = Duration::from_millis(500);
    pub const LONG_3: Duration = Duration::from_millis(550);
    pub const LONG_4: Duration = Duration::from_millis(600);

    // Extra long durations (700-1000ms) for major state changes
    pub const EXTRA_LONG_1: Duration = Duration::from_millis(700);
    pub const EXTRA_LONG_2: Duration = Duration::from_millis(800);
    pub const EXTRA_LONG_3: Duration = Duration::from_millis(900);
    pub const EXTRA_LONG_4: Duration = Duration::from_millis(1000);
}

/// Material Design duration categories
///
/// Groups animation durations into semantic categories based on
/// the complexity and scope of the animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurationCategory {
    /// Short animations (50-200ms) for micro-interactions and simple state changes
    Short,
    /// Medium animations (250-400ms) for standard UI transitions
    Medium,
    /// Long animations (450-600ms) for complex transitions and large area changes
    Long,
    /// Extra long animations (700-1000ms) for major state changes and page transitions
    ExtraLong,
}

/// Duration level for each category (1-4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurationLevel {
    /// Level 1 - Shortest duration in category
    Level1 = 1,
    /// Level 2 - Short-medium duration in category
    Level2 = 2,
    /// Level 3 - Medium-long duration in category
    Level3 = 3,
    /// Level 4 - Longest duration in category
    Level4 = 4,
}

impl From<u8> for DurationLevel {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Level1,
            2 => Self::Level2,
            3 => Self::Level3,
            4 => Self::Level4,
            _ => Self::Level2, // Default fallback
        }
    }
}

impl DurationLevel {
    /// Create `DurationLevel` from u8, returning None for invalid values
    #[must_use]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Level1),
            2 => Some(Self::Level2),
            3 => Some(Self::Level3),
            4 => Some(Self::Level4),
            _ => None,
        }
    }
}

/// Efficient motion token access
///
/// Provides O(1) access to Material Design 3 duration tokens using
/// compile-time constants. This replaces the old `MaterialMotion` struct
/// with a more memory-efficient approach.
pub struct MotionTokens;

impl MotionTokens {
    /// Get duration by category and level    ///
    /// This is a compile-time constant function that provides O(1) access
    /// to duration tokens without any memory allocation.
    #[must_use]
    pub const fn duration(category: DurationCategory, level: DurationLevel) -> Duration {
        use duration_constants::{
            EXTRA_LONG_1, EXTRA_LONG_2, EXTRA_LONG_3, EXTRA_LONG_4, LONG_1, LONG_2, LONG_3, LONG_4,
            MEDIUM_1, MEDIUM_2, MEDIUM_3, MEDIUM_4, SHORT_1, SHORT_2, SHORT_3, SHORT_4,
        };

        match (category, level) {
            (DurationCategory::Short, DurationLevel::Level1) => SHORT_1,
            (DurationCategory::Short, DurationLevel::Level2) => SHORT_2,
            (DurationCategory::Short, DurationLevel::Level3) => SHORT_3,
            (DurationCategory::Short, DurationLevel::Level4) => SHORT_4,
            (DurationCategory::Medium, DurationLevel::Level1) => MEDIUM_1,
            (DurationCategory::Medium, DurationLevel::Level2) => MEDIUM_2,
            (DurationCategory::Medium, DurationLevel::Level3) => MEDIUM_3,
            (DurationCategory::Medium, DurationLevel::Level4) => MEDIUM_4,
            (DurationCategory::Long, DurationLevel::Level1) => LONG_1,
            (DurationCategory::Long, DurationLevel::Level2) => LONG_2,
            (DurationCategory::Long, DurationLevel::Level3) => LONG_3,
            (DurationCategory::Long, DurationLevel::Level4) => LONG_4,
            (DurationCategory::ExtraLong, DurationLevel::Level1) => EXTRA_LONG_1,
            (DurationCategory::ExtraLong, DurationLevel::Level2) => EXTRA_LONG_2,
            (DurationCategory::ExtraLong, DurationLevel::Level3) => EXTRA_LONG_3,
            (DurationCategory::ExtraLong, DurationLevel::Level4) => EXTRA_LONG_4,
        }
    }

    /// Get duration with reduced motion support
    ///
    /// When `reduced_motion` is true, returns 10% of the normal duration
    /// for accessibility compliance.
    #[must_use]
    pub fn duration_with_config(
        category: DurationCategory,
        level: DurationLevel,
        scale: f32,
        reduced_motion: bool,
    ) -> Duration {        let base_duration = Self::duration(category, level);
        let base_millis = base_duration.as_millis();
        let base_f64 = base_millis as f64;
        
        if reduced_motion {
            // Safe conversion for reduced motion (10% of base)
            let scaled_ms = (base_f64 * 0.1).round().clamp(0.0, u64::MAX as f64);
            Duration::from_millis(scaled_ms as u64)
        } else {
            // Safe conversion for normal motion
            let scale_clamped = f64::from(scale.max(0.1));
            let scaled_ms = (base_f64 * scale_clamped)
                .round()
                .clamp(0.0, u64::MAX as f64);
            Duration::from_millis(scaled_ms as u64)
        }
    }

    /// Get all durations for a category as an array
    #[must_use]
    pub const fn category_durations(category: DurationCategory) -> [Duration; 4] {
        [
            Self::duration(category, DurationLevel::Level1),
            Self::duration(category, DurationLevel::Level2),
            Self::duration(category, DurationLevel::Level3),
            Self::duration(category, DurationLevel::Level4),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_constants() {
        assert_eq!(
            MotionTokens::duration(DurationCategory::Short, DurationLevel::Level1),
            Duration::from_millis(50)
        );
        assert_eq!(
            MotionTokens::duration(DurationCategory::Short, DurationLevel::Level4),
            Duration::from_millis(200)
        );
        assert_eq!(
            MotionTokens::duration(DurationCategory::ExtraLong, DurationLevel::Level4),
            Duration::from_millis(1000)
        );
    }

    #[test]
    fn test_duration_level_conversion() {
        assert_eq!(DurationLevel::from(1), DurationLevel::Level1);
        assert_eq!(DurationLevel::from(4), DurationLevel::Level4);
        assert_eq!(DurationLevel::from(99), DurationLevel::Level2); // Fallback

        assert_eq!(DurationLevel::from_u8(1), Some(DurationLevel::Level1));
        assert_eq!(DurationLevel::from_u8(99), None);
    }

    #[test]
    fn test_reduced_motion() {
        let normal = MotionTokens::duration_with_config(
            DurationCategory::Medium,
            DurationLevel::Level2,
            1.0,
            false,
        );
        let reduced = MotionTokens::duration_with_config(
            DurationCategory::Medium,
            DurationLevel::Level2,
            1.0,
            true,
        );

        assert_eq!(normal, Duration::from_millis(300));
        assert_eq!(reduced, Duration::from_millis(30)); // 10% of 300ms
    }

    #[test]
    fn test_duration_scaling() {
        let normal = MotionTokens::duration_with_config(
            DurationCategory::Short,
            DurationLevel::Level1,
            1.0,
            false,
        );
        let scaled = MotionTokens::duration_with_config(
            DurationCategory::Short,
            DurationLevel::Level1,
            2.0,
            false,
        );

        assert_eq!(normal, Duration::from_millis(50));
        assert_eq!(scaled, Duration::from_millis(100));
    }

    #[test]
    fn test_category_durations() {
        let short_durations = MotionTokens::category_durations(DurationCategory::Short);
        assert_eq!(
            short_durations,
            [
                Duration::from_millis(50),
                Duration::from_millis(100),
                Duration::from_millis(150),
                Duration::from_millis(200),
            ]
        );
    }
}
