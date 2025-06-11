//! Modern Material Design 3 Radio Button Implementation
//!
//! This module provides a completely redesigned radio button component with:
//! - Generic value type support for type-safe radio groups
//! - Built-in validation and error handling
//! - Animation support for smooth transitions
//! - Modern builder pattern with fluent API
//! - Improved radio group management

use super::builder::{ComponentBuilder, Radio, RadioBuilder};
use super::common::*;

// ============================================================================
// Component Implementation
// ============================================================================

// Note: Radio component struct is defined in builder/components.rs
// All Radio methods are implemented there to avoid duplication

// ============================================================================
// Radio Group Management
// ============================================================================

/// State management for radio button groups
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupState<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    /// Currently selected value
    selected: Option<T>,
    /// All radio buttons in the group
    radios: Vec<Radio<T>>,
    /// Group-wide properties
    props: ComponentProps,
    /// Validation configuration for the group
    validation_config: ValidationConfig,
}

impl<T> RadioGroupState<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    /// Create a new radio group state
    #[must_use]
    pub fn new() -> Self {
        Self {
            selected: None,
            radios: Vec::new(),
            props: ComponentProps::new(),
            validation_config: validation_config_for_toggles(),
        }
    }

    /// Add a radio button to the group
    pub fn add_radio(&mut self, radio: Radio<T>) {
        self.radios.push(radio);
    }

    /// Get the currently selected value
    #[must_use]
    pub fn selected_value(&self) -> Option<T> {
        self.selected.clone()
    }

    /// Set the selected value
    pub fn select(&mut self, value: T) -> Result<(), SelectionError> {
        // Validate that the value exists in the group
        if !self.radios.iter().any(|r| r.value == value) {
            return Err(SelectionError::InvalidState {
                details: "Value not found in radio group".to_string(),
            });
        }

        self.selected = Some(value);
        Ok(())
    }

    /// Clear the selection
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    /// Get all radio buttons in the group
    #[must_use]
    pub fn radios(&self) -> &[Radio<T>] {
        &self.radios
    }

    /// Get the group properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    /// Set group properties
    pub fn set_props(&mut self, props: ComponentProps) {
        self.props = props;
    }

    /// Check if a value is selected
    #[must_use]
    pub fn is_selected(&self, value: &T) -> bool {
        self.selected.as_ref() == Some(value)
    }

    /// Get the number of radio buttons in the group
    #[must_use]
    pub fn len(&self) -> usize {
        self.radios.len()
    }

    /// Check if the group is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.radios.is_empty()
    }

    /// Validate the entire radio group
    pub fn validate(&self) -> Result<(), SelectionError> {
        // Validate each radio button
        for radio in &self.radios {
            radio.validate()?;
        }

        // Validate group-specific constraints
        if self.radios.is_empty() {
            return Err(SelectionError::InvalidState {
                details: "Radio group cannot be empty".to_string(),
            });
        }

        // Check for duplicate values
        let mut values = std::collections::HashSet::new();
        for radio in &self.radios {
            if !values.insert(&radio.value) {
                return Err(SelectionError::ConflictingStates {
                    details: "Duplicate values found in radio group".to_string(),
                });
            }
        }

        Ok(())
    }
}

impl<T> Default for RadioGroupState<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Radio Group Builder
// ============================================================================

/// Builder for creating radio groups with validation
#[derive(Debug, Clone)]
pub struct RadioGroupBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    radios: Vec<Radio<T>>,
    selected: Option<T>,
    props: ComponentProps,
    validation_config: ValidationConfig,
}

impl<T> RadioGroupBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    /// Create a new radio group builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            radios: Vec::new(),
            selected: None,
            props: ComponentProps::new(),
            validation_config: validation_config_for_toggles(),
        }
    }

    /// Add a radio button to the group
    #[must_use]
    pub fn radio(mut self, radio: Radio<T>) -> Self {
        self.radios.push(radio);
        self
    }

    /// Add a radio button with value and label
    #[must_use]
    pub fn option<S: Into<String>>(mut self, value: T, label: S) -> Self {
        let radio = RadioBuilder::new(value)
            .label(label)
            .size(self.props.size)
            .disabled(self.props.disabled)
            .build_unchecked();
        self.radios.push(radio);
        self
    }

    /// Set the initially selected value
    #[must_use]
    pub fn selected(mut self, value: T) -> Self {
        self.selected = Some(value);
        self
    }
    /// Set group properties
    #[must_use]
    pub fn props(mut self, props: ComponentProps) -> Self {
        self.props = props;
        self
    }

    /// Set group size (applies to all radio buttons)
    #[must_use]
    pub const fn size(mut self, size: ComponentSize) -> Self {
        self.props.size = size;
        self
    }

    /// Set group disabled state (applies to all radio buttons)
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }
    /// Set validation configuration
    #[must_use]
    pub fn validation(mut self, config: ValidationConfig) -> Self {
        self.validation_config = config;
        self
    }

    /// Build the radio group with validation
    pub fn build(self) -> Result<RadioGroupState<T>, SelectionError> {
        let group = RadioGroupState {
            selected: self.selected,
            radios: self.radios,
            props: self.props,
            validation_config: self.validation_config,
        };

        group.validate()?;
        Ok(group)
    }

    /// Build the radio group without validation
    #[must_use]
    pub fn build_unchecked(self) -> RadioGroupState<T> {
        RadioGroupState {
            selected: self.selected,
            radios: self.radios,
            props: self.props,
            validation_config: self.validation_config,
        }
    }
}

