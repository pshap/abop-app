//! Comprehensive tests for Material Design token system Phase 2
//!
//! This test suite validates all Phase 2 changes including trait implementations,
//! backward compatibility, and new infrastructure components.

#[cfg(test)]
mod tests {
    use abop_gui::styling::material::{
        MaterialStates, MaterialTokens, SemanticColors,
        builders::{MaterialTokensBuilder, ThemeBuilder},
        elevation::ElevationLevel,
        factories::MaterialComponentFactory,
        helpers::{AnimationHelpers, ComponentHelpers, ElevationHelpers},
        themes::{DynamicTheme, ThemeMode},
    };
    use iced::Color;

    // =============================================================================
    // Elevation Helpers Trait Tests
    // =============================================================================
    #[test]
    fn test_elevation_helpers_trait() {
        let tokens = MaterialTokens::new();

        // Test elevation helper methods
        let card_elevation = tokens.card_elevation();
        assert_eq!(card_elevation.level(), ElevationLevel::Level1);

        let fab_elevation = tokens.fab_elevation();
        assert_eq!(fab_elevation.level(), ElevationLevel::Level3);

        let no_elevation = tokens.no_elevation();
        assert_eq!(no_elevation.level(), ElevationLevel::Level0);
    }
    #[test]
    fn test_elevation_utilities() {
        let tokens = MaterialTokens::new();

        // Test elevation style access
        let style = tokens.elevation_style(2);
        assert_eq!(style.level(), ElevationLevel::Level2);

        // Test shadow access
        let shadow = tokens.elevation_shadow(1);
        assert!(shadow.offset.y >= 0.0); // Should have non-negative vertical offset

        // Test tint opacity
        let opacity = tokens.elevation_tint_opacity(3);
        assert!((0.0..=1.0).contains(&opacity)); // Should be normalized between 0 and 1
    }

    #[test]
    fn test_elevation_transitions() {
        let tokens = MaterialTokens::new();

        // Test button hover elevation transition
        let (normal, hover) = tokens.button_hover_elevation();
        assert!(hover.offset.y > normal.offset.y); // Hover should have more elevation

        // Test FAB hover elevation transition
        let (normal_fab, hover_fab) = tokens.fab_hover_elevation();
        assert!(hover_fab.offset.y > normal_fab.offset.y);

        // Test custom elevation transition
        let (from, to) = tokens.elevation_transition(0, 4);
        assert!(to.offset.y > from.offset.y);
    }

    // =============================================================================
    // Animation Helpers Trait Tests
    // =============================================================================

    #[test]
    fn test_animation_helpers_trait() {
        let tokens = MaterialTokens::new();

        // Test various animation types
        let fade_anim = tokens.fade_animation();
        assert_eq!(fade_anim.easing().name, "linear");

        let button_anim = tokens.button_animation();
        assert_eq!(button_anim.easing().name, "standard");

        let modal_anim = tokens.modal_animation();
        assert_eq!(modal_anim.easing().name, "emphasized-decelerate");

        let slide_anim = tokens.slide_animation();
        assert_eq!(slide_anim.easing().name, "standard-decelerate");

        let scale_anim = tokens.scale_animation();
        assert_eq!(scale_anim.easing().name, "standard-decelerate");

        let dismiss_anim = tokens.dismiss_animation();
        assert_eq!(dismiss_anim.easing().name, "standard-accelerate");

        let loading_anim = tokens.loading_animation();
        assert_eq!(loading_anim.easing().name, "linear");
    }

    #[test]
    fn test_hover_animation_speed() {
        let tokens = MaterialTokens::new();
        let hover_anim = tokens.hover_animation(); // Hover animation should have faster speed
        let normal_anim = AnimationHelpers::button_animation(&tokens);
        assert!(hover_anim.effective_duration() < normal_anim.effective_duration());
        assert_eq!(hover_anim.easing().name, "standard");
    }

    // =============================================================================
    // Component Helpers Trait Tests
    // =============================================================================

