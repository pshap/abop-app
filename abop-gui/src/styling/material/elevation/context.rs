//! Elevation context and caching system for Material Design 3

use super::{
    ElevationError, ElevationLevel, ElevationRegistry, ElevationStyle, MaterialElevation,
    constants::MIN_SCALE_FACTOR,
};
use crate::styling::material::MaterialColors;
use iced::Color;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Float comparison epsilon for elevation calculations
const FLOAT_EPSILON: f32 = f32::EPSILON * 4.0;

/// Wrapper for `iced::Color` to make it hashable
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorKey(pub Color);

impl std::hash::Hash for ColorKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the RGBA components as u32 for consistency
        (self.0.r.to_bits()).hash(state);
        (self.0.g.to_bits()).hash(state);
        (self.0.b.to_bits()).hash(state);
        (self.0.a.to_bits()).hash(state);
    }
}

impl Eq for ColorKey {}

impl From<Color> for ColorKey {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl From<ColorKey> for Color {
    fn from(key: ColorKey) -> Self {
        key.0
    }
}

/// Cache key for elevation context
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ElevationCacheKey {
    level: ElevationLevel,
    shadow_color: ColorKey,
    tint_color: ColorKey,
    scale: u32, // Store scale as u32 for hashing
}

/// Theme-aware elevation context with caching
#[derive(Debug, Clone)]
pub struct ElevationContext {
    /// The base material elevation system
    material_elevation: MaterialElevation,
    /// Registry for custom elevations
    registry: ElevationRegistry,
    /// Cache for computed elevation styles
    cache: Arc<Mutex<HashMap<ElevationCacheKey, ElevationStyle>>>,
    /// Current theme colors
    shadow_color: Color,
    tint_color: Color,
    /// Current scale factor
    scale: f32,
}

impl ElevationContext {
    /// Create a new elevation context with material colors
    #[must_use]
    pub fn new(colors: &MaterialColors) -> Self {
        Self {
            material_elevation: MaterialElevation::new(colors),
            registry: ElevationRegistry::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),
            shadow_color: colors.shadow,
            tint_color: colors.surface_tint,
            scale: 1.0,
        }
    }

    /// Create with custom colors
    #[must_use]
    pub fn with_colors(shadow_color: Color, tint_color: Color) -> Self {
        Self {
            material_elevation: MaterialElevation::with_colors(shadow_color, tint_color),
            registry: ElevationRegistry::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),
            shadow_color,
            tint_color,
            scale: 1.0,
        }
    }

    /// Update theme colors and clear cache
    pub fn update_colors(&mut self, shadow_color: Color, tint_color: Color) {
        self.shadow_color = shadow_color;
        self.tint_color = tint_color;
        self.material_elevation = MaterialElevation::with_colors(shadow_color, tint_color);
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Set scale factor and clear cache
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale.max(MIN_SCALE_FACTOR);
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Get elevation style with caching
    #[must_use]
    pub fn get_elevation(&self, level: ElevationLevel) -> ElevationStyle {
        let cache_key = ElevationCacheKey {
            level,
            shadow_color: ColorKey(self.shadow_color),
            tint_color: ColorKey(self.tint_color),
            scale: (self.scale * 1000.0).round().clamp(0.0, u32::MAX as f32) as u32, // Scale to avoid floating point issues
        };

        // Try to get from cache first
        if let Ok(cache) = self.cache.lock()
            && let Some(style) = cache.get(&cache_key)
        {
            return style.clone();
        }

        // Compute the style
        let base_style = self.material_elevation.get_level(level);
        let scaled_style = if (self.scale - 1.0).abs() < FLOAT_EPSILON {
            base_style.clone()
        } else {
            MaterialElevation::scale_elevation_style(base_style, self.scale)
        };

        // Cache the result
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(cache_key, scaled_style.clone());
        }

        scaled_style
    }

    /// Get custom elevation by name
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The custom elevation name is not found in the registry
    pub fn get_custom_elevation(&self, name: &str) -> Result<ElevationStyle, ElevationError> {
        self.registry
            .get(name)
            .cloned()
            .ok_or_else(|| ElevationError::CustomNotFound(name.to_string()))
    }

    /// Register a custom elevation
    pub fn register_custom_elevation(&mut self, name: String, style: ElevationStyle) {
        self.registry.register(name, style);
    }

    /// Get elevation for an Elevatable component
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Custom elevation key is not found in the registry
    /// - Component elevation configuration is invalid
    pub fn get_component_elevation<T: crate::styling::material::elevation::Elevatable>(
        &self,
        component: &T,
    ) -> Result<ElevationStyle, ElevationError> {
        component.custom_elevation_key().map_or_else(
            || Ok(self.get_elevation(component.elevation_level())),
            |custom_key| self.get_custom_elevation(custom_key),
        )
    }

    /// Clear the cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }
    /// Get cache statistics
    #[must_use]
    pub fn cache_stats(&self) -> (usize, usize) {
        self.cache
            .lock()
            .map_or((0, 0), |cache| (cache.len(), cache.capacity()))
    }
}
