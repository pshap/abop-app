//! Modern Material Design 3 Chip Implementation
//!
//! This module provides a completely redesigned chip component with:
//! - State-based design using ChipState enum
//! - Multiple chip variants (Assist, Filter, Input, Suggestion)
//! - Built-in validation and error handling
//! - Animation support for smooth transitions
//! - Modern builder pattern with fluent API

use super::builder::{Chip, ChipBuilder, ComponentBuilder};
use super::common::*;
use crate::styling::material::colors::MaterialColors;
use crate::styling::material::components::selection_style::{
    SelectionSize as LegacySelectionSize, SelectionStyleBuilder, SelectionVariant,
};

use iced::{
    Element, Renderer,
    theme::Theme,
    widget::{Text, button},
};

// ============================================================================
// Component Implementation
// ============================================================================

impl Chip {
    /// Create a new chip with the specified label and variant
    #[must_use]
    pub fn new<S: Into<String>>(label: S, variant: ChipVariant) -> ChipBuilder {
        ChipBuilder::new(label, variant)
    }

    /// Create a filter chip
    #[must_use]
    pub fn filter<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::filter(label)
    }

    /// Create an assist chip
    #[must_use]
    pub fn assist<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::assist(label)
    }

    /// Create an input chip
    #[must_use]
    pub fn input<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::input(label)
    }

    /// Create a suggestion chip
    #[must_use]
    pub fn suggestion<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::suggestion(label)
    }

    /// Get the chip label
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the current chip state
    #[must_use]
    pub const fn state(&self) -> ChipState {
        self.state
    }

    /// Get the chip variant
    #[must_use]
    pub const fn variant(&self) -> ChipVariant {
        self.variant
    }

    /// Get the component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    /// Get the animation configuration
    #[must_use]
    pub const fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    /// Update the chip state with validation
    pub fn update_state(&mut self, new_state: ChipState) -> Result<(), SelectionError> {
        validate_chip_state(new_state, self.variant, &self.props)?;
        self.state = new_state;
        Ok(())
    }

    /// Toggle the chip selection state
    pub fn toggle(&mut self) -> Result<ChipState, SelectionError> {
        let new_state = self.state.toggle();
        self.update_state(new_state)?;
        Ok(new_state)
    }

    /// Set chip as selected
    pub fn select(&mut self) -> Result<(), SelectionError> {
        self.update_state(ChipState::Selected)
    }

    /// Set chip as unselected
    pub fn unselect(&mut self) -> Result<(), SelectionError> {
        self.update_state(ChipState::Unselected)
    }

    /// Check if chip is selected
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        self.state.is_selected()
    }

    /// Check if chip is unselected
    #[must_use]
    pub const fn is_unselected(&self) -> bool {
        matches!(self.state, ChipState::Unselected)
    }

    /// Check if chip is being pressed
    #[must_use]
    pub const fn is_pressed(&self) -> bool {
        matches!(self.state, ChipState::Pressed)
    }

    /// Create the Iced widget element for this chip
    ///
    /// # Arguments
    /// * `on_press` - Optional callback when the chip is pressed
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to the UI
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        on_press: Option<Message>,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        // Convert modern size to legacy size
        let legacy_size = match self.props.size {
            ComponentSize::Small => LegacySelectionSize::Small,
            ComponentSize::Medium => LegacySelectionSize::Medium,
            ComponentSize::Large => LegacySelectionSize::Large,
        };

        // Create styling function
        let style_fn = SelectionStyleBuilder::new(color_scheme.clone(), SelectionVariant::Chip)
            .size(legacy_size)
            .chip_style(self.is_selected());

        // Create chip content
        let content = Text::new(&self.label).size(self.props.size.text_size());

        // Create chip button
        let mut chip_button = button(content).style(style_fn);

        // Only add on_press handler if the chip is not disabled and callback is provided
        if !self.props.disabled
            && let Some(message) = on_press
        {
            chip_button = chip_button.on_press(message);
        }

        chip_button.into()
    }
    /// Create a view that handles selection state changes automatically
    ///
    /// This is a convenience method for chips that should toggle their
    /// selection state when pressed.
    pub fn view_with_toggle<'a, Message: Clone + 'a>(
        &'a self,
        on_toggle: impl Fn(ChipState) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        let next_state = self.state.toggle();
        let dummy_message = on_toggle(next_state.clone());
        self.view(Some(dummy_message), color_scheme).map(move |_| on_toggle(next_state))
    }

    /// Create a view for filter chips with selection state management
    ///
    /// This is specifically designed for filter chips that need to
    /// maintain selected/unselected state.
    pub fn view_as_filter<'a, Message: Clone + 'a>(
        &'a self,
        on_selection_change: impl Fn(bool) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        let is_selected = self.is_selected();
        let new_selection = !is_selected;
        let dummy_message = on_selection_change(new_selection);
        self.view(Some(dummy_message), color_scheme).map(move |_| on_selection_change(new_selection))
    }
}

