//! Material Design 3 Shape System
//!
//! Implements the complete Material Design 3 shape system including:
//! - Corner radius tokens (none, extra-small, small, medium, large, extra-large, full)
//! - Shape families and their usage guidelines
//! - Component type recommendations
//! - Interactive state variations
//! - Utility functions for shape manipulation
//! - Integration with Iced border radius system
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use abop_gui::styling::material::shapes::{MaterialShapes, ShapeSize, ComponentType};
//!
//! // Create the default shape system
//! let shapes = MaterialShapes::new();
//!
//! // Get a specific shape
//! let button_shape = shapes.get_shape(ShapeSize::Small);
//! let card_shape = shapes.card(); // Semantic accessor
//!
//! // Get recommended shape for component
//! let recommended = ComponentType::Button.recommended_shape();
//! ```
//!
//! ## Shape Families
//!
//! ```rust
//! use abop_gui::styling::material::shapes::{MaterialShapes, ShapeFamily, AppStyle};
//!
//! let shapes = MaterialShapes::new();
//!
//! // Apply a shape family for consistent styling
//! let sharp_shapes = shapes.for_family(ShapeFamily::Sharp);
//! let playful_shapes = shapes.for_family(ShapeFamily::Circular);
//!
//! // Get family recommendation from app style
//! let family = AppStyle::Professional.recommended_family();
//! ```
//!
//! ## State Variations
//!
//! ```rust
//! use abop_gui::styling::material::shapes::{ShapeStyle, ShapeSize, ComponentState, utils};
//!
//! let base_shape = ShapeStyle::new(ShapeSize::Medium);
//!
//! // Apply state variations
//! let hovered_shape = utils::shape_for_state(&base_shape, ComponentState::Hovered);
//! let pressed_shape = utils::shape_for_state(&base_shape, ComponentState::Pressed);
//! ```

// Module declarations
pub mod components;
pub mod constants;
pub mod core;
pub mod families;
pub mod states;
pub mod utils;

#[cfg(test)]
mod tests;

// Re-exports for public API
pub use components::{ComponentCategory, ComponentType};
pub use constants::*;
pub use core::{MaterialShapes, ShapeSize, ShapeStyle};
pub use families::{AppStyle, ShapeFamily};
pub use states::ComponentState;

// Re-export utils module for convenient access
pub use utils as shape_utils;

// Convenience re-exports of commonly used utility functions
pub use utils::{
    get_category_shape, get_recommended_family, get_recommended_shape, interpolate_shapes,
    responsive_shape, shape_for_state,
};

// Type aliases for convenience
/// Type alias for [`MaterialShapes`] for shorter imports
pub type Shapes = MaterialShapes;
/// Type alias for [`ShapeStyle`] for shorter imports
pub type Shape = ShapeStyle;
/// Type alias for [`ShapeSize`] for shorter imports
pub type Size = ShapeSize;

/// Create the default Material Design shape system
///
/// This is a convenience function that creates a new `MaterialShapes` instance
/// with all the standard Material Design 3 corner radius tokens.
///
/// # Returns
/// A `MaterialShapes` instance with default values
///
/// # Examples
///
/// ```rust
/// use abop_gui::styling::material::shapes;
///
/// let shapes = shapes::default();
/// let button_radius = shapes.button().to_radius();
/// ```
#[must_use]
pub fn default() -> MaterialShapes {
    MaterialShapes::new()
}

/// Create a scaled version of the default shape system
///
/// This is a convenience function that creates a new `MaterialShapes` instance
/// with all radius values scaled by the given factor.
///
/// # Arguments
/// * `scale` - Scale factor to apply to all radius values
///
/// # Returns
/// A scaled `MaterialShapes` instance
///
/// # Examples
///
/// ```rust
/// use abop_gui::styling::material::shapes;
///
/// // Create shapes scaled up by 50%
/// let large_shapes = shapes::scaled(1.5);
/// ```
#[must_use]
pub fn scaled(scale: f32) -> MaterialShapes {
    MaterialShapes::new().with_scale(scale)
}