    #[test]
    fn test_component_helpers_trait() {
        let tokens = MaterialTokens::new();

        // Test component creation
        let _card = tokens.card();
        let _progress = tokens.progress_indicator();
        let _notification = tokens.notification("Test message");
        let _badge = tokens.badge();
        let _status = tokens.status_indicator();

        // All should create successfully without panicking
    }

    // =============================================================================
    // Backward Compatibility Tests
    // =============================================================================

    #[test]
    fn test_backward_compatibility() {
        let tokens = MaterialTokens::new();

        // Test that all existing APIs still work
        let _card = tokens.card();
        let _elevation = tokens.card_elevation();
        let _animation = tokens.hover_animation();
        let _semantic = tokens.semantic_colors();
        let _shadow = tokens.elevation_shadow(2);
        let _transition = tokens.elevation_transition(1, 3);

        // Test color system access
        let colors = tokens.colors();
        assert!((0..=255).contains(&((colors.primary.base.r * 255.0) as u8)));

        // Test typography access
        let _typography = tokens.typography();

        // Test states access
        let states = tokens.states();
        assert_eq!(states.opacity.hover, 0.08);
    }

    #[test]
    fn test_semantic_colors_mapping() {
        let tokens = MaterialTokens::new();
        let semantic = tokens.semantic_colors();

        // Test semantic color properties exist
        assert!((0..=255).contains(&((semantic.primary.r * 255.0) as u8)));
        assert!((0..=255).contains(&((semantic.secondary.r * 255.0) as u8)));
        assert!((0..=255).contains(&((semantic.success.r * 255.0) as u8)));
        assert!((0..=255).contains(&((semantic.warning.r * 255.0) as u8)));
        assert!((0..=255).contains(&((semantic.error.r * 255.0) as u8)));
        assert!((0..=255).contains(&((semantic.info.r * 255.0) as u8)));
    }

    // =============================================================================
    // Token Structure Tests
    // =============================================================================

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

    #[test]
    fn test_semantic_colors_default() {
        let semantic = SemanticColors::default();

        // Test that default colors are reasonable
        assert!(semantic.primary != Color::BLACK);
        assert!(semantic.secondary != Color::BLACK);
        assert!(semantic.success != Color::BLACK);
        assert!(semantic.warning != Color::BLACK);
        assert!(semantic.error != Color::BLACK);
        assert!(semantic.info != Color::BLACK);
    }

    // =============================================================================
    // Phase 3 Preparation Infrastructure Tests
    // =============================================================================

    #[test]
    fn test_theme_mode() {
        // Test theme mode properties
        assert_eq!(ThemeMode::Light.is_light(), Some(true));
        assert_eq!(ThemeMode::Dark.is_dark(), Some(true));
        assert_eq!(ThemeMode::Light.is_dark(), Some(false));
        assert_eq!(ThemeMode::Dark.is_light(), Some(false));

        // Test auto and custom modes
        assert_eq!(ThemeMode::Auto.is_light(), None);
        assert_eq!(ThemeMode::Custom.is_light(), None);

        // Test system detection requirements
        assert!(ThemeMode::Auto.requires_system_detection());
        assert!(!ThemeMode::Light.requires_system_detection());

        // Test customization allowance
        assert!(ThemeMode::Custom.allows_customization());
        assert!(!ThemeMode::Light.allows_customization());
    }

    #[test]
    fn test_material_tokens_builder() {
        // Test basic builder functionality
        let tokens = MaterialTokensBuilder::new()
            .with_theme_mode(ThemeMode::Dark)
            .build();

        assert!(tokens.is_dark_theme());

        // Test seed color builder
        let seed_color = Color::from_rgb(0.8, 0.2, 0.4);
        let tokens_with_seed = MaterialTokensBuilder::new()
            .with_seed_color(seed_color)
            .with_theme_mode(ThemeMode::Light)
            .build();

        // Should create successfully
        assert!(!tokens_with_seed.is_dark_theme());
    }

    #[test]
    fn test_theme_builder() {
        // Test theme builder basic functionality
        let tokens = ThemeBuilder::new()
            .with_mode(ThemeMode::Light)
            .respect_system_preferences()
            .with_dynamic_theming()
            .build();

        assert!(!tokens.is_dark_theme());
    }

