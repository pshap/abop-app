//! Strategy System Usage Examples
//!
//! This module provides complete examples of how to use the Color Strategy System
//! in Material Design 3 components, demonstrating the proper patterns for
//! consistent, accessible, and theme-aware styling.

use crate::styling::{
    MaterialTokens,
    strategy::{ButtonStyleVariant, ComponentState},
};
use iced::{Color, Element};

/// Example: Creating a custom button using the strategy system
///
/// This shows the correct way to implement component styling with the strategy system.
/// Notice how NO direct color access is used - everything goes through the strategy.
pub fn create_strategy_button<'a, Message: Clone + 'a>(
    label: &'a str,
    variant: ButtonStyleVariant,
    state: ComponentState,
    tokens: &'a MaterialTokens,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    // Step 1: Get the appropriate strategy for the button variant
    let strategy = variant.get_strategy();

    // Step 2: Get styling from the strategy (NO direct color access!)
    let styling = strategy.get_styling(state, tokens);

    // Step 3: Apply the styling to create the button
    let button = iced::widget::button(
        iced::widget::text(label).color(styling.text_color), // Use strategy-provided color
    );

    // Apply click handler if provided
    if let Some(message) = on_press {
        button.on_press(message).into()
    } else {
        button.into()
    }
}

/// Example: Bad pattern - DO NOT USE
///
/// This shows the WRONG way to handle colors directly.
/// This pattern should never be used in the application.
#[allow(dead_code)]
fn bad_example_direct_color_access<'a, Message: Clone + 'a>(
    label: &'a str,
    tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    // ❌ BAD: Direct color access bypasses the strategy system
    let colors = &tokens.colors;
    let _bad_background = colors.primary.base;
    let bad_text_color = colors.on_primary(); // Direct access!

    iced::widget::button(
        iced::widget::text(label).color(bad_text_color), // ❌ This bypasses accessibility checks
    )
    .into()
}

/// Example: Good pattern using semantic colors when needed
///
/// Sometimes you need semantic colors for specific purposes (success, error, etc.).
/// This shows the correct way to use them with the strategy system.
pub fn create_semantic_button<'a, Message: Clone + 'a>(
    label: &'a str,
    semantic_type: SemanticColorType,
    state: ComponentState,
    tokens: &'a MaterialTokens,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    // ✅ GOOD: Use semantic colors for semantic purposes
    let semantic_colors = tokens.semantic_colors();

    let (background_color, text_color) = match semantic_type {
        SemanticColorType::Success => (semantic_colors.success, Color::WHITE),
        SemanticColorType::Warning => (semantic_colors.warning, Color::BLACK),
        SemanticColorType::Error => (semantic_colors.error, Color::WHITE),
        SemanticColorType::Info => (semantic_colors.info, Color::WHITE),
    };

    // Apply state-based opacity
    let opacity = state.opacity_modifier();
    let _final_background = Color::new(
        background_color.r,
        background_color.g,
        background_color.b,
        background_color.a * opacity,
    );

    let button = iced::widget::button(iced::widget::text(label).color(text_color));

    if let Some(message) = on_press {
        button.on_press(message).into()
    } else {
        button.into()
    }
}

/// Semantic color types for special-purpose buttons
#[derive(Debug, Clone, Copy)]
pub enum SemanticColorType {
    /// Success action (e.g., Save, Submit)
    Success,
    /// Warning action (e.g., caution required)
    Warning,
    /// Error action (e.g., Delete, Cancel)
    Error,
    /// Informational action (e.g., Help, More Info)
    Info,
}

/// Example: Complete button widget with full strategy integration
///
/// This demonstrates a production-ready component that follows all best practices.
pub struct StrategyButton<'a, Message> {
    label: &'a str,
    variant: ButtonStyleVariant,
    state: ComponentState,
    tokens: &'a MaterialTokens,
    on_press: Option<Message>,
    width: iced::Length,
    height: iced::Length,
}

impl<'a, Message: Clone + 'a> StrategyButton<'a, Message> {
    /// Create a new strategy button
    pub fn new(label: &'a str, tokens: &'a MaterialTokens) -> Self {
        Self {
            label,
            variant: ButtonStyleVariant::Filled,
            state: ComponentState::Default,
            tokens,
            on_press: None,
            width: iced::Length::Shrink,
            height: iced::Length::Fixed(40.0),
        }
    }

    /// Set button style variant
    pub fn variant(mut self, variant: ButtonStyleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set button component state
    pub fn state(mut self, state: ComponentState) -> Self {
        self.state = state;
        self
    }

    /// Set button press message
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Set button width
    pub fn width(mut self, width: iced::Length) -> Self {
        self.width = width;
        self
    }

    /// Set button height
    pub fn height(mut self, height: iced::Length) -> Self {
        self.height = height;
        self
    }

    /// Build the button element
    pub fn build(self) -> Element<'a, Message> {
        create_strategy_button(
            self.label,
            self.variant,
            self.state,
            self.tokens,
            self.on_press,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strategy_button_creation() {
        let tokens = MaterialTokens::light();

        // Test that the builder pattern works
        let _button = StrategyButton::<()>::new("Test", &tokens)
            .variant(ButtonStyleVariant::Outlined)
            .state(ComponentState::Hovered);

        // Test passes if no panic occurs during button creation
    }

    #[test]
    fn test_semantic_button_types() {
        let tokens = MaterialTokens::light();

        // Test all semantic types compile and execute
        for semantic_type in [
            SemanticColorType::Success,
            SemanticColorType::Warning,
            SemanticColorType::Error,
            SemanticColorType::Info,
        ] {
            let _button = create_semantic_button(
                "Test",
                semantic_type,
                ComponentState::Default,
                &tokens,
                None::<()>,
            );
        }
    }

    #[test]
    fn test_all_button_variants_work() {
        let tokens = MaterialTokens::light();

        for variant in [
            ButtonStyleVariant::Filled,
            ButtonStyleVariant::FilledTonal,
            ButtonStyleVariant::Outlined,
            ButtonStyleVariant::Text,
            ButtonStyleVariant::Elevated,
        ] {
            let _button = create_strategy_button(
                "Test",
                variant,
                ComponentState::Default,
                &tokens,
                None::<()>,
            );
        }
    }

    #[test]
    fn test_all_component_states_work() {
        let tokens = MaterialTokens::light();

        for state in [
            ComponentState::Default,
            ComponentState::Hovered,
            ComponentState::Pressed,
            ComponentState::Focused,
            ComponentState::Disabled,
            ComponentState::Loading,
        ] {
            let _button = create_strategy_button(
                "Test",
                ButtonStyleVariant::Filled,
                state,
                &tokens,
                None::<()>,
            );
        }
    }
}
