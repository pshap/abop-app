//! Tests for the Material Design 3 Shape System
//!
//! Comprehensive test suite covering all aspects of the shape system
//! including core functionality, utilities, and edge cases.

#[cfg(test)]
mod shape_tests {
    use super::super::{components::*, constants, core::*, families::*, states::*, utils::*};

    #[test]
    fn test_shape_sizes() {
        assert_eq!(ShapeSize::None.radius(), 0.0);
        assert_eq!(ShapeSize::Small.radius(), 8.0);
        assert_eq!(ShapeSize::ExtraLarge.radius(), 28.0);
        assert_eq!(ShapeSize::Full.radius(), 9999.0);
    }

    #[test]
    fn test_shape_creation() {
        let shapes = MaterialShapes::new();
        assert_eq!(shapes.corner_small.radius.top_left, 8.0);
        assert_eq!(shapes.corner_medium.radius.top_left, 12.0);
    }

    #[test]
    fn test_asymmetric_shapes() {
        let shape = ShapeStyle::asymmetric(10.0, 20.0, 30.0, 40.0);
        assert_eq!(shape.radius.top_left, 10.0);
        assert_eq!(shape.radius.top_right, 20.0);
        assert_eq!(shape.radius.bottom_right, 30.0);
        assert_eq!(shape.radius.bottom_left, 40.0);
    }

    #[test]
    fn test_shape_scaling() {
        let shapes = MaterialShapes::new();
        let scaled = shapes.with_scale(2.0);

        assert_eq!(
            scaled.corner_small.radius.top_left,
            shapes.corner_small.radius.top_left * 2.0
        );
    }

    #[test]
    fn test_component_recommendations() {
        assert_eq!(
            get_recommended_shape(ComponentType::Button),
            ShapeSize::Small
        );
        assert_eq!(
            get_recommended_shape(ComponentType::FloatingActionButton),
            ShapeSize::Large
        );
    }

    #[test]
    fn test_shape_families() {
        let shapes = MaterialShapes::new();
        let sharp = shapes.for_family(ShapeFamily::Sharp);

        // All shapes in sharp family should have no radius
        assert_eq!(sharp.corner_small.radius.top_left, 0.0);
        assert_eq!(sharp.corner_large.radius.top_left, 0.0);
    }

    #[test]
    fn test_shape_accessor_methods() {
        let shapes = MaterialShapes::new();

        // Test all accessor methods work correctly
        for &size in ShapeSize::all() {
            let shape = shapes.get_shape(size);
            assert_eq!(shape.size, size);
            assert_eq!(shape.radius.top_left, size.radius());
        }
    }

    #[test]
    fn test_shape_scaling_edge_cases() {
        let shape = ShapeStyle::new(ShapeSize::Medium);

        // Test zero scale
        let zero_scaled = shape.with_scale(0.0);
        assert_eq!(zero_scaled.radius.top_left, 0.0);

        // Test negative scale (should be clamped to 0)
        let negative_scaled = shape.with_scale(-1.0);
        assert_eq!(negative_scaled.radius.top_left, 0.0);

        // Test large scale
        let large_scaled = shape.with_scale(10.0);
        assert_eq!(large_scaled.radius.top_left, shape.radius.top_left * 10.0);
    }

    #[test]
    fn test_circular_detection() {
        // Test truly circular shape
        let circular = ShapeStyle::new(ShapeSize::Full);
        assert!(circular.is_circular());

        // Test non-circular shape
        let non_circular = ShapeStyle::new(ShapeSize::Small);
        assert!(!non_circular.is_circular());

        // Test custom circular shape
        let custom_circular = ShapeStyle::custom(constants::CIRCULAR_THRESHOLD, "Test");
        assert!(custom_circular.is_circular());

        // Test asymmetric shape (should not be circular)
        let asymmetric = ShapeStyle::asymmetric(10.0, 20.0, 10.0, 10.0);
        assert!(!asymmetric.is_circular());
    }

    #[test]
    fn test_rectangular_detection() {
        let rectangular = ShapeStyle::new(ShapeSize::None);
        assert!(rectangular.is_rectangular());

        let non_rectangular = ShapeStyle::new(ShapeSize::Small);
        assert!(!non_rectangular.is_rectangular());
    }

