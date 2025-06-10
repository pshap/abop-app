//! Material Design 3 Selection Components
//!
//! This module provides a comprehensive set of selection components following Material Design 3
//! guidelines. The components feature modern state-based design, builder patterns, comprehensive
//! validation, and preparation for future animation support.
//!
//! # Components
//!
//! - **Checkbox**: State-based checkbox with indeterminate support preparation
//! - **Radio**: Type-safe radio buttons with group management
//! - **Switch**: Modern switch component with Material Design 3 compliance
//! - **Chip**: Multiple chip variants (Assist, Filter, Input, Suggestion) with collections
//!
//! # Features
//!
//! - **State-Based Design**: Type-safe enums instead of boolean flags
//! - **Builder Patterns**: Fluent APIs with validation and conditional building
//! - **Modern Error Handling**: Comprehensive error types with `thiserror`
//! - **Trait System**: Unified behavior across components
//! - **Validation Framework**: Built-in validation with configurable rules
//! - **Animation Preparation**: Ready for Phase 6 animation integration
//! - **Material Design 3**: Compliant dimensions and styling
//!
//! # Quick Start
//!
//! ```rust
//! use abop_gui::styling::material::components::selection::*;
//!
//! // Create a checkbox
//! let checkbox = CheckboxBuilder::checked()
//!     .label("I agree to the terms")
//!     .size(ComponentSize::Large)
//!     .build()
//!     .expect("Valid checkbox");
//!
//! // Create a switch
//! let notifications = SwitchBuilder::on()
//!     .label("Enable notifications")
//!     .size(ComponentSize::Medium)
//!     .build()
//!     .expect("Valid switch");
//!
//! // Create a chip collection
//! let filters = ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
//!     .chip(ChipBuilder::filter("Price").build().unwrap())
//!     .chip(ChipBuilder::filter("Rating").build().unwrap())
//!     .build()
//!     .expect("Valid chip collection");
//! ```
//!
//! # Architecture
//!
//! The module is organized into focused submodules:
//!
//! - `common`: Shared types, traits, and validation framework
//! - `builder`: Advanced builder patterns with conditional/batch support
//! - `checkbox`: Checkbox component with indeterminate preparation
//! - `radio`: Radio buttons with type-safe group management
//! - `switch`: Switch component with Material Design 3 dimensions
//! - `chip`: Chip variants and collection management
//!
//! # Phase 4+ Preparation
//!
//! This module includes preparation for future phases:
//!
//! - **Phase 4**: Custom switch widget with Material Design 3 appearance
//! - **Phase 5**: Indeterminate checkbox visual rendering
//! - **Phase 6**: Animation support integration
//!
//! The API design ensures forward compatibility while providing immediate
//! modern functionality.

// Core module exports
pub mod builder;
pub mod checkbox;
pub mod chip;
pub mod common;
pub mod defaults;
pub mod radio;
pub mod switch;

#[cfg(test)]
pub mod tests;

// Re-export core types and traits from builder module only
pub use builder::{
    BatchBuilder, Checkbox, CheckboxBuilder, Chip, ChipBuilder, ComponentBuilder,
    ConditionalBuilder, Radio, RadioBuilder, Switch, SwitchBuilder,
};
// Re-export additional chip collection types from chip module
pub use chip::{
    ChipCollection, ChipCollectionBuilder, ChipSelectionMode, DEFAULT_ANIMATION_DURATION,
    MAX_CHIP_LABEL_LENGTH, filter_chip_collection, single_select_chip_collection,
};
pub use common::{
    AnimatedWidget, AnimationConfig, CheckboxState, ChipState, ChipVariant, ComponentProps,
    ComponentSize, EasingCurve, SelectionError, SelectionWidget, StatefulWidget, SwitchState,
    ValidationConfig, ValidationRule,
};
pub use radio::{RadioGroupBuilder, RadioGroupState};
pub use switch::SwitchDimensions;

/// A collection of filter chips supporting multiple selection
pub type FilterChipCollection = ChipCollection;

/// A collection of input chips (tags)
pub type InputChipCollection = ChipCollection;

/// Radio group for theme selection with static string references
pub type ThemeRadioGroup = RadioGroupState<&'static str>;

/// Switch component for settings toggles
pub type SettingsSwitch = Switch;

/// Checkbox component for agreement/consent dialogs
pub type AgreementCheckbox = Checkbox;

/// Convenient builder functions for quick component creation
pub mod builders {
    use super::*;
    /// Create a checkbox with common settings
    pub fn checkbox() -> CheckboxBuilder {
        Checkbox::new(CheckboxState::Unchecked)
    }

