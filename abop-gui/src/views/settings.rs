//! Settings view module
//!
//! This module provides the settings view for configuring application preferences.
//! Settings are now handled directly in the main view using `MaterialDialog` components
//! rather than separate dialog components for better ergonomics and type safety.

use iced::Length;
use iced::widget::{column, container, text};

use crate::messages::Message;
use crate::state::UiState;
use crate::styling::container::LayoutContainerStyles;

/// Creates the settings view
///
/// The settings view displays configuration options for the ABOP application.
/// Note: Modal settings dialogs are handled directly in the main view module
/// using `MaterialDialog` components for better integration and type safety.
#[must_use]
pub fn settings_view(state: &UiState) -> iced::Element<'_, Message> {
    let content = column![
        text("Settings").size(32),
        text("Configure ABOP preferences and options."),
        text("Use the settings button in the navigation bar to open the settings dialog."),
    ]
    .spacing(state.material_tokens.spacing().sm); // Reduced from MD to SM (8px)

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(LayoutContainerStyles::content(state.theme_mode))
        .padding(state.material_tokens.spacing().md) // Reduced from LG to MD (16px)
        .into()
}
