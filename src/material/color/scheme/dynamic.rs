//! Dynamic theming for Material Design 3
//!
//! This module provides support for dynamic theming in Material Design 3,
//! including color extraction from images and dynamic theme generation.

use super::{Theme, ThemeVariant};
use crate::material::color::{ColorRoles, Srgb, MaterialPalette, TonalPalette};
use std::collections::HashMap;
use std::f32::consts::PI;

/// Dynamic theme settings
#[derive(Debug, Clone)]
pub struct DynamicTheme {
    /// Base theme variant (light/dark)
    pub base_variant: ThemeVariant,
    /// Custom seed color
    pub seed_color: Option<Srgb>,
    /// Custom color overrides
    pub custom_colors: HashMap<String, Srgb>,
    /// Whether to use system color scheme
    pub use_system_theme: bool,
}

impl Default for DynamicTheme {
    fn default() -> Self {
        Self {
            base_variant: ThemeVariant::Light,
            seed_color: None,
            custom_colors: HashMap::new(),
            use_system_theme: false,
        }
    }
}

impl DynamicTheme {
    /// Create a new dynamic theme with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the base theme variant
    pub fn with_variant(mut self, variant: ThemeVariant) -> Self {
        self.base_variant = variant;
        self
    }
    
    /// Set a custom seed color
    pub fn with_seed_color(mut self, color: Srgb) -> Self {
        self.seed_color = Some(color);
        self
    }
    
    /// Add a custom color override
    pub fn with_custom_color(mut self, name: &str, color: Srgb) -> Self {
        self.custom_colors.insert(name.to_string(), color);
        self
    }
    
    /// Enable/disable system theme detection
    pub fn use_system_theme(mut self, enabled: bool) -> Self {
        self.use_system_theme = enabled;
        self
    }
    
