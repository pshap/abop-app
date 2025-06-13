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
//! // Create a primary button
//! let save_btn = buttons::button(tokens)
//!     .label("Save")
//!     .variant(ButtonVariant::Filled)
//!     .on_press(Message::Save)
//!     .build()
//!     .expect("Failed to build button");
//!
//! // Create a button with an icon
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
}
