//! Animation patterns for Material Design 3 motion system
//!
//! Provides high-level animation patterns for common UI interactions,
//! mapping user intents to appropriate duration and easing combinations.

use super::{DurationCategory, DurationLevel, EasingType};

/// Animation patterns for different interaction types
///
/// Defines common animation patterns used in Material Design interfaces.
/// Each pattern has recommended duration and easing characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationPattern {
    /// Simple state changes like hover or focus states (short, standard easing)
    SimpleStateChange,
    /// Complex state changes like expand/collapse (medium, emphasized easing)
    ComplexStateChange,
    /// Container transformations like page transitions (long, emphasized decelerate)
    ContainerTransform,
    /// Shared element transitions between views (long, emphasized)
    SharedElementTransition,
    /// Fade in/out animations (short, linear)
    FadeInOut,
    /// Scale animations for emphasis (short, standard decelerate)
    Scale,
    /// Slide animations for navigation (medium, standard decelerate)
    Slide,
    /// Bounce animations for feedback (short, emphasized)
    Bounce,
    /// Elastic animations for playful interactions (medium, emphasized decelerate)
    Elastic,
    /// Reveal animations for progressive disclosure (medium, standard)
    Reveal,
    /// Dismiss animations for removing elements (short, standard accelerate)
    Dismiss,
    /// Loading animations for asynchronous operations (long, linear)
    Loading,
}

/// Animation pattern configuration
///
/// Maps animation patterns to their recommended duration and easing settings.
/// This provides a high-level interface for common animation scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatternConfig {
    /// Duration category for this pattern
    pub duration_category: DurationCategory,
    /// Duration level within the category
    pub duration_level: DurationLevel,
    /// Recommended easing type
    pub easing_type: EasingType,
}

impl AnimationPattern {
    /// Get the configuration for this pattern
    ///
    /// Returns the recommended duration and easing settings based on
    /// Material Design 3 motion guidelines.
    #[must_use]
    pub const fn config(self) -> PatternConfig {
        match self {
            Self::SimpleStateChange => PatternConfig {
                duration_category: DurationCategory::Short,
                duration_level: DurationLevel::Level2,
                easing_type: EasingType::Standard,
            },
            Self::ComplexStateChange => PatternConfig {
                duration_category: DurationCategory::Medium,
                duration_level: DurationLevel::Level2,
                easing_type: EasingType::Emphasized,
            },
            Self::ContainerTransform => PatternConfig {
                duration_category: DurationCategory::Long,
                duration_level: DurationLevel::Level3,
                easing_type: EasingType::EmphasizedDecelerate,
            },
            Self::SharedElementTransition => PatternConfig {
                duration_category: DurationCategory::Long,
                duration_level: DurationLevel::Level2,
                easing_type: EasingType::Emphasized,
            },
            Self::FadeInOut => PatternConfig {
                duration_category: DurationCategory::Short,
                duration_level: DurationLevel::Level1,
                easing_type: EasingType::Linear,
            },
            Self::Scale => PatternConfig {
                duration_category: DurationCategory::Short,
                duration_level: DurationLevel::Level2,
                easing_type: EasingType::StandardDecelerate,
            },
            Self::Slide => PatternConfig {
                duration_category: DurationCategory::Medium,
                duration_level: DurationLevel::Level2,
                easing_type: EasingType::StandardDecelerate,
            },
            Self::Bounce => PatternConfig {
                duration_category: DurationCategory::Short,
                duration_level: DurationLevel::Level3,
                easing_type: EasingType::Emphasized,
            },
            Self::Elastic => PatternConfig {
                duration_category: DurationCategory::Medium,
                duration_level: DurationLevel::Level3,
                easing_type: EasingType::EmphasizedDecelerate,
            },
            Self::Reveal => PatternConfig {
                duration_category: DurationCategory::Medium,
                duration_level: DurationLevel::Level1,
                easing_type: EasingType::Standard,
            },
            Self::Dismiss => PatternConfig {
                duration_category: DurationCategory::Short,
                duration_level: DurationLevel::Level2,
                easing_type: EasingType::StandardAccelerate,
            },
            Self::Loading => PatternConfig {
                duration_category: DurationCategory::Long,
                duration_level: DurationLevel::Level1,
                easing_type: EasingType::Linear,
            },
        }
    }

    /// Get duration for this pattern
    #[must_use]
    pub const fn duration(self) -> std::time::Duration {
        let config = self.config();
        super::MotionTokens::duration(config.duration_category, config.duration_level)
    }

    /// Get easing curve for this pattern
    #[must_use]
    pub fn easing(self) -> &'static super::EasingCurve {
        let config = self.config();
        super::MotionTokens::easing(config.easing_type)
    }

    /// Get all available patterns grouped by use case
    #[must_use]
    pub const fn by_use_case() -> PatternsByUseCase {
        PatternsByUseCase {
            micro_interactions: &[
                Self::SimpleStateChange,
                Self::FadeInOut,
                Self::Scale,
                Self::Bounce,
            ],
            navigation: &[
                Self::Slide,
                Self::ContainerTransform,
                Self::SharedElementTransition,
            ],
            state_changes: &[Self::ComplexStateChange, Self::Reveal, Self::Dismiss],
            feedback: &[Self::Bounce, Self::Elastic, Self::Loading],
        }
    }
}

