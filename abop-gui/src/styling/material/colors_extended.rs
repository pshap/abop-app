//! Extended Material Design 3 Color Tokens
//! 
//! This module contains the complete set of MD3 color tokens including
//! missing tokens identified in the MD3 Color Action Plan.

use iced::Color;
use super::colors::{MaterialColors, ColorRole};

/// Extended MaterialColors with complete MD3 token coverage
#[derive(Debug, Clone, PartialEq)]
pub struct ExtendedMaterialColors {
    /// Existing MaterialColors base
    pub base: MaterialColors,
    
    /// Missing container tokens  
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,
    pub on_error_container: Color,
    
    /// Fixed variant tokens (exposed from ColorRole)
    pub primary_fixed: Color,
    pub primary_fixed_dim: Color,
    pub on_primary_fixed: Color,
    pub on_primary_fixed_variant: Color,
    
    pub secondary_fixed: Color,
    pub secondary_fixed_dim: Color,
    pub on_secondary_fixed: Color,
    pub on_secondary_fixed_variant: Color,
    
    pub tertiary_fixed: Color,
    pub tertiary_fixed_dim: Color,
    pub on_tertiary_fixed: Color,
    pub on_tertiary_fixed_variant: Color,
    
    pub error_fixed: Color,
    pub error_fixed_dim: Color,
    pub on_error_fixed: Color,
    pub on_error_fixed_variant: Color,
}

impl ExtendedMaterialColors {
    /// Create extended colors from base MaterialColors
    pub fn from_base(base: MaterialColors) -> Self {
        let primary_fixed = base.primary_fixed_container();
        let secondary_fixed = base.secondary_fixed_container();
        let tertiary_fixed = base.tertiary_fixed_container();
        let error_fixed = base.error_fixed_container();
        
        Self {
            // Extract fixed variants from base color containers
            primary_fixed: primary_fixed.fixed,
            primary_fixed_dim: primary_fixed.fixed_dim,
            on_primary_fixed: primary_fixed.on_fixed,
            on_primary_fixed_variant: primary_fixed.on_fixed_variant,
            
            secondary_fixed: secondary_fixed.fixed,
            secondary_fixed_dim: secondary_fixed.fixed_dim,
            on_secondary_fixed: secondary_fixed.on_fixed,
            on_secondary_fixed_variant: secondary_fixed.on_fixed_variant,
            
            tertiary_fixed: tertiary_fixed.fixed,
            tertiary_fixed_dim: tertiary_fixed.fixed_dim,
            on_tertiary_fixed: tertiary_fixed.on_fixed,
            on_tertiary_fixed_variant: tertiary_fixed.on_fixed_variant,
            
            error_fixed: error_fixed.fixed,
            error_fixed_dim: error_fixed.fixed_dim,
            on_error_fixed: error_fixed.on_fixed,
            on_error_fixed_variant: error_fixed.on_fixed_variant,
              // Container tokens from field access
            tertiary_container: base.tertiary.container,
            on_tertiary_container: base.tertiary.on_container,
            on_error_container: base.error.on_container,
            
            base,
        }
    }
}

/// Specialized container for fixed color variants
#[derive(Debug, Clone, PartialEq)]
pub struct FixedColorContainer {
    /// Standard fixed color
    pub fixed: Color,
    /// Dimmed fixed color variant
    pub fixed_dim: Color,
    /// Color for content on fixed surfaces
    pub on_fixed: Color,
    /// Variant color for content on fixed surfaces
    pub on_fixed_variant: Color,
}

impl FixedColorContainer {
    /// Create from ColorRole
    pub const fn from_color_role(role: &ColorRole) -> Self {
        Self {
            fixed: role.fixed,
            fixed_dim: role.fixed_dim,
            on_fixed: role.on_fixed,
            on_fixed_variant: role.on_fixed_variant,
        }
    }
    
    /// Get fixed color for light surfaces
    pub const fn light_surface(&self) -> Color {
        self.fixed
    }
    
    /// Get fixed color for dark surfaces  
    pub const fn dark_surface(&self) -> Color {
        self.fixed_dim
    }
    
    /// Get appropriate text color for fixed surface
    pub const fn text_color(&self, is_variant: bool) -> Color {
        if is_variant {
            self.on_fixed_variant
        } else {
            self.on_fixed
        }
    }
}

/// Extension trait for MaterialColors to provide fixed variant access
pub trait FixedVariantAccess {
    /// Get primary fixed color container
    fn primary_fixed_container(&self) -> FixedColorContainer;
    
    /// Get secondary fixed color container
    fn secondary_fixed_container(&self) -> FixedColorContainer;
    
    /// Get tertiary fixed color container
    fn tertiary_fixed_container(&self) -> FixedColorContainer;
    
    /// Get error fixed color container
    fn error_fixed_container(&self) -> FixedColorContainer;
}

impl FixedVariantAccess for MaterialColors {
    fn primary_fixed_container(&self) -> FixedColorContainer {
        // For method-based MaterialColors, we'll provide sensible defaults
        // since the fixed variants are typically used for surface elements
        FixedColorContainer {
            fixed: self.primary_container(),
            fixed_dim: {
                let mut color = self.primary_container();
                color.a = 0.8; // Slightly more transparent
                color
            },
            on_fixed: self.on_primary_container(),
            on_fixed_variant: self.on_primary(),
        }
    }
    
    fn secondary_fixed_container(&self) -> FixedColorContainer {
        FixedColorContainer {
            fixed: self.secondary_container(),
            fixed_dim: {
                let mut color = self.secondary_container();
                color.a = 0.8;
                color
            },
            on_fixed: self.on_secondary_container(),
            on_fixed_variant: self.on_secondary(),
        }
    }
    
    fn tertiary_fixed_container(&self) -> FixedColorContainer {
        FixedColorContainer {
            fixed: self.tertiary_container(),
            fixed_dim: {
                let mut color = self.tertiary_container();
                color.a = 0.8;
                color
            },
            on_fixed: self.on_tertiary_container(),
            on_fixed_variant: self.on_tertiary(),
        }
    }
    
    fn error_fixed_container(&self) -> FixedColorContainer {
        FixedColorContainer {
            fixed: self.error_container(),
            fixed_dim: {
                let mut color = self.error_container();
                color.a = 0.8;
                color
            },
            on_fixed: self.on_error_container(),
            on_fixed_variant: self.on_error(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::material::{MaterialPalette, MaterialColors};    #[test]
    fn test_extended_colors_creation() {
        let colors = MaterialColors::light_default();
        let primary_container = colors.primary_fixed_container();
        
        // Verify fixed variants are properly exposed
        assert_eq!(primary_container.fixed, colors.primary_container());
        assert_eq!(primary_container.on_fixed, colors.on_primary_container());
    }
    
    #[test]
    fn test_fixed_color_container() {
        let colors = MaterialColors::light_default();
        
        let primary_container = colors.primary_fixed_container();
        assert_eq!(primary_container.fixed, colors.primary_container());
        assert_eq!(primary_container.on_fixed, colors.on_primary_container());
    }
      #[test]
    fn test_fixed_container_text_colors() {
        let colors = MaterialColors::light_default();
        let container = colors.primary_fixed_container();
        
        let normal_text = container.text_color(false);
        let variant_text = container.text_color(true);
        
        assert_eq!(normal_text, container.on_fixed);
        assert_eq!(variant_text, container.on_fixed_variant);
    }
}