impl<T> Default for RadioGroupBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
// Note: SelectionWidget implementation is in builder/components.rs to avoid conflicts
// Note: AnimatedWidget implementation is provided via EnhancedSelectionWidget trait

// ============================================================================
// Convenience Functions
// ============================================================================

/// Create a new radio button builder
#[must_use]
pub fn radio<T>(value: T) -> RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    RadioBuilder::new(value)
}

/// Create a new radio group builder
#[must_use]
pub fn radio_group<T>() -> RadioGroupBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    RadioGroupBuilder::new()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestOption {
        A,
        B,
        C,
    }

    #[test]
    fn test_radio_creation() {
        let radio = Radio::new(TestOption::A)
            .label("Option A")
            .size(ComponentSize::Large)
            .build()
            .expect("Should create valid radio");

        assert_eq!(radio.value(), &TestOption::A);
        assert_eq!(radio.props().label, Some("Option A".to_string()));
        assert_eq!(radio.props().size, ComponentSize::Large);
    }

    #[test]
    fn test_radio_selection() {
        let radio = Radio::new(TestOption::A)
            .build()
            .expect("Should create valid radio");

        assert!(radio.is_selected(Some(&TestOption::A)));
        assert!(!radio.is_selected(Some(&TestOption::B)));
        assert!(!radio.is_selected(None));
    }

    #[test]
    fn test_radio_group_creation() {
        let group = RadioGroupBuilder::new()
            .option(TestOption::A, "Option A")
            .option(TestOption::B, "Option B")
            .option(TestOption::C, "Option C")
            .selected(TestOption::B)
            .build()
            .expect("Should create valid radio group");

        assert_eq!(group.len(), 3);
        assert_eq!(group.selected_value(), Some(TestOption::B));
        assert!(group.is_selected(&TestOption::B));
        assert!(!group.is_selected(&TestOption::A));
    }

    #[test]
    fn test_radio_group_state_management() {
        let mut group = RadioGroupBuilder::new()
            .option(TestOption::A, "Option A")
            .option(TestOption::B, "Option B")
            .build()
            .expect("Should create valid radio group");

        assert_eq!(group.selected_value(), None);

        // Select a value
        group.select(TestOption::A).expect("Should select value");
        assert_eq!(group.selected_value(), Some(TestOption::A));
        assert!(group.is_selected(&TestOption::A));

        // Clear selection
        group.clear_selection();
        assert_eq!(group.selected_value(), None);
    }

    #[test]
    fn test_radio_group_validation() {
        // Valid group
        let valid_group = RadioGroupBuilder::new()
            .option(TestOption::A, "Option A")
            .option(TestOption::B, "Option B")
            .build();
        assert!(valid_group.is_ok());

        // Invalid group - empty
        let empty_group = RadioGroupBuilder::<TestOption>::new().build();
        assert!(empty_group.is_err());
    }

    #[test]
    fn test_radio_error_state() {
        let mut radio = Radio::new(TestOption::A)
            .error(true)
            .build()
            .expect("Should create radio with error state");

        assert!(radio.has_error());

        radio.set_error(false);
        assert!(!radio.has_error());
    }

    #[test]
    fn test_radio_traits() {
        let radio = Radio::new(TestOption::A)
            .build()
            .expect("Should create valid radio");

        // Test SelectionWidget trait
        assert_eq!(radio.state(), TestOption::A);
        assert!(radio.validate().is_ok());

        // Test animation support
        assert!(radio.animation_config().enabled);
    }

    #[test]
    fn test_convenience_functions() {
        let radio_builder = radio(TestOption::A);
        let group_builder = radio_group::<TestOption>();

        let radio = radio_builder.build().unwrap();
        let group = group_builder.option(TestOption::A, "A").build().unwrap();

        assert_eq!(radio.value(), &TestOption::A);
        assert_eq!(group.len(), 1);
    }

    #[test]
    fn test_radio_group_duplicate_values() {
        // This should fail due to duplicate values
        let radio1 = Radio::new(TestOption::A).build().unwrap();
        let radio2 = Radio::new(TestOption::A).build().unwrap(); // Duplicate!

        let mut group = RadioGroupState::new();
        group.add_radio(radio1);
        group.add_radio(radio2);

        assert!(group.validate().is_err());
    }
}