    /// Create a labeled checkbox
    pub fn labeled_checkbox(label: &str) -> CheckboxBuilder {
        Checkbox::new(CheckboxState::Unchecked).label(label)
    }
    /// Create a radio group builder
    pub fn radio_group<T>() -> RadioGroupBuilder<T>
    where
        T: Clone + PartialEq + Eq + std::hash::Hash + std::fmt::Debug,
    {
        RadioGroupBuilder::new()
    }
    /// Create a switch with common settings
    pub fn switch() -> SwitchBuilder {
        Switch::new(SwitchState::Off)
    }

    /// Create a labeled switch
    pub fn labeled_switch(label: &str) -> SwitchBuilder {
        Switch::new(SwitchState::Off).label(label)
    }
    /// Create a filter chip collection
    pub fn filter_chips() -> ChipCollection {
        ChipCollection::new(chip::ChipSelectionMode::Multiple)
    }

    /// Create an input chip collection (tags)
    pub fn input_chips() -> ChipCollection {
        ChipCollection::new(chip::ChipSelectionMode::None)
    }

    /// Create a single-selection chip collection
    pub fn choice_chips() -> ChipCollection {
        ChipCollection::new(chip::ChipSelectionMode::Single)
    }
    /// Create an assist chip
    pub fn assist_chip(label: &str) -> ChipBuilder {
        Chip::new(label, ChipVariant::Assist)
    }

    /// Create a filter chip
    pub fn filter_chip(label: &str) -> ChipBuilder {
        Chip::new(label, ChipVariant::Filter)
    }

    /// Create an input chip
    pub fn input_chip(label: &str) -> ChipBuilder {
        Chip::new(label, ChipVariant::Input)
    }

    /// Create a suggestion chip
    pub fn suggestion_chip(label: &str) -> ChipBuilder {
        Chip::new(label, ChipVariant::Suggestion)
    }
}

/// Validation utilities for selection components
pub mod validation {
    use super::*;

    /// Validate a collection of checkboxes
    pub fn validate_checkboxes(widgets: &[Checkbox]) -> Result<(), SelectionError> {
        for widget in widgets {
            widget.validate()?;
        }
        Ok(())
    }

    /// Check if all checkboxes in a collection are valid
    pub fn all_checkboxes_valid(widgets: &[Checkbox]) -> bool {
        widgets.iter().all(|w| w.validate().is_ok())
    }

    /// Get validation errors for a collection of checkboxes
    pub fn collect_checkbox_errors(widgets: &[Checkbox]) -> Vec<SelectionError> {
        widgets.iter().filter_map(|w| w.validate().err()).collect()
    }

    /// Validate component size constraints
    pub fn validate_size_constraints(
        size: ComponentSize,
        min_size: Option<ComponentSize>,
        max_size: Option<ComponentSize>,
    ) -> Result<(), SelectionError> {
        if let Some(min) = min_size
            && size < min
        {
            return Err(SelectionError::ValidationError(format!(
                "Size {size:?} is smaller than minimum {min:?}"
            )));
        }

        if let Some(max) = max_size
            && size > max
        {
            return Err(SelectionError::ValidationError(format!(
                "Size {size:?} is larger than maximum {max:?}"
            )));
        }

        Ok(())
    }
}

/// Utilities for working with selection component state
pub mod state_utils {
    use super::*;

    /// Convert checkbox state to boolean (treats indeterminate as false)
    pub fn checkbox_to_bool(state: CheckboxState) -> bool {
        matches!(state, CheckboxState::Checked)
    }

    /// Convert switch state to boolean
    pub fn switch_to_bool(state: SwitchState) -> bool {
        matches!(state, SwitchState::On)
    }

    /// Convert chip state to selection status
    pub fn chip_is_selected(state: ChipState) -> bool {
        matches!(state, ChipState::Selected)
    }
    /// Get all selected values from a radio group
    pub fn radio_group_selection<T>(group: &RadioGroupState<T>) -> Option<T>
    where
        T: Clone + PartialEq + Eq + std::hash::Hash,
    {
        group.selected_value()
    }
    /// Get all selected chip labels from a collection
    pub fn selected_chip_labels(collection: &ChipCollection) -> Vec<String> {
        collection
            .selected_chips()
            .iter()
            .map(|chip| chip.label().to_string())
            .collect()
    }

