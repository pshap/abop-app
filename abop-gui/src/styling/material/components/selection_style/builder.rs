//! Builder pattern and widget integration for selection components
//!
//! This module provides the builder pattern implementation and widget status mappers
//! for integrating with Iced widgets.

use iced::{Background, Border, Color, Theme};

use crate::styling::material::tokens::core::MaterialTokens;

use super::{
    colors::SelectionColors,
    state::{
        SelectionSize, SelectionState, SelectionStyleError, SelectionStyling, SelectionVariant,
    },
    strategy::SelectionStyleStrategy,
};

/// Generic status to selection state mapper trait
trait StatusMapper<T> {
    /// Map widget status to selection state
    fn map_status(&self, status: T, is_selected: bool) -> SelectionState;
}

/// Checkbox status mapper implementation
struct CheckboxStatusMapper;

impl StatusMapper<iced::widget::checkbox::Status> for CheckboxStatusMapper {
    fn map_status(
        &self,
        status: iced::widget::checkbox::Status,
        _is_selected: bool,
    ) -> SelectionState {
        match status {
            iced::widget::checkbox::Status::Active { is_checked: true } => {
                SelectionState::DefaultSelected
            }
            iced::widget::checkbox::Status::Active { is_checked: false } => {
                SelectionState::DefaultUnselected
            }
            iced::widget::checkbox::Status::Hovered { is_checked: true } => {
                SelectionState::HoveredSelected
            }
            iced::widget::checkbox::Status::Hovered { is_checked: false } => {
                SelectionState::HoveredUnselected
            }
            iced::widget::checkbox::Status::Disabled { is_checked: true } => {
                SelectionState::DisabledSelected
            }
            iced::widget::checkbox::Status::Disabled { is_checked: false } => {
                SelectionState::DisabledUnselected
            }
        }
    }
}

/// Radio status mapper implementation
struct RadioStatusMapper;

impl StatusMapper<iced::widget::radio::Status> for RadioStatusMapper {
    fn map_status(
        &self,
        status: iced::widget::radio::Status,
        _is_selected: bool,
    ) -> SelectionState {
        match status {
            iced::widget::radio::Status::Active { is_selected: true } => {
                SelectionState::DefaultSelected
            }
            iced::widget::radio::Status::Active { is_selected: false } => {
                SelectionState::DefaultUnselected
            }
            iced::widget::radio::Status::Hovered { is_selected: true } => {
                SelectionState::HoveredSelected
            }
            iced::widget::radio::Status::Hovered { is_selected: false } => {
                SelectionState::HoveredUnselected
            }
        }
    }
}

/// Button status mapper implementation (for chips and switches)
struct ButtonStatusMapper;

impl StatusMapper<iced::widget::button::Status> for ButtonStatusMapper {
    fn map_status(
        &self,
        status: iced::widget::button::Status,
        is_selected: bool,
    ) -> SelectionState {
        if is_selected {
            match status {
                iced::widget::button::Status::Active => SelectionState::DefaultSelected,
                iced::widget::button::Status::Hovered => SelectionState::HoveredSelected,
                iced::widget::button::Status::Pressed => SelectionState::PressedSelected,
                iced::widget::button::Status::Disabled => SelectionState::DisabledSelected,
            }
        } else {
            match status {
                iced::widget::button::Status::Active => SelectionState::DefaultUnselected,
                iced::widget::button::Status::Hovered => SelectionState::HoveredUnselected,
                iced::widget::button::Status::Pressed => SelectionState::PressedUnselected,
                iced::widget::button::Status::Disabled => SelectionState::DisabledUnselected,
            }
        }
    }
}

/// Builder for creating selection component styling with enhanced capabilities
///
/// Provides a fluent interface for creating Material Design selection styles
/// with comprehensive state handling.
#[derive(Debug)]
pub struct SelectionStyleBuilder<'a> {
    /// The token system to use for styling
    tokens: &'a MaterialTokens,
    /// The component variant being styled
    variant: SelectionVariant,
    /// The size of the component
    size: SelectionSize,
    /// Whether the component is in an error state
    error: bool,
}

impl<'a> SelectionStyleBuilder<'a> {
    /// Create a new selection style builder with borrowed tokens (optimized)
    pub fn new(tokens: &'a MaterialTokens, variant: SelectionVariant) -> Self {
        Self {
            tokens,
            variant,
            size: SelectionSize::Medium,
            error: false,
        }
    }

    /// Set the component size
    #[must_use]
    pub const fn size(mut self, size: SelectionSize) -> Self {
        self.size = size;
        self
    }

