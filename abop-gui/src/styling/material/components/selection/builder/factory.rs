//! Builder Factory Functions
//!
//! This module provides convenient factory functions for creating builder instances.
//! These functions offer a clean, ergonomic API for quick component creation.
//!
//! ## Factory Functions
//! - [`checkbox`] - Create a checkbox builder
//! - [`radio`] - Create a radio button builder
//! - [`switch`] - Create a switch builder
//! - [`chip`] - Create a chip builder

use super::super::common::*;
use super::{
    checkbox::CheckboxBuilder, chip::ChipBuilder, radio::RadioBuilder, switch::SwitchBuilder,
};

// ============================================================================
// Primary Builder Factory Functions
// ============================================================================

/// Create a new checkbox builder
///
/// # Examples
/// ```
/// use abop_gui::styling::material::components::selection::builder::factory::checkbox;
/// use abop_gui::styling::material::components::selection::CheckboxState;
/// use abop_gui::styling::material::components::selection::common::ComponentSize;
///
/// let cb = checkbox(CheckboxState::Checked)
///     .label("Accept terms")
///     .size(ComponentSize::Large)
///     .build()
///     .unwrap();
/// ```
#[must_use]
pub fn checkbox(state: CheckboxState) -> CheckboxBuilder {
    CheckboxBuilder::new(state)
}

/// Create a new radio builder
///
/// # Examples
/// ```
/// use abop_gui::styling::material::components::selection::builder::factory::radio;
/// use abop_gui::styling::material::components::selection::common::ComponentSize;
///
/// let rb = radio("option_a")
///     .label("Option A")
///     .size(ComponentSize::Medium)
///     .build()
///     .unwrap();
/// ```
#[must_use]
pub fn radio<T>(value: T) -> RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    RadioBuilder::new(value)
}

/// Create a new switch builder
///
/// # Examples
/// ```
/// use abop_gui::styling::material::components::selection::builder::factory::switch;
/// use abop_gui::styling::material::components::selection::SwitchState;
/// use abop_gui::styling::material::components::selection::common::ComponentSize;
///
/// let sw = switch(SwitchState::On)
///     .label("Enable notifications")
///     .size(ComponentSize::Large)
///     .build()
///     .unwrap();
/// ```
#[must_use]
pub fn switch(state: SwitchState) -> SwitchBuilder {
    SwitchBuilder::new(state)
}

/// Create a new chip builder
///
/// # Examples
/// ```
/// use abop_gui::styling::material::components::selection::builder::factory::chip;
/// use abop_gui::styling::material::components::selection::ChipVariant;
///
/// let ch = chip("Category", ChipVariant::Filter)
///     .selected(true)
///     .with_leading_icon("filter")
///     .build()
///     .unwrap();
/// ```
#[must_use]
pub fn chip<S: Into<String>>(label: S, variant: ChipVariant) -> ChipBuilder {
    ChipBuilder::new(label, variant)
}

// ============================================================================
// Convenience Factory Functions
// ============================================================================

/// Create a checked checkbox builder
#[must_use]
pub fn checked_checkbox() -> CheckboxBuilder {
    CheckboxBuilder::checked()
}

/// Create an unchecked checkbox builder
#[must_use]
pub fn unchecked_checkbox() -> CheckboxBuilder {
    CheckboxBuilder::unchecked()
}

/// Create an indeterminate checkbox builder
#[must_use]
pub fn indeterminate_checkbox() -> CheckboxBuilder {
    CheckboxBuilder::indeterminate()
}

/// Create a checkbox builder from a boolean value
#[must_use]
pub fn checkbox_from_bool(checked: bool) -> CheckboxBuilder {
    CheckboxBuilder::from_bool(checked)
}

/// Create a switch builder in the on state
#[must_use]
pub fn switch_on() -> SwitchBuilder {
    SwitchBuilder::on()
}

/// Create a switch builder in the off state
#[must_use]
pub fn switch_off() -> SwitchBuilder {
    SwitchBuilder::off()
}

/// Create a switch builder from a boolean value
#[must_use]
pub fn switch_from_bool(enabled: bool) -> SwitchBuilder {
    SwitchBuilder::from_bool(enabled)
}

/// Create a filter chip builder
#[must_use]
pub fn filter_chip<S: Into<String>>(label: S) -> ChipBuilder {
    ChipBuilder::filter(label)
}

/// Create an assist chip builder
#[must_use]
pub fn assist_chip<S: Into<String>>(label: S) -> ChipBuilder {
    ChipBuilder::assist(label)
}

/// Create an input chip builder
#[must_use]
pub fn input_chip<S: Into<String>>(label: S) -> ChipBuilder {
    ChipBuilder::input(label)
}

/// Create a suggestion chip builder
#[must_use]
pub fn suggestion_chip<S: Into<String>>(label: S) -> ChipBuilder {
    ChipBuilder::suggestion(label)
}

/// Create a deletable input chip builder
#[must_use]
pub fn deletable_chip<S: Into<String>>(label: S) -> ChipBuilder {
    ChipBuilder::input(label).deletable()
}

// ============================================================================
// Semantic Factory Functions
// ============================================================================

/// Create a labeled checkbox builder
#[must_use]
pub fn labeled_checkbox<S: Into<String>>(label: S, checked: bool) -> CheckboxBuilder {
    checkbox_from_bool(checked).label(label)
}