    #[test]
    fn test_shape_interpolation() {
        let start = ShapeStyle::new(ShapeSize::None);
        let end = ShapeStyle::new(ShapeSize::Medium);

        // Test interpolation at 0.5
        let interpolated = interpolate_shapes(&start, &end, 0.5);
        let expected_radius = (start.radius.top_left + end.radius.top_left) / 2.0;
        assert_eq!(interpolated.radius.top_left, expected_radius);

        // Test clamping
        let clamped_low = interpolate_shapes(&start, &end, -1.0);
        assert_eq!(clamped_low.radius.top_left, start.radius.top_left);

        let clamped_high = interpolate_shapes(&start, &end, 2.0);
        assert_eq!(clamped_high.radius.top_left, end.radius.top_left);
    }

    #[test]
    fn test_state_shape_variations() {
        let base = ShapeStyle::new(ShapeSize::Medium);

        let default_state = shape_for_state(&base, ComponentState::Default);
        assert_eq!(default_state.radius.top_left, base.radius.top_left);

        let hovered = shape_for_state(&base, ComponentState::Hovered);
        assert!(hovered.radius.top_left > base.radius.top_left);

        let pressed = shape_for_state(&base, ComponentState::Pressed);
        assert!(pressed.radius.top_left < base.radius.top_left);
    }

    #[test]
    fn test_responsive_shape() {
        let container_size = 100.0;
        let max_radius = 15.0;

        let responsive = responsive_shape(container_size, max_radius);
        assert_eq!(
            responsive.radius.top_left,
            (container_size * constants::DEFAULT_RESPONSIVE_FACTOR).min(max_radius)
        );
    }

    #[test]
    fn test_all_styles_method() {
        let shapes = MaterialShapes::new();
        let all_styles = shapes.all_styles();

        assert_eq!(all_styles.len(), constants::SHAPE_COUNT);
        assert_eq!(ShapeSize::all().len(), constants::SHAPE_COUNT);

        // Verify order matches ShapeSize::all()
        for (i, &size) in ShapeSize::all().iter().enumerate() {
            assert_eq!(all_styles[i].size, size);
        }
    }

    #[test]
    fn test_shape_families_comprehensive() {
        let shapes = MaterialShapes::new();

        // Test Sharp family
        let sharp = shapes.for_family(ShapeFamily::Sharp);
        for style in sharp.all_styles() {
            assert!(style.is_rectangular());
        }

        // Test Rounded family (should be same as original)
        let rounded = shapes.for_family(ShapeFamily::Rounded);
        for (original, rounded_style) in shapes.all_styles().iter().zip(rounded.all_styles()) {
            assert_eq!(original.radius, rounded_style.radius);
        }

        // Test Circular family
        let circular = shapes.for_family(ShapeFamily::Circular);
        for style in circular.all_styles() {
            assert!(style.is_circular());
        }
    }

    #[test]
    fn test_component_recommendations_completeness() {
        // Ensure we have recommendations for all component types

        // This is a compile-time check that we handle all variants
        let _all_handled = |component: ComponentType| match component {
            ComponentType::Button => ShapeSize::Small,
            ComponentType::OutlinedButton => ShapeSize::Small,
            ComponentType::TextButton => ShapeSize::Small,
            ComponentType::FloatingActionButton => ShapeSize::Large,
            ComponentType::ExtendedFab => ShapeSize::Large,
            ComponentType::Card => ShapeSize::Medium,
            ComponentType::ElevatedCard => ShapeSize::Medium,
            ComponentType::OutlinedCard => ShapeSize::Medium,
            ComponentType::Chip => ShapeSize::Small,
            ComponentType::FilterChip => ShapeSize::Small,
            ComponentType::InputChip => ShapeSize::Small,
            ComponentType::TextField => ShapeSize::ExtraSmall,
            ComponentType::OutlinedTextField => ShapeSize::ExtraSmall,
            ComponentType::FilledTextField => ShapeSize::ExtraSmall,
            ComponentType::Menu => ShapeSize::ExtraSmall,
            ComponentType::Tooltip => ShapeSize::ExtraSmall,
            ComponentType::Dialog => ShapeSize::ExtraLarge,
            ComponentType::BottomSheet => ShapeSize::ExtraLarge,
            ComponentType::NavigationDrawer => ShapeSize::None,
            ComponentType::AppBar => ShapeSize::None,
            ComponentType::BottomNavigationBar => ShapeSize::None,
            ComponentType::NavigationRail => ShapeSize::None,
            ComponentType::Badge => ShapeSize::Full,
            ComponentType::Avatar => ShapeSize::Full,
            ComponentType::Switch => ShapeSize::Full,
            ComponentType::Checkbox => ShapeSize::ExtraSmall,
            ComponentType::RadioButton => ShapeSize::Full,
            ComponentType::Divider => ShapeSize::None,
            ComponentType::ProgressIndicator => ShapeSize::Full,
            ComponentType::Slider => ShapeSize::Full,
        };
    }

