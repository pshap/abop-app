//! Shape utilities for common operations in the Material Design 3 Shape System
//!
//! Provides utility functions for working with shapes including component
//! recommendations, responsive shapes, and shape interpolation.

use super::{
    components::{ComponentCategory, ComponentType},
    constants,
    core::{ShapeSize, ShapeStyle},
    families::{AppStyle, ShapeFamily},
    states::ComponentState,
};
use iced::border::Radius;

/// Get recommended shape size for component type
///
/// Returns the Material Design 3 recommended corner radius size
/// for different component types based on their role and importance.
///
/// # Arguments
/// * `component_type` - The type of component to get shape recommendation for
///
/// # Returns
/// The recommended `ShapeSize` for the component type
#[must_use]
pub const fn get_recommended_shape(component_type: ComponentType) -> ShapeSize {
    component_type.recommended_shape()
}

/// Get shape family recommendation for app style
///
/// Returns the recommended shape family based on the overall
/// application style and personality.
///
/// # Arguments
/// * `app_style` - The overall style personality of the application
///
/// # Returns
/// The recommended `ShapeFamily` for the app style
#[must_use]
pub const fn get_recommended_family(app_style: AppStyle) -> ShapeFamily {
    app_style.recommended_family()
}

/// Get recommended shape size for component category
///
/// Returns the default shape size for a component category,
/// useful for creating consistent shape systems.
///
/// # Arguments
/// * `category` - The component category to get recommendation for
///
/// # Returns
/// The recommended `ShapeSize` for the category
#[must_use]
pub const fn get_category_shape(category: ComponentCategory) -> ShapeSize {
    category.default_shape()
}

/// Create a shape that's responsive to container size
///
/// Generates a shape with radius proportional to container size,
/// clamped to a maximum value for usability.
///
/// # Arguments
/// * `container_size` - The size of the container in pixels
/// * `max_radius` - Maximum allowed radius value
///
/// # Returns
/// A `ShapeStyle` with responsive radius
#[must_use]
pub fn responsive_shape(container_size: f32, max_radius: f32) -> ShapeStyle {
    let radius = (container_size * constants::DEFAULT_RESPONSIVE_FACTOR).min(max_radius);
    ShapeStyle::custom(radius, "Responsive shape")
}

/// Create a responsive shape with custom factor
///
/// Like `responsive_shape` but allows customizing the responsiveness factor.
///
/// # Arguments
/// * `container_size` - The size of the container in pixels
/// * `max_radius` - Maximum allowed radius value
/// * `factor` - Custom responsiveness factor (0.0 to 1.0)
///
/// # Returns
/// A `ShapeStyle` with custom responsive radius
#[must_use]
pub fn responsive_shape_with_factor(
    container_size: f32,
    max_radius: f32,
    factor: f32,
) -> ShapeStyle {
    let factor = factor.clamp(0.0, 1.0);
    let radius = (container_size * factor).min(max_radius);
    ShapeStyle::custom(radius, "Custom responsive shape")
}

/// Interpolate between two shapes
///
/// Creates a smooth transition between two shape styles using linear interpolation.
/// Useful for animations and state transitions.
///
/// # Arguments
/// * `start` - The starting shape style
/// * `end` - The ending shape style
/// * `factor` - Interpolation factor (0.0 = start, 1.0 = end)
///
/// # Returns
/// A new `ShapeStyle` interpolated between start and end
#[must_use]
pub fn interpolate_shapes(start: &ShapeStyle, end: &ShapeStyle, factor: f32) -> ShapeStyle {
    let factor = factor.clamp(0.0, 1.0);
    let inv_factor = 1.0 - factor;

    ShapeStyle {
        size: start.size, // Keep the start size
        radius: Radius {
            top_left: start
                .radius
                .top_left
                .mul_add(inv_factor, end.radius.top_left * factor),
            top_right: start
                .radius
                .top_right
                .mul_add(inv_factor, end.radius.top_right * factor),
            bottom_right: start
                .radius
                .bottom_right
                .mul_add(inv_factor, end.radius.bottom_right * factor),
            bottom_left: start
                .radius
                .bottom_left
                .mul_add(inv_factor, end.radius.bottom_left * factor),
        },
        description: "Interpolated shape",
    }
}

/// Create shape variations for a component state
///
/// Applies appropriate scaling and modifications to a base shape
/// based on the component's interactive state.
///
/// # Arguments
/// * `base_shape` - The base shape style to modify
/// * `state` - The current component state
///
/// # Returns
/// A modified `ShapeStyle` appropriate for the state
#[must_use]
pub fn shape_for_state(base_shape: &ShapeStyle, state: ComponentState) -> ShapeStyle {
    match state {
        ComponentState::Default => base_shape.clone(),
        ComponentState::Hovered => base_shape.with_scale(constants::HOVER_SCALE),
        ComponentState::Pressed => base_shape.with_scale(constants::PRESSED_SCALE),
        ComponentState::Focused => base_shape.clone(),
        ComponentState::Disabled => base_shape.with_scale(constants::DISABLED_SCALE),
    }
}

