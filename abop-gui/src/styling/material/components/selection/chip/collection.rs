//! Chip collection management and selection modes
//!
//! This module provides the ChipCollection type for managing groups of chips
//! with different selection behaviors (None, Single, Multiple).

use super::super::builder::Chip;
use super::super::common::*;
use crate::styling::material::colors::MaterialColors;

use iced::{
    Element, Length, Renderer,
    theme::Theme,
    widget::{Column, Container, Row, Scrollable},
};

// ============================================================================
// Collection Layout Definition
// ============================================================================

/// Layout modes for chip collections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipCollectionLayout {
    /// Horizontal row with scrolling
    Row,
    /// Wrap to new lines
    Wrap,
    /// Grid with specified columns
    Grid(u16),
}

// ============================================================================
// Selection Mode Definition
// ============================================================================

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

// ============================================================================
// Chip Collection Implementation
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

impl ChipCollection {
    /// Validate index bounds for collection operations
    fn validate_index(&self, index: usize) -> Result<(), SelectionError> {
        if index >= self.chips.len() {
            Err(SelectionError::InvalidState {
                details: "Chip index out of bounds".to_string(),
            })
        } else {
            Ok(())
        }
    }

    /// Check if selection operations are allowed for this collection
    fn validate_selection_allowed(&self) -> Result<(), SelectionError> {
        if self.selection_mode == ChipSelectionMode::None {
            Err(SelectionError::InvalidState {
                details: "Selection not allowed in this collection".to_string(),
            })
        } else {
            Ok(())
        }
    }
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

