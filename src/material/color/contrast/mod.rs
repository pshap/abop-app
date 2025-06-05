//! Color contrast utilities for Material Design 3
//!
//! This module provides utilities for calculating and ensuring sufficient
//! color contrast for accessibility in accordance with WCAG 2.1 guidelines.

use super::Srgb;
use std::f32::consts::*;

/// Calculate the relative luminance of an sRGB color
/// 
/// # Arguments
/// * `color` - The sRGB color to calculate luminance for
/// 
/// # Returns
/// The relative luminance as a float between 0.0 (black) and 1.0 (white)
pub fn relative_luminance(color: Srgb) -> f32 {
    // Convert sRGB to linear RGB
    fn linearize(c: f32) -> f32 {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    
    let r = linearize(color.r);
    let g = linearize(color.g);
    let b = linearize(color.b);
    
    // Calculate relative luminance using sRGB coefficients
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Calculate the contrast ratio between two colors
/// 
/// # Arguments
/// * `color1` - First color
/// * `color2` - Second color
/// 
/// # Returns
/// The contrast ratio as a float between 1.0 (no contrast) and 21.0 (maximum contrast)
pub fn contrast_ratio(color1: Srgb, color2: Srgb) -> f32 {
    let l1 = relative_luminance(color1) + 0.05;
    let l2 = relative_luminance(color2) + 0.05;
    
    if l1 > l2 {
        l1 / l2
    } else {
        l2 / l1
    }
}

/// Check if two colors meet the WCAG 2.1 AA contrast ratio requirement
/// 
/// # Arguments
/// * `color1` - First color
/// * `color2` - Second color
/// * `large_text` - Whether the text is large (18pt or 14pt bold) or larger
/// * `enhanced` - Whether to use the enhanced (AAA) contrast ratio requirement
/// 
/// # Returns
/// `true` if the contrast meets the requirement, `false` otherwise
pub fn meets_contrast_aa(color1: Srgb, color2: Srgb, large_text: bool, enhanced: bool) -> bool {
    let ratio = contrast_ratio(color1, color2);
    
    if large_text {
        if enhanced {
            ratio >= 4.5
        } else {
            ratio >= 3.0
        }
    } else {
        if enhanced {
            ratio >= 7.0
        } else {
            ratio >= 4.5
        }
    }
}

/// Find a color with sufficient contrast against a background
/// 
/// # Arguments
/// * `background` - The background color
/// * `foreground_candidates` - A slice of potential foreground colors
/// * `min_contrast` - The minimum required contrast ratio (default: 4.5 for AA)
/// 
/// # Returns
/// The first color in `foreground_candidates` that meets the contrast requirement,
/// or `None` if none do.
pub fn find_contrasting_color(
    background: Srgb,
    foreground_candidates: &[Srgb],
    min_contrast: Option<f32>,
) -> Option<Srgb> {
    let min_contrast = min_contrast.unwrap_or(4.5);
    
    foreground_candidates
        .iter()
        .find(|&&fg| contrast_ratio(background, fg) >= min_contrast)
        .copied()
}

/// Adjust a color's luminance to meet a minimum contrast ratio with another color
/// 
/// # Arguments
/// * `foreground` - The color to adjust
/// * `background` - The background color to contrast against
/// * `min_contrast` - The minimum required contrast ratio (default: 4.5 for AA)
/// 
/// # Returns
/// A new color with adjusted luminance that meets the contrast requirement
pub fn ensure_contrast(foreground: Srgb, background: Srgb, min_contrast: Option<f32>) -> Srgb {
    let min_contrast = min_contrast.unwrap_or(4.5);
    
    // If contrast is already sufficient, return the original color
    if contrast_ratio(foreground, background) >= min_contrast {
        return foreground;
    }
    
    // Convert to HCT for better color adjustment
    // For now, we'll use a simplified approach with HSL
    let bg_lum = relative_luminance(background);
    let fg_lum = relative_luminance(foreground);
    
    // Determine target luminance based on background
    let target_lum = if bg_lum < 0.5 {
        // Light text on dark background
        (bg_lum + 0.05) * min_contrast - 0.05
    } else {
        // Dark text on light background
        (bg_lum + 0.05) / min_contrast - 0.05
    };
    
    // Clamp target luminance
    let target_lum = target_lum.max(0.0).min(1.0);
    
    // Adjust the color's luminance
    adjust_luminance(foreground, target_lum)
}

/// Adjust the luminance of a color to a target value
/// 
/// This is a simplified version that doesn't preserve hue and chroma perfectly
fn adjust_luminance(color: Srgb, target_lum: f32) -> Srgb {
    // Simple approach: scale the color to match the target luminance
    let current_lum = relative_luminance(color);
    
    if current_lum <= 0.0 {
        return Srgb::new(0.0, 0.0, 0.0);
    }
    
    if current_lum >= 1.0 {
        return Srgb::new(1.0, 1.0, 1.0);
    }
    
    let scale = if target_lum > current_lum {
        // Brighten
        let max_scale = 1.0 / current_lum;
        let target_scale = target_lum / current_lum;
        target_scale.min(max_scale)
    } else {
        // Darken
        target_lum / current_lum
    };
    
    Srgb::new(
        (color.r * scale).min(1.0),
        (color.g * scale).min(1.0),
        (color.b * scale).min(1.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;
    
    #[test]
    fn test_relative_luminance() {
        // Test black
        assert!(approx_eq!(f32, relative_luminance(Srgb::new(0.0, 0.0, 0.0)), 0.0, ulps = 2));
        
        // Test white
        assert!(approx_eq!(f32, relative_luminance(Srgb::new(1.0, 1.0, 1.0)), 1.0, ulps = 2));
        
        // Test gray
        let gray = relative_luminance(Srgb::new(0.5, 0.5, 0.5));
        assert!(gray > 0.2 && gray < 0.3);
        
        // Test pure red
        let red = relative_luminance(Srgb::new(1.0, 0.0, 0.0));
        assert!(red > 0.2 && red < 0.25);
    }
    
    #[test]
    fn test_contrast_ratio() {
        // Black and white should have maximum contrast
        assert!(approx_eq!(
            f32, 
            contrast_ratio(Srgb::new(0.0, 0.0, 0.0), Srgb::new(1.0, 1.0, 1.0)), 
            21.0, 
            ulps = 2
        ));
        
        // Same color should have minimum contrast
        assert!(approx_eq!(
            f32, 
            contrast_ratio(Srgb::new(0.5, 0.5, 0.5), Srgb::new(0.5, 0.5, 0.5)), 
            1.0, 
            ulps = 2
        ));
    }
    
    #[test]
    fn test_ensure_contrast() {
        // Test with light text on dark background
        let dark_bg = Srgb::new(0.1, 0.1, 0.1);
        let light_fg = ensure_contrast(Srgb::new(0.3, 0.3, 0.3), dark_bg, Some(4.5));
        assert!(contrast_ratio(light_fg, dark_bg) >= 4.5);
        
        // Test with dark text on light background
        let light_bg = Srgb::new(0.9, 0.9, 0.9);
        let dark_fg = ensure_contrast(Srgb::new(0.8, 0.8, 0.8), light_bg, Some(4.5));
        assert!(contrast_ratio(dark_fg, light_bg) >= 4.5);
    }
}
