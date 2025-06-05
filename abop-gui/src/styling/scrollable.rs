//! Scrollable styling components with design token integration
//!
//! This module provides professional scrollbar styling that uses design tokens
//! for consistent appearance and behavior.

use crate::theme::ThemeMode;
use iced::{Background, Border, Color, widget::scrollable};

/// Scrollable style types
#[derive(Clone, Copy)]
pub enum ScrollableStyleType {
    /// Default scrollable styling
    Default,
    /// Thin scrollbar for compact layouts
    Thin,
    /// Hidden scrollbar (auto-hide)
    Hidden,
    /// Custom styled scrollbar
    Custom,
}

/// Professional scrollable styling definitions
pub struct ScrollableStyles;

impl ScrollableStyles {
    /// Get scrollable style based on type and theme
    #[must_use]
    pub fn get_style(
        style_type: ScrollableStyleType,
        theme_mode: ThemeMode,
        tokens: &crate::styling::material::MaterialTokens,
    ) -> scrollable::Style {
        match style_type {
            ScrollableStyleType::Default => scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: scrollable::Rail {
                    background: Some(Background::Color(theme_mode.surface_variant_color())),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: tokens.shapes.corner_small.radius,
                    },
                    scroller: scrollable::Scroller {
                        color: theme_mode.text_secondary_color(),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: tokens.shapes.corner_small.radius,
                        },
                    },
                },
                horizontal_rail: scrollable::Rail {
                    background: Some(Background::Color(theme_mode.surface_variant_color())),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: tokens.shapes.corner_small.radius,
                    },
                    scroller: scrollable::Scroller {
                        color: theme_mode.text_secondary_color(),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: tokens.shapes.corner_small.radius,
                        },
                    },
                },
                gap: None,
            },
            ScrollableStyleType::Thin => scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: scrollable::Rail {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border: Border::default(),
                    scroller: scrollable::Scroller {
                        color: theme_mode.text_secondary_color(),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: tokens.shapes.corner_full.radius,
                        },
                    },
                },
                horizontal_rail: scrollable::Rail {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border: Border::default(),
                    scroller: scrollable::Scroller {
                        color: theme_mode.text_secondary_color(),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: tokens.shapes.corner_full.radius,
                        },
                    },
                },
                gap: None,
            },
            ScrollableStyleType::Hidden => scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: scrollable::Rail {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border: Border::default(),
                    scroller: scrollable::Scroller {
                        color: Color::TRANSPARENT,
                        border: Border::default(),
                    },
                },
                horizontal_rail: scrollable::Rail {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border: Border::default(),
                    scroller: scrollable::Scroller {
                        color: Color::TRANSPARENT,
                        border: Border::default(),
                    },
                },
                gap: None,
            },
            ScrollableStyleType::Custom => scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: scrollable::Rail {
                    background: Some(Background::Color(theme_mode.surface_color())),
                    border: Border {
                        color: theme_mode.border_color(),
                        width: 1.0,
                        radius: tokens.shapes.corner_medium.radius,
                    },
                    scroller: scrollable::Scroller {
                        color: theme_mode.primary_color(),
                        border: Border {
                            color: theme_mode.primary_color(),
                            width: 1.0,
                            radius: tokens.shapes.corner_medium.radius,
                        },
                    },
                },
                horizontal_rail: scrollable::Rail {
                    background: Some(Background::Color(theme_mode.surface_color())),
                    border: Border {
                        color: theme_mode.border_color(),
                        width: 1.0,
                        radius: tokens.shapes.corner_medium.radius,
                    },
                    scroller: scrollable::Scroller {
                        color: theme_mode.primary_color(),
                        border: Border {
                            color: theme_mode.primary_color(),
                            width: 1.0,
                            radius: tokens.shapes.corner_medium.radius,
                        },
                    },
                },
                gap: None,
            },
        }
    }

    // Convenience methods for common scrollable styles
    /// Creates the default scrollable style for standard scrollbars
    ///
    /// # Returns
    /// A style for regular scrollable widgets
    #[must_use]
    pub fn default(
        theme_mode: ThemeMode,
        tokens: &crate::styling::material::MaterialTokens,
    ) -> scrollable::Style {
        Self::get_style(ScrollableStyleType::Default, theme_mode, tokens)
    }

    /// Creates a thin scrollable style for compact scrollbars
    ///
    /// # Returns
    /// A style for thin scrollable widgets
    #[must_use]
    pub fn thin(
        theme_mode: ThemeMode,
        tokens: &crate::styling::material::MaterialTokens,
    ) -> scrollable::Style {
        Self::get_style(ScrollableStyleType::Thin, theme_mode, tokens)
    }

    /// Creates a hidden scrollable style (no visible scrollbar)
    ///
    /// # Returns
    /// A style for scrollable widgets with hidden scrollbars
    #[must_use]
    pub fn hidden(tokens: &crate::styling::material::MaterialTokens) -> scrollable::Style {
        Self::get_style(ScrollableStyleType::Hidden, ThemeMode::Dark, tokens)
    }

    /// Creates a custom scrollable style for advanced use cases
    ///
    /// # Returns
    /// A style for custom scrollable widgets
    #[must_use]
    pub fn custom(
        theme_mode: ThemeMode,
        tokens: &crate::styling::material::MaterialTokens,
    ) -> scrollable::Style {
        Self::get_style(ScrollableStyleType::Custom, theme_mode, tokens)
    }
}