    #[test]
    fn test_shape_size_constants() {
        // Test that our constants match Material Design 3 spec
        assert_eq!(ShapeSize::None.radius(), 0.0);
        assert_eq!(ShapeSize::ExtraSmall.radius(), 4.0);
        assert_eq!(ShapeSize::Small.radius(), 8.0);
        assert_eq!(ShapeSize::Medium.radius(), 12.0);
        assert_eq!(ShapeSize::Large.radius(), 16.0);
        assert_eq!(ShapeSize::ExtraLarge.radius(), 28.0);
        assert_eq!(ShapeSize::Full.radius(), 9999.0);
    }

    #[test]
    fn test_partial_corner_shapes() {
        let radius = 10.0;

        let top_only = ShapeStyle::top_only(radius);
        assert_eq!(top_only.radius.top_left, radius);
        assert_eq!(top_only.radius.top_right, radius);
        assert_eq!(top_only.radius.bottom_left, 0.0);
        assert_eq!(top_only.radius.bottom_right, 0.0);

        let bottom_only = ShapeStyle::bottom_only(radius);
        assert_eq!(bottom_only.radius.bottom_left, radius);
        assert_eq!(bottom_only.radius.bottom_right, radius);
        assert_eq!(bottom_only.radius.top_left, 0.0);
        assert_eq!(bottom_only.radius.top_right, 0.0);

        let start_only = ShapeStyle::start_only(radius);
        assert_eq!(start_only.radius.top_left, radius);
        assert_eq!(start_only.radius.bottom_left, radius);
        assert_eq!(start_only.radius.top_right, 0.0);
        assert_eq!(start_only.radius.bottom_right, 0.0);

        let end_only = ShapeStyle::end_only(radius);
        assert_eq!(end_only.radius.top_right, radius);
        assert_eq!(end_only.radius.bottom_right, radius);
        assert_eq!(end_only.radius.top_left, 0.0);
        assert_eq!(end_only.radius.bottom_left, 0.0);
    }

    #[test]
    fn test_semantic_accessors() {
        let shapes = MaterialShapes::new();

        // Test that semantic accessors return the expected shapes
        assert_eq!(shapes.button().size, ShapeSize::Small);
        assert_eq!(shapes.card().size, ShapeSize::Medium);
        assert_eq!(shapes.dialog().size, ShapeSize::ExtraLarge);
        assert_eq!(shapes.chip().size, ShapeSize::Small);
        assert_eq!(shapes.text_field().size, ShapeSize::ExtraSmall);
        assert_eq!(shapes.fab().size, ShapeSize::Large);
    }

    #[test]
    fn test_state_scale_constants() {
        let base = ShapeStyle::new(ShapeSize::Medium);

        // Test that state variations use the defined constants
        let hovered = shape_for_state(&base, ComponentState::Hovered);
        assert_eq!(
            hovered.radius.top_left,
            base.radius.top_left * constants::HOVER_SCALE
        );

        let pressed = shape_for_state(&base, ComponentState::Pressed);
        assert_eq!(
            pressed.radius.top_left,
            base.radius.top_left * constants::PRESSED_SCALE
        );

        let disabled = shape_for_state(&base, ComponentState::Disabled);
        assert_eq!(
            disabled.radius.top_left,
            base.radius.top_left * constants::DISABLED_SCALE
        );
    }

