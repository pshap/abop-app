//! Core shape types and implementations for the Material Design 3 Shape System
//!
//! This module contains the fundamental shape types including `ShapeSize`, `ShapeStyle`,
//! and `MaterialShapes`, along with their core implementations.

use super::constants;
use iced::border::Radius;

/// Float comparison epsilon for shape calculations
const FLOAT_EPSILON: f32 = f32::EPSILON * 4.0;

/// Material Design shape sizes
///
/// Defines the seven standard corner radius sizes in the Material Design 3 shape system.
/// Each size corresponds to a specific pixel value and semantic meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeSize {
    /// No corner radius (0px) - Sharp corners for focus and hierarchy
    None,
    /// Extra small radius (4px) - Subtle rounding for small elements
    ExtraSmall,
    /// Small radius (8px) - Gentle rounding for interactive elements
    Small,
    /// Medium radius (12px) - Comfortable rounding for containers
    Medium,
    /// Large radius (16px) - Prominent rounding for large surfaces
    Large,
    /// Extra large radius (28px) - Bold rounding for key surfaces
    ExtraLarge,
    /// Full radius (circular) - Complete rounding for pills and icons
    Full,
}

/// Individual shape style with radius properties
///
/// Represents a specific corner radius configuration with its associated
/// size category, radius values, and usage description.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeStyle {
    /// The semantic size category of this shape
    pub size: ShapeSize,
    /// The actual corner radius values for each corner
    pub radius: Radius,
    /// Human-readable description of when to use this shape
    pub description: &'static str,
}

/// Material Design 3 shape system
///
/// Provides the complete set of corner radius tokens and shape styles
/// defined in Material Design 3. Includes seven standard corner radius
/// sizes from none (0px) to full (circular).
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialShapes {
    /// No corner radius (0px) - used for dividers and separators
    pub corner_none: ShapeStyle,
    /// Extra small corner radius (4px) - used for small components like chips
    pub corner_extra_small: ShapeStyle,
    /// Small corner radius (8px) - used for buttons and small cards
    pub corner_small: ShapeStyle,
    /// Medium corner radius (12px) - used for cards and containers
    pub corner_medium: ShapeStyle,
    /// Large corner radius (16px) - used for large cards and sheets
    pub corner_large: ShapeStyle,
    /// Extra large corner radius (28px) - used for modal dialogs and prominent surfaces
    pub corner_extra_large: ShapeStyle,
    /// Full corner radius (circular) - used for FABs, pills, and circular elements
    pub corner_full: ShapeStyle,
}

/// Macro to generate accessor methods for `MaterialShapes`
macro_rules! impl_shape_accessors {
    ($($variant:ident => $field:ident),* $(,)?) => {
        impl MaterialShapes {
            /// Get shape style by size
            #[must_use] pub const fn get_shape(&self, size: ShapeSize) -> &ShapeStyle {
                match size {
                    $(ShapeSize::$variant => &self.$field,)*
                }
            }

            /// Get mutable shape style by size
            pub const fn get_shape_mut(&mut self, size: ShapeSize) -> &mut ShapeStyle {
                match size {
                    $(ShapeSize::$variant => &mut self.$field,)*
                }
            }
        }
    };
}

// Generate the accessor methods
impl_shape_accessors! {
    None => corner_none,
    ExtraSmall => corner_extra_small,
    Small => corner_small,
    Medium => corner_medium,
    Large => corner_large,
    ExtraLarge => corner_extra_large,
    Full => corner_full,
}

