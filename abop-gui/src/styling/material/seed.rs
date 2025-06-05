//! Material Design 3 Seed Color Generation
//!
//! Implements dynamic palette generation from seed colors using the Material Design 3
//! specification. This module converts any color into a complete Material Design
//! color system with proper tonal palettes and color harmony.

use crate::styling::material::colors::{MaterialPalette, TonalPalette};
use iced::Color;
use material_color_utilities_rs::htc::Hct;

/// Material Design 3 hue rotations for generating harmonious color palettes
const SECONDARY_HUE_ROTATION: f64 = 60.0;
const TERTIARY_HUE_ROTATION: f64 = 120.0;
const NEUTRAL_CHROMA: f64 = 4.0;
const NEUTRAL_VARIANT_CHROMA: f64 = 8.0;

/// Error palette - always uses the same red-based colors for consistency
const ERROR_HUE: f64 = 25.0;
const ERROR_CHROMA: f64 = 84.0;

/// Tone values for Material Design 3 tonal palettes
const TONE_VALUES: [f64; 24] = [
    0.0, 4.0, 6.0, 10.0, 12.0, 17.0, 20.0, 22.0, 24.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 87.0,
    90.0, 92.0, 94.0, 95.0, 96.0, 98.0, 99.0, 100.0,
];

/// HCT (Hue, Chroma, Tone) color representation used in Material Design 3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HctColor {
    /// Hue in degrees (0-360)
    pub hue: f64,
    /// Chroma (colorfulness) - typically 0-150, but can be higher
    pub chroma: f64,
    /// Tone (perceived lightness) from 0 (black) to 100 (white)
    pub tone: f64,
}

impl HctColor {
    /// Create a new HCT color
    #[must_use]
    pub fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        Self {
            hue: hue % 360.0,
            chroma: chroma.max(0.0),
            tone: tone.clamp(0.0, 100.0),
        }
    }

    /// Convert from RGB color using real HCT conversion
    #[must_use]
    pub fn from_rgb(color: Color) -> Self {
        // Convert Iced Color to ARGB byte array
        fn clamp_to_u8(val: f32) -> u8 {
            if val.is_nan() || val < 0.0 {
                0
            } else if val > 1.0 {
                255
            } else {
                (val * 255.0).round().clamp(0.0, 255.0) as u8
            }
        }
        let argb = [
            255, // Full alpha
            clamp_to_u8(color.r),
            clamp_to_u8(color.g),
            clamp_to_u8(color.b),
        ];
        let hct = Hct::from_int(argb);
        Self::new(hct.hue(), hct.chroma(), hct.tone())
    }

    /// Convert to RGB color using real HCT conversion
    #[must_use]
    pub fn to_rgb(&self) -> Color {
        let hct = Hct::from(self.hue, self.chroma, self.tone);
        let [_a, r, g, b] = hct.to_int();
        Color::from_rgb8(r, g, b)
    }

    /// Create a new color with different tone (useful for tonal palettes)
    #[must_use]
    pub fn with_tone(&self, tone: f64) -> Self {
        Self::new(self.hue, self.chroma, tone)
    }

    /// Create a new color with different chroma (useful for secondary/tertiary)
    #[must_use]
    pub fn with_chroma(&self, chroma: f64) -> Self {
        Self::new(self.hue, chroma, self.tone)
    }

    /// Create a new color with different hue (useful for secondary/tertiary)
    #[must_use]
    pub fn with_hue(&self, hue: f64) -> Self {
        Self::new(hue, self.chroma, self.tone)
    }
}

