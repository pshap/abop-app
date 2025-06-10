//! Tests for Material Design 3 elevation system

#[cfg(test)]
mod elevation_tests {
    use crate::styling::material::colors::MaterialColors;
    use crate::styling::material::elevation::{
        Elevatable, ElevationError, ElevationLevel, ElevationStyle, MaterialElevation,
        ShadowParams,
        builder::ElevationStyleBuilder,
        constants::TINT_OPACITIES,
        context::{ColorKey, ElevationContext},
        registry::ElevationRegistry,
        shadow_calculations,
        test_utils::setup::create_custom_elevation_style,
        utils::{ComponentType, get_recommended_level},
    };
    use iced::Color;
    use std::collections::HashSet;

    #[test]
    fn test_elevation_levels() {
        assert_eq!(ElevationLevel::Level0.dp(), 0.0);
        assert_eq!(ElevationLevel::Level1.dp(), 1.0);
        assert_eq!(ElevationLevel::Level5.dp(), 12.0);
    }

    #[test]
    fn test_elevation_creation() {
        let colors = MaterialColors::default();
        let elevation = MaterialElevation::new(&colors);

        assert_eq!(elevation.level0.dp, 0.0);
        assert_eq!(elevation.level5.dp, 12.0);
    }

    #[test]
    fn test_surface_tint() {
        let style = ElevationStyle::new(
            ElevationLevel::Level2,
            Color::BLACK,
            Color::from_rgb(0.42, 0.31, 0.65),
        );

        let base_color = Color::WHITE;
        let tinted = style.apply_surface_tint(base_color, Color::from_rgb(0.42, 0.31, 0.65));

        // Tinted color should be different from base
        assert_ne!(tinted, base_color);
    }

    #[test]
    fn test_elevation_scaling() {
        let elevation = MaterialElevation::default();
        let scaled = elevation.with_scale(2.0);

        assert_eq!(
            scaled.level1.shadow.blur_radius,
            elevation.level1.shadow.blur_radius * 2.0
        );
    }

    #[test]
    fn test_component_recommendations() {
        assert_eq!(
            get_recommended_level(ComponentType::FloatingActionButton),
            ElevationLevel::Level3
        );
        assert_eq!(
            get_recommended_level(ComponentType::Surface),
            ElevationLevel::Level0
        );
    }

    #[test]
    fn test_elevated_surface_with_tint() {
        let elevation = MaterialElevation::default();
        let base_color = Color::WHITE;
        let tint_color = Color::from_rgb(0.42, 0.31, 0.65);

        let elevated = elevation.elevated_surface(base_color, ElevationLevel::Level2, tint_color);

        // Elevated surface should be different from base
        assert_ne!(elevated, base_color);

        // Level 0 should return the same color (no tint)
        let no_elevation =
            elevation.elevated_surface(base_color, ElevationLevel::Level0, tint_color);
        assert_eq!(no_elevation, base_color);
    }

    #[test]
    fn test_shadow_params_consistency() {
        // Verify shadow parameters are applied correctly
        let style = ElevationStyle::new(ElevationLevel::Level3, Color::BLACK, Color::WHITE);

        assert_eq!(style.shadow.offset.y, 3.0);
        assert_eq!(style.shadow.blur_radius, 6.0);
        assert_eq!(style.shadow.color.a, 0.15);
    }

    #[test]
    fn test_tint_opacity_consistency() {
        // Verify tint opacities match constants
        for level in ElevationLevel::all() {
            let idx = level.as_u8() as usize;
            let expected = if idx < TINT_OPACITIES.len() {
                TINT_OPACITIES[idx]
            } else {
                TINT_OPACITIES[0]
            };
            let opacity = ElevationStyle::calculate_tint_opacity(*level);
            assert_eq!(
                opacity, expected,
                "Tint opacity mismatch for level {level:?}"
            );
        }
    }

    #[test]
    fn test_with_colors_helper() {
        let shadow_color = Color::from_rgb(0.1, 0.1, 0.1);
        let tint_color = Color::from_rgb(0.9, 0.9, 0.9);

        let elevation = MaterialElevation::with_colors(shadow_color, tint_color);

        // All non-zero levels should have the correct shadow color (with opacity)
        for level in [
            ElevationLevel::Level1,
            ElevationLevel::Level2,
            ElevationLevel::Level3,
        ] {
            let style = elevation.get_level(level);
            assert_eq!(style.shadow.color.r, shadow_color.r);
            assert_eq!(style.shadow.color.g, shadow_color.g);
            assert_eq!(style.shadow.color.b, shadow_color.b);
        }
    }