/// Animation patterns grouped by common use cases
#[derive(Debug)]
pub struct PatternsByUseCase {
    /// Quick micro-interactions and state changes
    pub micro_interactions: &'static [AnimationPattern],
    /// Navigation and page transitions
    pub navigation: &'static [AnimationPattern],
    /// Complex state changes and progressive disclosure
    pub state_changes: &'static [AnimationPattern],
    /// User feedback and loading states
    pub feedback: &'static [AnimationPattern],
}

/// Macro to generate pattern selector methods
///
/// This eliminates the DRY violation by centralizing the mapping between
/// selector method names and their corresponding AnimationPattern variants.
/// Each entry defines: (method_name, documentation, AnimationPattern_variant)
macro_rules! define_pattern_selectors {
    ($(
        ($method_name:ident, $doc:expr, $pattern:ident)
    ),* $(,)?) => {
        /// Context-aware pattern selection
        ///
        /// Provides intelligent pattern selection based on animation context.
        /// This helps developers choose appropriate patterns for their use cases.
        pub struct PatternSelector;

        impl PatternSelector {
            $(
                #[doc = $doc]
                #[must_use]
                pub const fn $method_name() -> AnimationPattern {
                    AnimationPattern::$pattern
                }
            )*
        }
    };
}

// Define all pattern selectors in one centralized location
define_pattern_selectors! {
    (hover_focus, "Select pattern for hover/focus state changes", SimpleStateChange),
    (button_press, "Select pattern for button press feedback", Scale),
    (menu_appear, "Select pattern for menu animations", Reveal),
    (menu_dismiss, "Select pattern for menu dismissal", Dismiss),
    (modal_dialog, "Select pattern for modal dialogs", ContainerTransform),
    (page_transition, "Select pattern for page transitions", SharedElementTransition),
    (loading_indicator, "Select pattern for loading indicators", Loading),
    (element_removal, "Select pattern for element removal", Dismiss),
    (element_appearance, "Select pattern for element appearance", FadeInOut),
    (drawer_slide, "Select pattern for drawer/sidebar animations", Slide),
    (accordion_toggle, "Select pattern for accordion/collapsible content", ComplexStateChange),
    (tab_transition, "Select pattern for tab transitions", Slide),
    (toast_notification, "Select pattern for toast/snackbar animations", Slide),
    (fab_animation, "Select pattern for floating action button animations", Scale),
    (card_interaction, "Select pattern for card interactions", Scale),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_pattern_configs() {
        let simple = AnimationPattern::SimpleStateChange.config();
        assert_eq!(simple.duration_category, DurationCategory::Short);
        assert_eq!(simple.duration_level, DurationLevel::Level2);
        assert_eq!(simple.easing_type, EasingType::Standard);

        let complex = AnimationPattern::ComplexStateChange.config();
        assert_eq!(complex.duration_category, DurationCategory::Medium);
        assert_eq!(complex.easing_type, EasingType::Emphasized);
    }

    #[test]
    fn test_pattern_duration_access() {
        let duration = AnimationPattern::SimpleStateChange.duration();
        assert_eq!(duration, Duration::from_millis(100));

        let long_duration = AnimationPattern::ContainerTransform.duration();
        assert_eq!(long_duration, Duration::from_millis(550));
    }

    #[test]
    fn test_pattern_easing_access() {
        let easing = AnimationPattern::SimpleStateChange.easing();
        assert_eq!(easing.name, "standard");

        let emphasized = AnimationPattern::ComplexStateChange.easing();
        assert_eq!(emphasized.name, "emphasized");
    }

    #[test]
    fn test_pattern_use_cases() {
        let use_cases = AnimationPattern::by_use_case();

        assert!(
            use_cases
                .micro_interactions
                .contains(&AnimationPattern::SimpleStateChange)
        );
        assert!(use_cases.navigation.contains(&AnimationPattern::Slide));
        assert!(
            use_cases
                .state_changes
                .contains(&AnimationPattern::ComplexStateChange)
        );
        assert!(use_cases.feedback.contains(&AnimationPattern::Bounce));
    }

    #[test]
    fn test_pattern_selectors() {
        assert_eq!(
            PatternSelector::hover_focus(),
            AnimationPattern::SimpleStateChange
        );
        assert_eq!(PatternSelector::button_press(), AnimationPattern::Scale);
        assert_eq!(
            PatternSelector::modal_dialog(),
            AnimationPattern::ContainerTransform
        );
        assert_eq!(
            PatternSelector::loading_indicator(),
            AnimationPattern::Loading
        );
    }

    #[test]
    fn test_all_patterns_have_valid_configs() {
        let all_patterns = [
            AnimationPattern::SimpleStateChange,
            AnimationPattern::ComplexStateChange,
            AnimationPattern::ContainerTransform,
            AnimationPattern::SharedElementTransition,
            AnimationPattern::FadeInOut,
            AnimationPattern::Scale,
            AnimationPattern::Slide,
            AnimationPattern::Bounce,
            AnimationPattern::Elastic,
            AnimationPattern::Reveal,
            AnimationPattern::Dismiss,
            AnimationPattern::Loading,
        ];

        for pattern in &all_patterns {
            let _config = pattern.config();
            let duration = pattern.duration();
            let easing = pattern.easing();

            // All patterns should have non-zero durations
            assert!(duration.as_millis() > 0);

            // All patterns should have valid easing curves
            assert!(!easing.name.is_empty());
        }
    }
}
