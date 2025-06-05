//! Material Design 3 Color System
//!
//! Implements the complete Material Design 3 color system including:
//! - System color tokens (light/dark schemes)
//! - Reference palette with tonal scales  
//! - Color roles for semantic usage
//! - Dynamic color generation from seed colors
//!
//! This module provides a high-level interface to the core MD3 color system.

use iced::Color;

/// Tone indices for the tonal palette (corresponding to Material Design 3 tone values)
const TONE_INDICES: [u8; 24] = [
    0, 4, 6, 10, 12, 17, 20, 22, 24, 30, 40, 50, 60, 70, 80, 87, 90, 92, 94, 95, 96, 98, 99, 100,
];

/// Helper function to convert hex string to Color
#[allow(dead_code)]
fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = f32::from(u8::from_str_radix(&hex[0..2], 16).unwrap_or(0)) / 255.0;
    let g = f32::from(u8::from_str_radix(&hex[2..4], 16).unwrap_or(0)) / 255.0;
    let b = f32::from(u8::from_str_radix(&hex[4..6], 16).unwrap_or(0)) / 255.0;
    Color::from_rgb(r, g, b)
}

/// Data-driven `TonalPalette`
#[derive(Debug, Clone, PartialEq)]
pub struct TonalPalette {
    /// Array of 24 tonal colors from darkest (tone 0) to lightest (tone 100)
    pub tones: [Color; 24],
}

impl TonalPalette {
    /// Gets the color for a specific tone value (0-100)
    /// Returns the closest available tone from the predefined set
    #[must_use]
    pub fn get_tone(&self, tone: u8) -> Color {
        let idx = TONE_INDICES.iter().position(|&t| t >= tone).unwrap_or(0);
        self.tones[idx]
    }
}

/// Data-driven `MaterialPalette`
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialPalette {
    /// Primary color tonal palette
    pub primary: TonalPalette,
    /// Secondary color tonal palette
    pub secondary: TonalPalette,
    /// Tertiary color tonal palette
    pub tertiary: TonalPalette,
    /// Neutral color tonal palette
    pub neutral: TonalPalette,
    /// Neutral variant color tonal palette
    pub neutral_variant: TonalPalette,
    /// Error color tonal palette
    pub error: TonalPalette,
}

impl MaterialPalette {
    /// Creates a new `MaterialPalette` with default Material Design 3 colors
    #[must_use]
    pub fn new() -> Self {
        // Use Material Design 3 baseline color (#6750A4) for better contrast
        // This is the official Material Design 3 primary color that ensures WCAG AA compliance
        Self::from_seed(Color::from_rgb(0.404, 0.314, 0.643)) // Material blue-purple
    }

    /// Creates a `MaterialPalette` from a seed color
    #[must_use]
    pub fn from_seed(seed: Color) -> Self {
        crate::styling::material::seed::generate_palette_from_seed(seed)
    }
}

impl Default for MaterialPalette {
    fn default() -> Self {
        Self::new()
    }
}

/// Color roles for a single color group
#[derive(Debug, Clone, PartialEq)]
pub struct ColorRole {
    /// Base color for the role
    pub base: Color,
    /// Color for content on the base color
    pub on_base: Color,
    /// Container color for the role
    pub container: Color,
    /// Color for content on the container
    pub on_container: Color,
    /// Fixed color variant
    pub fixed: Color,
    /// Dimmed fixed color variant
    pub fixed_dim: Color,
    /// Color for content on fixed surfaces
    pub on_fixed: Color,
    /// Variant color for content on fixed surfaces
    pub on_fixed_variant: Color,
}

