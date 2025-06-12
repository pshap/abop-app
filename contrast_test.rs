use iced::Color;

// Import the modules we need to test
mod styling {
    pub mod material {
        pub mod colors;
        pub mod color_utilities;
        pub mod unified_colors;
    }
}

use crate::styling::material::{
    colors::{MaterialColors, MaterialPalette},
    color_utilities::ColorUtilities,
};

fn main() {
    println!("Testing button contrast in dark theme...");
    
    // Create dark theme colors
    let palette = MaterialPalette::default();
    let dark_colors = MaterialColors::dark(&palette);
    
    // Test critical button combinations in dark theme
    test_button_contrast("Primary button", dark_colors.primary.base, dark_colors.primary.on_base);
    test_button_contrast("Surface variant button", dark_colors.surface_variant, dark_colors.on_surface_variant);
    test_button_contrast("Secondary container", dark_colors.secondary.container, dark_colors.secondary.on_container);
    test_button_contrast("Surface", dark_colors.surface, dark_colors.on_surface);
    
    // Test icon button scenarios (common issue)
    test_button_contrast("Icon on surface", dark_colors.surface, dark_colors.on_surface);
    test_button_contrast("Icon on surface variant", dark_colors.surface_variant, dark_colors.on_surface_variant);
    
    println!("\nTesting light theme for comparison...");
    let light_colors = MaterialColors::light(&palette);
    test_button_contrast("Light primary", light_colors.primary.base, light_colors.primary.on_base);
    test_button_contrast("Light surface", light_colors.surface, light_colors.on_surface);
}

fn test_button_contrast(name: &str, background: Color, foreground: Color) {
    let contrast = ColorUtilities::contrast_ratio(foreground, background);
    let aa_pass = contrast >= 4.5;
    let aaa_pass = contrast >= 7.0;
    
    let status = if aaa_pass {
        "AAA ✓"
    } else if aa_pass {
        "AA ✓"
    } else {
        "FAIL ✗"
    };
    
    println!("{}: {:.2}:1 [{}] - BG({:.2},{:.2},{:.2}) FG({:.2},{:.2},{:.2})", 
        name, contrast, status,
        background.r, background.g, background.b,
        foreground.r, foreground.g, foreground.b
    );
    
    if !aa_pass {
        println!("  ⚠️  Insufficient contrast! Need 4.5:1 minimum");
    }
}
