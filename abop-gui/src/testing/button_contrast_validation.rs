//! Button contrast validation tests
//!
//! Tests to investigate and validate button contrast ratios in dark theme
//! to address the reported issue of white icons/logos on light backgrounds.

#[cfg(test)]
mod tests {
    use crate::styling::{
        color_utils::ColorUtils,
        material::{
            MaterialColors, MaterialPalette, MaterialTokens,
            components::button_style::strategy::ButtonStyleStrategy,
        },
    };
    use iced::Color;

    /// Test current button contrast ratios in dark theme
    #[test]
    fn test_dark_theme_button_contrast_analysis() {
        println!("=== DARK THEME BUTTON CONTRAST ANALYSIS ===");

        let palette = MaterialPalette::default();
        let dark_colors = MaterialColors::dark(&palette);

        // Test primary buttons (most common)
        test_contrast_scenario(
            "Primary Button",
            dark_colors.primary.base,
            dark_colors.primary.on_base,
        );

        // Test surface variant buttons (common for icons)
        test_contrast_scenario(
            "Surface Variant Button",
            dark_colors.surface_variant,
            dark_colors.on_surface_variant,
        );

        // Test secondary container (tonal buttons)
        test_contrast_scenario(
            "Secondary Container",
            dark_colors.secondary.container,
            dark_colors.secondary.on_container,
        );

        // Test outline buttons (text on surface)
        test_contrast_scenario(
            "Outline Button on Surface",
            dark_colors.surface,
            dark_colors.primary.base,
        );

        // Test icon buttons
        test_contrast_scenario(
            "Icon on Surface",
            dark_colors.surface,
            dark_colors.on_surface,
        );
        test_contrast_scenario(
            "Icon on Background",
            dark_colors.background,
            dark_colors.on_background,
        );

        // Test the specific problem case: white icons on light lavender
        let light_lavender = Color::from_rgb(0.85, 0.82, 0.92);
        test_contrast_scenario(
            "WHITE ICON ON LIGHT LAVENDER (PROBLEM)",
            light_lavender,
            Color::WHITE,
        );

        // Test what the correct contrast should be
        let suggested_dark_icon = Color::from_rgb(0.2, 0.2, 0.2);
        test_contrast_scenario(
            "DARK ICON ON LIGHT LAVENDER (FIXED)",
            light_lavender,
            suggested_dark_icon,
        );

        println!("\n=== COMPARISON WITH LIGHT THEME ===");
        let light_colors = MaterialColors::light(&palette);
        test_contrast_scenario(
            "Light Primary",
            light_colors.primary.base,
            light_colors.primary.on_base,
        );
        test_contrast_scenario(
            "Light Surface",
            light_colors.surface,
            light_colors.on_surface,
        );
    }
    /// Test button strategy implementations for proper contrast
    #[test]
    fn test_button_strategy_contrast_compliance() {
        println!("=== BUTTON STRATEGY CONTRAST VALIDATION ===");
        let dark_colors = MaterialColors::dark_default();
        let tokens = MaterialTokens::dark();

        // Test actual button strategy implementations

        // Filled button strategy
        let filled_strategy = crate::styling::material::components::button_style::variants::filled::FilledButtonStrategy;
        let filled_styling = filled_strategy.get_styling(
            crate::styling::material::components::button_style::strategy::ButtonState::Default,
            &tokens,
            &dark_colors,
            &tokens.elevation,
            &tokens.shapes,
        );
        if let iced::Background::Color(bg) = filled_styling.background {
            test_contrast_scenario("Filled Strategy", bg, filled_styling.text_color);
        }

        // Text button strategy
        let text_strategy =
            crate::styling::material::components::button_style::variants::text::TextButtonStrategy;
        let text_styling = text_strategy.get_styling(
            crate::styling::material::components::button_style::strategy::ButtonState::Default,
            &tokens,
            &dark_colors,
            &tokens.elevation,
            &tokens.shapes,
        );
        if let iced::Background::Color(bg) = text_styling.background {
            test_contrast_scenario("Text Strategy", bg, text_styling.text_color);
        } else {
            // Text buttons have transparent background, so test against surface
            test_contrast_scenario(
                "Text Strategy",
                dark_colors.surface,
                text_styling.text_color,
            );
        }

        // Icon button strategy
        let icon_strategy =
            crate::styling::material::components::button_style::variants::icon::IconButtonStrategy;
        let icon_styling = icon_strategy.get_styling(
            crate::styling::material::components::button_style::strategy::ButtonState::Default,
            &tokens,
            &dark_colors,
            &tokens.elevation,
            &tokens.shapes,
        );
        if let iced::Background::Color(bg) = icon_styling.background {
            test_contrast_scenario("Icon Strategy", bg, icon_styling.text_color);
        } else {
            // Icon buttons have transparent background, so test against surface
            test_contrast_scenario(
                "Icon Strategy",
                dark_colors.surface,
                icon_styling.text_color,
            );
        }

        // Outlined button strategy
        let outlined_strategy = crate::styling::material::components::button_style::variants::outlined::OutlinedButtonStrategy;
        let outlined_styling = outlined_strategy.get_styling(
            crate::styling::material::components::button_style::strategy::ButtonState::Default,
            &tokens,
            &dark_colors,
            &tokens.elevation,
            &tokens.shapes,
        );
        if let iced::Background::Color(bg) = outlined_styling.background {
            test_contrast_scenario("Outlined Strategy", bg, outlined_styling.text_color);
        } else {
            // Outlined buttons have transparent background, so test against surface
            test_contrast_scenario(
                "Outlined Strategy",
                dark_colors.surface,
                outlined_styling.text_color,
            );
        }

        // Check if any strategies fail WCAG AA
        println!("\n🔍 ANALYZING POTENTIAL ISSUES:");
        check_potential_contrast_issues(&dark_colors);
    }

