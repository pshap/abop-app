//! Utility styling functions with design token integration
//!
//! This module provides utility functions and helper styles that can be reused
//! across different components.

use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;
use iced::{Background, Border, Color, Length, Padding};

/// Utility functions for common styling patterns
pub struct StyleUtils;

impl StyleUtils {
    /// Get standard padding values using `MaterialTokens`
    ///
    /// # Returns
    /// Padding for extra small spacing
    #[must_use]
    pub fn padding_xs(tokens: &MaterialTokens) -> Padding {
        Padding::from(tokens.spacing().xs)
    }

    /// Get small padding value using `MaterialTokens`
    ///
    /// # Returns
    /// Padding for small spacing
    #[must_use]
    pub fn padding_sm(tokens: &MaterialTokens) -> Padding {
        Padding::from(tokens.spacing().sm)
    }

    /// Get medium padding value using `MaterialTokens`
    ///
    /// # Returns
    /// Padding for medium spacing
    #[must_use]
    pub fn padding_md(tokens: &MaterialTokens) -> Padding {
        Padding::from(tokens.spacing().md)
    }

    /// Get large padding value using `MaterialTokens`
    ///
    /// # Returns
    /// Padding for large spacing
    #[must_use]
    pub fn padding_lg(tokens: &MaterialTokens) -> Padding {
        Padding::from(tokens.spacing().lg)
    }

    /// Get extra large padding value using `MaterialTokens`
    ///
    /// # Returns
    /// Padding for extra large spacing
    #[must_use]
    pub fn padding_xl(tokens: &MaterialTokens) -> Padding {
        Padding::from(tokens.spacing().xl)
    }

    /// Get double extra large padding value using `MaterialTokens`
    ///
    /// # Returns
    /// Padding for double extra large spacing
    #[must_use]
    pub fn padding_xxl(tokens: &MaterialTokens) -> Padding {
        Padding::from(tokens.spacing().xxl)
    }

    /// Get asymmetric padding values
    #[must_use]
    pub const fn padding_horizontal(amount: f32) -> Padding {
        Padding {
            top: 0.0,
            right: amount,
            bottom: 0.0,
            left: amount,
        }
    }

    /// Get vertical padding with specified amount
    ///
    /// # Arguments
    /// * `amount` - The vertical padding value
    ///
    /// # Returns
    /// Padding with vertical spacing
    #[must_use]
    pub const fn padding_vertical(amount: f32) -> Padding {
        Padding {
            top: amount,
            right: 0.0,
            bottom: amount,
            left: 0.0,
        }
    }

    /// Get standard spacing values
    #[must_use]
    pub const fn spacing_xs(tokens: &MaterialTokens) -> f32 {
        tokens.spacing().xs
    }

    /// Get small spacing value using `MaterialTokens`
    ///
    /// # Returns
    /// Small spacing value
    #[must_use]
    pub const fn spacing_sm(tokens: &MaterialTokens) -> f32 {
        tokens.spacing().sm
    }

    /// Get medium spacing value using `MaterialTokens`
    ///
    /// # Returns
    /// Medium spacing value
    #[must_use]
    pub const fn spacing_md(tokens: &MaterialTokens) -> f32 {
        tokens.spacing().md
    }

    /// Get large spacing value using `MaterialTokens`
    ///
    /// # Returns
    /// Large spacing value
    #[must_use]
    pub const fn spacing_lg(tokens: &MaterialTokens) -> f32 {
        tokens.spacing().lg
    }

    /// Get extra large spacing value using `MaterialTokens`
    ///
    /// # Returns
    /// Extra large spacing value
    #[must_use]
    pub const fn spacing_xl(tokens: &MaterialTokens) -> f32 {
        tokens.spacing().xl
    }

    /// Get double extra large spacing value using `MaterialTokens`
    ///
    /// # Returns
    /// Double extra large spacing value
    #[must_use]
    pub const fn spacing_xxl(tokens: &MaterialTokens) -> f32 {
        tokens.spacing().xxl
    }

