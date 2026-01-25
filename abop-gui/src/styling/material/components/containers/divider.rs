//! Divider Component
//!
//! This module provides Material Design 3 Divider component with
//! horizontal and vertical orientations, configurable thickness, and styling integration.

use iced::widget::Rule;
use iced::{Element, Length, Theme, Vector};

/// Divider orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerOrientation {
    /// Horizontal divider
    Horizontal,
    /// Vertical divider
    Vertical,
}

/// Material Design 3 Divider component
#[derive(Debug, Clone)]
pub struct MaterialDivider {
    /// The orientation of the divider
    pub orientation: DividerOrientation,
    /// The thickness of the divider
    pub thickness: f32,
    /// The length of the divider
    pub length: Length,
    /// Whether to use full width/height
    pub full_bleed: bool,
}

impl Default for MaterialDivider {
    fn default() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            thickness: 1.0,
            length: Length::Fill,
            full_bleed: false,
        }
    }
}

impl MaterialDivider {
    /// Create a new Material divider
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a horizontal divider
    #[must_use]
    pub fn horizontal() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            ..Self::default()
        }
    }

    /// Create a vertical divider
    #[must_use]
    pub fn vertical() -> Self {
        Self {
            orientation: DividerOrientation::Vertical,
            ..Self::default()
        }
    }

    /// Set the thickness of the divider
    #[must_use]
    pub const fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set the length of the divider
    #[must_use]
    pub const fn length(mut self, length: Length) -> Self {
        self.length = length;
        self
    }

    /// Set whether the divider should be full bleed
    #[must_use]
    pub const fn full_bleed(mut self, full_bleed: bool) -> Self {
        self.full_bleed = full_bleed;
        self
    }

    /// Create a rule element with the divider styling
    #[must_use]
    pub fn rule<'a, Message: 'a>(&self, tokens: &MaterialTokens) -> Element<'a, Message> {
        let colors = &tokens.colors;
        let outline_color = colors.outline_variant;
        let thickness = self.thickness;

        match self.orientation {
            DividerOrientation::Horizontal => Rule::horizontal(thickness)
                .style(move |_theme: &Theme| rule::Style {
                    color: outline_color,
                    width: thickness.round().clamp(0.0, f32::from(u16::MAX)) as u16,
                    radius: 0.0.into(),
                    fill_mode: rule::FillMode::Full,
                })
                .into(),
            DividerOrientation::Vertical => Rule::vertical(thickness)
                .style(move |_theme: &Theme| rule::Style {
                    color: outline_color,
                    width: thickness.round().clamp(0.0, f32::from(u16::MAX)) as u16,
                    radius: 0.0.into(),
                    fill_mode: rule::FillMode::Full,
                })
                .into(),
        }
    }
}
