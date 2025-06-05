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
    
    /// Missing surface tokens
    pub surface_bright: Color,
    pub surface_dim: Color,
    
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
    pub const fn from_base(base: MaterialColors) -> Self {
        Self {
            // Extract fixed variants from base color roles
            primary_fixed: base.primary.fixed,
            primary_fixed_dim: base.primary.fixed_dim,
            on_primary_fixed: base.primary.on_fixed,
            on_primary_fixed_variant: base.primary.on_fixed_variant,
            
            secondary_fixed: base.secondary.fixed,
            secondary_fixed_dim: base.secondary.fixed_dim,
            on_secondary_fixed: base.secondary.on_fixed,
            on_secondary_fixed_variant: base.secondary.on_fixed_variant,
            
            tertiary_fixed: base.tertiary.fixed,
            tertiary_fixed_dim: base.tertiary.fixed_dim,
            on_tertiary_fixed: base.tertiary.on_fixed,
            on_tertiary_fixed_variant: base.tertiary.on_fixed_variant,
            
            error_fixed: base.error.fixed,
            error_fixed_dim: base.error.fixed_dim,
            on_error_fixed: base.error.on_fixed,
            on_error_fixed_variant: base.error.on_fixed_variant,
            
            // Missing container tokens
            tertiary_container: base.tertiary.container,
            on_tertiary_container: base.tertiary.on_container,
            on_error_container: base.error.on_container,
            
            // Surface tokens - will be properly implemented with palette mapping
            surface_bright: base.surface, // Placeholder - will be updated
            surface_dim: base.surface,    // Placeholder - will be updated
            
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
        FixedColorContainer::from_color_role(&self.primary)
    }
    
    fn secondary_fixed_container(&self) -> FixedColorContainer {
        FixedColorContainer::from_color_role(&self.secondary)
    }
    
    fn tertiary_fixed_container(&self) -> FixedColorContainer {
        FixedColorContainer::from_color_role(&self.tertiary)
    }
    
    fn error_fixed_container(&self) -> FixedColorContainer {
        FixedColorContainer::from_color_role(&self.error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::material::{MaterialPalette, MaterialColors};

    #[test]
    fn test_extended_colors_creation() {
        let palette = MaterialPalette::default();
        let base_colors = MaterialColors::light(&palette);
        let extended = ExtendedMaterialColors::from_base(base_colors);
        
        // Verify fixed variants are properly exposed
        assert_eq!(extended.primary_fixed, base_colors.primary.fixed);
        assert_eq!(extended.on_primary_fixed, base_colors.primary.on_fixed);
    }
    
    #[test]
    fn test_fixed_color_container() {
        let palette = MaterialPalette::default();
        let colors = MaterialColors::light(&palette);
        
        let primary_container = colors.primary_fixed_container();
        assert_eq!(primary_container.fixed, colors.primary.fixed);
        assert_eq!(primary_container.on_fixed, colors.primary.on_fixed);
    }
    
    #[test]
    fn test_fixed_container_text_colors() {
        let palette = MaterialPalette::default();
        let colors = MaterialColors::light(&palette);
        let container = colors.primary_fixed_container();
        
        let normal_text = container.text_color(false);
        let variant_text = container.text_color(true);
        
        assert_eq!(normal_text, container.on_fixed);
        assert_eq!(variant_text, container.on_fixed_variant);
    }
}