    /// Enable error state for validation
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Create a color calculator for this configuration
    pub fn colors(&self) -> SelectionColors {
        SelectionColors::with_tokens(self.tokens, self.variant)
            .with_size(self.size)
            .with_error(self.error)
    }
    /// Get the strategy for this variant
    fn get_strategy(&self) -> Result<Box<dyn SelectionStyleStrategy>, SelectionStyleError> {
        super::strategy::create_strategy(self.variant)
    }
    /// Get styling using the strategy pattern
    fn get_styling(&self, state: SelectionState) -> Result<SelectionStyling, SelectionStyleError> {
        let strategy = self.get_strategy()?;
        strategy.get_styling(state, self.tokens, self.size, self.error)
    }

    /// Create checkbox styling function
    ///
    /// Returns a function that can be used with Iced's checkbox styling system.
    pub fn checkbox_style(
        self,
    ) -> impl Fn(&Theme, iced::widget::checkbox::Status) -> iced::widget::checkbox::Style {
        move |_theme: &Theme, status| {
            let state = CheckboxStatusMapper.map_status(status, false);

            let styling = self.get_styling(state).unwrap_or_else(|_| {
                // Fallback to basic styling if strategy fails
                SelectionStyling {
                    background: Background::Color(Color::TRANSPARENT),
                    text_color: Color::BLACK,
                    border: Border::default(),
                    shadow: None,
                    foreground_color: Color::BLACK,
                    state_layer: None,
                }
            });

            iced::widget::checkbox::Style {
                background: styling.background,
                icon_color: styling.foreground_color,
                border: iced::Border {
                    radius: styling.border.radius,
                    width: styling.border.width,
                    color: styling.border.color,
                },
                text_color: Some(styling.text_color),
            }
        }
    }

    /// Create radio button styling function
    ///
    /// Returns a function that can be used with Iced's radio styling system.
    pub fn radio_style(
        self,
    ) -> impl Fn(&Theme, iced::widget::radio::Status) -> iced::widget::radio::Style {
        move |_theme: &Theme, status: iced::widget::radio::Status| {
            let state = RadioStatusMapper.map_status(status, false);

            let styling = self.get_styling(state).unwrap_or_else(|_| {
                // Fallback to basic styling if strategy fails
                SelectionStyling {
                    background: Background::Color(Color::TRANSPARENT),
                    text_color: Color::BLACK,
                    border: Border::default(),
                    shadow: None,
                    foreground_color: Color::BLACK,
                    state_layer: None,
                }
            });

            iced::widget::radio::Style {
                background: styling.background,
                dot_color: styling.foreground_color,
                border_width: styling.border.width,
                border_color: styling.border.color,
                text_color: Some(styling.text_color),
            }
        }
    }

    /// Create chip button styling function
    ///
    /// Returns a function that can be used with Iced's button styling system for chips.
    pub fn chip_style(
        self,
        is_selected: bool,
    ) -> impl Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        move |_theme: &Theme, status: iced::widget::button::Status| {
            let state = ButtonStatusMapper.map_status(status, is_selected);

            let styling = self.get_styling(state).unwrap_or_else(|_| {
                // Fallback to basic styling if strategy fails
                SelectionStyling {
                    background: Background::Color(Color::TRANSPARENT),
                    text_color: Color::BLACK,
                    border: Border::default(),
                    shadow: None,
                    foreground_color: Color::BLACK,
                    state_layer: None,
                }
            });

            iced::widget::button::Style {
                background: Some(styling.background),
                text_color: styling.text_color,
                border: styling.border,
                shadow: styling.shadow.unwrap_or_default(),
            }
        }
    }

    /// Create switch styling function
    ///
    /// Returns a function that can be used with Iced's button styling system for switches.
    pub fn switch_style(
        self,
        is_enabled: bool,
    ) -> impl Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        move |_theme: &Theme, status: iced::widget::button::Status| {
            let state = ButtonStatusMapper.map_status(status, is_enabled);

            let styling = self.get_styling(state).unwrap_or_else(|_| {
                // Fallback to basic styling if strategy fails
                SelectionStyling {
                    background: Background::Color(Color::TRANSPARENT),
                    text_color: Color::BLACK,
                    border: Border::default(),
                    shadow: None,
                    foreground_color: Color::BLACK,
                    state_layer: None,
                }
            });

            iced::widget::button::Style {
                background: Some(styling.background),
                text_color: styling.text_color,
                border: Border {
                    radius: 16.0.into(), // Switch-specific border radius
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: styling.shadow.unwrap_or_default(),
            }
        }
    }
}