/// `MaterialColors`: all semantic color roles
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialColors {
    /// Primary color role
    pub primary: ColorRole,
    /// Secondary color role
    pub secondary: ColorRole,
    /// Tertiary color role
    pub tertiary: ColorRole,
    /// Error color role
    pub error: ColorRole,
    /// Main surface color
    pub surface: Color,
    /// Color for content on main surface
    pub on_surface: Color,
    /// Surface variant color
    pub surface_variant: Color,
    /// Color for content on surface variant
    pub on_surface_variant: Color,
    /// Background color
    pub background: Color,
    /// Color for content on background
    pub on_background: Color,
    /// Outline color for borders
    pub outline: Color,
    /// Variant outline color for borders
    pub outline_variant: Color,
    /// Inverse surface color
    pub inverse_surface: Color,
    /// Color for content on inverse surface
    pub inverse_on_surface: Color,
    /// Inverse primary color
    pub inverse_primary: Color,
    /// Shadow color
    pub shadow: Color,
    /// Scrim color for overlays
    pub scrim: Color,
    /// Surface tint color
    pub surface_tint: Color,
    /// Standard surface container color
    pub surface_container: Color,
    /// Low emphasis surface container color
    pub surface_container_low: Color,
    /// Lowest emphasis surface container color
    pub surface_container_lowest: Color,
    /// High emphasis surface container color
    pub surface_container_high: Color,
    /// Highest emphasis surface container color
    pub surface_container_highest: Color,
    /// Color for content on primary surfaces
    pub on_primary: Color,
    /// Primary container color
    pub primary_container: Color,
    /// Color for content on primary container
    pub on_primary_container: Color,
    /// Secondary container color
    pub secondary_container: Color,
    /// Color for content on secondary container
    pub on_secondary_container: Color,
    /// Color for content on error surfaces
    pub on_error: Color,
}

impl Default for MaterialColors {
    fn default() -> Self {
        Self::light(&MaterialPalette::default())
    }
}

// Tone indices for each color role
struct ColorRoleToneMap {
    base: usize,
    on_base: usize,
    container: usize,
    on_container: usize,
    fixed: usize,
    fixed_dim: usize,
    on_fixed: usize,
    on_fixed_variant: usize,
}

const LIGHT_PRIMARY_TONES: ColorRoleToneMap = ColorRoleToneMap {
    base: 9,         // tone 30 - Darker primary for better contrast with white text (was tone 40)
    on_base: 23,     // tone 100 (white text on dark primary)
    container: 16,   // tone 90 (light container)
    on_container: 3, // tone 10 (dark text on light container)
    fixed: 16,
    fixed_dim: 14,
    on_fixed: 3,
    on_fixed_variant: 9,
};

const LIGHT_SECONDARY_TONES: ColorRoleToneMap = ColorRoleToneMap {
    base: 9,         // tone 30 - Darker for better contrast
    on_base: 23,     // tone 100
    container: 16,   // tone 90
    on_container: 3, // tone 10
    fixed: 16,
    fixed_dim: 14,
    on_fixed: 3,
    on_fixed_variant: 9,
};

const LIGHT_TERTIARY_TONES: ColorRoleToneMap = ColorRoleToneMap {
    base: 9,         // tone 30 - Darker for better contrast
    on_base: 23,     // tone 100
    container: 16,   // tone 90
    on_container: 3, // tone 10
    fixed: 16,
    fixed_dim: 14,
    on_fixed: 3,
    on_fixed_variant: 9,
};

const LIGHT_ERROR_TONES: ColorRoleToneMap = ColorRoleToneMap {
    base: 9,         // tone 30 - Darker for better contrast
    on_base: 23,     // tone 100
    container: 16,   // tone 90
    on_container: 3, // tone 10
    fixed: 16,
    fixed_dim: 14,
    on_fixed: 3,
    on_fixed_variant: 9,
};

