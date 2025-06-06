//! Test to debug color generation issues

use abop_gui::styling::color_utils::ColorUtils;
use abop_gui::styling::material::colors::{MaterialColors, MaterialPalette};
use iced::Color;

#[test]
fn debug_color_generation() {
    println!("\n=== Color Generation Debug ===");

    // Test the default MaterialPalette
    let palette = MaterialPalette::default();
    println!("Default palette primary tones (first 12):");
    for i in 0..12 {
        let value = palette.primary.tones[i];
        println!("  Index {i}: {value:?}");
    }

    // Test MaterialColors from default palette
    let colors = MaterialColors::default();
    println!("\nMaterialColors primary colors:");
    println!("  primary.base: {:?}", colors.primary.base);
    println!("  primary.container: {:?}", colors.primary.container);
    println!("  on_primary: {:?}", colors.on_primary);
    println!("  on_primary_container: {:?}", colors.primary.on_container);

    // Test with a known seed color
    let seed = Color::from_rgb(0.404, 0.314, 0.643); // Material Design baseline
    let palette_from_seed = MaterialPalette::from_seed(seed);
    println!("\nPalette from seed {seed:?}:");
    println!("Primary tones (first 12):");
    for i in 0..12 {
        let value = palette_from_seed.primary.tones[i];
        println!("  Index {i}: {value:?}");
    }

    let colors_from_seed = MaterialColors::light(&palette_from_seed);
    println!("\nMaterialColors from seed (light theme):");
    println!("  primary.base: {:?}", colors_from_seed.primary.base);
    println!(
        "  primary.container: {:?}",
        colors_from_seed.primary.container
    );
    println!("  on_primary: {:?}", colors_from_seed.on_primary);
    // Check contrast ratios to verify if we have the right colors
    println!("\nContrast checks:");
    let white = Color::WHITE;
    let base_contrast = ColorUtils::contrast_ratio(colors_from_seed.primary.base, white);
    let container_contrast = ColorUtils::contrast_ratio(colors_from_seed.primary.container, white);
    println!("  primary.base vs white: {base_contrast:.3}");
    println!("  primary.container vs white: {container_contrast:.3}");

    // Expected: base should be dark (high contrast with white), container should be light (low contrast)
    println!("\nExpected: base contrast > 3.0, container contrast < 3.0");
}
