//! A modern, type-safe button builder for Material Design 3 buttons.
//! 
//! This module provides a flexible and type-safe API for creating Material Design 3
//! buttons with consistent styling and behavior.
//!
//! # Examples
//!
//! ```no_run
//! use abop_gui::components::buttons::{self, ButtonVariant, IconPosition};
//! use abop_gui::styling::material::MaterialTokens;
//! use iced::Element;
//!
//! #[derive(Debug, Clone)]
//! enum Message { Save, Cancel }
//!
//! # fn example(tokens: &MaterialTokens) -> Element<'static, Message> {
//! // Create a primary button with error handling
//! let save_btn = buttons::create_button(
//!     || buttons::button(tokens)
//!         .label("Save")
//!         .variant(ButtonVariant::Filled)
//!         .on_press(Message::Save)
//!         .build(),
//!     "save",
//!     Some("Save")
//! );
//!
//! // Create a button with an icon using the direct builder (if you want custom error handling)
//! let export_btn = buttons::button(tokens)
//!     .label("Export")
//!     .icon("download", IconPosition::Leading)
//!     .variant(ButtonVariant::Outlined)
//!     .on_press(Message::Save)
//!     .build()
//!     .expect("Failed to build button");
//! # save_btn
//! # }
//! ```

pub mod builder;
pub mod error;
pub mod variants;
pub mod icons;
pub mod conversions;

#[doc(inline)]
pub use builder::ButtonBuilder;
#[doc(inline)]
pub use error::ButtonError;
#[doc(inline)]
pub use variants::ButtonVariant;

use crate::styling::material::MaterialTokens;
use iced::Element;

/// Create a new button builder with default settings.
///
/// This is the main entry point for creating buttons. The builder provides
/// a fluent interface for configuring the button's appearance and behavior.
///
/// # Arguments
/// * `tokens` - The Material Design tokens for theming
///
/// # Returns
/// A new `ButtonBuilder` instance
pub fn button<'a, M: Clone + 'a>(tokens: &'a MaterialTokens) -> ButtonBuilder<'a, M> {
    ButtonBuilder::new(tokens)
}

/// Create a button with consistent error handling.
///
/// This helper function eliminates duplicated error handling logic across button creation sites.
/// When a button fails to build, it logs a warning and returns a fallback text element.
///
/// # Arguments
/// * `build_fn` - A closure that returns a Result<Element<Message>, ButtonError>
/// * `button_name` - A descriptive name for the button (used in log messages)
/// * `fallback_text` - Optional fallback text to display on error (defaults to "⚠️")
///
/// # Returns
/// An Element that is either the successfully built button or a fallback text widget
///
/// # Examples
///
/// ```no_run
/// use abop_gui::components::buttons;
/// use abop_gui::styling::material::MaterialTokens;
///
/// #[derive(Debug, Clone)]
/// enum Message { Save }
///
/// # fn example(tokens: &MaterialTokens) -> iced::Element<'static, Message> {
/// let save_btn = buttons::create_button(
///     || buttons::button(tokens)
///         .label("Save")
///         .on_press(Message::Save)
///         .build(),
///     "save",
///     Some("Save")
/// );
/// # save_btn
/// # }
/// ```
pub fn create_button<'a, M: Clone + 'a>(
    build_fn: impl FnOnce() -> Result<Element<'a, M>, ButtonError>, 
    button_name: &str,
    fallback_text: Option<&'a str>
) -> Element<'a, M> {
    build_fn().unwrap_or_else(|e| {
        log::warn!("Failed to build {} button: {}", button_name, e);
        iced::widget::Text::new(fallback_text.unwrap_or("⚠️")).into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    enum TestMessage { Action }

    #[test]
    fn test_button_creation() {
        let tokens = MaterialTokens::default();
        
        // Test basic button creation
        let button = button(&tokens)
            .label("Test")
            .variant(ButtonVariant::Filled)
            .on_press(TestMessage::Action)
            .build()
            .unwrap();
            
        // Just verify it compiles and runs without panicking
        // Note: Element doesn't have a public Widget variant we can match against
        // so we just ensure the button was created successfully
        std::mem::drop(button); // Explicitly drop to verify it's valid
    }

    #[test]
    fn test_create_button_helper_success() {
        let tokens = MaterialTokens::default();
        
        // Test successful button creation with helper
        let button = create_button::<TestMessage>(
            || button(&tokens)
                .label("Test")
                .variant(ButtonVariant::Filled)
                .on_press(TestMessage::Action)
                .build(),
            "test",
            Some("Test")
        );
        
        std::mem::drop(button); // Verify it's valid
    }

    #[test]
    fn test_create_button_helper_fallback() {
        let tokens = MaterialTokens::default();
        
        // Test button creation that fails and falls back to text
        let button = create_button::<TestMessage>(
            || button(&tokens)
                .variant(ButtonVariant::Filled)
                // Missing both label and on_press, should fail
                .build(),
            "test",
            Some("Fallback")
        );
        
        std::mem::drop(button); // Verify it's valid (should be text widget)
    }
}