/// Create a labeled switch builder
#[must_use]
pub fn labeled_switch<S: Into<String>>(label: S, enabled: bool) -> SwitchBuilder {
    switch_from_bool(enabled).label(label)
}

/// Create a selected filter chip builder
#[must_use]
pub fn selected_filter_chip<S: Into<String>>(label: S) -> ChipBuilder {
    filter_chip(label).selected(true)
}

/// Create a chip with an icon
#[must_use]
#[allow(dead_code)] // Public API function - may not be used internally yet
pub fn icon_chip<S: Into<String>, I: Into<String>>(
    label: S,
    variant: ChipVariant,
    icon: I,
) -> ChipBuilder {
    chip(label, variant).with_leading_icon(icon)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::super::patterns::ComponentBuilder;
    use super::*;

    #[test]
    fn test_primary_factory_functions() {
        let cb = checkbox(CheckboxState::Checked).build().unwrap();
        assert_eq!(cb.state(), CheckboxState::Checked);

        let rb = radio("test").build().unwrap();
        assert_eq!(rb.value(), &"test");

        let sw = switch(SwitchState::On).build().unwrap();
        assert_eq!(sw.state(), SwitchState::On);

        let ch = chip("test", ChipVariant::Filter).build().unwrap();
        assert_eq!(ch.label(), "test");
        assert_eq!(ch.variant(), ChipVariant::Filter);
    }

    #[test]
    fn test_convenience_factory_functions() {
        let checked = checked_checkbox().build().unwrap();
        assert_eq!(checked.state(), CheckboxState::Checked);

        let unchecked = unchecked_checkbox().build().unwrap();
        assert_eq!(unchecked.state(), CheckboxState::Unchecked);

        let indeterminate = indeterminate_checkbox().build().unwrap();
        assert_eq!(indeterminate.state(), CheckboxState::Indeterminate);

        let bool_checkbox = checkbox_from_bool(true).build().unwrap();
        assert_eq!(bool_checkbox.state(), CheckboxState::Checked);

        let on_switch = switch_on().build().unwrap();
        assert_eq!(on_switch.state(), SwitchState::On);

        let off_switch = switch_off().build().unwrap();
        assert_eq!(off_switch.state(), SwitchState::Off);

        let bool_switch = switch_from_bool(true).build().unwrap();
        assert_eq!(bool_switch.state(), SwitchState::On);

        let filter = filter_chip("Filter").build().unwrap();
        assert_eq!(filter.variant(), ChipVariant::Filter);

        let assist = assist_chip("Assist").build().unwrap();
        assert_eq!(assist.variant(), ChipVariant::Assist);

        let input = input_chip("Input").build().unwrap();
        assert_eq!(input.variant(), ChipVariant::Input);

        let suggestion = suggestion_chip("Suggestion").build().unwrap();
        assert_eq!(suggestion.variant(), ChipVariant::Suggestion);
    }

    #[test]
    fn test_semantic_factory_functions() {
        let labeled_cb = labeled_checkbox("Test Label", true).build().unwrap();
        assert_eq!(labeled_cb.state(), CheckboxState::Checked);
        assert_eq!(labeled_cb.props().label, Some("Test Label".to_string()));

        let labeled_sw = labeled_switch("Switch Label", false).build().unwrap();
        assert_eq!(labeled_sw.state(), SwitchState::Off);
        assert_eq!(labeled_sw.props().label, Some("Switch Label".to_string()));

        let selected_filter = selected_filter_chip("Selected").build().unwrap();
        assert_eq!(selected_filter.state(), ChipState::Selected);
        assert_eq!(selected_filter.variant(), ChipVariant::Filter);

        let icon_chip_result = icon_chip("Icon", ChipVariant::Assist, "star")
            .build()
            .unwrap();
        assert_eq!(icon_chip_result.variant(), ChipVariant::Assist);
        assert_eq!(icon_chip_result.label(), "Icon");
        // Check metadata for icon
        assert!(
            icon_chip_result
                .props()
                .metadata
                .contains_key("leading_icon")
        );
    }

    #[test]
    fn test_deletable_chip() {
        let deletable = deletable_chip("Delete Me").build().unwrap();
        assert_eq!(deletable.variant(), ChipVariant::Input);
        assert!(deletable.props().metadata.contains_key("trailing_icon"));
        assert_eq!(
            deletable.props().metadata.get("trailing_icon"),
            Some(&"times".to_string())
        );
    }

    #[test]
    fn test_factory_functions_chaining() {
        let complex_checkbox = labeled_checkbox("Complex", false)
            .size(ComponentSize::Large)
            .disabled(false)
            .error(false)
            .build()
            .unwrap();

        assert_eq!(complex_checkbox.state(), CheckboxState::Unchecked);
        assert_eq!(complex_checkbox.props().size, ComponentSize::Large);
        assert!(!complex_checkbox.props().disabled);
        assert!(!complex_checkbox.has_error());

        let complex_chip = selected_filter_chip("Complex Filter")
            .with_leading_icon("filter")
            .with_badge(5)
            .size(ComponentSize::Small)
            .build()
            .unwrap();

        assert_eq!(complex_chip.state(), ChipState::Selected);
        assert_eq!(complex_chip.props().size, ComponentSize::Small);
        assert!(complex_chip.props().metadata.contains_key("leading_icon"));
        assert!(complex_chip.props().metadata.contains_key("badge_count"));
    }
}