    #[test]
    fn test_elevation_iterator() {
        let elevation = MaterialElevation::default();
        let levels: Vec<_> = elevation.iter().map(|(level, _)| level).collect();

        assert_eq!(levels, ElevationLevel::all());
    }

    #[test]
    fn test_elevation_styling_tuple() {
        let elevation = MaterialElevation::default();
        let base_color = Color::WHITE;
        let tint_color = Color::from_rgb(0.5, 0.5, 0.5);

        let (shadow, surface) =
            elevation.get_elevation_styling(base_color, ElevationLevel::Level2, tint_color);

        // Should match individual calls
        let style = elevation.get_level(ElevationLevel::Level2);
        assert_eq!(shadow, style.shadow);
        assert_eq!(surface, style.apply_surface_tint(base_color, tint_color));
    }

    #[test]
    fn test_custom_elevation_style() {
        let shadow_color = Color::BLACK;
        let tint_color = Color::WHITE;
        let custom_params = ShadowParams {
            offset_y: 5.0,
            blur_radius: 10.0,
            opacity: 0.2,
        };

        let style = ElevationStyle::custom(4.0, shadow_color, tint_color, Some(custom_params));

        assert_eq!(style.dp, 4.0);
        assert_eq!(style.shadow.offset.y, 5.0);
        assert_eq!(style.shadow.blur_radius, 10.0);
        assert_eq!(style.shadow.color.a, 0.2);
    }

    #[test]
    fn test_custom_tint_opacity_interpolation() {
        // Test exact matches for level 0 only (which should be 0)
        assert_eq!(
            shadow_calculations::calculate_custom_tint_opacity(0.0),
            TINT_OPACITIES[0]
        );

        // Test that values are approximately correct for standard levels
        let tint_1dp = shadow_calculations::calculate_custom_tint_opacity(1.0);
        let tint_12dp = shadow_calculations::calculate_custom_tint_opacity(12.0);

        // Should be close to the expected values but not necessarily exact
        assert!(tint_1dp > 0.0 && tint_1dp <= 0.05);
        assert!((tint_12dp - TINT_OPACITIES[5]).abs() < 0.1);

        // Test interpolation - should be monotonically increasing
        let tint_2dp = shadow_calculations::calculate_custom_tint_opacity(2.0);
        let tint_6dp = shadow_calculations::calculate_custom_tint_opacity(6.0);
        assert!(tint_1dp < tint_2dp);
        assert!(tint_2dp < tint_6dp);
        assert!(tint_6dp < tint_12dp);
    }

    #[test]
    fn test_elevation_descriptions() {
        for level in ElevationLevel::all() {
            let style = ElevationStyle::new(*level, Color::BLACK, Color::WHITE);
            let description = style.description();
            assert!(!description.is_empty());
        }
    }

    #[test]
    fn test_elevation_display() {
        let level = ElevationLevel::Level3;
        let display = format!("{level}");
        assert!(display.contains("Level 3"));
        assert!(display.contains("6dp"));
    }

    #[test]
    fn test_structure_comparison() {
        let elevation1 = MaterialElevation::default();
        let elevation2 = MaterialElevation::with_colors(
            Color::from_rgb(1.0, 0.0, 0.0),
            Color::from_rgb(0.0, 0.0, 1.0),
        );
        let elevation3 = elevation1.with_scale(2.0);

        // Same structure, different colors
        assert!(elevation1.has_same_structure(&elevation2));

        // Different structure (scaled)
        assert!(!elevation1.has_same_structure(&elevation3));
    }

    #[test]
    fn test_flat_elevation() {
        let flat_style = ElevationStyle::new(ElevationLevel::Level0, Color::BLACK, Color::WHITE);
        assert!(flat_style.is_flat());

        let raised_style = ElevationStyle::new(ElevationLevel::Level1, Color::BLACK, Color::WHITE);
        assert!(!raised_style.is_flat());
    }

    #[test]
    fn test_elevation_context() {
        let colors = MaterialColors::default();
        let mut context = ElevationContext::new(&colors);

        // Test basic elevation retrieval
        let style = context.get_elevation(ElevationLevel::Level2);
        assert_eq!(style.level(), ElevationLevel::Level2);
        assert_eq!(style.dp, 3.0);

        // Test scaling
        context.set_scale(2.0);
        let scaled_style = context.get_elevation(ElevationLevel::Level2);
        assert_eq!(
            scaled_style.shadow.blur_radius,
            style.shadow.blur_radius * 2.0
        );

        // Test cache stats
        let (cache_size, _) = context.cache_stats();
        assert!(cache_size > 0); // Should have cached items
    }

