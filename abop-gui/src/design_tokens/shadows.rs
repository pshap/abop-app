//! Shadow definitions for consistent elevation effects
//!
//! # Deprecation Notice
//! This module is being phased out in favor of the comprehensive Material Design 3
//! elevation system in `crate::styling::material::elevation`. The constants in this
//! module provide basic compatibility but do not follow Material Design 3 specifications.
//!
//! ## Migration Path
//! Instead of using these constants directly, use:
//! - `MaterialTokens::elevation_shadow(level)` for proper MD3 shadows
//! - `MaterialTokens::elevation_style(level)` for complete elevation styling
//!
//! ## Material Design 3 Compliance Issues
//! - Shadow values don't match MD3 elevation levels (0,1,3,6,8,12dp)
//! - Missing surface tinting component required by MD3
//! - Opacity values don't follow MD3 standard (0.15)

use iced::{Color, Shadow, Vector};

/// Soft shadow for low elevation
///
/// # Deprecation Warning
/// Use `MaterialTokens::elevation_shadow(1)` instead for MD3 compliance.
/// This constant maps approximately to Material Design elevation level 1.
#[deprecated(
    since = "0.1.0",
    note = "Use MaterialTokens::elevation_shadow(1) for Material Design 3 compliance"
)]
pub const SHADOW_SOFT: Shadow = Shadow {
    color: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.1,
    },
    offset: Vector { x: 0.0, y: 1.0 },
    blur_radius: 3.0,
};

/// Medium shadow for moderate elevation
///
/// # Deprecation Warning
/// Use `MaterialTokens::elevation_shadow(3)` instead for MD3 compliance.
/// This constant maps approximately to Material Design elevation level 3.
#[deprecated(
    since = "0.1.0",
    note = "Use MaterialTokens::elevation_shadow(3) for Material Design 3 compliance"
)]
pub const SHADOW_MEDIUM: Shadow = Shadow {
    color: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.2,
    },
    offset: Vector { x: 0.0, y: 2.0 },
    blur_radius: 4.0,
};

/// Strong shadow for high elevation
///
/// # Deprecation Warning
/// Use `MaterialTokens::elevation_shadow(5)` instead for MD3 compliance.
/// This constant maps approximately to Material Design elevation level 5.
#[deprecated(
    since = "0.1.0",
    note = "Use MaterialTokens::elevation_shadow(5) for Material Design 3 compliance"
)]
pub const SHADOW_STRONG: Shadow = Shadow {
    color: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.25,
    },
    offset: Vector { x: 0.0, y: 3.0 },
    blur_radius: 6.0,
};
