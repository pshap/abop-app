//! Constants and parameters for the Material Design 3 elevation system

use super::ShadowParams;

/// Default shadow opacity for Material Design elevation
pub const DEFAULT_SHADOW_OPACITY: f32 = 0.15;

/// Minimum shadow opacity for custom elevations
pub const MIN_SHADOW_OPACITY: f32 = 0.05;

/// Shadow offset multiplier for custom elevations
pub const SHADOW_OFFSET_MULTIPLIER: f32 = 0.5;

/// Shadow blur multiplier for custom elevations
pub const SHADOW_BLUR_MULTIPLIER: f32 = 1.0;

/// Maximum valid elevation level
pub const MAX_ELEVATION_LEVEL: u8 = 5;

/// Minimum scale factor for elevation
pub const MIN_SCALE_FACTOR: f32 = 0.0;

/// Level calculation multiplier for custom elevation dp values
pub const LEVEL_CALCULATION_DIVISOR: f32 = 2.0;

/// Number of elevation levels in the system
pub const ELEVATION_LEVEL_COUNT: usize = 6;

/// Surface tint opacity values for each elevation level
pub const TINT_OPACITIES: [f32; 6] = [0.0, 0.05, 0.08, 0.11, 0.12, 0.14];

/// Material Design shadow parameters for each elevation level
pub const SHADOW_PARAMS: [ShadowParams; 6] = [
    ShadowParams {
        offset_y: 0.0,
        blur_radius: 0.0,
        opacity: 0.0,
    }, // Level 0
    ShadowParams {
        offset_y: 1.0,
        blur_radius: 2.0,
        opacity: DEFAULT_SHADOW_OPACITY,
    }, // Level 1
    ShadowParams {
        offset_y: 2.0,
        blur_radius: 4.0,
        opacity: DEFAULT_SHADOW_OPACITY,
    }, // Level 2
    ShadowParams {
        offset_y: 3.0,
        blur_radius: 6.0,
        opacity: DEFAULT_SHADOW_OPACITY,
    }, // Level 3
    ShadowParams {
        offset_y: 4.0,
        blur_radius: 8.0,
        opacity: DEFAULT_SHADOW_OPACITY,
    }, // Level 4
    ShadowParams {
        offset_y: 6.0,
        blur_radius: 12.0,
        opacity: DEFAULT_SHADOW_OPACITY,
    }, // Level 5
];
