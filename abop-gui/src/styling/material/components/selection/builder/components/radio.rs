//! Material Design 3 Radio Button component
//!
//! This module contains the Radio component implementation with all its
//! trait implementations and methods for state management.

use crate::styling::material::components::selection::common::prelude::*;
use crate::styling::material::components::selection_style::{
    SelectionSize as LegacySelectionSize, SelectionStyleBuilder, SelectionVariant,
};
use iced::{Element, Renderer, theme::Theme, widget::Radio as IcedRadio};

/// Material Design 3 Radio Button component (modern implementation)
#[derive(Debug, Clone)]
pub struct Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    pub(crate) value: T,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

impl<T> Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    /// Create a new radio button with the specified value
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            value,
            props: ComponentProps::default(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        }
    }

    /// Create a new radio button with the specified value
    #[must_use]
    pub fn builder(
        value: T,
    ) -> crate::styling::material::components::selection::builder::RadioBuilder<T> {
        crate::styling::material::components::selection::builder::RadioBuilder::new(value)
    }

    /// Get radio button value
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    /// Check if radio button is in error state
    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.error_state
    }

    /// Get animation configuration
    #[must_use]
    pub const fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    /// Set error state
    pub fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }

    /// Check if this radio button is selected based on the current group value
    #[must_use]
    pub fn is_selected(&self, selected_value: Option<&T>) -> bool {
        selected_value == Some(&self.value)
    }

    /// Create an Iced widget element for this radio button
    ///
    /// # Arguments
    /// * `selected_value` - The currently selected value in radio group
    /// * `on_select` - Callback function called when this radio button is selected
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to UI
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        selected_value: Option<T>,
        on_select: impl FnOnce(T) -> Message + Copy + 'a,
        _color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        T: Copy + 'a,
    {
        // Create radio button label
        let default_label = String::new();
        let label = self.props.label.as_ref().unwrap_or(&default_label);

        // Create tokens outside of closure to ensure they live long enough
        let tokens = &*LIGHT_TOKENS; // Default to light tokens for now

        // Create style function with tokens
        let style_fn = {
            let selection_size = match self.props.size {
                ComponentSize::Small => LegacySelectionSize::Small,
                ComponentSize::Medium => LegacySelectionSize::Medium,
                ComponentSize::Large => LegacySelectionSize::Large,
            };

            let builder = SelectionStyleBuilder::new(tokens, SelectionVariant::Radio)
                .size(selection_size)
                .error(self.error_state);

            // Create style function
            builder.radio_style()
        };

        // Create radio widget with style function
        let radio = IcedRadio::new(label, self.value, selected_value, on_select).style(style_fn);

        // Note: Radio widget always requires on_select callback in Iced API
        // Disabled state is handled through styling and visual appearance only

        radio.into()
    }
}

impl<T> PartialEq for Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl<T> Eq for Radio<T> where T: Clone + PartialEq + Eq + std::hash::Hash + Copy {}

// ============================================================================
// Core Trait Implementations for Radio
// ============================================================================

impl<T> SelectionComponent for Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    type State = T;
    type Message = T;

    fn state(&self) -> Self::State {
        self.value
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }

    fn validate(&self) -> Result<(), SelectionError> {
        // Radio validation would go here
        Ok(())
    }

    fn has_error(&self) -> bool {
        self.error_state
    }
}