    /// Validate that all Material Design color roles meet contrast requirements
    #[test]
    fn test_material_color_role_contrast() {
        println!("=== MATERIAL COLOR ROLE CONTRAST VALIDATION ===");

        let palette = MaterialPalette::default();
        let dark_colors = MaterialColors::dark(&palette);

        // Test all primary color combinations
        test_contrast_scenario(
            "Primary base/on_base",
            dark_colors.primary.base,
            dark_colors.primary.on_base,
        );
        test_contrast_scenario(
            "Primary container/on_container",
            dark_colors.primary.container,
            dark_colors.primary.on_container,
        );

        // Test all secondary combinations
        test_contrast_scenario(
            "Secondary base/on_base",
            dark_colors.secondary.base,
            dark_colors.secondary.on_base,
        );
        test_contrast_scenario(
            "Secondary container/on_container",
            dark_colors.secondary.container,
            dark_colors.secondary.on_container,
        );

        // Test tertiary combinations
        test_contrast_scenario(
            "Tertiary base/on_base",
            dark_colors.tertiary.base,
            dark_colors.tertiary.on_base,
        );
        test_contrast_scenario(
            "Tertiary container/on_container",
            dark_colors.tertiary.container,
            dark_colors.tertiary.on_container,
        );

        // Test error combinations
        test_contrast_scenario(
            "Error base/on_base",
            dark_colors.error.base,
            dark_colors.error.on_base,
        );
        test_contrast_scenario(
            "Error container/on_container",
            dark_colors.error.container,
            dark_colors.error.on_container,
        );

        // Test surface combinations
        test_contrast_scenario(
            "Surface/on_surface",
            dark_colors.surface,
            dark_colors.on_surface,
        );
        test_contrast_scenario(
            "Surface_variant/on_surface_variant",
            dark_colors.surface_variant,
            dark_colors.on_surface_variant,
        );
        test_contrast_scenario(
            "Background/on_background",
            dark_colors.background,
            dark_colors.on_background,
        );
    }

