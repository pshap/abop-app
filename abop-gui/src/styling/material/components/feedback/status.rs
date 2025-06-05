//! Material Design 3 Status Indicators
//!
//! This module provides Material Design 3 status indicator components for
//! showing success, warning, error, info, loading, and neutral states.

use iced::{
    Alignment, Element, Padding,
    widget::{Row, Space, container, text},
};

use crate::styling::container::FeedbackContainerStyles;
use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;

use super::style_utils::{StatusType, get_status_colors};

/// Status indicator size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusSize {
    /// Small status indicator (16px)
    Small,
    /// Medium status indicator (24px)
    Medium,
    /// Large status indicator (32px)
    Large,
}

/// Material Design 3 Status Indicator
///
/// Status indicators provide visual feedback about the state of a process,
/// validation result, or system status.
#[derive(Debug, Clone)]
pub struct MaterialStatusIndicator {
    status: StatusType,
    size: StatusSize,
    with_icon: bool,
    with_text: bool,
    text_content: Option<String>,
}

impl Default for MaterialStatusIndicator {
    fn default() -> Self {
        Self {
            status: StatusType::Info,
            size: StatusSize::Medium,
            with_icon: true,
            with_text: false,
            text_content: None,
        }
    }
}

impl MaterialStatusIndicator {
    /// Create a new status indicator
    #[must_use]
    pub fn new(status: StatusType) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }

    /// Set the status indicator size
    #[must_use]
    pub const fn size(mut self, size: StatusSize) -> Self {
        self.size = size;
        self
    }

    /// Include an icon in the status indicator
    #[must_use]
    pub const fn with_icon(mut self) -> Self {
        self.with_icon = true;
        self
    }

    /// Include text in the status indicator
    #[must_use]
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.with_text = true;
        self.text_content = Some(text.into());
        self
    }

    /// Get the default text for a status type
    const fn default_text_for_status(&self) -> &'static str {
        match self.status {
            StatusType::Success => "Success",
            StatusType::Warning => "Warning",
            StatusType::Error => "Error",
            StatusType::Info => "Information",
            StatusType::Loading => "Loading...",
            StatusType::Neutral => "Status",
        }
    }

    /// Create the status indicator element
    #[must_use]
    pub fn view<'a, Message>(&'a self, tokens: &'a MaterialTokens) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let (background_color, text_color) = get_status_colors(tokens, &self.status);

        // Determine if we should use the theme's dark or light mode
        let theme_mode = if tokens.is_dark_theme() {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        let size_px = match self.size {
            StatusSize::Small => 16,
            StatusSize::Medium => 24,
            StatusSize::Large => 32,
        };

        let status_icon: Element<'_, Message> = if self.status == StatusType::Loading {
            // For loading status, use a simple loading indicator (avoiding lifetime issues)
            text("⟳")
                .size(f32::from(size_px))
                .color(background_color)
                .into()
        } else if self.with_icon {
            // For other statuses, use a colored dot
            container(Space::new(size_px / 2, size_px / 2))
                .style(move |_| container::Style {
                    background: Some(iced::Background::Color(background_color)),
                    border: iced::Border {
                        radius: (f32::from(size_px) / 4.0).into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into()
        } else {
            Space::new(0, 0).into()
        };

        let mut row = Row::new().spacing(8).align_y(Alignment::Center);

        if self.with_icon {
            row = row.push(status_icon);
        }

        if self.with_text {
            let display_text = self
                .text_content
                .as_deref()
                .unwrap_or_else(|| self.default_text_for_status());
            row = row.push(
                text(display_text)
                    .size(tokens.typography.label_medium.size)
                    .color(text_color),
            );
        }

        // Use the appropriate container style based on status
        let mut result = container(row).padding(Padding::from([4, 8]));

        if self.status == StatusType::Success {
            result = result.style(FeedbackContainerStyles::success(theme_mode));
        } else if self.status == StatusType::Warning {
            result = result.style(FeedbackContainerStyles::warning(theme_mode));
        } else if self.status == StatusType::Error {
            result = result.style(FeedbackContainerStyles::error(theme_mode));
        } else if self.status == StatusType::Info {
            result = result.style(FeedbackContainerStyles::info(theme_mode));
        } else if self.status == StatusType::Loading {
            result = result.style(FeedbackContainerStyles::status_info(theme_mode));
        } else {
            result = result.style(FeedbackContainerStyles::status(theme_mode));
        }

        result.into()
    }
}

/// Extension methods for `MaterialTokens`
impl MaterialTokens {
    /// Create a status badge
    #[must_use]
    pub fn status_badge(&self, status: StatusType) -> MaterialStatusIndicator {
        MaterialStatusIndicator::new(status)
    }
}
