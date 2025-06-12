//! Professional theme system for ABOP GUI
//!
//! This module provides a clean, professional theming system with two carefully
//! crafted themes: a dark sunset theme and a matching light theme.

use crate::styling::material::{MaterialColors, MaterialPalette, MaterialTokens};
use iced::{Color, Theme as IcedTheme, theme::Palette};

// ================================================================================================
// THEME TYPES
// ================================================================================================

/// Theme modes available in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    /// Professional dark sunset theme with warm oranges and cool blues
    Dark,
    /// Professional light theme matching the dark sunset palette
    Light,
    /// System theme that follows OS preference
    System,
    /// Material Design dark theme
    MaterialDark,
    /// Material Design light theme
    MaterialLight,
    /// Dynamic Material theme based on seed color
    MaterialDynamic,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Dark
    }
}

impl ThemeMode {
    /// Get the Iced theme for this mode
    #[must_use]
    pub fn theme(&self) -> IcedTheme {
        match self {
            Self::Dark => dark_sunset_theme(),
            Self::Light => light_sunset_theme(),
            Self::System => {
                // For now, default to dark. Could be enhanced to detect OS theme
                dark_sunset_theme()
            }
            Self::MaterialDark => {
                let colors = MaterialColors::dark(&MaterialPalette::default());
                material_theme_from_colors(&colors)
            }
            Self::MaterialLight => {
                let colors = MaterialColors::light(&MaterialPalette::default());
                material_theme_from_colors(&colors)
            }
            Self::MaterialDynamic => {
                // Use a default seed color for dynamic theme
                let seed_color = Color::from_rgb(0.5, 0.2, 0.8);
                let colors = MaterialColors::from_seed(seed_color, true);
                material_theme_from_colors(&colors)
            }
        }
    }
    /// Get the display name for this theme
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Dark => "Dark Sunset",
            Self::Light => "Light Sunset",
            Self::System => "System",
            Self::MaterialDark => "Material Dark",
            Self::MaterialLight => "Material Light",
            Self::MaterialDynamic => "Material Dynamic",
        }
    }
    /// Check if this is a dark theme
    #[must_use]
    pub const fn is_dark(&self) -> bool {
        matches!(
            self,
            Self::Dark | Self::MaterialDark | Self::MaterialDynamic
        )
    }
    /// Centralized helper to resolve System theme to appropriate palette values
    /// Currently defaults to Dark palette for System mode as temporary behavior.
    /// TODO: Enhance to detect actual OS theme preference
    pub fn resolve_palette<T>(&self, dark_value: T, light_value: T) -> T {
        match self {
            Self::Dark | Self::System | Self::MaterialDark | Self::MaterialDynamic => dark_value,
            Self::Light | Self::MaterialLight => light_value,
        }
    }

    /// Get background color for the current theme mode
    #[must_use]
    pub fn background_color(&self) -> Color {
        self.resolve_palette(
            DarkSunsetPalette::BACKGROUND,
            LightSunsetPalette::BACKGROUND,
        )
    }

    /// Get surface color for the current theme mode
    #[must_use]
    pub fn surface_color(&self) -> Color {
        self.resolve_palette(DarkSunsetPalette::SURFACE, LightSunsetPalette::SURFACE)
    }

    /// Get surface variant color for the current theme mode
    #[must_use]
    pub fn surface_variant_color(&self) -> Color {
        self.resolve_palette(
            DarkSunsetPalette::SURFACE_VARIANT,
            LightSunsetPalette::SURFACE_VARIANT,
        )
    }

    /// Get primary color for the current theme mode
    #[must_use]
    pub fn primary_color(&self) -> Color {
        self.resolve_palette(DarkSunsetPalette::PRIMARY, LightSunsetPalette::PRIMARY)
    }

    /// Get primary light color for the current theme mode
    #[must_use]
    pub fn primary_light_color(&self) -> Color {
        self.resolve_palette(
            DarkSunsetPalette::PRIMARY_LIGHT,
            LightSunsetPalette::PRIMARY_LIGHT,
        )
    }

    /// Get secondary color for the current theme mode
    #[must_use]
    pub fn secondary_color(&self) -> Color {
        self.resolve_palette(DarkSunsetPalette::SECONDARY, LightSunsetPalette::SECONDARY)
    }

    /// Get primary text color for the current theme mode
    #[must_use]
    pub fn text_primary_color(&self) -> Color {
        self.resolve_palette(
            DarkSunsetPalette::TEXT_PRIMARY,
            LightSunsetPalette::TEXT_PRIMARY,
        )
    }

    /// Get secondary text color for the current theme mode
    #[must_use]
    pub fn text_secondary_color(&self) -> Color {
        self.resolve_palette(
            DarkSunsetPalette::TEXT_SECONDARY,
            LightSunsetPalette::TEXT_SECONDARY,
        )
    }

    /// Get border color for the current theme mode
    #[must_use]
    pub fn border_color(&self) -> Color {
        self.resolve_palette(DarkSunsetPalette::BORDER, LightSunsetPalette::BORDER)
    }

    /// Get outline color for the current theme mode
    #[must_use]
    pub fn outline_color(&self) -> Color {
        self.resolve_palette(DarkSunsetPalette::OUTLINE, LightSunsetPalette::OUTLINE)
    }

    /// Get error color for the current theme mode
    #[must_use]
    pub fn error_color(&self) -> Color {
        self.resolve_palette(DarkSunsetPalette::ERROR, LightSunsetPalette::ERROR)
    }

    /// Get success color for the current theme mode
    #[must_use]
    pub fn success_color(&self) -> Color {
        self.resolve_palette(DarkSunsetPalette::SUCCESS, LightSunsetPalette::SUCCESS)
    }

    /// Get info color for the current theme mode
    #[must_use]
    pub fn info_color(&self) -> Color {
        self.resolve_palette(DarkSunsetPalette::INFO, LightSunsetPalette::INFO)
    }

    /// Get orange accent color for rare/special use
    #[must_use]
    pub fn orange_accent_color(&self) -> Color {
        self.resolve_palette(
            DarkSunsetPalette::ORANGE_ACCENT,
            LightSunsetPalette::ORANGE_ACCENT,
        )
    }

    /// Get warning color for the current theme mode
    #[must_use]
    pub fn warning_color(&self) -> Color {
        self.resolve_palette(DarkSunsetPalette::WARNING, LightSunsetPalette::WARNING)
    }

    /// Get disabled text color for the current theme mode
    #[must_use]
    pub fn text_disabled_color(&self) -> Color {
        self.resolve_palette(
            DarkSunsetPalette::TEXT_DISABLED,
            LightSunsetPalette::TEXT_DISABLED,
        )
    }

    /// Get semantic colors for the current theme mode
    #[must_use]
    pub const fn semantic_colors(&self) -> crate::styling::material::SemanticColors {
        if self.is_dark() {
            crate::styling::material::SemanticColors {
                primary: iced::Color::from_rgb(0.2, 0.6, 0.8),   // Blue
                secondary: iced::Color::from_rgb(0.5, 0.5, 0.5), // Gray
                success: iced::Color::from_rgb(0.2, 0.7, 0.3),   // Green
                warning: iced::Color::from_rgb(0.9, 0.6, 0.1),   // Orange
                error: iced::Color::from_rgb(0.8, 0.2, 0.2),     // Red
                info: iced::Color::from_rgb(0.3, 0.7, 0.9),      // Light Blue
                surface: iced::Color::from_rgb(0.15, 0.15, 0.15), // Dark Gray
                on_surface: iced::Color::from_rgb(0.9, 0.9, 0.9), // Light Gray
            }
        } else {
            crate::styling::material::SemanticColors {
                primary: iced::Color::from_rgb(0.1, 0.4, 0.7), // Darker Blue
                secondary: iced::Color::from_rgb(0.4, 0.4, 0.4), // Dark Gray
                success: iced::Color::from_rgb(0.1, 0.6, 0.2), // Dark Green
                warning: iced::Color::from_rgb(0.8, 0.5, 0.0), // Dark Orange
                error: iced::Color::from_rgb(0.7, 0.1, 0.1),   // Dark Red
                info: iced::Color::from_rgb(0.2, 0.5, 0.8),    // Dark Blue
                surface: iced::Color::from_rgb(0.98, 0.98, 0.98), // Off White
                on_surface: iced::Color::from_rgb(0.1, 0.1, 0.1), // Dark Gray
            }
        }
    }

    /// Get Material Design tokens for this theme mode if it's a Material theme
    /// Returns None for non-Material themes
    #[must_use]
    pub fn material_tokens_if_material(&self) -> Option<MaterialTokens> {
        match self {
            Self::MaterialDark => Some(MaterialTokens::dark()),
            Self::MaterialLight => Some(MaterialTokens::light()),
            Self::MaterialDynamic => {
                // Use purple seed color for dynamic Material Design theme
                let seed_color = Color::from_rgb(0.4, 0.2, 0.8);
                Some(MaterialTokens::from_seed_color(seed_color, true))
            }
            _ => None,
        }
    }

    /// Check if this is a Material Design theme
    #[must_use]
    pub const fn is_material(&self) -> bool {
        matches!(
            self,
            Self::MaterialDark | Self::MaterialLight | Self::MaterialDynamic
        )
    }

    /// Get Material tokens for the current theme
    ///
    /// # Returns
    /// Material design tokens for the current theme
    #[must_use]
    pub fn material_tokens(&self) -> MaterialTokens {
        match self {
            Self::Light | Self::MaterialLight => MaterialTokens::light(),
            Self::Dark | Self::System | Self::MaterialDark | Self::MaterialDynamic => {
                MaterialTokens::dark()
            } // TODO: Implement dynamic
        }
    }
}

