//! Button sizing system for Material Design 3
//!
//! This module provides centralized sizing calculations for all button variants
//! and sizes, replacing the complex pattern matching in the original implementation.

use super::constants::{padding, sizing};
use super::{ButtonSizeVariant, ButtonStyleVariant};
use crate::styling::material::MaterialTokens;
use iced::{Length, Padding};

/// Button size properties structure
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonSizeProperties {
    /// Button width (can be fixed, fill, or shrink)
    pub width: Length,
    /// Button height (can be fixed, fill, or shrink)
    pub height: Length,
    /// Internal padding around button content
    pub padding: Padding,
    /// Optional minimum width constraint
    pub min_width: Option<f32>,
}

/// Centralized sizing calculator for buttons
pub struct ButtonSizeCalculator;

impl ButtonSizeCalculator {
    /// Calculate size properties for a button variant and size combination
    #[must_use]
    pub fn calculate_size(
        variant: ButtonStyleVariant,
        size: ButtonSizeVariant,
        tokens: &MaterialTokens,
    ) -> ButtonSizeProperties {
        match variant {
            ButtonStyleVariant::Icon => Self::calculate_icon_size(size),
            ButtonStyleVariant::Fab => Self::calculate_fab_size(size),
            _ => Self::calculate_standard_size(variant, size, tokens),
        }
    }

    /// Calculate sizes for standard text-containing buttons
    fn calculate_standard_size(
        _variant: ButtonStyleVariant,
        size: ButtonSizeVariant,
        tokens: &MaterialTokens,
    ) -> ButtonSizeProperties {
        let spacing = tokens.spacing();

        match size {
            ButtonSizeVariant::Small => ButtonSizeProperties {
                width: Length::Shrink,
                height: Length::Fixed(sizing::SMALL_HEIGHT),
                padding: Padding::from([padding::SMALL_HORIZONTAL, spacing.xs]),
                min_width: Some(88.0), // Material Design minimum touch target
            },
            ButtonSizeVariant::Medium => ButtonSizeProperties {
                width: Length::Shrink,
                height: Length::Fixed(sizing::MEDIUM_HEIGHT),
                padding: Padding::from([padding::MEDIUM_HORIZONTAL, spacing.md]),
                min_width: Some(100.0),
            },
            ButtonSizeVariant::Large => ButtonSizeProperties {
                width: Length::Shrink,
                height: Length::Fixed(sizing::LARGE_HEIGHT),
                padding: Padding::from([padding::LARGE_HORIZONTAL, spacing.lg]),
                min_width: Some(120.0),
            },
        }
    }

    /// Calculate sizes for icon buttons (square, fixed dimensions)
    const fn calculate_icon_size(size: ButtonSizeVariant) -> ButtonSizeProperties {
        let (dimension, padding_value) = match size {
            ButtonSizeVariant::Small => (sizing::ICON_SMALL, padding::ICON),
            ButtonSizeVariant::Medium => (sizing::ICON_MEDIUM, padding::ICON),
            ButtonSizeVariant::Large => (sizing::ICON_LARGE, padding::ICON),
        };

        ButtonSizeProperties {
            width: Length::Fixed(dimension),
            height: Length::Fixed(dimension),
            padding: Padding::new(padding_value),
            min_width: None,
        }
    }

    /// Calculate sizes for FAB buttons (square, fixed dimensions)
    fn calculate_fab_size(size: ButtonSizeVariant) -> ButtonSizeProperties {
        let (dimension, padding_value) = match size {
            ButtonSizeVariant::Small => (sizing::FAB_SMALL, padding::FAB),
            ButtonSizeVariant::Medium => (sizing::FAB_MEDIUM, padding::FAB),
            ButtonSizeVariant::Large => (sizing::FAB_LARGE, padding::FAB * 1.5),
        };

        ButtonSizeProperties {
            width: Length::Fixed(dimension),
            height: Length::Fixed(dimension),
            padding: Padding::new(padding_value),
            min_width: None,
        }
    }

    /// Get recommended icon size for a button size
    #[must_use]
    pub const fn get_icon_size(size: ButtonSizeVariant) -> f32 {
        match size {
            ButtonSizeVariant::Small => 16.0,
            ButtonSizeVariant::Medium => 18.0,
            ButtonSizeVariant::Large => 20.0,
        }
    }

    /// Check if variant supports text content
    #[must_use]
    pub const fn supports_text(variant: ButtonStyleVariant) -> bool {
        matches!(
            variant,
            ButtonStyleVariant::Filled
                | ButtonStyleVariant::FilledTonal
                | ButtonStyleVariant::Outlined
                | ButtonStyleVariant::Text
                | ButtonStyleVariant::Elevated
        )
    }

    /// Check if variant is icon-only
    #[must_use]
    pub const fn is_icon_only(variant: ButtonStyleVariant) -> bool {
        matches!(variant, ButtonStyleVariant::Icon | ButtonStyleVariant::Fab)
    }
}

/// Builder pattern for button sizing with validation
pub struct ButtonSizeBuilder {
    variant: Option<ButtonStyleVariant>,
    size: Option<ButtonSizeVariant>,
    custom_width: Option<Length>,
    custom_height: Option<Length>,
    custom_padding: Option<Padding>,
}

impl ButtonSizeBuilder {
    /// Create a new button size builder
    #[must_use]
    pub const fn new() -> Self {
        Self {
            variant: None,
            size: None,
            custom_width: None,
            custom_height: None,
            custom_padding: None,
        }
    }

    /// Set the button style variant
    #[must_use]
    pub const fn variant(mut self, variant: ButtonStyleVariant) -> Self {
        self.variant = Some(variant);
        self
    }

    /// Set the button size variant
    #[must_use]
    pub const fn size(mut self, size: ButtonSizeVariant) -> Self {
        self.size = Some(size);
        self
    }

    /// Set a custom width for the button
    #[must_use]
    pub const fn custom_width(mut self, width: Length) -> Self {
        self.custom_width = Some(width);
        self
    }

    /// Set a custom height for the button
    #[must_use]
    pub const fn custom_height(mut self, height: Length) -> Self {
        self.custom_height = Some(height);
        self
    }

    /// Set custom padding for the button
    #[must_use]
    pub const fn custom_padding(mut self, padding: Padding) -> Self {
        self.custom_padding = Some(padding);
        self
    }

    /// Build the button size properties from the configured options
    pub fn build(self, tokens: &MaterialTokens) -> Result<ButtonSizeProperties, &'static str> {
        let variant = self.variant.ok_or("Button variant is required")?;
        let size = self.size.ok_or("Button size is required")?;

        let mut properties = ButtonSizeCalculator::calculate_size(variant, size, tokens);

        // Apply customizations
        if let Some(width) = self.custom_width {
            properties.width = width;
            properties.min_width = None; // Custom width overrides min_width
        }
        if let Some(height) = self.custom_height {
            properties.height = height;
        }
        if let Some(padding) = self.custom_padding {
            properties.padding = padding;
        }

        Ok(properties)
    }
}

impl Default for ButtonSizeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