impl ShapeSize {
    /// Get all shape sizes
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::None,
            Self::ExtraSmall,
            Self::Small,
            Self::Medium,
            Self::Large,
            Self::ExtraLarge,
            Self::Full,
        ]
    }

    /// Get the radius value in pixels
    #[must_use]
    pub const fn radius(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::ExtraSmall => 4.0,
            Self::Small => 8.0,
            Self::Medium => 12.0,
            Self::Large => 16.0,
            Self::ExtraLarge => 28.0,
            Self::Full => 9999.0, // Effectively fully rounded
        }
    }

    /// Get the size name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "corner-none",
            Self::ExtraSmall => "corner-extra-small",
            Self::Small => "corner-small",
            Self::Medium => "corner-medium",
            Self::Large => "corner-large",
            Self::ExtraLarge => "corner-extra-large",
            Self::Full => "corner-full",
        }
    }

    /// Get description of when to use this shape size
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::None => "No rounding. Use for dividers and separators",
            Self::ExtraSmall => "Subtle rounding. Use for small components like chips",
            Self::Small => "Gentle rounding. Use for buttons and small cards",
            Self::Medium => "Comfortable rounding. Use for cards and containers",
            Self::Large => "Prominent rounding. Use for large cards and sheets",
            Self::ExtraLarge => "Bold rounding. Use for modal dialogs and prominent surfaces",
            Self::Full => "Fully rounded. Use for FABs, pills, and circular elements",
        }
    }
}

impl ShapeStyle {
    /// Create a new shape style
    #[must_use]
    pub fn new(size: ShapeSize) -> Self {
        let radius_value = size.radius();

        Self {
            size,
            radius: Radius::from(radius_value),
            description: size.description(),
        }
    }

    /// Create a shape style with custom radius
    #[must_use]
    pub fn custom(radius: f32, description: &'static str) -> Self {
        Self {
            size: ShapeSize::None, // Custom size
            radius: Radius::from(radius),
            description,
        }
    }

    /// Create a shape style with different corner radii
    #[must_use]
    pub const fn asymmetric(
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    ) -> Self {
        Self {
            size: ShapeSize::None, // Custom size
            radius: Radius {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            },
            description: "Custom asymmetric shape",
        }
    }

    /// Create a shape with only top corners rounded
    #[must_use]
    pub const fn top_only(radius: f32) -> Self {
        Self::asymmetric(radius, radius, 0.0, 0.0)
    }

    /// Create a shape with only bottom corners rounded
    #[must_use]
    pub const fn bottom_only(radius: f32) -> Self {
        Self::asymmetric(0.0, 0.0, radius, radius)
    }

    /// Create a shape with only left corners rounded
    #[must_use]
    pub const fn start_only(radius: f32) -> Self {
        Self::asymmetric(radius, 0.0, 0.0, radius)
    }

    /// Create a shape with only right corners rounded
    #[must_use]
    pub const fn end_only(radius: f32) -> Self {
        Self::asymmetric(0.0, radius, radius, 0.0)
    }

    /// Scale the radius by a factor
    #[must_use]
    pub fn with_scale(&self, scale: f32) -> Self {
        let scale = scale.max(0.0);

        Self {
            radius: Radius {
                top_left: self.radius.top_left * scale,
                top_right: self.radius.top_right * scale,
                bottom_right: self.radius.bottom_right * scale,
                bottom_left: self.radius.bottom_left * scale,
            },
            ..self.clone()
        }
    }

    /// Convert to Iced Radius
    #[must_use]
    pub const fn to_radius(&self) -> Radius {
        self.radius
    }

    /// Check if shape is circular (all corners equal and large)
    #[must_use]
    pub fn is_circular(&self) -> bool {
        let radius = self.radius;
        (radius.top_left - radius.top_right).abs() < FLOAT_EPSILON
            && (radius.top_right - radius.bottom_right).abs() < FLOAT_EPSILON
            && (radius.bottom_right - radius.bottom_left).abs() < FLOAT_EPSILON
            && radius.top_left >= constants::CIRCULAR_THRESHOLD
    }

    /// Check if shape is rectangular (no rounding)
    #[must_use]
    pub fn is_rectangular(&self) -> bool {
        let radius = self.radius;
        radius.top_left == 0.0
            && radius.top_right == 0.0
            && radius.bottom_right == 0.0
            && radius.bottom_left == 0.0
    }

