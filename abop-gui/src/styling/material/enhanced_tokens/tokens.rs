//! Elevation, Shape, and Motion Token Definitions
//!
//! This module contains elevation, shape, and motion token definitions
//! following Material Design 3 specifications.

/// Elevation tokens for shadow and depth effects
#[derive(Debug, Clone)]
pub struct ElevationTokens {
    /// Level 0 (0dp) - No elevation
    pub level0: f32,
    /// Level 1 (1dp) - Low elevation
    pub level1: f32,
    /// Level 2 (3dp) - Medium-low elevation
    pub level2: f32,
    /// Level 3 (6dp) - Medium elevation
    pub level3: f32,
    /// Level 4 (8dp) - Medium-high elevation
    pub level4: f32,
    /// Level 5 (12dp) - High elevation
    pub level5: f32,
}

impl Default for ElevationTokens {
    fn default() -> Self {
        Self {
            level0: 0.0,
            level1: 1.0,
            level2: 3.0,
            level3: 6.0,
            level4: 8.0,
            level5: 12.0,
        }
    }
}

/// Shape tokens for Material Design 3 shape system
#[derive(Debug, Clone)]
pub struct ShapeTokens {
    /// Extra small corner radius (4dp)
    pub corner_xs: f32,
    /// Small corner radius (8dp)
    pub corner_sm: f32,
    /// Medium corner radius (12dp)
    pub corner_md: f32,
    /// Large corner radius (16dp)
    pub corner_lg: f32,
    /// Extra large corner radius (28dp)
    pub corner_xl: f32,
    /// Full corner radius (50% of height)
    pub corner_full: f32,
}

impl Default for ShapeTokens {
    fn default() -> Self {
        Self {
            corner_xs: 4.0,
            corner_sm: 8.0,
            corner_md: 12.0,
            corner_lg: 16.0,
            corner_xl: 28.0,
            corner_full: 100.0, // Large value to ensure full rounding
        }
    }
}

/// Motion tokens for Material Design 3 motion system
#[derive(Debug, Clone)]
pub struct MotionTokens {
    /// Short duration (150ms)
    pub duration_short: u64,
    /// Medium duration (250ms)
    pub duration_medium: u64,
    /// Long duration (400ms)
    pub duration_long: u64,
    /// Extra long duration (700ms)
    pub duration_extra_long: u64,
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            duration_short: 150,
            duration_medium: 250,
            duration_long: 400,
            duration_extra_long: 700,
        }
    }
}
