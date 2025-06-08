//! Performance optimizations for the elevation system

use super::{constants, ElevationLevel, ElevationStyle, MaterialElevation, ShadowParams};
use crate::styling::material::colors::MaterialColors;
use iced::Color;
use std::sync::LazyLock;

/// Pre-computed elevation styles for common use cases
static PRECOMPUTED_STYLES: LazyLock<[ElevationStyle; 6]> = LazyLock::new(|| {
    [
        ElevationStyle::new(ElevationLevel::Level0, Color::BLACK, Color::WHITE),
        ElevationStyle::new(ElevationLevel::Level1, Color::BLACK, Color::WHITE),
        ElevationStyle::new(ElevationLevel::Level2, Color::BLACK, Color::WHITE),
        ElevationStyle::new(ElevationLevel::Level3, Color::BLACK, Color::WHITE),
        ElevationStyle::new(ElevationLevel::Level4, Color::BLACK, Color::WHITE),
        ElevationStyle::new(ElevationLevel::Level5, Color::BLACK, Color::WHITE),
    ]
});

/// Pre-computed material elevation with default colors
static DEFAULT_MATERIAL_ELEVATION: LazyLock<MaterialElevation> = LazyLock::new(|| {
    // Create default MaterialColors for the elevation system
    let default_colors = MaterialColors::default();
    MaterialElevation::new(&default_colors)
});

/// Compile-time computed shadow offset based on level
const fn compute_shadow_offset(level: u8) -> f32 {
    match level {
        0 => 0.0,
        1 => 1.0,
        2 => 2.0,
        3 => 3.0,
        4 => 4.0,
        5 => 6.0,
        _ => 0.0,
    }
}

/// Compile-time computed blur radius based on level
const fn compute_blur_radius(level: u8) -> f32 {
    match level {
        0 => 0.0,
        1 => 2.0,
        2 => 4.0,
        3 => 6.0,
        4 => 8.0,
        5 => 12.0,
        _ => 0.0,
    }
}

/// Performance-optimized elevation style retrieval
pub struct FastElevation;

impl FastElevation {
    /// Get a pre-computed elevation style for the given level
    /// This is much faster than creating a new style each time
    #[must_use]
    pub fn get_style(level: ElevationLevel) -> &'static ElevationStyle {
        &PRECOMPUTED_STYLES[level.as_u8() as usize]
    }

    /// Get the default material elevation system
    /// Uses cached instance for better performance
    #[must_use]
    pub fn get_default_system() -> &'static MaterialElevation {
        &DEFAULT_MATERIAL_ELEVATION
    }

    /// Fast shadow parameter computation using compile-time constants
    #[must_use]
    pub const fn compute_shadow_params_fast(level: u8) -> ShadowParams {
        ShadowParams {
            offset_y: compute_shadow_offset(level),
            blur_radius: compute_blur_radius(level),
            opacity: if level == 0 { 0.0 } else { constants::DEFAULT_SHADOW_OPACITY },
        }
    }    /// Batch create elevation styles for all levels with custom colors
    /// More efficient than creating them individually
    #[must_use]
    pub fn create_all_styles_fast(shadow_color: Color, tint_color: Color) -> [ElevationStyle; 6] {
        ElevationLevel::all().iter().copied().map(|level| ElevationStyle::new(level, shadow_color, tint_color)).collect::<Vec<_>>().try_into().unwrap()
    }

    /// Fast interpolation between two elevation levels
    /// Uses compile-time computed parameters where possible
    #[must_use]
    pub fn interpolate_fast(from: ElevationLevel, to: ElevationLevel, factor: f32) -> ShadowParams {
        let from_params = Self::compute_shadow_params_fast(from.as_u8());
        let to_params = Self::compute_shadow_params_fast(to.as_u8());

        ShadowParams {
            offset_y: from_params.offset_y + (to_params.offset_y - from_params.offset_y) * factor,
            blur_radius: from_params.blur_radius + (to_params.blur_radius - from_params.blur_radius) * factor,
            opacity: from_params.opacity + (to_params.opacity - from_params.opacity) * factor,
        }
    }
}

/// Cache for frequently used elevation styles with custom colors
pub struct ElevationCache {
    cache: std::collections::HashMap<(u32, u32), MaterialElevation>,
}

impl ElevationCache {
    /// Create a new elevation cache
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }    /// Get or create a material elevation system for the given colors
    pub fn get_or_create(&mut self, shadow_color: Color, tint_color: Color) -> &MaterialElevation {
        // Use color components as hash key
        let shadow_key = (shadow_color.r.to_bits(), shadow_color.g.to_bits());
        let tint_key = (tint_color.r.to_bits(), tint_color.g.to_bits());
        let key = (
            shadow_key.0 ^ shadow_key.1,
            tint_key.0 ^ tint_key.1,
        );

        self.cache.entry(key).or_insert_with(|| {
            MaterialElevation::with_colors(shadow_color, tint_color)
        })
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    #[must_use]
    pub fn stats(&self) -> (usize, usize) {
        (self.cache.len(), self.cache.capacity())
    }
}

impl Default for ElevationCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_elevation_style_retrieval() {
        let style = FastElevation::get_style(ElevationLevel::Level2);
        assert_eq!(style.level(), ElevationLevel::Level2);
        assert_eq!(style.dp, 3.0);
    }

    #[test]
    fn test_fast_shadow_params_computation() {
        let params = FastElevation::compute_shadow_params_fast(2);
        assert_eq!(params.offset_y, 2.0);
        assert_eq!(params.blur_radius, 4.0);
        assert_eq!(params.opacity, constants::DEFAULT_SHADOW_OPACITY);
    }

    #[test]
    fn test_fast_interpolation() {
        let params = FastElevation::interpolate_fast(
            ElevationLevel::Level1,
            ElevationLevel::Level3,
            0.5,
        );
        
        // Should be halfway between Level1 (1.0, 2.0) and Level3 (3.0, 6.0)
        assert_eq!(params.offset_y, 2.0);
        assert_eq!(params.blur_radius, 4.0);
    }    #[test]
    fn test_elevation_cache() {
        let mut cache = ElevationCache::new();
        let shadow_color = Color::from_rgb(0.1, 0.1, 0.1);
        let tint_color = Color::from_rgb(0.9, 0.9, 0.9);

        // First call creates the entry
        {
            let _elevation1 = cache.get_or_create(shadow_color, tint_color);
        }
        
        // Second call should use the cached entry
        {
            let _elevation2 = cache.get_or_create(shadow_color, tint_color);
        }
        
        let (count, _) = cache.stats();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_create_all_styles_fast() {
        let shadow_color = Color::from_rgb(0.2, 0.2, 0.2);
        let tint_color = Color::from_rgb(0.8, 0.8, 0.8);
        
        let styles = FastElevation::create_all_styles_fast(shadow_color, tint_color);
        
        assert_eq!(styles.len(), 6);
        assert_eq!(styles[0].level(), ElevationLevel::Level0);
        assert_eq!(styles[5].level(), ElevationLevel::Level5);
        
        // Check that colors are applied
        assert_eq!(styles[1].shadow.color.r, shadow_color.r);
    }

    #[test]
    fn test_default_system_caching() {
        let system1 = FastElevation::get_default_system();
        let system2 = FastElevation::get_default_system();
        
        // Should return the same cached instance
        assert!(std::ptr::eq(system1, system2));
    }
}
