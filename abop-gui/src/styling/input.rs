//! Input styling components with design token integration
//!
//! This module provides professional input styling that uses design tokens
//! for consistent spacing, sizing, and visual hierarchy.

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::MaterialTokens;
use crate::styling::traits::StyleVariant;
use crate::theme::ThemeMode;
use iced::{Background, Border, Color, border::Radius, widget::text_input};

/// Input style types
#[derive(Clone, Copy)]
pub enum InputStyleType {
    /// Standard text input
    Default,
    /// Input with error state
    Error,
    /// Input with success state
    Success,
    /// Input with warning state
    Warning,
    /// Search input styling
    Search,
    /// Password input styling
    Password,
    /// Large input for prominent fields
    Large,
    /// Small input for compact layouts
    Small,
}

/// Professional input styling definitions
pub struct InputStyles;

impl InputStyles {
    /// Get input style based on type and theme
    #[must_use]
    pub fn get_style(style_type: InputStyleType, theme_mode: ThemeMode) -> text_input::Style {
        let material_tokens = MaterialTokens::new();
        match style_type {
            InputStyleType::Default | InputStyleType::Password => text_input::Style {
                background: Background::Color(theme_mode.background_color()),
                border: Border {
                    color: theme_mode.border_color(),
                    width: 1.0,
                    radius: material_tokens.shapes().text_field().to_radius(),
                },
                icon: theme_mode.text_secondary_color(),
                placeholder: theme_mode.text_secondary_color(),
                value: theme_mode.text_primary_color(),
                selection: theme_mode.primary_light_color(),
            },
            InputStyleType::Error => text_input::Style {
                background: Background::Color(theme_mode.background_color()),
                border: Border {
                    color: theme_mode.error_color(),
                    width: 2.0,
                    radius: material_tokens.shapes().text_field().to_radius(),
                },
                icon: theme_mode.error_color(),
                placeholder: theme_mode.text_secondary_color(),
                value: theme_mode.text_primary_color(),
                selection: Color::from_rgba(1.0, 0.0, 0.0, 0.2),
            },
            InputStyleType::Success => text_input::Style {
                background: Background::Color(theme_mode.background_color()),
                border: Border {
                    color: theme_mode.success_color(),
                    width: 2.0,
                    radius: material_tokens.shapes().text_field().to_radius(),
                },
                icon: theme_mode.success_color(),
                placeholder: theme_mode.text_secondary_color(),
                value: theme_mode.text_primary_color(),
                selection: Color::from_rgba(0.0, 1.0, 0.0, 0.2),
            },
            InputStyleType::Warning => text_input::Style {
                background: Background::Color(theme_mode.background_color()),
                border: Border {
                    color: theme_mode.warning_color(),
                    width: 2.0,
                    radius: material_tokens.shapes().text_field().to_radius(),
                },
                icon: theme_mode.warning_color(),
                placeholder: theme_mode.text_secondary_color(),
                value: theme_mode.text_primary_color(),
                selection: Color::from_rgba(1.0, 1.0, 0.0, 0.2),
            },
            InputStyleType::Search => text_input::Style {
                background: Background::Color(theme_mode.surface_variant_color()),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: material_tokens.shapes().text_field().to_radius(),
                },
                icon: theme_mode.text_secondary_color(),
                placeholder: theme_mode.text_secondary_color(),
                value: theme_mode.text_primary_color(),
                selection: theme_mode.primary_light_color(),
            },
            InputStyleType::Large => text_input::Style {
                background: Background::Color(theme_mode.background_color()),
                border: Border {
                    color: theme_mode.border_color(),
                    width: 2.0,
                    radius: {
                        let base_radius = material_tokens.shapes().text_field().to_radius();
                        Radius::from(base_radius.top_left * 1.5)
                    },
                },
                icon: theme_mode.text_secondary_color(),
                placeholder: theme_mode.text_secondary_color(),
                value: theme_mode.text_primary_color(),
                selection: theme_mode.primary_light_color(),
            },
            InputStyleType::Small => text_input::Style {
                background: Background::Color(theme_mode.background_color()),
                border: Border {
                    color: theme_mode.border_color(),
                    width: 1.0,
                    radius: {
                        let base_radius = material_tokens.shapes().text_field().to_radius();
                        Radius::from(base_radius.top_left * 0.75)
                    },
                },
                icon: theme_mode.text_secondary_color(),
                placeholder: theme_mode.text_secondary_color(),
                value: theme_mode.text_primary_color(),
                selection: theme_mode.primary_light_color(),
            },
        }
    }

    /// Creates a default input style for standard text fields
    ///
    /// # Returns
    /// A style for regular text input fields
    #[must_use]
    pub fn default(theme_mode: ThemeMode) -> text_input::Style {
        Self::get_style(InputStyleType::Default, theme_mode)
    }

    /// Creates an error input style for invalid or error states
    ///
    /// # Returns
    /// A style for text inputs in error state
    #[must_use]
    pub fn error(theme_mode: ThemeMode) -> text_input::Style {
        Self::get_style(InputStyleType::Error, theme_mode)
    }

    /// Creates a success input style for valid or confirmed states
    ///
    /// # Returns
    /// A style for text inputs in success state
    #[must_use]
    pub fn success(theme_mode: ThemeMode) -> text_input::Style {
        Self::get_style(InputStyleType::Success, theme_mode)
    }

    /// Creates a warning input style for cautionary states
    ///
    /// # Returns
    /// A style for text inputs in warning state
    #[must_use]
    pub fn warning(theme_mode: ThemeMode) -> text_input::Style {
        Self::get_style(InputStyleType::Warning, theme_mode)
    }

    /// Creates a search input style for search fields
    ///
    /// # Returns
    /// A style for search input fields
    #[must_use]
    pub fn search(theme_mode: ThemeMode) -> text_input::Style {
        Self::get_style(InputStyleType::Search, theme_mode)
    }

    /// Creates a password input style for obscured fields
    ///
    /// # Returns
    /// A style for password input fields
    #[must_use]
    pub fn password(theme_mode: ThemeMode) -> text_input::Style {
        Self::get_style(InputStyleType::Password, theme_mode)
    }

    /// Creates a large input style for prominent fields
    ///
    /// # Returns
    /// A style for large input fields
    #[must_use]
    pub fn large(theme_mode: ThemeMode) -> text_input::Style {
        Self::get_style(InputStyleType::Large, theme_mode)
    }

    /// Creates a small input style for compact layouts
    ///
    /// # Returns
    /// A style for small input fields
    #[must_use]
    pub fn small(theme_mode: ThemeMode) -> text_input::Style {
        Self::get_style(InputStyleType::Small, theme_mode)
    }

    /// Creates an input style using semantic colors
    ///
    /// This method uses the new semantic color system for consistent theming.
    ///
    /// # Arguments
    /// * `variant` - The style variant (Success, Warning, Error, etc.)
    /// * `theme_mode` - The current theme mode
    ///
    /// # Returns
    /// A text input style using semantic colors
    #[must_use]
    pub fn semantic_style(variant: StyleVariant, theme_mode: ThemeMode) -> text_input::Style {
        let semantic_colors = theme_mode.semantic_colors();
        let material_tokens = MaterialTokens::new();

        let (border_color, border_width, selection_color) = match variant {
            StyleVariant::Primary => (semantic_colors.primary, 1.0, semantic_colors.primary),
            StyleVariant::Secondary => (semantic_colors.secondary, 1.0, semantic_colors.secondary),
            StyleVariant::Success => (semantic_colors.success, 2.0, semantic_colors.success),
            StyleVariant::Warning => (semantic_colors.warning, 2.0, semantic_colors.warning),
            StyleVariant::Error => (semantic_colors.error, 2.0, semantic_colors.error),
            StyleVariant::Info => (semantic_colors.info, 1.0, semantic_colors.info),
            StyleVariant::Outline => (semantic_colors.primary, 1.0, semantic_colors.primary),
            StyleVariant::Text => (Color::TRANSPARENT, 0.0, semantic_colors.primary),
            StyleVariant::Active => (semantic_colors.primary, 2.0, semantic_colors.primary),
            StyleVariant::Disabled => {
                let disabled_border = ColorUtils::with_alpha(semantic_colors.on_surface, 0.3);
                (disabled_border, 1.0, disabled_border)
            }
        };

        text_input::Style {
            background: Background::Color(semantic_colors.surface),
            border: Border {
                color: border_color,
                width: border_width,
                radius: material_tokens.shapes().text_field().to_radius(),
            },
            icon: border_color,
            placeholder: ColorUtils::with_alpha(semantic_colors.on_surface, 0.6),
            value: semantic_colors.on_surface,
            selection: ColorUtils::with_alpha(selection_color, 0.2),
        }
    }
}