// ================================================================================================
// COLOR PALETTES
// ================================================================================================

/// Professional dark sunset theme colors
pub struct DarkSunsetPalette;

impl DarkSunsetPalette {
    /// Background color for main application surfaces (deep twilight blue)
    pub const BACKGROUND: Color = Color::from_rgb(0.09, 0.10, 0.15);
    /// Elevated surface color for panels and cards
    pub const SURFACE: Color = Color::from_rgb(0.13, 0.14, 0.20);
    /// Card background color for surface variants
    pub const SURFACE_VARIANT: Color = Color::from_rgb(0.17, 0.18, 0.25);
    /// Primary accent color (deep purple)
    pub const PRIMARY: Color = Color::from_rgb(0.35, 0.25, 0.55);
    /// Lighter primary accent for highlights
    pub const PRIMARY_LIGHT: Color = Color::from_rgb(0.45, 0.35, 0.65);
    /// Secondary accent color (muted blue)
    pub const SECONDARY: Color = Color::from_rgb(0.20, 0.45, 0.75);
    /// Tertiary color for special elements (very dark purple)
    pub const TERTIARY: Color = Color::from_rgb(0.12, 0.08, 0.20);
    /// Tertiary variant for backgrounds (even darker purple)
    pub const TERTIARY_VARIANT: Color = Color::from_rgb(0.08, 0.05, 0.15);
    /// Orange accent for rare/special use
    pub const ORANGE_ACCENT: Color = Color::from_rgb(0.75, 0.35, 0.12);
    /// Primary text color (warm white)
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.96, 0.94, 0.97);
    /// Secondary text color (muted text)
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.75, 0.72, 0.78);
    /// Disabled text color
    pub const TEXT_DISABLED: Color = Color::from_rgb(0.55, 0.52, 0.58);
    /// Error color for error states (muted red)
    pub const ERROR: Color = Color::from_rgb(0.85, 0.25, 0.25);
    /// Success color for success states (muted green)
    pub const SUCCESS: Color = Color::from_rgb(0.30, 0.70, 0.40);
    /// Warning color for warning states (muted orange-yellow)
    pub const WARNING: Color = Color::from_rgb(0.80, 0.55, 0.15);
    /// Info color for informational states (muted blue)
    pub const INFO: Color = Color::from_rgb(0.20, 0.45, 0.75);
    /// Border color for subtle borders
    pub const BORDER: Color = Color::from_rgb(0.25, 0.26, 0.33);
    /// Outline color for outlines and focus rings
    pub const OUTLINE: Color = Color::from_rgb(0.20, 0.21, 0.28);
}

