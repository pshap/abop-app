//! Shared styling utilities for feedback components
//!
//! This module provides common styling functions and utilities used across
//! different feedback components to ensure consistency and reduce code duplication.

use iced::{
    Background, Border, Color, Shadow,
    widget::{Text, button, container},
};

// Removed unused imports: DialogContainerStyles, FeedbackContainerStyles
use crate::styling::material::{MaterialTokens, elevation::ElevationLevel};

/// Get semantic colors for different status types
#[must_use]
pub const fn get_status_colors(
    tokens: &MaterialTokens,
    status_type: &StatusType,
) -> (Color, Color) {
    match status_type {
        StatusType::Success => (Color::from_rgb(0.0, 0.8, 0.0), Color::WHITE),
        StatusType::Warning => (Color::from_rgb(1.0, 0.6, 0.0), Color::WHITE),
        StatusType::Error => (tokens.colors.error.base, tokens.colors.error.on_base),
        StatusType::Info => (tokens.colors.primary.base, tokens.colors.primary.on_base),
        StatusType::Loading => (tokens.colors.primary.base, tokens.colors.primary.on_base),
        StatusType::Neutral => (
            tokens.colors.surface_variant,
            tokens.colors.on_surface_variant,
        ),
    }
}

/// Create a standard action button with consistent styling
pub fn create_action_button<'a, Message>(
    tokens: &'a MaterialTokens,
    label: &'a str,
    on_press: Message,
    is_primary: bool,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    let button_style = if is_primary {
        button::Style {
            background: Some(Background::Color(tokens.colors.primary.base)),
            text_color: tokens.colors.primary.on_base,
            border: Border {
                radius: tokens.shapes.corner_small.radius,
                ..Default::default()
            },
            shadow: tokens.elevation.get_level(ElevationLevel::Level1).shadow,
        }
    } else {
        button::Style {
            background: Some(Background::Color(tokens.colors.surface)),
            text_color: tokens.colors.primary.base,
            border: Border {
                color: tokens.colors.outline,
                width: 1.0,
                radius: tokens.shapes.corner_small.radius,
            },
            shadow: Shadow::default(),
        }
    };

    button(Text::new(label).size(tokens.typography.label_large.size))
        .padding([8, 16])
        .style(move |_theme, _status| button_style)
        .on_press(on_press)
}

/// Create a container with elevation
pub fn create_elevated_container<'a, Message>(
    tokens: &'a MaterialTokens,
    content: impl Into<iced::Element<'a, Message>>,
    elevation_level: ElevationLevel,
) -> container::Container<'a, Message>
where
    Message: 'a,
{
    container(content).style(move |_theme| container::Style {
        background: Some(Background::Color(tokens.colors.surface)),
        border: Border {
            radius: tokens.shapes.corner_medium.radius,
            ..Default::default()
        },
        shadow: tokens.elevation.get_level(elevation_level).shadow,
        ..Default::default()
    })
}

/// Status types with semantic meaning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusType {
    /// Success state (green)
    Success,
    /// Warning state (amber/orange)
    Warning,
    /// Error state (red)
    Error,
    /// Info state (blue)
    Info,
    /// Loading/pending state
    Loading,
    /// Neutral/inactive state
    Neutral,
}
