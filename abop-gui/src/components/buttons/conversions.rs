//! Conversion traits and functions between material and custom button types

use super::variants::{ButtonSize as CustomButtonSize, IconPosition as CustomIconPosition};
use crate::styling::material::components::widgets::{
    material_button::{ButtonSize as MaterialButtonSize, IconPosition as MaterialIconPosition},
};

// Implement From trait for bidirectional conversion between custom and material ButtonSize
impl From<MaterialButtonSize> for CustomButtonSize {
    fn from(size: MaterialButtonSize) -> Self {
        match size {
            MaterialButtonSize::Small => CustomButtonSize::Small,
            MaterialButtonSize::Medium => CustomButtonSize::Medium,
            MaterialButtonSize::Large => CustomButtonSize::Large,
        }
    }
}

impl From<CustomButtonSize> for MaterialButtonSize {
    fn from(size: CustomButtonSize) -> Self {
        match size {
            CustomButtonSize::Small => MaterialButtonSize::Small,
            CustomButtonSize::Medium => MaterialButtonSize::Medium,
            CustomButtonSize::Large => MaterialButtonSize::Large,
        }
    }
}

// Implement From trait for bidirectional conversion between custom and material IconPosition
impl From<MaterialIconPosition> for CustomIconPosition {
    fn from(pos: MaterialIconPosition) -> Self {
        match pos {
            MaterialIconPosition::Leading => CustomIconPosition::Leading,
            MaterialIconPosition::Trailing => CustomIconPosition::Trailing,
            MaterialIconPosition::Only => CustomIconPosition::Only,
        }
    }
}

impl From<CustomIconPosition> for MaterialIconPosition {
    fn from(pos: CustomIconPosition) -> Self {
        match pos {
            CustomIconPosition::Leading => MaterialIconPosition::Leading,
            CustomIconPosition::Trailing => MaterialIconPosition::Trailing,
            CustomIconPosition::Only => MaterialIconPosition::Only,
        }
    }
}

// Keep the old conversion functions for backward compatibility
/// Convert from material ButtonSize to custom ButtonSize
#[deprecated(note = "Use `CustomButtonSize::from()` instead")]
pub fn from_material_button_size(size: MaterialButtonSize) -> CustomButtonSize {
    size.into()
}

/// Convert from custom ButtonSize to material ButtonSize
#[deprecated(note = "Use `MaterialButtonSize::from()` instead")]
pub fn to_material_button_size(size: CustomButtonSize) -> MaterialButtonSize {
    size.into()
}

/// Convert from material IconPosition to custom IconPosition
#[deprecated(note = "Use `CustomIconPosition::from()` instead")]
pub fn from_material_icon_position(pos: MaterialIconPosition) -> CustomIconPosition {
    pos.into()
}

/// Convert from custom IconPosition to material IconPosition
#[deprecated(note = "Use `MaterialIconPosition::from()` instead")]
pub fn to_material_icon_position(pos: CustomIconPosition) -> MaterialIconPosition {
    pos.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_size_conversion() {
        // Test From trait implementations
        assert_eq!(
            CustomButtonSize::from(MaterialButtonSize::Small),
            CustomButtonSize::Small
        );
        assert_eq!(
            MaterialButtonSize::from(CustomButtonSize::Small),
            MaterialButtonSize::Small
        );
        
        // Test the deprecated functions for backward compatibility
        assert_eq!(
            from_material_button_size(MaterialButtonSize::Small),
            CustomButtonSize::Small
        );
        assert_eq!(
            to_material_button_size(CustomButtonSize::Small),
            MaterialButtonSize::Small
        );
    }

    #[test]
    fn test_icon_position_conversion() {
        // Test From trait implementations
        assert_eq!(
            CustomIconPosition::from(MaterialIconPosition::Leading),
            CustomIconPosition::Leading
        );
        assert_eq!(
            MaterialIconPosition::from(CustomIconPosition::Leading),
            MaterialIconPosition::Leading
        );
        
        // Test the deprecated functions for backward compatibility
        assert_eq!(
            from_material_icon_position(MaterialIconPosition::Leading),
            CustomIconPosition::Leading
        );
        assert_eq!(
            to_material_icon_position(CustomIconPosition::Leading),
            MaterialIconPosition::Leading
        );
    }
}
