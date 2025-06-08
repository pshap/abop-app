//! Base container styles for fundamental UI elements
//!
//! Provides core container styling functionality including basic containers,
//! cards, panels, and utility containers. These form the foundation of the
//! container styling system with common patterns and behaviors.

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::helpers::ElevationHelpers;
use crate::styling::material::MaterialTokens;
use crate::styling::traits::StyleVariant;
use crate::theme::ThemeMode;
use iced::border::Radius;
use iced::widget::container;
use iced::{Background, Border, Color, Shadow};

/// Container style type for better type safety
pub type ContainerStyle = fn(&iced::Theme) -> container::Style;

/// Base container style types for fundamental UI elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseContainerType {
    /// Card container with subtle elevation and rounded corners
    /// Used for content grouping and visual separation
    Card,
    /// Panel container with flat styling and subtle borders
    /// Used for sections and content organization
    Panel,
    /// Primary color container for emphasized content
    /// Used to highlight important sections
    Primary,
    /// Secondary color container for supporting content
    /// Used for less prominent but important sections
    Secondary,
    /// Transparent container with no background
    /// Used for overlay elements and minimal styling needs
    Transparent,
    /// Outlined container with border emphasis
    /// Used when clear boundaries are needed without background
    Outlined,
    /// Elevated container with enhanced shadow
    /// Used for floating elements and emphasis
    Elevated,
    /// Flat container with minimal visual weight
    /// Used for subtle content grouping
    Flat,
    /// Rounded container with emphasized corner radius
    /// Used for modern, friendly interface elements
    Rounded,
    /// Sharp container with no corner radius
    /// Used for technical or grid-based layouts
    Sharp,
    /// Separator container for visual division
    /// Used for dividing content sections
    Separator,
    /// Progress bar container for loading indication
    /// Used for progress displays and loading states
    ProgressBar,
}

/// Base container styling utilities
pub struct BaseContainerStyles;

