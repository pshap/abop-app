//! Container Component
//!
//! This module provides Material Design 3 Container component with
//! configurable surface variants, padding, spacing, and alignment options.

use crate::styling::material::MaterialTokens;
use iced::{Background, Color, Element, Length, Padding, Theme, Vector};

/// Container alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerAlignment {
    /// Align items to the start
    Start,
    /// Center items
    Center,
    /// Align items to the end
    End,
    /// Stretch items to fill
    Fill,
}

/// Material Design 3 Container component
#[derive(Debug, Clone)]
pub struct MaterialContainer {
    /// Optional background surface variant
    pub surface_variant: Option<SurfaceVariant>,
    /// Container padding
    pub padding: Padding,
    /// Container spacing for child elements
    pub spacing: f32,
    /// Optional width
    pub width: Option<Length>,
    /// Optional height
    pub height: Option<Length>,
    /// Container alignment
    pub alignment: ContainerAlignment,
}

impl Default for MaterialContainer {
    fn default() -> Self {
        Self {
            surface_variant: None,
            padding: Padding::from(16),
            spacing: 16.0,
            width: None,
            height: None,
            alignment: ContainerAlignment::Fill,
        }
    }
}

impl MaterialContainer {
    /// Create a new Material container
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set surface variant
    #[must_use]
    pub const fn surface_variant(mut self, variant: SurfaceVariant) -> Self {
        self.surface_variant = Some(variant);
        self
    }

    /// Set container padding
    #[must_use]
    pub const fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Set spacing between child elements
    #[must_use]
    pub const fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set container width
    #[must_use]
    pub const fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    /// Set container height
    #[must_use]
    pub const fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    /// Set container alignment
    #[must_use]
    pub const fn alignment(mut self, alignment: ContainerAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Create a container element with material styling
    #[must_use]
    pub fn container<'a, Message: 'a>(
        &self,
        content: Element<'a, Message>,
        tokens: &MaterialTokens,
    ) -> Element<'a, Message> {
        let colors = &tokens.colors;
        let style = match &self.surface_variant {
            Some(variant) => {
                let background_color = variant.style(tokens);
                iced::container::Style {
                    background: Some(Background::Color(background_color)),
                    ..Default::default()
                }
            }
            None => iced::container::Style::default(),
        };

        let mut container = iced::container(content)
            .style(style)
            .padding(self.padding)
            .width(self.width.unwrap_or(iced::Length::Fill))
            .height(self.height.unwrap_or(iced::Length::Shrink))
            .align_x(match self.alignment {
                ContainerAlignment::Start => iced::Alignment::Start,
                ContainerAlignment::Center => iced::Alignment::Center,
                ContainerAlignment::End => iced::Alignment::End,
                ContainerAlignment::Fill => iced::Alignment::Fill,
            })
            .spacing(self.spacing);

        container.into()
    }
}