/// Create shapes for a specific family
///
/// This is a convenience function that creates a `MaterialShapes` instance
/// configured for a specific shape family.
///
/// # Arguments
/// * `family` - The shape family to use
///
/// # Returns
/// A `MaterialShapes` instance configured for the family
///
/// # Examples
///
/// ```rust
/// use abop_gui::styling::material::shapes::{self, ShapeFamily};
///
/// // Create sharp shapes for professional apps
/// let sharp_shapes = shapes::for_family(ShapeFamily::Sharp);
/// ```
#[must_use]
pub fn for_family(family: ShapeFamily) -> MaterialShapes {
    MaterialShapes::new().for_family(family)
}

/// Create shapes for a specific app style
///
/// This is a convenience function that creates a `MaterialShapes` instance
/// configured for a specific application style.
///
/// # Arguments
/// * `app_style` - The application style to use
///
/// # Returns
/// A `MaterialShapes` instance configured for the app style
///
/// # Examples
///
/// ```rust
/// use abop_gui::styling::material::shapes::{self, AppStyle};
///
/// // Create shapes for a friendly app
/// let friendly_shapes = shapes::for_app_style(AppStyle::Friendly);
/// ```
#[must_use]
pub fn for_app_style(app_style: AppStyle) -> MaterialShapes {
    let family = app_style.recommended_family();
    MaterialShapes::new().for_family(family)
}

/// Get the recommended shape for a component type
///
/// This is a convenience function that returns the Material Design 3
/// recommended corner radius size for a specific component type.
///
/// # Arguments
/// * `component_type` - The component type to get recommendation for
///
/// # Returns
/// The recommended `ShapeSize`
///
/// # Examples
///
/// ```rust
/// use abop_gui::styling::material::shapes::{self, ComponentType};
///
/// let button_size = shapes::recommended_for(ComponentType::Button);
/// let card_size = shapes::recommended_for(ComponentType::Card);
/// ```
#[must_use]
pub const fn recommended_for(component_type: ComponentType) -> ShapeSize {
    component_type.recommended_shape()
}

/// Create a responsive shape based on container size
///
/// This is a convenience function that creates a shape with radius
/// proportional to the container size, useful for responsive designs.
///
/// # Arguments
/// * `container_size` - The size of the container in pixels
/// * `max_radius` - Maximum allowed radius value
///
/// # Returns
/// A responsive `ShapeStyle`
///
/// # Examples
///
/// ```rust
/// use abop_gui::styling::material::shapes;
///
/// // Create a shape that scales with container size
/// let responsive = shapes::responsive(200.0, 16.0);
/// ```
#[must_use]
pub fn responsive(container_size: f32, max_radius: f32) -> ShapeStyle {
    utils::responsive_shape(container_size, max_radius)
}

/// Create a shape transition between two shapes
///
/// This is a convenience function for creating smooth transitions
/// between shapes, useful for animations.
///
/// # Arguments
/// * `start` - The starting shape
/// * `end` - The ending shape
/// * `progress` - Transition progress (0.0 to 1.0)
///
/// # Returns
/// An interpolated `ShapeStyle`
///
/// # Examples
///
/// ```rust
/// use abop_gui::styling::material::shapes::{self, ShapeStyle, ShapeSize};
///
/// let start = ShapeStyle::new(ShapeSize::Small);
/// let end = ShapeStyle::new(ShapeSize::Large);
/// let transition = shapes::transition(&start, &end, 0.5);
/// ```
#[must_use]
pub fn transition(start: &ShapeStyle, end: &ShapeStyle, progress: f32) -> ShapeStyle {
    utils::interpolate_shapes(start, end, progress)
}

/// Create a shape variation for a component state
///
/// This is a convenience function that applies appropriate scaling
/// and modifications based on component state.
///
/// # Arguments
/// * `base_shape` - The base shape to modify
/// * `state` - The component state
///
/// # Returns
/// A state-modified `ShapeStyle`
///
/// # Examples
///
/// ```rust
/// use abop_gui::styling::material::shapes::{self, ShapeStyle, ShapeSize, ComponentState};
///
/// let base = ShapeStyle::new(ShapeSize::Medium);
/// let hovered = shapes::for_state(&base, ComponentState::Hovered);
/// ```
#[must_use]
pub fn for_state(base_shape: &ShapeStyle, state: ComponentState) -> ShapeStyle {
    utils::shape_for_state(base_shape, state)
}
