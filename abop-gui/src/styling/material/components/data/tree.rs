//! Material Design 3 Tree View component implementation

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

/// Material Design 3 Tree View Component
pub struct MaterialTreeView;

impl Default for MaterialTreeView {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialTreeView {
    /// Create a Material Tree View widget
    pub fn new() -> Self {
        Self
    }

    /// Style for tree container
    pub fn tree_container(tokens: &MaterialTokens) -> impl Fn(&iced::Theme) -> container::Style {
        let tokens = tokens.clone();
        move |_theme| container::Style {
            background: Some(iced::Background::Color(tokens.colors.surface)),
            border: transparent_border(),
            ..Default::default()
        }
    }

    /// Style for tree node
    pub fn tree_node(
        tokens: &MaterialTokens,
        is_selected: bool,
        _level: usize,
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