// ============================================================================
// Chip Collection Management
// ============================================================================

/// State management for chip collections (e.g., filter chip groups)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChipCollection {
    /// All chips in the collection
    chips: Vec<Chip>,
    /// Collection-wide properties
    props: ComponentProps,
    /// Selection mode for the collection
    selection_mode: ChipSelectionMode,
    /// Validation configuration
    validation_config: ValidationConfig,
}

/// Selection mode for chip collections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipSelectionMode {
    /// No selection allowed (for assist/suggestion chips)
    None,
    /// Single selection (radio button behavior)
    Single,
    /// Multiple selection allowed
    Multiple,
}

impl ChipCollection {
    /// Create a new chip collection
    #[must_use]
    pub fn new(selection_mode: ChipSelectionMode) -> Self {
        Self {
            chips: Vec::new(),
            props: ComponentProps::new(),
            selection_mode,
            validation_config: validation_config_for_chips(),
        }
    }

    /// Add a chip to the collection
    pub fn add_chip(&mut self, chip: Chip) {
        self.chips.push(chip);
    }

    /// Get all chips in the collection
    #[must_use]
    pub fn chips(&self) -> &[Chip] {
        &self.chips
    }

    /// Get mutable access to chips
    pub fn chips_mut(&mut self) -> &mut [Chip] {
        &mut self.chips
    }

    /// Get selected chip indices
    #[must_use]
    pub fn selected_indices(&self) -> Vec<usize> {
        self.chips
            .iter()
            .enumerate()
            .filter_map(|(i, chip)| if chip.is_selected() { Some(i) } else { None })
            .collect()
    }

    /// Get selected chips
    #[must_use]
    pub fn selected_chips(&self) -> Vec<&Chip> {
        self.chips
            .iter()
            .filter(|chip| chip.is_selected())
            .collect()
    }

    /// Select a chip by index
    pub fn select_chip(&mut self, index: usize) -> Result<(), SelectionError> {
        if index >= self.chips.len() {
            return Err(SelectionError::InvalidState {
                details: "Chip index out of bounds".to_string(),
            });
        }

        match self.selection_mode {
            ChipSelectionMode::None => {
                return Err(SelectionError::InvalidState {
                    details: "Selection not allowed in this collection".to_string(),
                });
            }
            ChipSelectionMode::Single => {
                // Deselect all other chips
                for (i, chip) in self.chips.iter_mut().enumerate() {
                    if i == index {
                        chip.select()?;
                    } else {
                        chip.unselect()?;
                    }
                }
            }
            ChipSelectionMode::Multiple => {
                self.chips[index].select()?;
            }
        }

        Ok(())
    }

    /// Deselect a chip by index
    pub fn deselect_chip(&mut self, index: usize) -> Result<(), SelectionError> {
        if index >= self.chips.len() {
            return Err(SelectionError::InvalidState {
                details: "Chip index out of bounds".to_string(),
            });
        }

        if self.selection_mode == ChipSelectionMode::None {
            return Err(SelectionError::InvalidState {
                details: "Selection not allowed in this collection".to_string(),
            });
        }

        self.chips[index].unselect()
    }