    #[test]
    fn test_elevation_registry() {
        let mut registry = ElevationRegistry::new();

        let custom_style = create_custom_elevation_style();

        // Test registration
        registry.register("custom".to_string(), custom_style.clone());
        assert!(registry.contains("custom"));
        assert_eq!(registry.get("custom"), Some(&custom_style));

        // Test listing names
        let names = registry.list_names();
        assert!(names.contains(&&"custom".to_string()));

        // Test removal
        let removed = registry.remove("custom");
        assert_eq!(removed, Some(custom_style));
        assert!(!registry.contains("custom"));
    }

    #[test]
    fn test_elevation_builder() {
        let custom_shadow = ShadowParams {
            offset_y: 4.0,
            blur_radius: 8.0,
            opacity: 0.25,
        };
        let style = ElevationStyleBuilder::new(ElevationLevel::Level3)
            .with_shadow_color(Color::from_rgb(0.2, 0.2, 0.2))
            .with_tint_color(Color::from_rgb(0.8, 0.8, 0.8))
            .with_custom_shadow(custom_shadow)
            .with_custom_tint_opacity(0.15)
            .build()
            .unwrap();

        assert_eq!(style.level(), ElevationLevel::Level3);
        assert_eq!(style.tint_opacity, 0.15);
        assert_eq!(style.shadow.offset.y, 4.0);
        assert_eq!(style.shadow.blur_radius, 8.0);
        assert_eq!(style.shadow.color.a, 0.25);
    }

    #[test]
    fn test_color_key_hashing() {
        let color1 = Color::from_rgb(1.0, 0.0, 0.0);
        let color2 = Color::from_rgb(1.0, 0.0, 0.0);
        let color3 = Color::from_rgb(0.0, 1.0, 0.0);

        let key1 = ColorKey::from(color1);
        let key2 = ColorKey::from(color2);
        let key3 = ColorKey::from(color3);

        let mut set = HashSet::new();
        set.insert(key1);
        set.insert(key2); // Should not increase size (same color)
        set.insert(key3);

        assert_eq!(set.len(), 2); // Only 2 unique colors
    }

    #[test]
    fn test_elevatable_trait() {
        use crate::styling::material::elevation::ExampleComponent;

        let component = ExampleComponent;
        assert_eq!(component.elevation_level(), ElevationLevel::Level2);
        assert_eq!(component.custom_elevation_key(), None);

        // Test with elevation context
        let colors = MaterialColors::default();
        let context = ElevationContext::new(&colors);
        let style = context.get_component_elevation(&component).unwrap();
        assert_eq!(style.level(), ElevationLevel::Level2);
    }

    #[test]
    fn test_custom_elevation_in_context() {
        let colors = MaterialColors::default();
        let mut context = ElevationContext::new(&colors);

        let custom_style = create_custom_elevation_style();

        context.register_custom_elevation("my_custom".to_string(), custom_style.clone());

        let retrieved = context.get_custom_elevation("my_custom").unwrap();
        assert_eq!(retrieved.dp, custom_style.dp);

        // Test error case
        let result = context.get_custom_elevation("nonexistent");
        assert!(result.is_err());
        matches!(result.unwrap_err(), ElevationError::CustomNotFound(_));
    }

    #[test]
    fn test_dp_and_opacity_newtypes() {
        use crate::styling::material::elevation::{Dp, Opacity};

        let dp = Dp(5.5);
        assert_eq!(dp.as_f32(), 5.5);
        assert_eq!(dp.clamp(0.0, 5.0), Dp(5.0));
        assert_eq!(format!("{dp}"), "5.50dp");

        let opacity = Opacity(0.75);
        assert_eq!(opacity.as_f32(), 0.75);
        assert_eq!(opacity.clamp(0.0, 0.5), Opacity(0.5));
        assert_eq!(format!("{opacity}"), "0.75");
    }

    #[test]
    fn test_elevation_level_enum_methods() {
        assert_eq!(ElevationLevel::all().len(), 6);

        for level in ElevationLevel::all() {
            let u8_val = level.as_u8();
            assert_eq!(ElevationLevel::from_u8(u8_val), Some(*level));
            assert!(!level.as_str().is_empty());
        }

        assert_eq!(ElevationLevel::from_u8(10), None); // Invalid level
    }

    #[test]
    fn test_error_types() {
        let err = ElevationError::InvalidLevel(10);
        assert!(err.to_string().contains("Invalid elevation level"));
        let err = ElevationError::CustomNotFound("test".to_string());
        assert!(err.to_string().contains("Custom elevation not found"));
    }
}