/// Generates a complete Material Design 3 palette from a seed color
#[must_use]
pub fn generate_palette_from_seed(seed: Color) -> MaterialPalette {
    // Convert seed to HCT color space
    let seed_hct = HctColor::from_rgb(seed);

    // Extract hue and use appropriate chroma for each palette
    let primary_hue = seed_hct.hue;
    let primary_chroma = seed_hct.chroma.max(48.0); // Ensure minimum vibrancy

    // Generate all tonal palettes using HCT
    let primary = generate_tonal_palette_hct(primary_hue, primary_chroma);
    let secondary = generate_tonal_palette_hct(
        (primary_hue + SECONDARY_HUE_ROTATION) % 360.0,
        16.0, // Lower chroma for secondary
    );
    let tertiary = generate_tonal_palette_hct(
        (primary_hue + TERTIARY_HUE_ROTATION) % 360.0,
        24.0, // Medium chroma for tertiary
    );
    let neutral = generate_tonal_palette_hct(primary_hue, NEUTRAL_CHROMA);
    let neutral_variant = generate_tonal_palette_hct(primary_hue, NEUTRAL_VARIANT_CHROMA);
    let error = generate_tonal_palette_hct(ERROR_HUE, ERROR_CHROMA);

    MaterialPalette {
        primary,
        secondary,
        tertiary,
        neutral,
        neutral_variant,
        error,
    }
}

/// Generates a tonal palette using HCT color space
fn generate_tonal_palette_hct(hue: f64, chroma: f64) -> TonalPalette {
    let mut tones = [Color::BLACK; 24];

    for (i, &tone) in TONE_VALUES.iter().enumerate() {
        let hct_color = HctColor::new(hue, chroma, tone);
        tones[i] = hct_color.to_rgb();
    }

    TonalPalette { tones }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hct_color_creation() {
        let hct = HctColor::new(240.0, 50.0, 50.0);
        assert_eq!(hct.hue, 240.0);
        assert_eq!(hct.chroma, 50.0);
        assert_eq!(hct.tone, 50.0);
    }

    #[test]
    fn test_hct_from_rgb() {
        let blue = Color::from_rgb(0.0, 0.5, 1.0);
        let hct = HctColor::from_rgb(blue);

        // Should be roughly blue hue (around 210-270 degrees)
        assert!(
            hct.hue > 180.0 && hct.hue < 280.0,
            "Hue was {}, expected blue range 180-280",
            hct.hue
        );
        assert!(hct.chroma > 0.0);
        assert!(hct.tone > 0.0 && hct.tone < 100.0);
    }

    #[test]
    fn test_hct_round_trip() {
        let original = Color::from_rgb(0.8, 0.2, 0.6);
        let hct = HctColor::from_rgb(original);
        let converted_back = hct.to_rgb();

        // Should be approximately the same (allowing for conversion precision)
        let tolerance = 0.1;
        assert!((original.r - converted_back.r).abs() < tolerance);
        assert!((original.g - converted_back.g).abs() < tolerance);
        assert!((original.b - converted_back.b).abs() < tolerance);
    }

    #[test]
    fn test_generate_palette_from_blue() {
        let blue = Color::from_rgb(0.0, 0.5, 1.0);
        let palette = generate_palette_from_seed(blue);

        // Basic sanity checks
        assert_ne!(palette.primary.tones[0], Color::WHITE);
        assert_ne!(palette.primary.tones[23], Color::BLACK);

        // Ensure we have different palettes
        assert_ne!(palette.primary.tones[10], palette.secondary.tones[10]);
        assert_ne!(palette.primary.tones[10], palette.tertiary.tones[10]);
    }

    #[test]
    fn test_generate_palette_from_red() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let palette = generate_palette_from_seed(red);

        // Test that error palette is consistent regardless of seed
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let palette2 = generate_palette_from_seed(blue);

        // Error palettes should be the same (they're always red-based)
        assert_eq!(palette.error.tones[10], palette2.error.tones[10]);
    }

    #[test]
    fn test_tone_progression() {
        let hct_base = HctColor::new(240.0, 50.0, 50.0);
        let tone_0 = hct_base.with_tone(0.0).to_rgb();
        let tone_100 = hct_base.with_tone(100.0).to_rgb();

        // Tone 0 should be much darker than tone 100
        let brightness_0 = tone_0.r + tone_0.g + tone_0.b;
        let brightness_100 = tone_100.r + tone_100.g + tone_100.b;
        assert!(brightness_0 < brightness_100);
    }

    #[test]
    fn test_chroma_variation() {
        let hct_base = HctColor::new(240.0, 50.0, 50.0);
        let low_chroma = hct_base.with_chroma(10.0).to_rgb();
        let high_chroma = hct_base.with_chroma(80.0).to_rgb();

        // Colors should be different
        assert_ne!(low_chroma, high_chroma);
    }
}