    /// Toggle chip selection by index
    pub fn toggle_chip(&mut self, index: usize) -> Result<ChipState, SelectionError> {
        if index >= self.chips.len() {
            return Err(SelectionError::InvalidState {
                details: "Chip index out of bounds".to_string(),
            });
        }

        match self.selection_mode {
            ChipSelectionMode::None => Err(SelectionError::InvalidState {
                details: "Selection not allowed in this collection".to_string(),
            }),
            ChipSelectionMode::Single => {
                if self.chips[index].is_selected() {
                    // Don't allow deselecting in single mode - always have one selected
                    Ok(self.chips[index].state())
                } else {
                    self.select_chip(index)?;
                    Ok(ChipState::Selected)
                }
            }
            ChipSelectionMode::Multiple => self.chips[index].toggle(),
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) -> Result<(), SelectionError> {
        if self.selection_mode == ChipSelectionMode::None {
            return Ok(()); // Nothing to clear
        }

        for chip in &mut self.chips {
            chip.unselect()?;
        }

        Ok(())
    }

    /// Get the number of selected chips
    #[must_use]
    pub fn selected_count(&self) -> usize {
        self.chips.iter().filter(|chip| chip.is_selected()).count()
    }

    /// Check if any chips are selected
    #[must_use]
    pub fn has_selection(&self) -> bool {
        self.selected_count() > 0
    }

    /// Get the selection mode
    #[must_use]
    pub const fn selection_mode(&self) -> ChipSelectionMode {
        self.selection_mode
    }

    /// Get collection properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    /// Set collection properties
    pub fn set_props(&mut self, props: ComponentProps) {
        self.props = props;
    }

    /// Validate the entire chip collection
    pub fn validate(&self) -> Result<(), SelectionError> {
        // Validate each chip
        for chip in &self.chips {
            chip.validate()?;
        }

        // Validate collection-specific constraints
        match self.selection_mode {
            ChipSelectionMode::Single => {
                let selected_count = self.selected_count();
                if selected_count > 1 {
                    return Err(SelectionError::ConflictingStates {
                        details: "Single selection mode allows only one selected chip".to_string(),
                    });
                }
            }
            _ => {} // No additional constraints for other modes
        }

        Ok(())
    }

    /// Get the number of chips in the collection
    #[must_use]
    pub fn len(&self) -> usize {
        self.chips.len()
    }

    /// Check if the collection is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.chips.is_empty()
    }
}

// ============================================================================
// Chip Collection Builder
// ============================================================================

/// Builder for creating chip collections
#[derive(Debug, Clone)]
pub struct ChipCollectionBuilder {
    chips: Vec<Chip>,
    selection_mode: ChipSelectionMode,
    props: ComponentProps,
    validation_config: ValidationConfig,
}

impl ChipCollectionBuilder {
    /// Create a new chip collection builder
    #[must_use]
    pub const fn new(selection_mode: ChipSelectionMode) -> Self {
        Self {
            chips: Vec::new(),
            selection_mode,
            props: ComponentProps::new(),
            validation_config: ValidationConfig {
                max_label_length: 100,
                allow_empty_label: false,
                custom_rules: Vec::new(),
            },
        }
    }

    /// Add a chip to the collection
    #[must_use]
    pub fn chip(mut self, chip: Chip) -> Self {
        self.chips.push(chip);
        self
    }

    /// Add a chip with label and variant
    #[must_use]
    pub fn add<S: Into<String>>(mut self, label: S, variant: ChipVariant) -> Self {
        let chip = ChipBuilder::new(label, variant)
            .size(self.props.size)
            .disabled(self.props.disabled)
            .build_unchecked();
        self.chips.push(chip);
        self
    }
    /// Add a filter chip
    #[must_use]
    pub fn filter<S: Into<String>>(self, label: S) -> Self {
        self.add(label, ChipVariant::Filter)
    }