    /// Create a chip collection from builder components (internal constructor)
    #[must_use]
    pub(super) fn from_builder_parts(
        chips: Vec<Chip>,
        selection_mode: ChipSelectionMode,
        props: ComponentProps,
        validation_config: ValidationConfig,
    ) -> Self {
        Self {
            chips,
            props,
            selection_mode,
            validation_config,
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
        self.validate_index(index)?;

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
        self.validate_index(index)?;
        self.validate_selection_allowed()?;
        self.chips[index].unselect()
    }

    /// Toggle chip selection by index
    pub fn toggle_chip(&mut self, index: usize) -> Result<ChipState, SelectionError> {
        self.validate_index(index)?;
        self.validate_selection_allowed()?;

        match self.selection_mode {
            ChipSelectionMode::Single => {
                // In single mode, selecting toggles off others
                if self.chips[index].is_selected() {
                    self.chips[index].unselect()?;
                } else {
                    self.select_chip(index)?;
                }
                Ok(self.chips[index].state())
            }
            ChipSelectionMode::Multiple => self.chips[index].toggle(),
            ChipSelectionMode::None => Err(SelectionError::InvalidState {
                details: "Selection not allowed in this collection".to_string(),
            }),
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) -> Result<(), SelectionError> {
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
        if self.selection_mode == ChipSelectionMode::Single {
            let selected_count = self.selected_count();
            if selected_count > 1 {
                return Err(SelectionError::ConflictingStates {
                    details: "Single selection mode allows only one selected chip".to_string(),
                });
            }
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

    // ========================================================================
    // Collection View Methods
    // ========================================================================

    /// Render entire chip collection with automatic layout and spacing
    ///
    /// This is the primary method for rendering chip collections with
    /// consistent spacing and layout.
    ///
    /// # Arguments
    /// * `on_chip_press` - Callback function that receives chip index and state
    /// * `color_scheme` - Material Design color scheme for styling
    ///
    /// # Returns
    /// An Iced Element containing the rendered chip collection
    pub fn view_collection<'a, Message: Clone + 'a>(
        &'a self,
        on_chip_press: impl Fn(usize, ChipState) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        self.view_with_layout(
            ChipCollectionLayout::Wrap,
            8.0, // Standard spacing
            on_chip_press,
            color_scheme,
        )
    }

    /// View with built-in selection state management
    ///
    /// This convenience method automatically handles selection state changes
    /// and provides the updated selection indices.
    ///
    /// # Arguments
    /// * `on_selection_change` - Callback that receives updated selection indices
    /// * `color_scheme` - Material Design color scheme for styling
    pub fn view_with_selection<'a, Message: Clone + 'a>(
        &'a self,
        on_selection_change: impl Fn(Vec<usize>) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        let selected_indices = self.selected_indices();

        self.view_collection(
            move |index, _state| {
                let mut new_selection = selected_indices.clone();

                match self.selection_mode {
                    ChipSelectionMode::None => {
                        // No selection change for assist/suggestion chips
                        on_selection_change(vec![])
                    }
                    ChipSelectionMode::Single => {
                        // Single selection: replace with new index
                        on_selection_change(vec![index])
                    }
                    ChipSelectionMode::Multiple => {
                        // Multiple selection: toggle the clicked chip
                        if let Some(pos) = new_selection.iter().position(|&x| x == index) {
                            new_selection.remove(pos);
                        } else {
                            new_selection.push(index);
                            new_selection.sort_unstable();
                        }
                        on_selection_change(new_selection)
                    }
                }
            },
            color_scheme,
        )
    }

    /// Render collection in different layout modes
    ///
    /// This method provides full control over the layout and spacing of chips
    /// in the collection.
    ///
    /// # Arguments
    /// * `layout` - The layout mode (Row, Wrap, Grid)
    /// * `spacing` - Spacing between chips in pixels
    /// * `on_chip_press` - Callback function for chip interactions
    /// * `color_scheme` - Material Design color scheme for styling
    pub fn view_with_layout<'a, Message: Clone + 'a>(
        &'a self,
        layout: ChipCollectionLayout,
        spacing: f32,
        on_chip_press: impl Fn(usize, ChipState) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        if self.chips.is_empty() {
            return Container::new(iced::widget::Text::new("No chips"))
                .width(Length::Fill)
                .height(Length::Shrink)
                .into();
        }

        match layout {
            ChipCollectionLayout::Row => {
                let row = self.chips.iter().enumerate().fold(
                    Row::new().spacing(spacing),
                    |row, (index, chip)| {
                        let chip_view =
                            chip.view(Some(on_chip_press(index, chip.state())), color_scheme);
                        row.push(chip_view)
                    },
                );

                Scrollable::new(row)
                    .direction(iced::widget::scrollable::Direction::Horizontal(
                        iced::widget::scrollable::Scrollbar::new(),
                    ))
                    .into()
            }
            ChipCollectionLayout::Wrap => {
                // For wrap layout, use a column of rows
                // This is a simplified implementation - in a real app you might want
                // a proper wrapping container
                let column = self
                    .chips
                    .chunks(6) // Wrap after 6 chips per row (adjust as needed)
                    .enumerate()
                    .fold(Column::new().spacing(spacing), |column, (_, chunk)| {
                        let row = chunk.iter().enumerate().fold(
                            Row::new().spacing(spacing),
                            |row, (chunk_index, chip)| {
                                // Calculate the actual index in the full collection
                                let chip_index = chunk_index; // This needs proper calculation
                                let chip_view = chip.view(
                                    Some(on_chip_press(chip_index, chip.state())),
                                    color_scheme,
                                );
                                row.push(chip_view)
                            },
                        );
                        column.push(row)
                    });

                column.into()
            }
            ChipCollectionLayout::Grid(columns) => {
                // Grid layout with specified number of columns
                let column = self.chips.chunks(columns as usize).enumerate().fold(
                    Column::new().spacing(spacing),
                    |column, (row_index, chunk)| {
                        let row = chunk.iter().enumerate().fold(
                            Row::new().spacing(spacing),
                            |row, (col_index, chip)| {
                                let chip_index = row_index * (columns as usize) + col_index;
                                let chip_view = chip.view(
                                    Some(on_chip_press(chip_index, chip.state())),
                                    color_scheme,
                                );
                                row.push(chip_view)
                            },
                        );
                        column.push(row)
                    },
                );

                column.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::material::components::selection::ComponentBuilder;

    #[test]
    fn test_chip_collection_creation() {
        let collection = ChipCollection::new(ChipSelectionMode::Multiple);
        assert_eq!(collection.selection_mode(), ChipSelectionMode::Multiple);
        assert_eq!(collection.len(), 0);
        assert!(collection.selected_chips().is_empty());
    }

    #[test]
    fn test_chip_collection_selection() {
        let mut collection = ChipCollection::new(ChipSelectionMode::Multiple);

        // Add chips
        collection.add_chip(Chip::filter("Option 1").build().unwrap());
        collection.add_chip(Chip::filter("Option 2").build().unwrap());
        collection.add_chip(Chip::filter("Option 3").build().unwrap());

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
        let mut collection = ChipCollection::new(ChipSelectionMode::Single);

        // Add chips
        collection.add_chip(Chip::filter("A").build().unwrap());
        collection.add_chip(Chip::filter("B").build().unwrap());
        collection.add_chip(Chip::filter("C").build().unwrap());

        // Select first chip
        assert!(collection.select_chip(0).is_ok());
        assert_eq!(collection.selected_chips().len(), 1);
        assert!(collection.chips()[0].is_selected());

        // Select second chip (should deselect first in single selection mode)
        assert!(collection.select_chip(1).is_ok());
        assert_eq!(collection.selected_chips().len(), 1);
        assert!(!collection.chips()[0].is_selected());
        assert!(collection.chips()[1].is_selected());
        assert_eq!(collection.selected_chips()[0].label(), "B");
    }

    #[test]
    fn test_chip_collection_none_selection() {
        let mut collection = ChipCollection::new(ChipSelectionMode::None);
        collection.add_chip(Chip::assist("test").build().unwrap());

        // Should not allow selection in None mode
        assert!(collection.select_chip(0).is_err());
        assert!(collection.selected_chips().is_empty());
    }

    #[test]
    fn test_chip_collection_validation() {
        let collection = ChipCollection::new(ChipSelectionMode::Single);
        assert!(collection.validate().is_ok());
    }
}
