//! Unified view methods for chip rendering
//!
//! This module provides view methods for rendering chips as Iced widgets with
//! Material Design 3 styling. It includes different view methods for various
//! use cases such as basic view, toggle view, and filter chip view.

use super::super::builder::Chip;
use super::super::common::{ChipState, ComponentSize};
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
// Chip View Methods Implementation
// ============================================================================

impl Chip {
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
        let legacy_size = match self.props().size {
            ComponentSize::Small => LegacySelectionSize::Small,
            ComponentSize::Medium => LegacySelectionSize::Medium,
            ComponentSize::Large => LegacySelectionSize::Large,
        };

        // Create styling function
        let style_fn = SelectionStyleBuilder::new(color_scheme.clone(), SelectionVariant::Chip)
            .size(legacy_size)
            .chip_style(self.is_selected());

        // Create chip content
        let content = Text::new(self.label()).size(self.props().size.text_size());

        // Create chip button
        let mut chip_button = button(content).style(style_fn);

        // Only add on_press handler if the chip is not disabled and callback is provided
        if !self.props().disabled
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
        let next_state = self.state().toggle();
        let dummy_message = on_toggle(next_state);
        self.view(Some(dummy_message), color_scheme)
            .map(move |_| on_toggle(next_state))
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
        self.view(Some(dummy_message), color_scheme)
            .map(move |_| on_selection_change(new_selection))
    }
}