    /// Generate a theme based on the current settings
    pub fn generate_theme(&self) -> Theme {
        // Start with the base theme (light or dark)
        let mut theme = match self.base_variant {
            ThemeVariant::Light => Theme::light(),
            ThemeVariant::Dark => Theme::dark(),
        };
        
        // Apply seed color if provided
        if let Some(seed) = &self.seed_color {
            // Create a material palette from the seed color
            let palette = MaterialPalette::from_seed(seed.r, seed.g, seed.b);
            
            // Generate tonal palettes for primary, secondary, and tertiary colors
            let primary_palette = TonalPalette::from_color(seed.r, seed.g, seed.b);
            
            // Generate secondary color (rotated hue by 60 degrees)
            let secondary_hue = (palette.hue + 60.0) % 360.0;
            let secondary_palette = TonalPalette::from_hue(secondary_hue, palette.chroma);
            
            // Generate tertiary color (rotated hue by -30 degrees from primary)
            let tertiary_hue = (palette.hue + 330.0) % 360.0; // 360 - 30
            let tertiary_palette = TonalPalette::from_hue(tertiary_hue, palette.chroma);
            
            // Generate error color palette (red-based)
            let error_palette = TonalPalette::from_hue(25.0, 0.84); // Red hue
            
            // Neutral and neutral-variant palettes
            let neutral_palette = TonalPalette::from_hue(palette.hue, 0.05);
            let neutral_variant_palette = TonalPalette::from_hue(palette.hue, 0.08);
            
            // Apply colors based on theme variant
            match self.base_variant {
                ThemeVariant::Light => {
                    // Light theme colors
                    theme.colors.primary = primary_palette.tone(40.0);
                    theme.colors.on_primary = primary_palette.tone(100.0);
                    theme.colors.primary_container = primary_palette.tone(90.0);
                    theme.colors.on_primary_container = primary_palette.tone(10.0);
                    
                    theme.colors.secondary = secondary_palette.tone(40.0);
                    theme.colors.on_secondary = secondary_palette.tone(100.0);
                    theme.colors.secondary_container = secondary_palette.tone(90.0);
                    theme.colors.on_secondary_container = secondary_palette.tone(10.0);
                    
                    theme.colors.tertiary = tertiary_palette.tone(40.0);
                    theme.colors.on_tertiary = tertiary_palette.tone(100.0);
                    theme.colors.tertiary_container = tertiary_palette.tone(90.0);
                    theme.colors.on_tertiary_container = tertiary_palette.tone(10.0);
                    
                    theme.colors.error = error_palette.tone(40.0);
                    theme.colors.on_error = error_palette.tone(100.0);
                    theme.colors.error_container = error_palette.tone(90.0);
                    theme.colors.on_error_container = error_palette.tone(10.0);
                    
                    // Surface and background colors
                    theme.colors.background = neutral_palette.tone(99.0);
                    theme.colors.on_background = neutral_palette.tone(10.0);
                    theme.colors.surface = neutral_palette.tone(99.0);
                    theme.colors.on_surface = neutral_palette.tone(10.0);
                    theme.colors.surface_variant = neutral_variant_palette.tone(90.0);
                    theme.colors.on_surface_variant = neutral_variant_palette.tone(30.0);
                    
                    // Outline colors
                    theme.colors.outline = neutral_variant_palette.tone(50.0);
                    theme.colors.outline_variant = neutral_variant_palette.tone(80.0);
                    
                    // Shadow and scrim
                    theme.colors.shadow = Srgb::new(0.0, 0.0, 0.0);
                    theme.colors.scrim = Srgb::new(0.0, 0.0, 0.0);
                    
                    // Inverse colors
                    theme.colors.inverse_surface = neutral_palette.tone(20.0);
                    theme.colors.inverse_on_surface = neutral_palette.tone(95.0);
                    theme.colors.inverse_primary = primary_palette.tone(80.0);
                },
                ThemeVariant::Dark => {
                    // Dark theme colors
                    theme.colors.primary = primary_palette.tone(80.0);
                    theme.colors.on_primary = primary_palette.tone(20.0);
                    theme.colors.primary_container = primary_palette.tone(30.0);
                    theme.colors.on_primary_container = primary_palette.tone(90.0);
                    
                    theme.colors.secondary = secondary_palette.tone(80.0);
                    theme.colors.on_secondary = secondary_palette.tone(20.0);
                    theme.colors.secondary_container = secondary_palette.tone(30.0);
                    theme.colors.on_secondary_container = secondary_palette.tone(90.0);
                    
                    theme.colors.tertiary = tertiary_palette.tone(80.0);
                    theme.colors.on_tertiary = tertiary_palette.tone(20.0);
                    theme.colors.tertiary_container = tertiary_palette.tone(30.0);
                    theme.colors.on_tertiary_container = tertiary_palette.tone(90.0);
                    
                    theme.colors.error = error_palette.tone(80.0);
                    theme.colors.on_error = error_palette.tone(20.0);
                    theme.colors.error_container = error_palette.tone(30.0);
                    theme.colors.on_error_container = error_palette.tone(90.0);
                    
                    // Surface and background colors
                    theme.colors.background = neutral_palette.tone(10.0);
                    theme.colors.on_background = neutral_palette.tone(90.0);
                    theme.colors.surface = neutral_palette.tone(10.0);
                    theme.colors.on_surface = neutral_palette.tone(90.0);
                    theme.colors.surface_variant = neutral_variant_palette.tone(30.0);
                    theme.colors.on_surface_variant = neutral_variant_palette.tone(80.0);
                    
                    // Outline colors
                    theme.colors.outline = neutral_variant_palette.tone(60.0);
                    theme.colors.outline_variant = neutral_variant_palette.tone(30.0);
                    
                    // Shadow and scrim
                    theme.colors.shadow = Srgb::new(0.0, 0.0, 0.0);
                    theme.colors.scrim = Srgb::new(0.0, 0.0, 0.0);
                    
                    // Inverse colors
                    theme.colors.inverse_surface = neutral_palette.tone(90.0);
                    theme.colors.inverse_on_surface = neutral_palette.tone(20.0);
                    theme.colors.inverse_primary = primary_palette.tone(40.0);
                }
            }
        }
        
        // Apply custom color overrides
        for (name, color) in &self.custom_colors {
            // Map custom colors to theme colors
            match name.as_str() {
                "primary" => theme.colors.primary = *color,
                "on_primary" => theme.colors.on_primary = *color,
                "primary_container" => theme.colors.primary_container = *color,
                "on_primary_container" => theme.colors.on_primary_container = *color,
                "secondary" => theme.colors.secondary = *color,
                "on_secondary" => theme.colors.on_secondary = *color,
                "secondary_container" => theme.colors.secondary_container = *color,
                "on_secondary_container" => theme.colors.on_secondary_container = *color,
                "tertiary" => theme.colors.tertiary = *color,
                "on_tertiary" => theme.colors.on_tertiary = *color,
                "tertiary_container" => theme.colors.tertiary_container = *color,
                "on_tertiary_container" => theme.colors.on_tertiary_container = *color,
                "error" => theme.colors.error = *color,
                "on_error" => theme.colors.on_error = *color,
                "error_container" => theme.colors.error_container = *color,
                "on_error_container" => theme.colors.on_error_container = *color,
                "background" => theme.colors.background = *color,
                "on_background" => theme.colors.on_background = *color,
                "surface" => theme.colors.surface = *color,
                "on_surface" => theme.colors.on_surface = *color,
                "surface_variant" => theme.colors.surface_variant = *color,
                "on_surface_variant" => theme.colors.on_surface_variant = *color,
                "outline" => theme.colors.outline = *color,
                "outline_variant" => theme.colors.outline_variant = *color,
                "shadow" => theme.colors.shadow = *color,
                "scrim" => theme.colors.scrim = *color,
                "inverse_surface" => theme.colors.inverse_surface = *color,
                "inverse_on_surface" => theme.colors.inverse_on_surface = *color,
                "inverse_primary" => theme.colors.inverse_primary = *color,
                _ => println!("Unknown color role: {}", name),
            }
        }
        
        theme
    }
    
