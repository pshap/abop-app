//! Essential tests for Material Design token system
//!
//! This test suite validates core Material Design token functionality.

#[cfg(test)]
mod tests {
    use abop_gui::styling::material::{
        MaterialStates, MaterialTokens, SemanticColors,
        elevation::ElevationLevel,
        themes::ThemeMode,
    };
    use iced::Color;

    #[test]
    fn test_material_tokens_creation() {
        let tokens = MaterialTokens::new();
        
        // Test basic token access
        let colors = tokens.colors();
        assert!((0..=255).contains(&((colors.primary.base.r * 255.0) as u8)));
        
        let states = tokens.states();
        assert_eq!(states.opacity.hover, 0.08);
    }

    #[test]
    fn test_elevation_helpers() {
        let tokens = MaterialTokens::new();

        let card_elevation = tokens.card_elevation();
        assert_eq!(card_elevation.level(), ElevationLevel::Level1);

        let fab_elevation = tokens.fab_elevation();
        assert_eq!(fab_elevation.level(), ElevationLevel::Level3);
    }

    #[test]
    fn test_semantic_colors() {
        let semantic = SemanticColors::default();

        // Test that semantic colors are properly defined
        assert!(semantic.primary != Color::BLACK);
        assert!(semantic.secondary != Color::BLACK);
        assert!(semantic.success != Color::BLACK);
        assert!(semantic.warning != Color::BLACK);
        assert!(semantic.error != Color::BLACK);
        assert!(semantic.info != Color::BLACK);
    }

    #[test]
    fn test_theme_modes() {
        let light_tokens = MaterialTokens::light();
        let dark_tokens = MaterialTokens::dark();

        assert!(!light_tokens.is_dark_theme());
        assert!(dark_tokens.is_dark_theme());
    }

    #[test]
    fn test_material_states() {
        let states = MaterialStates::new();

        // Test opacity values
        assert_eq!(states.opacity.disabled, 0.38);
        assert_eq!(states.opacity.hover, 0.08);
        assert_eq!(states.opacity.focus, 0.12);
        assert_eq!(states.opacity.pressed, 0.12);
        assert_eq!(states.opacity.dragged, 0.16);

        // Test overlay colors
        assert_eq!(states.overlay.hover, Color::from_rgba(0.0, 0.0, 0.0, 0.08));
        assert_eq!(states.overlay.focus, Color::from_rgba(0.0, 0.0, 0.0, 0.12));
    }
}
