//! Material Design 3 Modal Components
//!
//! This module provides Material Design 3 modal overlay components for
//! creating modal dialogs and other overlay content.

use iced::{
    Alignment, Element, Length,
    widget::{Column, container},
};

use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;

/// Modal state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalState {
    /// Modal is visible
    Visible,
    /// Modal is hidden
    Hidden,
    /// Modal is being dismissed
    Dismissed,
}

/// Material Design 3 Modal overlay for dialogs
#[derive(Debug, Clone)]
pub struct MaterialModal {
    /// Background overlay opacity
    overlay_opacity: f32,
    /// Whether clicking overlay dismisses modal
    dismissible: bool,
}

impl Default for MaterialModal {
    fn default() -> Self {
        Self {
            overlay_opacity: 0.5,
            dismissible: true,
        }
    }
}

impl MaterialModal {
    /// Create a new modal overlay
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set overlay opacity
    #[must_use]
    pub const fn opacity(mut self, opacity: f32) -> Self {
        self.overlay_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set whether clicking overlay dismisses modal
    #[must_use]
    pub const fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Create the modal overlay element
    ///
    /// Note: This component is deprecated in favor of the stack-based modal
    /// approach used in the main view module. New code should use the modal
    /// function in views/mod.rs for creating modal overlays.
    pub fn view<'a, Message>(
        &'a self,
        tokens: &'a MaterialTokens,
        content: impl Into<Element<'a, Message>>,
        on_dismiss: Option<Message>,
    ) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        // Determine if we should use the theme's dark or light mode
        let _theme_mode = if tokens.is_dark_theme() {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        // Fallback style for deprecated modal (should not be used in new code)
        let overlay_style = |_: &iced::Theme| {
            use iced::widget::container;
            container::Style {
                background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
                ..container::Style::default()
            }
        };

        // Create the overlay container
        let overlay = container(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .push(content.into()),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(overlay_style); // Add on_press if dismissible - wrap in MaterialButton since Container doesn't have on_press
        if self.dismissible {
            if let Some(msg) = on_dismiss {
                use crate::styling::material::components::widgets::MaterialButton;
                MaterialButton::new_with_content(overlay, tokens)
                    .variant(
                        crate::styling::material::components::widgets::MaterialButtonVariant::Text,
                    )
                    .on_press(msg)
                    .into()
            } else {
                overlay.into()
            }
        } else {
            overlay.into()
        }
    }
}
