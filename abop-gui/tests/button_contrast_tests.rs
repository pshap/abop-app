//! Tests for button color contrast

use iced::Color;

use abop_gui::styling::color_utils::ColorUtils;
use abop_gui::styling::material::components::button_style::strategy::{
    ButtonState, ButtonStyleStrategy,
};
use abop_gui::styling::material::components::button_style::variants::{
    elevated::ElevatedButtonStrategy, filled::FilledButtonStrategy,
    filled_tonal::FilledTonalButtonStrategy, icon::IconButtonStrategy,
    outlined::OutlinedButtonStrategy, text::TextButtonStrategy,
};
use abop_gui::styling::material::{MaterialColors, MaterialPalette, MaterialTokens};

/// Test button styling to ensure proper contrast ratios
/// WCAG 2.1 AA requires:
/// - 4.5:1 for normal text
/// - 3:1 for large text and UI components
#[test]
fn test_filled_button_contrast() {
    let tokens = MaterialTokens::default();
    println!(
        "MaterialTokens primary.base: {:?}",
        tokens.colors.primary.base
    );

    // Compare with direct MaterialColors creation
    let direct_colors = MaterialColors::default();
    println!(
        "Direct MaterialColors primary.base: {:?}",
        direct_colors.primary.base
    );

    // Compare with MaterialColors::dark
    let dark_colors = MaterialColors::dark(&MaterialPalette::default());
    println!(
        "Dark MaterialColors primary.base: {:?}",
        dark_colors.primary.base
    );

    let strategy = FilledButtonStrategy;

    // Test all states
    for state in [
        ButtonState::Default,
        ButtonState::Hovered,
        ButtonState::Pressed,
        ButtonState::Focused,
        ButtonState::Disabled,
    ] {
        let styling = strategy.get_styling(
            state,
            &tokens,
            &tokens.colors,
            &tokens.elevation,
            &tokens.shapes,
        );

        // Extract background color (unwrap from Background enum)
        let bg_color = match styling.background {
            iced::Background::Color(color) => color,
            _ => Color::TRANSPARENT,
        }; // Check text contrast
        let text_contrast = ColorUtils::contrast_ratio(bg_color, styling.text_color);
        println!(
            "State {:?}: bg_color={:?}, text_color={:?}, text_contrast={}",
            state, bg_color, styling.text_color, text_contrast
        );
        assert!(
            text_contrast >= 3.0,
            "Filled button text contrast ratio below 3.0 in state {state:?}: {text_contrast}"
        ); // Check icon contrast - icon should use a different color for better visibility
        if let Some(icon_color) = styling.icon_color {
            let icon_contrast = ColorUtils::contrast_ratio(bg_color, icon_color);
            println!(
                "State {state:?}: bg_color={bg_color:?}, icon_color={icon_color:?}, icon_contrast={icon_contrast}"
            );
            assert!(
                icon_contrast >= 3.0,
                "Filled button icon contrast ratio below 3.0 in state {state:?}: {icon_contrast}"
            );
        }
    }
}

#[test]
fn test_icon_button_contrast() {
    let tokens = MaterialTokens::default();
    let strategy = IconButtonStrategy;

    // Test all states
    for state in [
        ButtonState::Default,
        ButtonState::Hovered,
        ButtonState::Pressed,
        ButtonState::Focused,
        ButtonState::Disabled,
    ] {
        let styling = strategy.get_styling(
            state,
            &tokens,
            &tokens.colors,
            &tokens.elevation,
            &tokens.shapes,
        );

        // Extract background color (unwrap from Background enum)
        let bg_color = match styling.background {
            iced::Background::Color(color) => color,
            _ => Color::TRANSPARENT,
        };

        // If background is transparent, use the surface color as fallback
        let bg_color = if bg_color == Color::TRANSPARENT {
            tokens.colors.surface
        } else {
            bg_color
        };
        // Check icon contrast
        if let Some(icon_color) = styling.icon_color {
            let icon_contrast = ColorUtils::contrast_ratio(bg_color, icon_color);
            assert!(
                icon_contrast >= 3.0,
                "Icon button icon contrast ratio below 3.0 in state {state:?}: {icon_contrast}"
            );
        } else {
            // If icon_color is not provided, fall back to text_color
            let text_contrast = ColorUtils::contrast_ratio(bg_color, styling.text_color);
            assert!(
                text_contrast >= 3.0,
                "Icon button text contrast ratio below 3.0 in state {state:?}: {text_contrast}"
            );
        }
    }
}

#[test]
fn test_all_button_variants_contrast() {
    let tokens = MaterialTokens::default();
    let strategies: Vec<Box<dyn ButtonStyleStrategy>> = vec![
        Box::new(FilledButtonStrategy),
        Box::new(FilledTonalButtonStrategy),
        Box::new(OutlinedButtonStrategy),
        Box::new(TextButtonStrategy),
        Box::new(ElevatedButtonStrategy),
        Box::new(IconButtonStrategy),
    ];

    for strategy in strategies {
        for state in [
            ButtonState::Default,
            ButtonState::Hovered,
            ButtonState::Pressed,
            ButtonState::Focused,
            ButtonState::Disabled,
        ] {
            let styling = strategy.get_styling(
                state,
                &tokens,
                &tokens.colors,
                &tokens.elevation,
                &tokens.shapes,
            );

            // Extract background color
            let bg_color = match styling.background {
                iced::Background::Color(color) => color,
                _ => Color::TRANSPARENT,
            };

            // If background is transparent, use the surface color as fallback
            let bg_color = if bg_color == Color::TRANSPARENT {
                tokens.colors.surface
            } else {
                bg_color
            };
            // Check text contrast
            let text_contrast = ColorUtils::contrast_ratio(bg_color, styling.text_color);
            assert!(
                text_contrast >= 3.0,
                "{} button text contrast ratio below 3.0 in state {:?}: {}",
                strategy.variant_name(),
                state,
                text_contrast
            );

            // Check icon contrast if provided
            if let Some(icon_color) = styling.icon_color {
                let icon_contrast = ColorUtils::contrast_ratio(bg_color, icon_color);
                assert!(
                    icon_contrast >= 3.0,
                    "{} button icon contrast ratio below 3.0 in state {:?}: {}",
                    strategy.variant_name(),
                    state,
                    icon_contrast
                );
            }
        }
    }
}
