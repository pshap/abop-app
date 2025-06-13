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
        
        assert_eq!(
            CustomButtonSize::from(MaterialButtonSize::Medium),
            CustomButtonSize::Medium
        );
        assert_eq!(
            MaterialButtonSize::from(CustomButtonSize::Medium),
            MaterialButtonSize::Medium
        );
        
        assert_eq!(
            CustomButtonSize::from(MaterialButtonSize::Large),
            CustomButtonSize::Large
        );
        assert_eq!(
            MaterialButtonSize::from(CustomButtonSize::Large),
            MaterialButtonSize::Large
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
        
        assert_eq!(
            CustomIconPosition::from(MaterialIconPosition::Trailing),
            CustomIconPosition::Trailing
        );
        assert_eq!(
            MaterialIconPosition::from(CustomIconPosition::Trailing),
            MaterialIconPosition::Trailing
        );
        
        assert_eq!(
            CustomIconPosition::from(MaterialIconPosition::Only),
            CustomIconPosition::Only
        );
        assert_eq!(
            MaterialIconPosition::from(CustomIconPosition::Only),
            MaterialIconPosition::Only
        );
    }
}