impl BaseContainerStyles {
    /// Get base container style based on type and theme
    #[must_use]
    pub fn get_style(style_type: BaseContainerType, theme_mode: ThemeMode) -> container::Style {
        let material_tokens = MaterialTokens::new();
        match style_type {
            BaseContainerType::Card => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_color())),
                border: Border {
                    color: theme_mode.border_color(),
                    width: material_tokens.ui().border_width_thick,
                    radius: material_tokens.shapes.corner_medium.radius,
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            BaseContainerType::Panel => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_color())),
                border: Border {
                    color: theme_mode.border_color(),
                    width: material_tokens.ui().border_width_thick,
                    radius: material_tokens.shapes.corner_small.radius,
                },
                shadow: Shadow::default(),
            },
            BaseContainerType::Primary => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(theme_mode.primary_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: material_tokens.shapes.corner_medium.radius,
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            BaseContainerType::Secondary => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.secondary_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: material_tokens.shapes.corner_medium.radius,
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            BaseContainerType::Transparent => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(Color::TRANSPARENT)),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(0.0),
                },
                shadow: Shadow::default(),
            },
            BaseContainerType::Outlined => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(Color::TRANSPARENT)),
                border: Border {
                    color: theme_mode.border_color(),
                    width: material_tokens.ui().border_width_thick,
                    radius: material_tokens.shapes.corner_medium.radius,
                },
                shadow: Shadow::default(),
            },
            BaseContainerType::Elevated => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: material_tokens.shapes.corner_medium.radius,
                },
                shadow: material_tokens.elevation_shadow(5),
            },
            BaseContainerType::Flat => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(0.0),
                },
                shadow: Shadow::default(),
            },
            BaseContainerType::Rounded => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_color())),
                border: Border {
                    color: theme_mode.border_color(),
                    width: material_tokens.ui().border_width_thick,
                    radius: material_tokens.shapes.corner_large.radius,
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            BaseContainerType::Sharp => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_color())),
                border: Border {
                    color: theme_mode.border_color(),
                    width: material_tokens.ui().border_width_thick,
                    radius: Radius::from(0.0),
                },
                shadow: material_tokens.elevation_shadow(1),
            },
            BaseContainerType::Separator => container::Style {
                text_color: Some(theme_mode.text_secondary_color()),
                background: Some(Background::Color(theme_mode.border_color())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(0.0),
                },
                shadow: Shadow::default(),
            },
            BaseContainerType::ProgressBar => container::Style {
                text_color: Some(theme_mode.text_primary_color()),
                background: Some(Background::Color(theme_mode.surface_variant_color())),
                border: Border {
                    color: theme_mode.border_color(),
                    width: material_tokens.ui().border_width_thick,
                    radius: material_tokens.shapes.corner_large.radius,
                },
                shadow: Shadow::default(),
            },
        }
    }

    /// Creates a card container style with subtle elevation and rounded corners
    ///
    /// Card containers provide visual grouping for related content with
    /// gentle shadows and rounded borders for modern appearance.
    ///
    /// # Returns
    /// A style function that creates card container appearance
    pub fn card(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Card, theme_mode);
        move |_| style
    }

    /// Creates a panel container style with flat styling and subtle borders
    ///
    /// Panel containers organize sections of content with minimal
    /// visual weight while maintaining clear boundaries.
    ///
    /// # Returns
    /// A style function that creates panel container appearance
    pub fn panel(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Panel, theme_mode);
        move |_| style
    }

    /// Creates a primary color container style for emphasized content
    ///
    /// Primary containers highlight important sections using the theme's primary color.
    ///
    /// # Returns
    /// A style function that creates primary container appearance
    pub fn primary(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Primary, theme_mode);
        move |_| style
    }

    /// Creates a secondary color container style for supporting content
    ///
    /// Secondary containers use the theme's secondary color for less prominent but important sections.
    ///
    /// # Returns
    /// A style function that creates secondary container appearance
    pub fn secondary(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Secondary, theme_mode);
        move |_| style
    }

    /// Creates a transparent container style with no background
    ///
    /// Transparent containers are useful for overlays and minimal styling needs.
    ///
    /// # Returns
    /// A style function that creates transparent container appearance
    pub fn transparent(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Transparent, theme_mode);
        move |_| style
    }

    /// Creates an outlined container style with border emphasis
    ///
    /// Outlined containers provide clear boundaries without a background fill.
    ///
    /// # Returns
    /// A style function that creates outlined container appearance
    pub fn outlined(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Outlined, theme_mode);
        move |_| style
    }

    /// Creates an elevated container style with enhanced shadow
    ///
    /// Elevated containers are used for floating elements and emphasis.
    ///
    /// # Returns
    /// A style function that creates elevated container appearance
    pub fn elevated(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Elevated, theme_mode);
        move |_| style
    }

    /// Creates a flat container style with minimal visual weight
    ///
    /// Flat containers are used for subtle content grouping.
    ///
    /// # Returns
    /// A style function that creates flat container appearance
    pub fn flat(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Flat, theme_mode);
        move |_| style
    }

    /// Creates a rounded container style with emphasized corner radius
    ///
    /// Rounded containers provide a modern, friendly interface appearance.
    ///
    /// # Returns
    /// A style function that creates rounded container appearance
    pub fn rounded(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Rounded, theme_mode);
        move |_| style
    }

    /// Creates a sharp container style with no corner radius
    ///
    /// Sharp containers are used for technical or grid-based layouts.
    ///
    /// # Returns
    /// A style function that creates sharp container appearance
    pub fn sharp(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Sharp, theme_mode);
        move |_| style
    }

    /// Creates a separator container style for visual division
    ///
    /// Separator containers are used for dividing content sections.
    ///
    /// # Returns
    /// A style function that creates separator container appearance
    pub fn separator(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::Separator, theme_mode);
        move |_| style
    }

    /// Creates a progress bar container style for loading indication
    ///
    /// Progress bar containers are used for progress displays and loading states.
    ///
    /// # Returns
    /// A style function that creates progress bar container appearance
    pub fn progress_bar(theme_mode: ThemeMode) -> impl Fn(&iced::Theme) -> container::Style {
        let style = Self::get_style(BaseContainerType::ProgressBar, theme_mode);
        move |_| style
    }

    /// Creates a container style using semantic colors
    ///
    /// This method uses the new semantic color system for consistent theming.
    ///
    /// # Arguments
    /// * `variant` - The style variant (Success, Warning, Error, etc.)
    /// * `theme_mode` - The current theme mode
    ///
    /// # Returns
    /// A style function that creates container appearance using semantic colors
    pub fn semantic_style(
        variant: StyleVariant,
        theme_mode: ThemeMode,
    ) -> impl Fn(&iced::Theme) -> container::Style {
        move |_| {
            let semantic_colors = theme_mode.semantic_colors();
            let material_tokens = MaterialTokens::new();

            let (bg_color, text_color, border_color) = match variant {
                StyleVariant::Primary => (
                    ColorUtils::with_alpha(semantic_colors.primary, 0.1),
                    semantic_colors.on_surface,
                    semantic_colors.primary,
                ),
                StyleVariant::Secondary => (
                    ColorUtils::with_alpha(semantic_colors.secondary, 0.1),
                    semantic_colors.on_surface,
                    semantic_colors.secondary,
                ),
                StyleVariant::Success => (
                    ColorUtils::with_alpha(semantic_colors.success, 0.1),
                    semantic_colors.on_surface,
                    semantic_colors.success,
                ),
                StyleVariant::Warning => (
                    ColorUtils::with_alpha(semantic_colors.warning, 0.1),
                    semantic_colors.on_surface,
                    semantic_colors.warning,
                ),
                StyleVariant::Error => (
                    ColorUtils::with_alpha(semantic_colors.error, 0.1),
                    semantic_colors.on_surface,
                    semantic_colors.error,
                ),
                StyleVariant::Info => (
                    ColorUtils::with_alpha(semantic_colors.info, 0.1),
                    semantic_colors.on_surface,
                    semantic_colors.info,
                ),
                StyleVariant::Outline => (
                    Color::TRANSPARENT,
                    semantic_colors.on_surface,
                    semantic_colors.primary,
                ),
                StyleVariant::Text => (
                    Color::TRANSPARENT,
                    semantic_colors.on_surface,
                    Color::TRANSPARENT,
                ),
                StyleVariant::Active => (
                    semantic_colors.surface,
                    semantic_colors.on_surface,
                    semantic_colors.primary,
                ),
                StyleVariant::Disabled => {
                    let disabled_bg = ColorUtils::with_alpha(semantic_colors.surface, 0.5);
                    let disabled_text = ColorUtils::with_alpha(semantic_colors.on_surface, 0.5);
                    let disabled_border = ColorUtils::with_alpha(semantic_colors.on_surface, 0.3);
                    (disabled_bg, disabled_text, disabled_border)
                }
            };

            container::Style {
                text_color: Some(text_color),
                background: Some(Background::Color(bg_color)),
                border: Border {
                    color: border_color,
                    width: if matches!(variant, StyleVariant::Outline) {
                        1.0
                    } else {
                        0.0
                    },
                    radius: material_tokens.shapes.corner_medium.radius,
                },
                shadow: if matches!(variant, StyleVariant::Text) {
                    Shadow::default()
                } else {
                    material_tokens.elevation_shadow(1)
                },
            }
        }
    }
}
