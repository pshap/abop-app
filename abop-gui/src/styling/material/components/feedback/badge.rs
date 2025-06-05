//! Material Design 3 Badge Components
//!
//! This module provides Material Design 3 badge components for notifications,
//! status indicators, and other small informational elements.

use iced::{
    Background, Border, Color, Element, Padding,
    widget::{Space, container, text},
};

use super::style_utils::StatusType;
use crate::styling::material::MaterialTokens;

/// Material Design 3 Badge variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    /// Small dot badge (no content, just indicator)
    Dot,
    /// Number badge (shows count)
    Number,
    /// Text badge (shows short text)
    Text,
}

/// Badge color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeColor {
    /// Error/danger state (red)
    Error,
    /// Warning state (orange/amber)
    Warning,
    /// Success state (green)
    Success,
    /// Info state (blue)
    Info,
    /// Neutral state (uses on-surface)
    Neutral,
}

/// Material Design 3 Badge
///
/// Badges are small components that typically appear near other elements
/// to provide additional information or indicate status.
#[derive(Debug, Clone)]
pub struct MaterialBadge {
    variant: BadgeVariant,
    color: BadgeColor,
    content: Option<String>,
    max_count: Option<u32>, // For number badges, show "99+" style overflow
}

impl Default for MaterialBadge {
    fn default() -> Self {
        Self {
            variant: BadgeVariant::Dot,
            color: BadgeColor::Error,
            content: None,
            max_count: Some(99),
        }
    }
}

impl MaterialBadge {
    /// Create a new dot badge
    #[must_use]
    pub fn dot() -> Self {
        Self {
            variant: BadgeVariant::Dot,
            ..Default::default()
        }
    }

    /// Create a new number badge
    #[must_use]
    pub fn number(count: u32) -> Self {
        Self {
            variant: BadgeVariant::Number,
            content: Some(count.to_string()),
            ..Default::default()
        }
    }

    /// Create a new text badge
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            variant: BadgeVariant::Text,
            content: Some(text.into()),
            ..Default::default()
        }
    }

    /// Set the badge color
    #[must_use]
    pub const fn color(mut self, color: BadgeColor) -> Self {
        self.color = color;
        self
    }

    /// Set the maximum count for number badges
    #[must_use]
    pub const fn max_count(mut self, max: u32) -> Self {
        self.max_count = Some(max);
        self
    }

    /// Get the background and text colors for the badge
    const fn get_colors(&self, tokens: &MaterialTokens) -> (Color, Color) {
        match self.color {
            BadgeColor::Error => (tokens.colors.error.base, tokens.colors.on_error),
            BadgeColor::Warning => (Color::from_rgb(1.0, 0.6, 0.0), Color::WHITE),
            BadgeColor::Success => (Color::from_rgb(0.0, 0.8, 0.0), Color::WHITE),
            BadgeColor::Info => (tokens.colors.primary.base, tokens.colors.on_primary),
            BadgeColor::Neutral => (
                tokens.colors.surface_variant,
                tokens.colors.on_surface_variant,
            ),
        }
    }

    /// Create the badge element
    #[must_use]
    pub fn view<'a, Message>(&'a self, tokens: &'a MaterialTokens) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let (background_color, text_color) = self.get_colors(tokens);

        match self.variant {
            BadgeVariant::Dot => container(Space::new(8, 8))
                .style({
                    move |_theme| container::Style {
                        background: Some(Background::Color(background_color)),
                        border: Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .into(),
            BadgeVariant::Number | BadgeVariant::Text => {
                let display_text = if let Some(content) = &self.content {
                    if self.variant == BadgeVariant::Number {
                        if let Ok(num) = content.parse::<u32>() {
                            if let Some(max) = self.max_count {
                                if num > max {
                                    format!("{max}+")
                                } else {
                                    content.clone()
                                }
                            } else {
                                content.clone()
                            }
                        } else {
                            content.clone()
                        }
                    } else {
                        content.clone()
                    }
                } else {
                    String::new()
                };

                container(
                    text(display_text)
                        .size(tokens.typography.label_small.size)
                        .color(text_color),
                )
                .padding(Padding::from([2, 6]))
                .style({
                    move |_theme| container::Style {
                        background: Some(Background::Color(background_color)),
                        border: Border {
                            radius: 12.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .into()
            }
        }
    }
}

/// Convert from `StatusType` to `BadgeColor`
impl From<StatusType> for BadgeColor {
    fn from(status: StatusType) -> Self {
        match status {
            StatusType::Success => Self::Success,
            StatusType::Warning => Self::Warning,
            StatusType::Error => Self::Error,
            StatusType::Info => Self::Info,
            StatusType::Loading => Self::Info,
            StatusType::Neutral => Self::Neutral,
        }
    }
}

/// Extension methods for `MaterialTokens`
impl MaterialTokens {
    /// Create a notification badge
    #[must_use]
    pub fn notification_badge(&self, count: u32) -> MaterialBadge {
        MaterialBadge::number(count).color(BadgeColor::Error)
    }
}