impl MaterialColors {
    /// Creates a light theme color scheme from the given palette
    #[must_use]
    pub const fn light(palette: &MaterialPalette) -> Self {
        const fn role(pal: &TonalPalette, map: &ColorRoleToneMap) -> ColorRole {
            ColorRole {
                base: pal.tones[map.base],
                on_base: pal.tones[map.on_base],
                container: pal.tones[map.container],
                on_container: pal.tones[map.on_container],
                fixed: pal.tones[map.fixed],
                fixed_dim: pal.tones[map.fixed_dim],
                on_fixed: pal.tones[map.on_fixed],
                on_fixed_variant: pal.tones[map.on_fixed_variant],
            }
        }
        Self {
            primary: role(&palette.primary, &LIGHT_PRIMARY_TONES),
            secondary: role(&palette.secondary, &LIGHT_SECONDARY_TONES),
            tertiary: role(&palette.tertiary, &LIGHT_TERTIARY_TONES),
            error: role(&palette.error, &LIGHT_ERROR_TONES),
            surface: palette.neutral.tones[19],
            on_surface: palette.neutral.tones[3],
            surface_variant: palette.neutral_variant.tones[16],
            on_surface_variant: palette.neutral_variant.tones[9],
            background: palette.neutral.tones[19],
            on_background: palette.neutral.tones[3],
            outline: palette.neutral_variant.tones[10],
            outline_variant: palette.neutral_variant.tones[14],
            inverse_surface: palette.neutral.tones[16],
            inverse_on_surface: palette.neutral.tones[3],
            inverse_primary: palette.primary.tones[14],
            shadow: palette.neutral.tones[0],
            scrim: palette.neutral.tones[0],
            surface_tint: palette.primary.tones[10],
            // Material Design 3 container fields
            surface_container: palette.neutral.tones[15],
            surface_container_low: palette.neutral.tones[17],
            surface_container_lowest: palette.neutral.tones[20],
            surface_container_high: palette.neutral.tones[13],
            surface_container_highest: palette.neutral.tones[10],
            // Additional color fields
            on_primary: palette.primary.tones[23],
            primary_container: palette.primary.tones[16],
            on_primary_container: palette.primary.tones[3],
            secondary_container: palette.secondary.tones[16],
            on_secondary_container: palette.secondary.tones[3],
            on_error: palette.error.tones[23],
        }
    }

