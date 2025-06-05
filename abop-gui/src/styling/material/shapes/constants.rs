//! Constants for the Material Design 3 Shape System
//!
//! This module contains all the constant values used throughout the shape system,
//! including thresholds, scale factors, and configuration values.

/// Threshold for considering a shape "circular" - any radius above this is treated as circular
pub const CIRCULAR_THRESHOLD: f32 = 100.0;

/// Number of standard Material Design shape sizes
#[allow(dead_code)]
pub const SHAPE_COUNT: usize = 7;

/// Default scale factor for responsive calculations
pub const DEFAULT_RESPONSIVE_FACTOR: f32 = 0.1;

/// Scale factor for hovered state (slightly larger)
pub const HOVER_SCALE: f32 = 1.1;

/// Scale factor for pressed state (slightly smaller)
pub const PRESSED_SCALE: f32 = 0.95;

/// Scale factor for disabled state (noticeably smaller)
pub const DISABLED_SCALE: f32 = 0.9;
