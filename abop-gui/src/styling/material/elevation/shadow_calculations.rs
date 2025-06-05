//! Shadow calculation utilities for Material Design 3 elevation system

use super::{
    ElevationLevel, ShadowParams,
    constants::{
        DEFAULT_SHADOW_OPACITY, MIN_SCALE_FACTOR, MIN_SHADOW_OPACITY, SHADOW_BLUR_MULTIPLIER,
        SHADOW_OFFSET_MULTIPLIER, SHADOW_PARAMS,
    },
};
use iced::{Color, Shadow, Vector};

/// Calculate shadow parameters for a custom elevation value in dp
#[must_use]
pub fn calculate_shadow_params(dp: f32) -> ShadowParams {
    if dp <= 0.0 {
        return ShadowParams {
            offset_y: 0.0,
            blur_radius: 0.0,
            opacity: 0.0,
        };
    }

    // Use Material Design 3 formula for shadow calculation
    // Based on elevation dp value, calculate appropriate offset and blur
    let offset_y = (dp * SHADOW_OFFSET_MULTIPLIER).max(1.0);
    let blur_radius = (dp * SHADOW_BLUR_MULTIPLIER * 2.0).max(2.0);
    let opacity = calculate_shadow_opacity(dp);

    ShadowParams {
        offset_y,
        blur_radius,
        opacity,
    }
}

/// Calculate shadow opacity based on elevation dp value
#[must_use]
pub fn calculate_shadow_opacity(dp: f32) -> f32 {
    if dp <= 0.0 {
        0.0
    } else if dp <= 1.0 {
        DEFAULT_SHADOW_OPACITY * 0.8
    } else if dp <= 3.0 {
        DEFAULT_SHADOW_OPACITY
    } else if dp <= 6.0 {
        DEFAULT_SHADOW_OPACITY * 1.1
    } else if dp <= 8.0 {
        DEFAULT_SHADOW_OPACITY * 1.2
    } else {
        DEFAULT_SHADOW_OPACITY * 1.3
    }
}

/// Calculate surface tint opacity for custom elevation value
#[must_use]
pub fn calculate_custom_tint_opacity(dp: f32) -> f32 {
    if dp <= 0.0 {
        0.0
    } else if dp <= 1.0 {
        0.05
    } else if dp <= 3.0 {
        0.08
    } else if dp <= 6.0 {
        0.11
    } else if dp <= 8.0 {
        0.12
    } else {
        0.14
    }
}

/// Create an iced Shadow from shadow parameters and color
#[must_use]
pub const fn create_shadow(params: ShadowParams, color: Color) -> Shadow {
    Shadow {
        color: Color {
            a: params.opacity,
            ..color
        },
        offset: Vector::new(0.0, params.offset_y),
        blur_radius: params.blur_radius,
    }
}

/// Calculate shadow for a specific elevation level with custom color
#[must_use]
pub const fn calculate_elevation_shadow(level: ElevationLevel, shadow_color: Color) -> Shadow {
    let idx = level.as_u8() as usize;
    let params = if idx < SHADOW_PARAMS.len() {
        SHADOW_PARAMS[idx]
    } else {
        SHADOW_PARAMS[0]
    };
    create_shadow(params, shadow_color)
}

/// Calculate shadow for custom elevation value
#[must_use]
pub fn calculate_custom_shadow(dp: f32, shadow_color: Color) -> Shadow {
    if dp == 0.0 {
        return Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::ZERO,
            blur_radius: 0.0,
        };
    }
    let offset_y = (dp * SHADOW_OFFSET_MULTIPLIER).max(1.0);
    let blur_radius = dp * SHADOW_BLUR_MULTIPLIER;
    let opacity = (DEFAULT_SHADOW_OPACITY * (dp / 12.0).min(1.0)).max(MIN_SHADOW_OPACITY);
    Shadow {
        color: Color {
            a: opacity,
            ..shadow_color
        },
        offset: Vector::new(0.0, offset_y),
        blur_radius,
    }
}

/// Scale shadow parameters by a factor
#[must_use]
pub fn scale_shadow_params(params: ShadowParams, scale: f32) -> ShadowParams {
    let scale = scale.max(MIN_SCALE_FACTOR);
    ShadowParams {
        offset_y: params.offset_y * scale,
        blur_radius: params.blur_radius * scale,
        opacity: params.opacity.min(1.0), // Don't scale opacity beyond 1.0
    }
}

/// Interpolate between two shadow parameter sets
#[must_use]
pub fn interpolate_shadow_params(from: ShadowParams, to: ShadowParams, t: f32) -> ShadowParams {
    let t = t.clamp(0.0, 1.0);
    ShadowParams {
        offset_y: (to.offset_y - from.offset_y).mul_add(t, from.offset_y),
        blur_radius: (to.blur_radius - from.blur_radius).mul_add(t, from.blur_radius),
        opacity: (to.opacity - from.opacity).mul_add(t, from.opacity),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_shadow_params() {
        let params = calculate_shadow_params(3.0);
        assert!(params.offset_y > 0.0);
        assert!(params.blur_radius > 0.0);
        assert!(params.opacity > 0.0);
    }

    #[test]
    fn test_zero_elevation() {
        let params = calculate_shadow_params(0.0);
        assert_eq!(params.offset_y, 0.0);
        assert_eq!(params.blur_radius, 0.0);
        assert_eq!(params.opacity, 0.0);
    }

    #[test]
    fn test_shadow_opacity_calculation() {
        assert_eq!(calculate_shadow_opacity(0.0), 0.0);
        assert!(calculate_shadow_opacity(1.0) > 0.0);
        assert!(calculate_shadow_opacity(6.0) > calculate_shadow_opacity(3.0));
    }

    #[test]
    fn test_tint_opacity_calculation() {
        assert_eq!(calculate_custom_tint_opacity(0.0), 0.0);
        assert!(calculate_custom_tint_opacity(1.0) > 0.0);
        assert!(calculate_custom_tint_opacity(8.0) > calculate_custom_tint_opacity(3.0));
    }

    #[test]
    fn test_scale_shadow_params() {
        let params = ShadowParams {
            offset_y: 2.0,
            blur_radius: 4.0,
            opacity: 0.15,
        };
        let scaled = scale_shadow_params(params, 2.0);
        assert_eq!(scaled.offset_y, 4.0);
        assert_eq!(scaled.blur_radius, 8.0);
        assert_eq!(scaled.opacity, 0.15); // Opacity shouldn't scale
    }

    #[test]
    fn test_interpolate_shadow_params() {
        let from = ShadowParams {
            offset_y: 1.0,
            blur_radius: 2.0,
            opacity: 0.1,
        };
        let to = ShadowParams {
            offset_y: 3.0,
            blur_radius: 6.0,
            opacity: 0.2,
        };
        let mid = interpolate_shadow_params(from, to, 0.5);
        assert_eq!(mid.offset_y, 2.0);
        assert_eq!(mid.blur_radius, 4.0);
        assert_eq!(mid.opacity, 0.15);
    }
}
