//! Test contrast ratios for button styling in dark theme
//! 
//! This test investigates the reported issue of white icons/logos on light lavender
//! backgrounds in dark theme that cause poor contrast.

use iced::Color;
use crate::styling::{
    color_utils::ColorUtils,
    material::{
        unified_colors::{MaterialColors, MaterialPalette},
    },
};

#[cfg(test)]
mod button_contrast_tests {
    use super::*;

    /// Test button contrast ratios in dark theme to identify issues
    #[test]
    fn test_dark_theme_button_contrast() {
        let palette = MaterialPalette::default();
        let dark_colors = MaterialColors::dark(&palette);
        
        println!("=== Dark Theme Button Contrast Analysis ===");
        
        // Test primary button (most common issue)
        test_contrast("Primary button", dark_colors.primary.base, dark_colors.primary.on_base);
        
        // Test surface variant (often used for icon buttons)
        test_contrast("Surface variant", dark_colors.surface_variant, dark_colors.on_surface_variant);
        
        // Test secondary container (tonal buttons)
        test_contrast("Secondary container", dark_colors.secondary.container, dark_colors.secondary.on_container);
        
        // Test outline buttons (background transparent, text primary)
        test_contrast("Outline button text", Color::TRANSPARENT, dark_colors.primary.base);
        test_contrast("Outline button on surface", dark_colors.surface, dark_colors.primary.base);
        
        // Test icon buttons on various surfaces
        test_contrast("Icon on surface", dark_colors.surface, dark_colors.on_surface);
        test_contrast("Icon on background", dark_colors.background, dark_colors.on_background);
        
        // Test the problematic light lavender scenario mentioned
        // This might be surface_variant with wrong icon color
        let lavender_like = Color::from_rgb(0.85, 0.82, 0.92); // Light lavender
        let white_icon = Color::WHITE;
        test_contrast("White icon on lavender", lavender_like, white_icon);
        
        println!("\n=== Light Theme Comparison ===");
        let light_colors = MaterialColors::light(&palette);
        test_contrast("Light primary", light_colors.primary.base, light_colors.primary.on_base);
        test_contrast("Light surface", light_colors.surface, light_colors.on_surface);
    }

    /// Test a specific contrast combination and report results
    fn test_contrast(name: &str, background: Color, foreground: Color) {
        let contrast = ColorUtils::contrast_ratio(foreground, background);
        let aa_normal = contrast >= 4.5;
        let aa_large = contrast >= 3.0;
        let aaa = contrast >= 7.0;
        
        let status = if aaa {
            "AAA ✓"
        } else if aa_normal {
            "AA ✓"
        } else if aa_large {
            "AA Large ✓"
        } else {
            "FAIL ✗"
        };
        
        println!("{:25}: {:5.2}:1 [{}]", name, contrast, status);
        
        if background != Color::TRANSPARENT {
            println!("  Background: rgb({:.0}, {:.0}, {:.0})", 
                background.r * 255.0, background.g * 255.0, background.b * 255.0);
        }
        println!("  Foreground: rgb({:.0}, {:.0}, {:.0})", 
            foreground.r * 255.0, foreground.g * 255.0, foreground.b * 255.0);
            
        if !aa_normal {
            println!("  ⚠️  Contrast issue detected!");
            
            // Suggest fixes
            let suggested_fg = suggest_contrasting_color(background, foreground);
            if let Some(suggested) = suggested_fg {
                let new_contrast = ColorUtils::contrast_ratio(suggested, background);
                println!("  💡 Suggested foreground: rgb({:.0}, {:.0}, {:.0}) - {:.2}:1", 
                    suggested.r * 255.0, suggested.g * 255.0, suggested.b * 255.0, new_contrast);
            }
        }
        println!();
    }

    /// Suggest a contrasting color for better accessibility
    fn suggest_contrasting_color(background: Color, _current: Color) -> Option<Color> {
        if background == Color::TRANSPARENT {
            return None;
        }
          // Simple suggestion: use white or black based on background luminance
        let luminance = ColorUtils::luminance(background);
        if luminance > 0.5 {
            Some(Color::BLACK) // Dark text on light background
        } else {
            Some(Color::WHITE) // Light text on dark background
        }
    }

    /// Test if button styling strategies properly handle contrast
    #[test]
    fn test_button_strategy_contrast() {
        println!("=== Button Strategy Contrast Test ===");
        
        let palette = MaterialPalette::default();
        let dark_colors = MaterialColors::dark(&palette);
        
        // Simulate what each button variant should use
        
        // Filled button (primary background)
        let filled_bg = dark_colors.primary.base;
        let filled_fg = dark_colors.primary.on_base;
        test_contrast("Filled strategy", filled_bg, filled_fg);
        
        // Text button (transparent background, primary text on surface)
        let text_bg = dark_colors.surface;
        let text_fg = dark_colors.primary.base; // This might be the issue!
        test_contrast("Text strategy", text_bg, text_fg);
        
        // Icon button (transparent background, on_surface text)
        let icon_bg = Color::TRANSPARENT;
        let icon_fg = dark_colors.on_surface;
        println!("Icon button on surface:");
        test_contrast("Icon strategy", dark_colors.surface, icon_fg);
        
        // Outlined button (transparent background, primary border/text)
        test_contrast("Outlined strategy", dark_colors.surface, dark_colors.primary.base);
    }
}