    /// Get common color utilities
    #[must_use]
    pub const fn color_with_alpha(color: Color, alpha: f32) -> Color {
        Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: alpha,
        }
    }

    /// Get semantic colors for different states
    #[must_use]
    pub fn state_color(theme_mode: ThemeMode, state: ComponentState) -> Color {
        match state {
            ComponentState::Default => theme_mode.text_primary_color(),
            ComponentState::Hover => theme_mode.primary_color(),
            ComponentState::Active => theme_mode.primary_light_color(),
            ComponentState::Disabled => theme_mode.text_disabled_color(),
            ComponentState::Success => theme_mode.success_color(),
            ComponentState::Error => theme_mode.error_color(),
            ComponentState::Warning => theme_mode.warning_color(),
            ComponentState::Info => theme_mode.info_color(),
        }
    }

    /// Get border styles for different elevations
    #[must_use]
    pub fn border_none() -> Border {
        Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        }
    }

    /// Get subtle border style for low emphasis
    ///
    /// # Returns
    /// Border with subtle color and width
    #[must_use]
    pub fn border_subtle(tokens: &MaterialTokens, theme_mode: ThemeMode) -> Border {
        Border {
            color: theme_mode.border_color(),
            width: 1.0,
            radius: tokens.shapes.corner_small.radius,
        }
    }

    /// Get default border style for standard emphasis
    ///
    /// # Returns
    /// Border with default color and width
    #[must_use]
    pub fn border_default(tokens: &MaterialTokens, theme_mode: ThemeMode) -> Border {
        Border {
            color: theme_mode.border_color(),
            width: 1.0,
            radius: tokens.shapes.corner_medium.radius,
        }
    }

    /// Get emphasis border style for high emphasis
    ///
    /// # Returns
    /// Border with emphasis color and width
    #[must_use]
    pub fn border_emphasis(tokens: &MaterialTokens, theme_mode: ThemeMode) -> Border {
        Border {
            color: theme_mode.primary_color(),
            width: 2.0,
            radius: tokens.shapes.corner_medium.radius,
        }
    }

    /// Get standard sizes using design tokens
    #[must_use]
    pub const fn width_full() -> Length {
        Length::Fill
    }

    /// Get automatic height for flexible layouts
    ///
    /// # Returns
    /// Length set to auto
    #[must_use]
    pub const fn height_auto() -> Length {
        Length::Shrink
    }

    /// Get full height for flexible layouts
    ///
    /// # Returns
    /// Length set to fill
    #[must_use]
    pub const fn height_full() -> Length {
        Length::Fill
    }

    /// Get standard input height
    ///
    /// # Returns
    /// Length for input widgets
    #[must_use]
    pub const fn input_height() -> Length {
        Length::Fixed(32.0)
    }

    /// Helper for creating consistent backgrounds
    #[must_use]
    pub const fn background_transparent() -> Option<Background> {
        Some(Background::Color(Color::TRANSPARENT))
    }

    /// Get background surface color for containers
    ///
    /// # Returns
    /// Optional background color for surface
    #[must_use]
    pub fn background_surface(theme_mode: ThemeMode) -> Option<Background> {
        Some(Background::Color(theme_mode.surface_color()))
    }

    /// Get background primary color for containers
    ///
    /// # Returns
    /// Optional background color for primary
    #[must_use]
    pub fn background_primary(theme_mode: ThemeMode) -> Option<Background> {
        Some(Background::Color(theme_mode.primary_color()))
    }

    /// Helper for common layout patterns
    #[must_use]
    pub const fn flex_row_centered() -> (
        Length,
        Length,
        iced::alignment::Horizontal,
        iced::alignment::Vertical,
    ) {
        (
            Length::Fill,
            Length::Shrink,
            iced::alignment::Horizontal::Center,
            iced::alignment::Vertical::Center,
        )
    }

    /// Get flex column start alignment tuple
    ///
    /// # Returns
    /// Tuple for flex column start alignment
    #[must_use]
    pub const fn flex_column_start() -> (
        Length,
        Length,
        iced::alignment::Horizontal,
        iced::alignment::Vertical,
    ) {
        (
            Length::Fill,
            Length::Fill,
            iced::alignment::Horizontal::Left,
            iced::alignment::Vertical::Top,
        )
    }
}

/// Component state enumeration for semantic styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentState {
    /// Default state (normal)
    Default,
    /// Hovered state (mouse over)
    Hover,
    /// Active state (pressed or selected)
    Active,
    /// Disabled state (not interactive)
    Disabled,
    /// Success state (positive feedback)
    Success,
    /// Error state (negative feedback)
    Error,
    /// Warning state (cautionary feedback)
    Warning,
    /// Info state (informational feedback)
    Info,
}

/// Typography utilities using design tokens
pub struct TypographyUtils;

impl TypographyUtils {
    /// Returns the font size for caption text using design tokens.
    #[must_use]
    pub const fn caption_size(tokens: &MaterialTokens) -> u16 {
        tokens.typography().label_small.size() as u16
    }
    /// Returns the font size for body text using design tokens.
    #[must_use]
    pub const fn body_size(tokens: &MaterialTokens) -> u16 {
        tokens.typography().body_medium.size() as u16
    }
    /// Returns the font size for large body text using design tokens.
    #[must_use]
    pub const fn body_large_size(tokens: &MaterialTokens) -> u16 {
        tokens.typography().body_large.size() as u16
    }
    /// Returns the font size for heading 3 text using design tokens.
    #[must_use]
    pub const fn heading_3_size(tokens: &MaterialTokens) -> u16 {
        tokens.typography().headline_small.size() as u16
    }
    /// Returns the font size for heading 2 text using design tokens.
    #[must_use]
    pub const fn heading_2_size(tokens: &MaterialTokens) -> u16 {
        tokens.typography().headline_medium.size() as u16
    }
    /// Returns the font size for heading 1 text using design tokens.
    #[must_use]
    pub const fn heading_1_size(tokens: &MaterialTokens) -> u16 {
        tokens.typography().headline_large.size() as u16
    }
    /// Returns the font size for display text using design tokens.
    #[must_use]
    pub const fn display_size(tokens: &MaterialTokens) -> u16 {
        tokens.typography().display_large.size() as u16
    }
}