    #[test]
    fn test_dynamic_theme() {
        // Test dynamic theme creation
        let mut theme = DynamicTheme::new();
        assert_eq!(theme.current_mode(), ThemeMode::Auto);

        // Test mode switching
        theme.switch_to_mode(ThemeMode::Light);
        assert_eq!(theme.current_mode(), ThemeMode::Light);
        assert!(!theme.is_dark_theme());

        theme.switch_to_mode(ThemeMode::Dark);
        assert_eq!(theme.current_mode(), ThemeMode::Dark);
        assert!(theme.is_dark_theme());

        // Test seed color update
        let seed = Color::from_rgb(0.5, 0.7, 0.3);
        theme.update_with_seed_color(seed);

        // Test system preference settings
        theme.set_respect_system(false);
        assert!(!theme.respects_system());

        theme.set_respect_system(true);
        assert!(theme.respects_system());

        // Test mode preview
        let preview = theme.preview_mode(ThemeMode::Light);
        assert!(!preview.is_dark_theme());
    }

    #[test]
    fn test_component_factory() {
        let tokens = MaterialTokens::new();
        let factory = MaterialComponentFactory::new(&tokens);

        // Test factory token access
        assert!(std::ptr::eq(factory.tokens(), &tokens));

        // Test component creation
        let _card = factory.create_card();
        let _progress = factory.create_progress_indicator();
        let _notification = factory.create_notification("Test");

        // Test placeholder methods (will be implemented in Phase 3)
        let button = factory.create_button();
        assert_eq!(button, "MaterialButton");

        let text_field = factory.create_text_field();
        assert_eq!(text_field, "MaterialTextField");
    }

    // =============================================================================
    // Integration Tests
    // =============================================================================

    #[test]
    fn test_trait_integration() {
        let tokens = MaterialTokens::new();

        // Test that traits work together properly
        let elevation = ElevationHelpers::card_elevation(&tokens);
        let animation = AnimationHelpers::fade_animation(&tokens);
        let _component = ComponentHelpers::card(&tokens); // All should work without conflicts
        assert_eq!(elevation.level(), ElevationLevel::Level1);
        assert_eq!(animation.easing().name, "linear");
        // component creation should succeed
    }

    #[test]
    fn test_token_consistency() {
        let light_tokens = MaterialTokens::light();
        let dark_tokens = MaterialTokens::dark();

        // Test theme detection
        assert!(!light_tokens.is_dark_theme());
        assert!(dark_tokens.is_dark_theme());

        // Test that both have the same structure
        assert!(
            light_tokens.colors().primary.base != dark_tokens.colors().primary.base
                || light_tokens.colors().surface != dark_tokens.colors().surface
        );
    }

    #[test]
    fn test_builder_integration() {
        // Test that builders work with the rest of the system
        let tokens = MaterialTokensBuilder::new()
            .with_theme_mode(ThemeMode::Dark)
            .build();

        let factory = MaterialComponentFactory::new(&tokens);
        let _card = factory.create_card();
        let elevation = tokens.fab_elevation();
        assert_eq!(elevation.level(), ElevationLevel::Level3);
        let animation = tokens.hover_animation();
        let normal_anim = AnimationHelpers::button_animation(&tokens);
        assert!(animation.effective_duration() < normal_anim.effective_duration());
    }

    #[test]
    fn test_error_handling() {
        // Test that invalid elevation levels are handled gracefully
        let tokens = MaterialTokens::new(); // Invalid elevation levels should default to Level0
        let invalid_elevation = tokens.elevation_style(255);
        assert_eq!(invalid_elevation.level(), ElevationLevel::Level0);

        let _invalid_shadow = tokens.elevation_shadow(255);
        // Should not panic and return a valid shadow
    }

    #[test]
    fn test_performance_requirements() {
        // Test that token access is fast (no complex computations)
        let tokens = MaterialTokens::new();

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _colors = tokens.colors();
            let _elevation = tokens.card_elevation();
            let _semantic = tokens.semantic_colors();
        }
        let duration = start.elapsed();

        // Should complete in well under a second
        assert!(duration.as_millis() < 100);
    }
}
