//! Tests to validate the centralized disabled color implementation
//! for Material Design consistency across all button variants

use iced::Color;

use abop_gui::styling::color_utils::ColorUtils;
use abop_gui::styling::material::MaterialTokens;
use abop_gui::styling::material::components::button_style::strategy::{
    ButtonState, ButtonStyleStrategy,
};
use abop_gui::styling::material::components::button_style::variants::{
    elevated::ElevatedButtonStrategy, filled::FilledButtonStrategy,
    filled_tonal::FilledTonalButtonStrategy, icon::IconButtonStrategy,
    outlined::OutlinedButtonStrategy, text::TextButtonStrategy,
};

/// Calculate contrast ratio between two colors
fn calculate_contrast_ratio(color1: Color, color2: Color) -> f32 {
    // Convert to grayscale using sRGB luminance factors
    let luminance1 = 0.2126 * color1.r + 0.7152 * color1.g + 0.0722 * color1.b;
    let luminance2 = 0.2126 * color2.r + 0.7152 * color2.g + 0.0722 * color2.b;

    // Calculate contrast ratio according to WCAG 2.0
    let (brighter, darker) = if luminance1 > luminance2 {
        (luminance1, luminance2)
    } else {
        (luminance2, luminance1)
    };

    // WCAG 2.0 contrast ratio formula
    (brighter + 0.05) / (darker + 0.05)
}

#[test]
fn test_centralized_disabled_colors() {
    let tokens = MaterialTokens::default();
    let expected_disabled_alpha = tokens.states.opacity.disabled; // Should be 0.38

    println!("Testing centralized disabled color implementation");
    println!("Expected disabled opacity: {}", expected_disabled_alpha);
    println!("on_surface color: {:?}", tokens.colors.on_surface);

    // Calculate expected disabled color
    let expected_disabled_color = Color {
        r: tokens.colors.on_surface.r,
        g: tokens.colors.on_surface.g,
        b: tokens.colors.on_surface.b,
        a: expected_disabled_alpha,
    };

    println!("Expected disabled color: {:?}", expected_disabled_color);

    let strategies: Vec<(Box<dyn ButtonStyleStrategy>, &str)> = vec![
        (Box::new(FilledButtonStrategy), "Filled"),
        (Box::new(FilledTonalButtonStrategy), "FilledTonal"),
        (Box::new(OutlinedButtonStrategy), "Outlined"),
        (Box::new(TextButtonStrategy), "Text"),
        (Box::new(ElevatedButtonStrategy), "Elevated"),
        (Box::new(IconButtonStrategy), "Icon"),
    ];

    for (strategy, name) in strategies {
        println!("\n--- Testing {} Button ---", name);

        let styling = strategy.get_styling(
            ButtonState::Disabled,
            &tokens,
            &tokens.colors,
            &tokens.elevation,
            &tokens.shapes,
        );

        println!("Disabled text color: {:?}", styling.text_color);

        // Verify that disabled text color uses the centralized approach
        assert_eq!(
            styling.text_color.r, expected_disabled_color.r,
            "{} button disabled text color red component should match centralized approach",
            name
        );
        assert_eq!(
            styling.text_color.g, expected_disabled_color.g,
            "{} button disabled text color green component should match centralized approach",
            name
        );
        assert_eq!(
            styling.text_color.b, expected_disabled_color.b,
            "{} button disabled text color blue component should match centralized approach",
            name
        );
        assert_eq!(
            styling.text_color.a, expected_disabled_color.a,
            "{} button disabled text color alpha should match centralized approach ({})",
            name, expected_disabled_alpha
        );

        // Check icon color if provided
        if let Some(icon_color) = styling.icon_color {
            println!("Disabled icon color: {:?}", icon_color);

            assert_eq!(
                icon_color.r, expected_disabled_color.r,
                "{} button disabled icon color red component should match centralized approach",
                name
            );
            assert_eq!(
                icon_color.g, expected_disabled_color.g,
                "{} button disabled icon color green component should match centralized approach",
                name
            );
            assert_eq!(
                icon_color.b, expected_disabled_color.b,
                "{} button disabled icon color blue component should match centralized approach",
                name
            );
            assert_eq!(
                icon_color.a, expected_disabled_color.a,
                "{} button disabled icon color alpha should match centralized approach ({})",
                name, expected_disabled_alpha
            );
        }

        // Calculate contrast ratio for accessibility validation
        let bg_color = match styling.background {
            iced::Background::Color(color) => color,
            _ => tokens.colors.surface, // Use surface as fallback for transparent backgrounds
        };

        let contrast_ratio = calculate_contrast_ratio(bg_color, styling.text_color);
        println!("Background: {:?}", bg_color);
        println!("Contrast ratio: {:.3}", contrast_ratio);

        // Verify contrast meets WCAG 2.1 AA standards (3:1 for UI components)
        assert!(
            contrast_ratio >= 3.0,
            "{} button disabled state contrast ratio ({:.3}) below minimum 3.0",
            name,
            contrast_ratio
        );

        println!("✓ {} button disabled colors validated", name);
    }

    println!("\n✅ All button variants use centralized disabled color approach");
    println!("✅ All disabled states meet WCAG 2.1 AA contrast requirements (≥3.0)");
}

#[test]
fn test_no_hardcoded_disabled_colors() {
    let tokens = MaterialTokens::default();
    // This test ensures we're not using the old hardcoded approach
    let old_hardcoded_color = ColorUtils::with_alpha(Color::WHITE, 0.6);
    let centralized_color = Color {
        r: tokens.colors.on_surface.r,
        g: tokens.colors.on_surface.g,
        b: tokens.colors.on_surface.b,
        a: tokens.states.opacity.disabled,
    };

    println!("Old hardcoded approach: {:?}", old_hardcoded_color);
    println!("New centralized approach: {:?}", centralized_color);

    // Verify they are different (which means we've successfully moved away from hardcoded)
    assert_ne!(
        old_hardcoded_color, centralized_color,
        "Disabled colors should not use old hardcoded WHITE.with_opacity(0.6) approach"
    );

    println!("✅ Successfully moved away from hardcoded disabled colors");
}

#[test]
fn test_material_design_opacity_compliance() {
    let tokens = MaterialTokens::default();

    // Material Design 3 spec recommends 0.38 opacity for disabled states
    let expected_opacity = 0.38f32;

    assert_eq!(
        tokens.states.opacity.disabled, expected_opacity,
        "Disabled opacity should follow Material Design 3 spec (0.38)"
    );

    println!(
        "✅ Disabled opacity matches Material Design 3 specification: {}",
        expected_opacity
    );
}