/// Professional light sunset theme colors
pub struct LightSunsetPalette;

impl LightSunsetPalette {
    /// Background color for main application surfaces (warm white)
    pub const BACKGROUND: Color = Color::from_rgb(0.98, 0.97, 0.95);
    /// Elevated surface color for panels and cards
    pub const SURFACE: Color = Color::from_rgb(0.95, 0.94, 0.92);
    /// Card background color for surface variants
    pub const SURFACE_VARIANT: Color = Color::from_rgb(0.92, 0.91, 0.89);
    /// Primary accent color (deep purple)
    pub const PRIMARY: Color = Color::from_rgb(0.40, 0.30, 0.60);
    /// Lighter primary accent for highlights
    pub const PRIMARY_LIGHT: Color = Color::from_rgb(0.50, 0.40, 0.70);
    /// Secondary accent color (muted blue)
    pub const SECONDARY: Color = Color::from_rgb(0.15, 0.40, 0.70);
    /// Tertiary color for special elements (very dark purple)
    pub const TERTIARY: Color = Color::from_rgb(0.08, 0.05, 0.15);
    /// Tertiary variant for backgrounds (even darker purple)
    pub const TERTIARY_VARIANT: Color = Color::from_rgb(0.05, 0.03, 0.12);
    /// Orange accent for rare/special use
    pub const ORANGE_ACCENT: Color = Color::from_rgb(0.70, 0.30, 0.08);
    /// Primary text color (dark text)
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.15, 0.15, 0.18);
    /// Secondary text color (muted text)
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.45, 0.45, 0.50);
    /// Disabled text color
    pub const TEXT_DISABLED: Color = Color::from_rgb(0.65, 0.65, 0.70);
    /// Error color for error states (muted red)
    pub const ERROR: Color = Color::from_rgb(0.75, 0.18, 0.24);
    /// Success color for success states (muted green)
    pub const SUCCESS: Color = Color::from_rgb(0.20, 0.60, 0.26);
    /// Warning color for warning states (muted orange-yellow)
    pub const WARNING: Color = Color::from_rgb(0.75, 0.45, 0.02);
    /// Info color for informational states (muted blue)
    pub const INFO: Color = Color::from_rgb(0.15, 0.40, 0.70);
    /// Border color for subtle borders
    pub const BORDER: Color = Color::from_rgb(0.80, 0.79, 0.77);
    /// Outline color for outlines and focus rings
    pub const OUTLINE: Color = Color::from_rgb(0.85, 0.84, 0.82);
}

