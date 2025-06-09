//! Material Design 3 List component implementation

use crate::styling::material::MaterialTokens;
use iced::widget::container;

// Transparent border for containers
fn transparent_border() -> iced::Border {
    iced::Border {
        color: iced::Color::TRANSPARENT,
        width: 0.0,
        radius: 0.0.into(),
    }
}

/// Material Design 3 List Component
pub struct MaterialList;

impl Default for MaterialList {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialList {
    /// Create a Material List widget
    pub fn new() -> Self {
        Self
    }

    /// Style for list container
    pub fn list_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| container::Style {
            background: Some(iced::Background::Color(tokens.colors.surface)),
            border: transparent_border(),
            ..Default::default()
        }
    }

    /// Style for list item
    pub fn list_item(
        tokens: &MaterialTokens,
        is_selected: bool,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| {
            let background = if is_selected {
                tokens.colors.surface_container_high
            } else {
                tokens.colors.surface
            };

            container::Style {
                background: Some(iced::Background::Color(background)),
                border: transparent_border(),
                ..Default::default()
            }
        }
    }
}
