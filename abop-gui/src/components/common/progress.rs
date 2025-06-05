//! Progress indicators and related UI components

use iced::Element;
use iced::Length;
use iced::widget::{container, text};

use crate::styling::container::BaseContainerStyles;
use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;

/// Creates a progress indicator with optional progress value
#[must_use]
pub fn create_progress_indicator<'a, Message: Clone + 'a>(
    progress: Option<f32>,
    message: &str,
    theme: ThemeMode,
    tokens: &MaterialTokens,
) -> Element<'a, Message> {
    let progress_text = match progress {
        Some(value) => format!("{:.0}% - {}", value * 100.0, message),
        None => message.to_string(),
    };
    container(text(progress_text))
        .style(BaseContainerStyles::card(theme))
        .padding(tokens.spacing().md)
        .width(Length::Fill)
        .into()
}

/// Creates a toolbar separator
#[must_use]
pub fn create_toolbar_separator<'a, Message: Clone + 'a>(theme: ThemeMode) -> Element<'a, Message> {
    container(text(""))
        .width(1)
        .height(20)
        .style(BaseContainerStyles::transparent(theme))
        .into()
}