    /// Helper function to test a contrast scenario and provide detailed feedback
    fn test_contrast_scenario(name: &str, background: Color, foreground: Color) {
        let contrast = ColorUtils::contrast_ratio(foreground, background);
        let aa_normal = contrast >= 4.5;
        let aa_large = contrast >= 3.0;
        let aaa = contrast >= 7.0;

        let status = if aaa {
            "AAA ✅"
        } else if aa_normal {
            "AA ✅ "
        } else if aa_large {
            "AA-L ⚠️"
        } else {
            "FAIL ❌"
        };

        println!("{name:35}: {contrast:5.2}:1 [{status}]");

        // Add color information for debugging
        if background != Color::TRANSPARENT {
            println!(
                "  📋 BG: rgb({:3.0}, {:3.0}, {:3.0}) | FG: rgb({:3.0}, {:3.0}, {:3.0})",
                background.r * 255.0,
                background.g * 255.0,
                background.b * 255.0,
                foreground.r * 255.0,
                foreground.g * 255.0,
                foreground.b * 255.0
            );
        }

        if !aa_normal {
            println!("  ⚠️  CONTRAST ISSUE DETECTED!");
            suggest_contrast_fix(background, foreground, contrast);
        }
        println!();
    }

    /// Suggest a contrast fix for failing combinations
    fn suggest_contrast_fix(background: Color, _current_fg: Color, current_contrast: f32) {
        if background == Color::TRANSPARENT {
            return;
        }

        let bg_luminance = ColorUtils::luminance(background);

        if bg_luminance > 0.5 {
            // Light background - suggest dark foreground
            let suggested = Color::from_rgb(0.1, 0.1, 0.1);
            let new_contrast = ColorUtils::contrast_ratio(suggested, background);
            println!(
                "  💡 Suggested: Dark foreground rgb(26, 26, 26) - {new_contrast:.2}:1 contrast"
            );
        } else {
            // Dark background - suggest light foreground
            let suggested = Color::from_rgb(0.9, 0.9, 0.9);
            let new_contrast = ColorUtils::contrast_ratio(suggested, background);
            println!(
                "  💡 Suggested: Light foreground rgb(230, 230, 230) - {new_contrast:.2}:1 contrast"
            );
        }

        println!(
            "  📊 Current: {current_contrast:.2}:1 | Target: 4.5:1 minimum"
        );
    }

    /// Check for potential contrast issues in the color scheme
    fn check_potential_contrast_issues(colors: &MaterialColors) {
        let mut issues = Vec::with_capacity(10); // Pre-allocate for typical number of contrast checks

        // Check if primary color on surface meets contrast
        let primary_on_surface = ColorUtils::contrast_ratio(colors.primary.base, colors.surface);
        if primary_on_surface < 4.5 {
            issues.push(format!("Primary on surface: {primary_on_surface:.2}:1"));
        }

        // Check surface variant combinations
        let surface_variant_contrast =
            ColorUtils::contrast_ratio(colors.on_surface_variant, colors.surface_variant);
        if surface_variant_contrast < 4.5 {
            issues.push(format!(
                "Surface variant text: {surface_variant_contrast:.2}:1"
            ));
        }

        if issues.is_empty() {
            println!("✅ No major contrast issues detected in button strategies!");
        } else {
            println!("❌ Potential issues found:");
            for issue in issues {
                println!("   • {issue}");
            }
        }
    }

    /// Test that contrast utilities work correctly
    #[test]
    fn test_contrast_utility_functions() {
        // Test with known values
        let white = Color::WHITE;
        let black = Color::BLACK;

        // Black and white should have ~21:1 contrast
        let max_contrast = ColorUtils::contrast_ratio(black, white);
        assert!(
            max_contrast > 20.0,
            "Max contrast should be ~21:1, got {max_contrast:.2}"
        );

        // Same colors should have 1:1 contrast
        let same_contrast = ColorUtils::contrast_ratio(white, white);
        assert!(
            (same_contrast - 1.0).abs() < 0.1,
            "Same color contrast should be ~1:1, got {same_contrast:.2}"
        );

        // Test WCAG standards
        let gray_bg = Color::from_rgb(0.5, 0.5, 0.5);
        let black_text = Color::BLACK;
        let white_text = Color::WHITE;

        let black_on_gray = ColorUtils::contrast_ratio(black_text, gray_bg);
        let white_on_gray = ColorUtils::contrast_ratio(white_text, gray_bg);

        println!("Black on gray: {black_on_gray:.2}:1");
        println!("White on gray: {white_on_gray:.2}:1");

        // At least one should pass WCAG AA
        assert!(
            black_on_gray >= 4.5 || white_on_gray >= 4.5,
            "At least one combination should meet WCAG AA standards"
        );
    }
}