// ================================================================================================
// THEME CONSTRUCTORS
// ================================================================================================

/// Create the dark sunset theme
#[must_use]
pub fn dark_sunset_theme() -> IcedTheme {
    let palette = Palette {
        background: DarkSunsetPalette::BACKGROUND,
        text: DarkSunsetPalette::TEXT_PRIMARY,
        primary: DarkSunsetPalette::PRIMARY,
        success: DarkSunsetPalette::SUCCESS,
        danger: DarkSunsetPalette::ERROR,
    };

    IcedTheme::custom("Dark Sunset".to_string(), palette)
}

/// Create the light sunset theme
#[must_use]
pub fn light_sunset_theme() -> IcedTheme {
    let palette = Palette {
        background: LightSunsetPalette::BACKGROUND,
        text: LightSunsetPalette::TEXT_PRIMARY,
        primary: LightSunsetPalette::PRIMARY,
        success: LightSunsetPalette::SUCCESS,
        danger: LightSunsetPalette::ERROR,
    };

    IcedTheme::custom("Light Sunset".to_string(), palette)
}

/// Create a Material Design theme from `MaterialColors`
fn material_theme_from_colors(colors: &MaterialColors) -> IcedTheme {
    let palette = Palette {
        background: colors.background,
        text: colors.on_background,
        primary: colors.primary.base,
        success: colors.tertiary.base, // Using tertiary as success color
        danger: colors.error.base,
    };
    IcedTheme::custom("Material".to_string(), palette)
}

/// Create Material Design dark theme
#[allow(dead_code)]
fn material_dark_theme() -> IcedTheme {
    let colors = MaterialColors::dark(&MaterialPalette::default());
    material_theme_from_colors(&colors)
}

/// Create Material Design light theme
#[allow(dead_code)]
fn material_light_theme() -> IcedTheme {
    let colors = MaterialColors::light(&MaterialPalette::default());
    material_theme_from_colors(&colors)
}

/// Create Material Design dynamic theme using seed color generation
#[allow(dead_code)]
fn material_dynamic_theme() -> IcedTheme {
    // Use a default seed color for demonstration
    let seed_color = Color::from_rgb(0.5, 0.2, 0.8);
    let colors = MaterialColors::from_seed(seed_color, true);
    material_theme_from_colors(&colors)
}