    /// Add an assist chip
    #[must_use]
    pub fn assist<S: Into<String>>(self, label: S) -> Self {
        self.add(label, ChipVariant::Assist)
    }

    /// Add an input chip
    #[must_use]
    pub fn input<S: Into<String>>(self, label: S) -> Self {
        self.add(label, ChipVariant::Input)
    }

    /// Add a suggestion chip
    #[must_use]
    pub fn suggestion<S: Into<String>>(self, label: S) -> Self {
        self.add(label, ChipVariant::Suggestion)
    }
    /// Set collection properties
    #[must_use]
    pub fn props(mut self, props: ComponentProps) -> Self {
        self.props = props;
        self
    }

    /// Set collection size (applies to all chips)
    #[must_use]
    pub const fn size(mut self, size: ComponentSize) -> Self {
        self.props.size = size;
        self
    }

    /// Set collection disabled state (applies to all chips)
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    /// Build the chip collection with validation
    pub fn build(self) -> Result<ChipCollection, SelectionError> {
        let collection = ChipCollection {
            chips: self.chips,
            selection_mode: self.selection_mode,
            props: self.props,
            validation_config: self.validation_config,
        };

        collection.validate()?;
        Ok(collection)
    }

    /// Build the chip collection without validation
    #[must_use]
    pub fn build_unchecked(self) -> ChipCollection {
        ChipCollection {
            chips: self.chips,
            selection_mode: self.selection_mode,
            props: self.props,
            validation_config: self.validation_config,
        }
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl SelectionWidget<ChipState> for Chip {
    type Message = ChipState;
    type Builder = ChipBuilder;
    fn new(state: ChipState) -> Self::Builder {
        // Note: Chips require label and variant, so this is a placeholder
        ChipBuilder::filter("Default").with_state(state)
    }

    fn validate(&self) -> Result<(), SelectionError> {
        validate_chip_state(self.state, self.variant, &self.props)
    }

    fn state(&self) -> ChipState {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }
}

impl StatefulWidget<ChipState> for Chip {
    fn update_state(&mut self, new_state: ChipState) -> Result<(), SelectionError> {
        self.update_state(new_state)
    }

    fn transition_to(&mut self, new_state: ChipState) -> Result<ChipState, SelectionError> {
        self.update_state(new_state)?;
        Ok(self.state)
    }
}

impl AnimatedWidget for Chip {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Create a new chip builder
#[must_use]
pub fn chip<S: Into<String>>(label: S, variant: ChipVariant) -> ChipBuilder {
    ChipBuilder::new(label, variant)
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

/// Create a new chip collection builder
#[must_use]
pub const fn chip_collection(selection_mode: ChipSelectionMode) -> ChipCollectionBuilder {
    ChipCollectionBuilder::new(selection_mode)
}

/// Create a filter chip collection (multiple selection)
#[must_use]
pub const fn filter_chip_collection() -> ChipCollectionBuilder {
    ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
}

/// Create a single-select chip collection
#[must_use]
pub const fn single_select_chip_collection() -> ChipCollectionBuilder {
    ChipCollectionBuilder::new(ChipSelectionMode::Single)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip_creation() {
        let chip = Chip::filter("Test Chip")
            .selected(true)
            .size(ComponentSize::Large)
            .build()
            .expect("Should create valid chip");

        assert_eq!(chip.label(), "Test Chip");
        assert_eq!(chip.state(), ChipState::Selected);
        assert_eq!(chip.variant(), ChipVariant::Filter);
        assert_eq!(chip.props().size, ComponentSize::Large);
        assert!(chip.is_selected());
    }

    #[test]
    fn test_chip_state_transitions() {
        let mut chip = Chip::filter("Test")
            .build()
            .expect("Should create valid chip");

        assert_eq!(chip.state(), ChipState::Unselected);
        assert!(chip.is_unselected());

        // Select the chip
        chip.select().expect("Should select successfully");
        assert_eq!(chip.state(), ChipState::Selected);
        assert!(chip.is_selected());

        // Toggle to unselected
        let new_state = chip.toggle().expect("Should toggle successfully");
        assert_eq!(new_state, ChipState::Unselected);
        assert!(chip.is_unselected());
    }

    #[test]
    fn test_chip_variants() {
        let assist = Chip::assist("Assist").build().unwrap();
        let filter = Chip::filter("Filter").build().unwrap();
        let input = Chip::input("Input").build().unwrap();
        let suggestion = Chip::suggestion("Suggestion").build().unwrap();

        assert_eq!(assist.variant(), ChipVariant::Assist);
        assert_eq!(filter.variant(), ChipVariant::Filter);
        assert_eq!(input.variant(), ChipVariant::Input);
        assert_eq!(suggestion.variant(), ChipVariant::Suggestion);
    }

    #[test]
    fn test_chip_validation() {
        // Valid chip
        let valid_chip = Chip::filter("Valid").build();
        assert!(valid_chip.is_ok());

        // Invalid chip - empty label
        let invalid_chip = Chip::filter("").build();
        assert!(invalid_chip.is_err());

        // Invalid chip - label too long
        let long_label = "x".repeat(101);
        let invalid_chip = Chip::filter(long_label).build();
        assert!(invalid_chip.is_err());
    }

    #[test]
    fn test_chip_collection_creation() {
        let collection = ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
            .filter("Category 1")
            .filter("Category 2")
            .filter("Category 3")
            .build()
            .expect("Should create valid chip collection");

        assert_eq!(collection.len(), 3);
        assert_eq!(collection.selection_mode(), ChipSelectionMode::Multiple);
        assert_eq!(collection.selected_count(), 0);
    }

    #[test]
    fn test_chip_collection_selection() {
        let mut collection = ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
            .filter("Option 1")
            .filter("Option 2")
            .filter("Option 3")
            .build()
            .expect("Should create valid collection");

        // Select multiple chips
        collection.select_chip(0).expect("Should select chip 0");
        collection.select_chip(2).expect("Should select chip 2");

        assert_eq!(collection.selected_count(), 2);
        assert_eq!(collection.selected_indices(), vec![0, 2]);
        assert!(collection.has_selection());

        // Clear selection
        collection
            .clear_selection()
            .expect("Should clear selection");
        assert_eq!(collection.selected_count(), 0);
        assert!(!collection.has_selection());
    }

    #[test]
    fn test_chip_collection_single_selection() {
        let mut collection = ChipCollectionBuilder::new(ChipSelectionMode::Single)
            .filter("Option 1")
            .filter("Option 2")
            .build()
            .expect("Should create valid collection");

        // Select first chip
        collection.select_chip(0).expect("Should select chip 0");
        assert_eq!(collection.selected_count(), 1);
        assert_eq!(collection.selected_indices(), vec![0]);

        // Select second chip (should deselect first)
        collection.select_chip(1).expect("Should select chip 1");
        assert_eq!(collection.selected_count(), 1);
        assert_eq!(collection.selected_indices(), vec![1]);
    }

    #[test]
    fn test_chip_traits() {
        let chip = Chip::filter("Test")
            .build()
            .expect("Should create valid chip");

        // Test SelectionWidget trait
        assert_eq!(chip.state(), ChipState::Unselected);
        assert!(chip.validate().is_ok());

        // Test animation support
        assert!(chip.animation_config().enabled);
    }

    #[test]
    fn test_convenience_functions() {
        let cb1 = filter_chip("Filter").build().unwrap();
        let cb2 = assist_chip("Assist").build().unwrap();
        let cb3 = input_chip("Input").build().unwrap();
        let cb4 = suggestion_chip("Suggestion").build().unwrap();

        assert_eq!(cb1.variant(), ChipVariant::Filter);
        assert_eq!(cb2.variant(), ChipVariant::Assist);
        assert_eq!(cb3.variant(), ChipVariant::Input);
        assert_eq!(cb4.variant(), ChipVariant::Suggestion);

        let collection = filter_chip_collection().filter("Test").build().unwrap();
        assert_eq!(collection.selection_mode(), ChipSelectionMode::Multiple);
    }
}
