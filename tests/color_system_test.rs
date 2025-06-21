//! Tests for the Material Design 3 color system

use abop_iced::material::color::{
    Theme, ThemeVariant, Srgb, DynamicTheme
};

#[test]
fn test_theme_from_seed() {
    // Test creating a theme from a seed color
    let seed = Srgb::new(0.3, 0.5, 0.8);
    let theme = Theme::from_seed(seed, ThemeVariant::Light);

    // Verify the theme was created with the correct variant
    assert!(theme.is_light());

    // Verify the primary color matches the seed
    assert_eq!(theme.colors.primary, seed);

    // Verify on_primary has sufficient contrast
    let primary_lum = luma(theme.colors.primary);
    let on_primary_lum = luma(theme.colors.on_primary);
    let contrast = if primary_lum > on_primary_lum {
        (primary_lum + 0.05) / (on_primary_lum + 0.05)
    } else {
        (on_primary_lum + 0.05) / (primary_lum + 0.05)
    };

    // WCAG AA requires at least 4.5:1 for normal text
    assert!(contrast >= 4.5, "Insufficient contrast: {}", contrast);
}

#[test]
fn test_dynamic_theme_builder() {
    // Test the DynamicTheme builder
    let theme = DynamicTheme::new()
        .with_seed_color(Srgb::new(0.2, 0.4, 0.8))
        .with_variant(ThemeVariant::Dark)
        .with_custom_color("error", Srgb::new(0.9, 0.2, 0.2))
        .generate_theme();

    // Verify the theme variant
    assert!(theme.is_dark());

    // Verify custom error color was applied
    assert_eq!(theme.colors.error, Srgb::new(0.9, 0.2, 0.2));
}

#[test]
fn test_theme_toggle() {
    // Test toggling between light and dark themes
    let mut theme = Theme::light();
    assert!(theme.is_light());

    theme.toggle();
    assert!(theme.is_dark());

    theme.toggle();
    assert!(theme.is_light());
}

#[test]
fn test_color_roles_consistency() {
    // Test that all color roles are properly initialized
    let theme = Theme::light();
    
    // Check primary colors
    assert_ne!(theme.colors.primary, theme.colors.on_primary);
    assert_ne!(theme.colors.primary, theme.colors.primary_container);
    assert_ne!(theme.colors.on_primary, theme.colors.on_primary_container);
    
    // Check secondary colors
    assert_ne!(theme.colors.secondary, theme.colors.on_secondary);
    
    // Check surface colors
    assert_ne!(theme.colors.surface, theme.colors.on_surface);
    
    // Check error colors
    assert_ne!(theme.colors.error, theme.colors.on_error);
}

/// Helper function to calculate relative luminance
fn luma(color: Srgb) -> f32 {
    // sRGB to linear RGB
    let r = if color.r <= 0.04045 {
        color.r / 12.92
    } else {
        ((color.r + 0.055) / 1.055).powf(2.4)
    };
    
    let g = if color.g <= 0.04045 {
        color.g / 12.92
    } else {
        ((color.g + 0.055) / 1.055).powf(2.4)
    };
    
    let b = if color.b <= 0.04045 {
        color.b / 12.92
    } else {
        ((color.b + 0.055) / 1.055).powf(2.4)
    };
    
    // Calculate relative luminance (sRGB)
    0.2126 * r + 0.7152 * g + 0.0722 * b
}
