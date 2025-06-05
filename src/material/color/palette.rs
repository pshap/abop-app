//! # Material Design 3 Color Palette
//!
//! This module provides the `TonalPalette` and `MaterialPalette` types for generating
//! and managing color palettes according to the Material Design 3 specification.
//!
//! ## Overview
//!
//! The palette system in Material Design 3 is based on the concept of tonal palettes,
//! which are generated from a base color and provide a range of tones (lightness values)
//! that maintain consistent hue and chroma relationships.
//!
//! ## Key Types
//!
//! - `TonalPalette`: A collection of colors sharing the same hue and chroma but with varying tones
//! - `MaterialPalette`: A complete set of tonal palettes for a Material Design color scheme
//!
//! ## Usage
//!
//! ### Creating a Tonal Palette
//! ```rust
//! use abop_iced::material::color::palette::TonalPalette;
//!
//! // Create a palette with a specific hue (0-360) and chroma (0-100+)
//! let mut palette = TonalPalette::new(120.0, 40.0); // Green hue, medium chroma
//!
//! // Get a specific tone (0-100)
//! let color = palette.get(40); // Dark green
//! let light_color = palette.get(90); // Very light green
//! ```
//!
//! ### Creating a Material Palette
//! ```rust
//! use abop_iced::material::color::{palette::MaterialPalette, Srgb};
//!
//! // Create a complete material palette from a seed color
//! let seed_color = Srgb::new(0.5, 0.2, 0.8); // Purple
//! let palette = MaterialPalette::from_seed(seed_color);
//!
//! // Access different tonal palettes
//! let primary = palette.primary.get(40); // Primary dark
//! let secondary = palette.secondary.get(70); // Secondary medium
//! let tertiary = palette.tertiary.get(90); // Tertiary light
//! ```
//!
//! ## Advanced Usage
//!
//! For dynamic theming, you can generate a new palette when the seed color changes:
//! ```rust
//! # use abop_iced::material::color::{palette::MaterialPalette, Srgb};
//! fn update_palette(new_color: Srgb) -> MaterialPalette {
//!     MaterialPalette::from_seed(new_color)
//! }
//! ```

use super::Srgb;

/// A tonal palette is a collection of colors sharing the same hue and chroma,
/// but with varying tones.
#[derive(Debug, Clone, PartialEq)]
pub struct TonalPalette {
    /// The base hue of the palette (0-360 degrees)
    pub hue: f32,
    /// The base chroma of the palette (0-1)
    pub chroma: f32,
    /// The colors in the palette, keyed by tone (0-100)
    colors: std::collections::HashMap<u8, Srgb>,
}

impl TonalPalette {
    /// Creates a new TonalPalette with the given hue and chroma.
    pub fn new(hue: f32, chroma: f32) -> Self {
        Self {
            hue,
            chroma: chroma.clamp(0.0, 1.0),
            colors: std::collections::HashMap::new(),
        }
    }

    /// Gets the color for a specific tone (0-100).
    /// If the color hasn't been computed yet, it will be computed and cached.
    pub fn get(&mut self, tone: u8) -> Srgb {
        let tone = tone.min(100);
        
        if !self.colors.contains_key(&tone) {
            let color = self.compute_tone(tone);
            self.colors.insert(tone, color);
        }
        
        self.colors[&tone]
    }

    /// Computes the color for a specific tone using the CAM16 color appearance model.
    /// This is a simplified version that will be replaced with the actual HCT implementation.
    fn compute_tone(&self, tone: u8) -> Srgb {
        // TODO: Replace with actual HCT implementation
        // This is a placeholder that returns a grayscale color based on tone
        let t = tone as f32 / 100.0;
        Srgb::new(t, t, t)
    }
}

/// A collection of tonal palettes for a complete Material Design 3 color scheme.
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialPalette {
    /// The primary color palette
    pub primary: TonalPalette,
    /// The secondary color palette
    pub secondary: TonalPalette,
    /// The tertiary color palette
    pub tertiary: TonalPalette,
    /// The error color palette
    pub error: TonalPalette,
    /// The neutral color palette
    pub neutral: TonalPalette,
    /// The neutral variant color palette
    pub neutral_variant: TonalPalette,
}

impl MaterialPalette {
    /// Creates a new MaterialPalette with the given seed color.
    pub fn from_seed(seed_color: Srgb) -> Self {
        // TODO: Extract hue and chroma from seed color using HCT
        // This is a simplified version that creates a basic palette
        
        // Convert RGB to HCT (placeholder values)
        let hue = 0.0; // Will be calculated from seed color
        let chroma = 0.5; // Will be calculated from seed color
        
        Self {
            primary: TonalPalette::new(hue, chroma),
            secondary: TonalPalette::new((hue + 30.0) % 360.0, chroma * 0.8),
            tertiary: TonalPalette::new((hue + 60.0) % 360.0, chroma * 0.6),
            error: TonalPalette::new(25.0, 0.8), // Reddish error color
            neutral: TonalPalette::new(hue, 0.05), // Very low chroma for neutrals
            neutral_variant: TonalPalette::new(hue, 0.1), // Slightly higher chroma for neutral variants
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tonal_palette() {
        let mut palette = TonalPalette::new(120.0, 0.5);
        
        // Test that getting the same tone multiple times returns the same color
        let color1 = palette.get(50);
        let color2 = palette.get(50);
        assert_eq!(color1, color2);
        
        // Test that different tones return different colors
        let color3 = palette.get(70);
        assert_ne!(color1, color3);
    }
    
    #[test]
    fn test_material_palette() {
        let seed = Srgb::new(0.5, 0.2, 0.8); // Purple-ish color
        let palette = MaterialPalette::from_seed(seed);
        
        // Basic validation of the generated palettes
        assert_eq!(palette.primary.hue, 0.0); // Will be updated with actual HCT
        assert_eq!(palette.secondary.hue, 30.0); // 30 degrees from primary
        assert_eq!(palette.tertiary.hue, 60.0); // 60 degrees from primary
        assert_eq!(palette.error.hue, 25.0); // Fixed hue for error
    }
}