    /// Create a transition to another shape
    #[must_use]
    pub fn transition_to(&self, target: &Self, progress: f32) -> Self {
        super::utils::interpolate_shapes(self, target, progress)
    }
}

impl MaterialShapes {
    /// Create the default Material Design shape system
    #[must_use]
    pub fn new() -> Self {
        Self {
            corner_none: ShapeStyle::new(ShapeSize::None),
            corner_extra_small: ShapeStyle::new(ShapeSize::ExtraSmall),
            corner_small: ShapeStyle::new(ShapeSize::Small),
            corner_medium: ShapeStyle::new(ShapeSize::Medium),
            corner_large: ShapeStyle::new(ShapeSize::Large),
            corner_extra_large: ShapeStyle::new(ShapeSize::ExtraLarge),
            corner_full: ShapeStyle::new(ShapeSize::Full),
        }
    }

    /// Create shapes with custom scale factor
    #[must_use]
    pub fn with_scale(&self, scale: f32) -> Self {
        Self {
            corner_none: self.corner_none.with_scale(scale),
            corner_extra_small: self.corner_extra_small.with_scale(scale),
            corner_small: self.corner_small.with_scale(scale),
            corner_medium: self.corner_medium.with_scale(scale),
            corner_large: self.corner_large.with_scale(scale),
            corner_extra_large: self.corner_extra_large.with_scale(scale),
            corner_full: self.corner_full.with_scale(scale),
        }
    }

    /// Create shapes for a specific family
    #[must_use]
    pub fn for_family(&self, family: super::families::ShapeFamily) -> Self {
        match family {
            super::families::ShapeFamily::Sharp => {
                let none_shape = self.corner_none.clone();
                Self {
                    corner_none: none_shape.clone(),
                    corner_extra_small: none_shape.clone(),
                    corner_small: none_shape.clone(),
                    corner_medium: none_shape.clone(),
                    corner_large: none_shape.clone(),
                    corner_extra_large: none_shape.clone(),
                    corner_full: none_shape,
                }
            }
            super::families::ShapeFamily::Rounded => self.clone(),
            super::families::ShapeFamily::Circular => {
                let full_shape = self.corner_full.clone();
                Self {
                    corner_none: full_shape.clone(),
                    corner_extra_small: full_shape.clone(),
                    corner_small: full_shape.clone(),
                    corner_medium: full_shape.clone(),
                    corner_large: full_shape.clone(),
                    corner_extra_large: full_shape.clone(),
                    corner_full: full_shape,
                }
            }
        }
    }

    /// Get all shape styles as a vector
    #[must_use]
    pub fn all_styles(&self) -> Vec<&ShapeStyle> {
        ShapeSize::all()
            .iter()
            .map(|&size| self.get_shape(size))
            .collect()
    }

    /// Get shape for buttons (semantic accessor)
    #[must_use]
    pub const fn button(&self) -> &ShapeStyle {
        &self.corner_small
    }

    /// Get shape for cards (semantic accessor)
    #[must_use]
    pub const fn card(&self) -> &ShapeStyle {
        &self.corner_medium
    }

    /// Get shape for dialogs (semantic accessor)
    #[must_use]
    pub const fn dialog(&self) -> &ShapeStyle {
        &self.corner_extra_large
    }

    /// Get shape for chips and small components (semantic accessor)
    #[must_use]
    pub const fn chip(&self) -> &ShapeStyle {
        &self.corner_small
    }

    /// Get shape for text fields (semantic accessor)
    #[must_use]
    pub const fn text_field(&self) -> &ShapeStyle {
        &self.corner_extra_small
    }

    /// Get shape for floating action buttons (semantic accessor)
    #[must_use]
    pub const fn fab(&self) -> &ShapeStyle {
        &self.corner_large
    }
}

impl Default for MaterialShapes {
    fn default() -> Self {
        Self::new()
    }
}