/// Create shape variations using state scale factor
///
/// Alternative to `shape_for_state` that uses the state's built-in scale factor.
///
/// # Arguments
/// * `base_shape` - The base shape style to modify
/// * `state` - The current component state
///
/// # Returns
/// A modified `ShapeStyle` using the state's scale factor
#[must_use]
pub fn shape_for_state_scaled(base_shape: &ShapeStyle, state: ComponentState) -> ShapeStyle {
    base_shape.with_scale(state.scale_factor())
}

/// Create a shape with minimum radius constraint
///
/// Ensures that a shape has at least a minimum radius value,
/// useful for maintaining usability and touch targets.
///
/// # Arguments
/// * `shape` - The base shape to constrain
/// * `min_radius` - Minimum radius value
///
/// # Returns
/// A `ShapeStyle` with constrained minimum radius
#[must_use]
pub const fn constrain_min_radius(shape: &ShapeStyle, min_radius: f32) -> ShapeStyle {
    let radius = shape.radius;
    ShapeStyle {
        size: shape.size,
        radius: Radius {
            top_left: radius.top_left.max(min_radius),
            top_right: radius.top_right.max(min_radius),
            bottom_right: radius.bottom_right.max(min_radius),
            bottom_left: radius.bottom_left.max(min_radius),
        },
        description: shape.description,
    }
}

/// Create a shape with maximum radius constraint
///
/// Ensures that a shape doesn't exceed a maximum radius value,
/// useful for maintaining visual consistency.
///
/// # Arguments
/// * `shape` - The base shape to constrain
/// * `max_radius` - Maximum radius value
///
/// # Returns
/// A `ShapeStyle` with constrained maximum radius
#[must_use]
pub const fn constrain_max_radius(shape: &ShapeStyle, max_radius: f32) -> ShapeStyle {
    let radius = shape.radius;
    ShapeStyle {
        size: shape.size,
        radius: Radius {
            top_left: radius.top_left.min(max_radius),
            top_right: radius.top_right.min(max_radius),
            bottom_right: radius.bottom_right.min(max_radius),
            bottom_left: radius.bottom_left.min(max_radius),
        },
        description: shape.description,
    }
}

/// Create a shape with radius constraints
///
/// Combines minimum and maximum radius constraints.
///
/// # Arguments
/// * `shape` - The base shape to constrain
/// * `min_radius` - Minimum radius value
/// * `max_radius` - Maximum radius value
///
/// # Returns
/// A `ShapeStyle` with constrained radius values
#[must_use]
pub const fn constrain_radius(shape: &ShapeStyle, min_radius: f32, max_radius: f32) -> ShapeStyle {
    let constrained = constrain_min_radius(shape, min_radius);
    constrain_max_radius(&constrained, max_radius)
}

/// Check if two shapes are visually similar
///
/// Determines if two shapes would appear similar to users,
/// useful for shape optimization and caching.
///
/// # Arguments
/// * `shape1` - First shape to compare
/// * `shape2` - Second shape to compare
/// * `tolerance` - Tolerance for radius differences
///
/// # Returns
/// `true` if shapes are visually similar within tolerance
#[must_use]
pub fn shapes_similar(shape1: &ShapeStyle, shape2: &ShapeStyle, tolerance: f32) -> bool {
    let r1 = shape1.radius;
    let r2 = shape2.radius;

    (r1.top_left - r2.top_left).abs() <= tolerance
        && (r1.top_right - r2.top_right).abs() <= tolerance
        && (r1.bottom_right - r2.bottom_right).abs() <= tolerance
        && (r1.bottom_left - r2.bottom_left).abs() <= tolerance
}

/// Get the average radius of a shape
///
/// Calculates the average corner radius, useful for
/// shape analysis and responsive calculations.
///
/// # Arguments
/// * `shape` - The shape to analyze
///
/// # Returns
/// The average radius value
#[must_use]
pub fn average_radius(shape: &ShapeStyle) -> f32 {
    let r = shape.radius;
    (r.top_left + r.top_right + r.bottom_right + r.bottom_left) / 4.0
}

/// Get the maximum radius of a shape
///
/// Returns the largest corner radius in the shape.
///
/// # Arguments
/// * `shape` - The shape to analyze
///
/// # Returns
/// The maximum radius value
#[must_use]
pub const fn max_radius(shape: &ShapeStyle) -> f32 {
    let r = shape.radius;
    r.top_left
        .max(r.top_right)
        .max(r.bottom_right)
        .max(r.bottom_left)
}

/// Get the minimum radius of a shape
///
/// Returns the smallest corner radius in the shape.
///
/// # Arguments
/// * `shape` - The shape to analyze
///
/// # Returns
/// The minimum radius value
#[must_use]
pub const fn min_radius(shape: &ShapeStyle) -> f32 {
    let r = shape.radius;
    r.top_left
        .min(r.top_right)
        .min(r.bottom_right)
        .min(r.bottom_left)
}