    /// Extract a color scheme from an image
    /// 
    /// # Arguments
    /// * `image_data` - Raw image data in RGBA format
    /// * `width` - Image width in pixels
    /// * `height` - Image height in pixels
    /// 
    /// # Returns
    /// A `DynamicTheme` configured with colors extracted from the image
    pub fn extract_from_image(
        &mut self, 
        image_data: &[u8], 
        width: u32, 
        height: u32
    ) -> Option<()> {
        // TODO: Implement image color extraction
        // This is a placeholder implementation
        
        if image_data.is_empty() || width == 0 || height == 0 {
            return None;
        }
        
        // Simple average color calculation (simplified)
        let mut r = 0u32;
        let mut g = 0u32;
        let mut b = 0u32;
        let mut count = 0u32;
        
        for pixel in image_data.chunks(4) {
            if pixel.len() == 4 {
                r += pixel[0] as u32;
                g += pixel[1] as u32;
                b += pixel[2] as u32;
                count += 1;
            }
        }
        
        if count > 0 {
            let r_avg = (r / count) as f32 / 255.0;
            let g_avg = (g / count) as f32 / 255.0;
            let b_avg = (b / count) as f32 / 255.0;
            
            self.seed_color = Some(Srgb::new(r_avg, g_avg, b_avg));
            Some(())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_theme_creation() {
        let theme = DynamicTheme::new()
            .with_variant(ThemeVariant::Dark)
            .with_seed_color(Srgb::new(0.5, 0.2, 0.8))
            .generate_theme();
        
        assert!(theme.is_dark());
        
        // Verify seed color was applied
        assert_eq!(theme.colors.primary.r, 0.5);
        assert_eq!(theme.colors.primary.g, 0.2);
        assert_eq!(theme.colors.primary.b, 0.8);
    }
    
    #[test]
    fn test_image_extraction() {
        let mut dynamic_theme = DynamicTheme::new();
        
        // Create a simple 2x2 red image (RGBA)
        let image_data = vec![
            255, 0, 0, 255,    // Red
            200, 0, 0, 255,    // Darker red
            150, 0, 0, 255,    // Even darker red
            100, 0, 0, 255,    // Very dark red
        ];
        
        let result = dynamic_theme.extract_from_image(&image_data, 2, 2);
        assert!(result.is_some());
        
        // The average should be a shade of red
        let seed = dynamic_theme.seed_color.unwrap();
        assert!(seed.r > 0.0);
        assert_eq!(seed.g, 0.0);
        assert_eq!(seed.b, 0.0);
    }
}