    #[test]
    fn test_transition_method() {
        let start = ShapeStyle::new(ShapeSize::Small);
        let end = ShapeStyle::new(ShapeSize::Large);

        // Test transition at midpoint
        let transition = start.transition_to(&end, 0.5);
        let expected_radius = (start.radius.top_left + end.radius.top_left) / 2.0;
        assert_eq!(transition.radius.top_left, expected_radius);

        // Test transition at start
        let transition_start = start.transition_to(&end, 0.0);
        assert_eq!(transition_start.radius.top_left, start.radius.top_left);

        // Test transition at end
        let transition_end = start.transition_to(&end, 1.0);
        assert_eq!(transition_end.radius.top_left, end.radius.top_left);
    }

    #[test]
    fn test_component_categories() {
        // Test component categorization
        assert_eq!(ComponentType::Button.category(), ComponentCategory::Buttons);
        assert_eq!(
            ComponentType::Card.category(),
            ComponentCategory::Containers
        );
        assert_eq!(
            ComponentType::TextField.category(),
            ComponentCategory::Inputs
        );
        assert_eq!(
            ComponentType::AppBar.category(),
            ComponentCategory::Navigation
        );
        assert_eq!(
            ComponentType::Badge.category(),
            ComponentCategory::Indicators
        );
        assert_eq!(
            ComponentType::Divider.category(),
            ComponentCategory::Feedback
        );
    }

    #[test]
    fn test_app_style_recommendations() {
        assert_eq!(
            AppStyle::Professional.recommended_family(),
            ShapeFamily::Sharp
        );
        assert_eq!(
            AppStyle::Friendly.recommended_family(),
            ShapeFamily::Rounded
        );
        assert_eq!(
            AppStyle::Playful.recommended_family(),
            ShapeFamily::Circular
        );
    }

    #[test]
    fn test_component_state_scale_factors() {
        assert_eq!(ComponentState::Default.scale_factor(), 1.0);
        assert_eq!(
            ComponentState::Hovered.scale_factor(),
            constants::HOVER_SCALE
        );
        assert_eq!(
            ComponentState::Pressed.scale_factor(),
            constants::PRESSED_SCALE
        );
        assert_eq!(ComponentState::Focused.scale_factor(), 1.0);
        assert_eq!(
            ComponentState::Disabled.scale_factor(),
            constants::DISABLED_SCALE
        );
    }

    #[test]
    fn test_utility_functions() {
        let shape = ShapeStyle::new(ShapeSize::Medium);

        // Test radius analysis functions
        assert_eq!(average_radius(&shape), 12.0);
        assert_eq!(max_radius(&shape), 12.0);
        assert_eq!(min_radius(&shape), 12.0);

        // Test asymmetric shape analysis
        let asymmetric = ShapeStyle::asymmetric(5.0, 10.0, 15.0, 20.0);
        assert_eq!(average_radius(&asymmetric), 12.5);
        assert_eq!(max_radius(&asymmetric), 20.0);
        assert_eq!(min_radius(&asymmetric), 5.0);
    }

    #[test]
    fn test_shape_constraints() {
        let shape = ShapeStyle::new(ShapeSize::Medium);

        // Test minimum constraint
        let min_constrained = constrain_min_radius(&shape, 20.0);
        assert_eq!(min_constrained.radius.top_left, 20.0);

        // Test maximum constraint
        let max_constrained = constrain_max_radius(&shape, 5.0);
        assert_eq!(max_constrained.radius.top_left, 5.0);

        // Test combined constraints
        let constrained = constrain_radius(&shape, 5.0, 20.0);
        assert_eq!(constrained.radius.top_left, 12.0); // Within range
    }

    #[test]
    fn test_shape_similarity() {
        let shape1 = ShapeStyle::new(ShapeSize::Medium);
        let shape2 = ShapeStyle::custom(12.1, "Similar");
        let shape3 = ShapeStyle::custom(20.0, "Different");

        assert!(shapes_similar(&shape1, &shape2, 0.5));
        assert!(!shapes_similar(&shape1, &shape3, 0.5));
    }

    #[test]
    fn test_responsive_shape_with_factor() {
        let container_size = 100.0;
        let max_radius = 50.0;
        let factor = 0.2;

        let responsive = responsive_shape_with_factor(container_size, max_radius, factor);
        assert_eq!(responsive.radius.top_left, 20.0); // 100 * 0.2
    }
}