    /// Create a state summary for debugging
    pub fn create_state_summary(
        checkboxes: &[Checkbox],
        switches: &[Switch],
        chip_collections: &[ChipCollection],
    ) -> String {
        let mut summary = String::new();

        summary.push_str("=== Selection Component State Summary ===\n");

        summary.push_str(&format!("Checkboxes ({}): ", checkboxes.len()));
        for checkbox in checkboxes {
            summary.push_str(&format!("{:?} ", checkbox.state()));
        }
        summary.push('\n');

        summary.push_str(&format!("Switches ({}): ", switches.len()));
        for switch in switches {
            summary.push_str(&format!("{:?} ", switch.state()));
        }
        summary.push('\n');

        summary.push_str(&format!("Chip Collections ({}): ", chip_collections.len()));
        for collection in chip_collections {
            summary.push_str(&format!("{} selected ", collection.selected_chips().len()));
        }
        summary.push('\n');

        summary
    }
}

/// Constants for Material Design 3 compliance
pub mod constants {
    /// Minimum touch target size (Material Design guideline)
    pub const MIN_TOUCH_TARGET_SIZE: f32 = 48.0;

    /// Maximum label length for accessibility
    pub const MAX_LABEL_LENGTH: usize = 200;

    /// Default animation duration in milliseconds
    pub const DEFAULT_ANIMATION_DURATION_MS: u32 = 200;

    /// Reduced motion animation duration
    pub const REDUCED_MOTION_DURATION_MS: u32 = 0;

    /// Component size dimensions
    pub mod sizes {
        /// Small component dimensions
        pub const SMALL_SIZE: f32 = 16.0;

        /// Medium component dimensions
        pub const MEDIUM_SIZE: f32 = 20.0;

        /// Large component dimensions
        pub const LARGE_SIZE: f32 = 24.0;
    }

    /// Switch-specific dimensions
    pub mod switch {
        /// Default track width
        pub const TRACK_WIDTH: f32 = 52.0;

        /// Default track height
        pub const TRACK_HEIGHT: f32 = 32.0;

        /// Default thumb size
        pub const THUMB_SIZE: f32 = 24.0;

        /// Default padding
        pub const PADDING: f32 = 8.0;
    }
}

/// Version information for the selection module
pub const VERSION: &str = "3.0.0";
/// Current development phase of the selection components
///
/// This constant tracks the current architectural phase of the selection components.
/// Phase 3+ indicates a modular architecture with future preparation for additional features.
pub const PHASE: &str = "Phase 3+ - Modular Architecture with Future Preparation";

#[cfg(test)]
mod module_tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Test that all major types are accessible
        let _checkbox = Checkbox::new(CheckboxState::Unchecked);
        let _switch = Switch::new(SwitchState::Off);
        let _radio_group = RadioGroupState::<&str>::new();
        let _chip = Chip::new("test", ChipVariant::Assist);
        let _collection = ChipCollection::new(ChipSelectionMode::Single);
    }

    #[test]
    fn test_builder_convenience_functions() {
        let _checkbox = builders::checkbox();
        let _labeled = builders::labeled_checkbox("Test");
        let _switch = builders::switch();
        let _labeled_switch = builders::labeled_switch("Toggle");
        let _filters = builders::filter_chips();
        let _inputs = builders::input_chips();
    }

    #[test]
    fn test_validation_utilities() {
        let checkboxes = vec![
            Checkbox::new(CheckboxState::Unchecked)
                .label("Valid")
                .build()
                .unwrap(),
            Checkbox::new(CheckboxState::Unchecked)
                .label("Also Valid")
                .build()
                .unwrap(),
        ];

        assert!(validation::validate_checkboxes(&checkboxes).is_ok());
        assert!(validation::all_checkboxes_valid(&checkboxes));
        assert!(validation::collect_checkbox_errors(&checkboxes).is_empty());
    }

    #[test]
    fn test_state_utilities() {
        assert!(state_utils::checkbox_to_bool(CheckboxState::Checked));
        assert!(!state_utils::checkbox_to_bool(CheckboxState::Unchecked));
        assert!(!state_utils::checkbox_to_bool(CheckboxState::Indeterminate));

        assert!(state_utils::switch_to_bool(SwitchState::On));
        assert!(!state_utils::switch_to_bool(SwitchState::Off));

        assert!(state_utils::chip_is_selected(ChipState::Selected));
        assert!(!state_utils::chip_is_selected(ChipState::Unselected));
    }

    #[test]
    fn test_constants() {
        assert_eq!(constants::MIN_TOUCH_TARGET_SIZE, 48.0);
        assert_eq!(constants::MAX_LABEL_LENGTH, 200);
        assert!(constants::DEFAULT_ANIMATION_DURATION_MS > 0);
        assert_eq!(constants::REDUCED_MOTION_DURATION_MS, 0);
    }
}