    /// Creates a dark theme color scheme from the given palette
    #[must_use]
    pub const fn dark(palette: &MaterialPalette) -> Self {
        const fn role(pal: &TonalPalette, map: &ColorRoleToneMap) -> ColorRole {
            ColorRole {
                base: pal.tones[map.base],
                on_base: pal.tones[map.on_base],
                container: pal.tones[map.container],
                on_container: pal.tones[map.on_container],
                fixed: pal.tones[map.fixed],
                fixed_dim: pal.tones[map.fixed_dim],
                on_fixed: pal.tones[map.on_fixed],
                on_fixed_variant: pal.tones[map.on_fixed_variant],
            }
        }

        // Dark theme tone mappings - Corrected for proper WCAG AA contrast compliance
        // These indices correspond to positions in the TONE_INDICES array, not actual tone values
        // For dark themes, we need DARK base colors with LIGHT text/icons for proper contrast
        const DARK_PRIMARY_TONES: ColorRoleToneMap = ColorRoleToneMap {
            base: 9,      // Index 9 is tone 30 - Dark base for consistent contrast with white icons/text
            on_base: 23,  // Index 23 is tone 100 - White text/icons on dark primary base
            container: 6, // Index 6 is tone 20 - Dark container for better dark theme consistency
            on_container: 16, // Index 16 is tone 90 - Light text on dark container
            fixed: 16,
            fixed_dim: 14,
            on_fixed: 3,
            on_fixed_variant: 9,
        };
        const DARK_SECONDARY_TONES: ColorRoleToneMap = ColorRoleToneMap {
            base: 9,          // Index 9 is tone 30 - Dark secondary base for better contrast
            on_base: 23,      // Index 23 is tone 100 - White text on dark secondary base
            container: 6,     // Index 6 is tone 20 - Dark container
            on_container: 16, // Index 16 is tone 90 - Light text on dark container
            fixed: 16,
            fixed_dim: 14,
            on_fixed: 3,
            on_fixed_variant: 9,
        };
        const DARK_TERTIARY_TONES: ColorRoleToneMap = ColorRoleToneMap {
            base: 9,          // Index 9 is tone 30 - Dark tertiary base for better contrast
            on_base: 23,      // Index 23 is tone 100 - White text on dark tertiary base
            container: 6,     // Index 6 is tone 20 - Dark container
            on_container: 16, // Index 16 is tone 90 - Light text on dark container
            fixed: 16,
            fixed_dim: 14,
            on_fixed: 3,
            on_fixed_variant: 9,
        };
        const DARK_ERROR_TONES: ColorRoleToneMap = ColorRoleToneMap {
            base: 9,          // Index 9 is tone 30 - Dark error base for better contrast
            on_base: 23,      // Index 23 is tone 100 - White text on dark error base
            container: 6,     // Index 6 is tone 20 - Dark container
            on_container: 16, // Index 16 is tone 90 - Light text on dark container
            fixed: 16,
            fixed_dim: 14,
            on_fixed: 3,
            on_fixed_variant: 9,
        };
        Self {
            primary: role(&palette.primary, &DARK_PRIMARY_TONES),
            secondary: role(&palette.secondary, &DARK_SECONDARY_TONES),
            tertiary: role(&palette.tertiary, &DARK_TERTIARY_TONES),
            error: role(&palette.error, &DARK_ERROR_TONES),
            surface: palette.neutral.tones[2], // Dark background
            on_surface: palette.neutral.tones[16], // Bright text (tone 90) for dark surfaces
            surface_variant: palette.neutral_variant.tones[6], // Darker variant for headers
            on_surface_variant: palette.neutral_variant.tones[15], // Bright text (tone 80) for dark surfaces
            background: palette.neutral.tones[1],                  // Very dark background
            on_background: palette.neutral.tones[16],              // Bright text
            outline: palette.neutral_variant.tones[12],
            outline_variant: palette.neutral_variant.tones[6],
            inverse_surface: palette.neutral.tones[16],
            inverse_on_surface: palette.neutral.tones[2],
            inverse_primary: palette.primary.tones[10],
            shadow: palette.neutral.tones[0],
            scrim: palette.neutral.tones[0],
            surface_tint: palette.primary.tones[14],
            // Material Design 3 container fields - darkened for better dark theme contrast
            surface_container: palette.neutral.tones[4], // Darker container
            surface_container_low: palette.neutral.tones[3], // Even darker
            surface_container_lowest: palette.neutral.tones[2], // Darkest (almost black)
            surface_container_high: palette.neutral.tones[7], // Slightly lighter
            surface_container_highest: palette.neutral.tones[10], // Lightest container
            // Additional color fields - Fixed for proper dark theme contrast
            on_primary: palette.primary.tones[23], // White text on dark primary
            primary_container: palette.primary.tones[6], // Dark container (tone 20)
            on_primary_container: palette.primary.tones[16], // Light text on dark container
            // Corrected secondary container colors for better dark theme readability
            secondary_container: palette.secondary.tones[6], // Dark background (tone 20)
            on_secondary_container: palette.secondary.tones[16], // Light text (tone 90)
            on_error: palette.error.tones[23],               // White text on dark error
        }
    }

    /// Creates a `MaterialColors` scheme from a seed color
    ///
    /// Generates a complete Material Design 3 color system from any input color,
    /// creating harmonious secondary and tertiary colors following MD3 specifications.
    #[must_use]
    pub fn from_seed(seed: Color, is_dark: bool) -> Self {
        let palette = crate::styling::material::seed::generate_palette_from_seed(seed);
        if is_dark {
            Self::dark(&palette)
        } else {
            Self::light(&palette)
        }
    }

    /// Creates a light theme color scheme using the default palette
    #[must_use]
    pub fn light_default() -> Self {
        Self::light(&MaterialPalette::default())
    }

    /// Creates a dark theme color scheme using the default palette
    #[must_use]
    pub fn dark_default() -> Self {
        Self::dark(&MaterialPalette::default())
    }
}
