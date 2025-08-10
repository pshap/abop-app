//! UI-specific conversion utilities

use super::super::error::{CastError, DomainCastError};

/// Safe conversion for UI spacing values
///
/// # Errors
/// Returns domain-specific UI errors
pub fn safe_spacing_to_pixels(spacing: f32) -> Result<u16, DomainCastError> {
    use crate::utils::casting::error::domain::UiCastError;

    if !spacing.is_finite() {
        return Err(UiCastError::InvalidSpacing(spacing).into());
    }

    if spacing < 0.0 {
        return Err(CastError::NegativeValue(spacing.to_string()).into());
    }

    // Clamp to u16::MAX and round to nearest integer
    let pixels = spacing.clamp(0.0, u16::MAX as f32).round();

    // Check if the rounded value is within u16 range
    if pixels > u16::MAX as f32 {
        return Err(CastError::ValueTooLarge(spacing.to_string(), u16::MAX.to_string()).into());
    }

    Ok(pixels as u16)
}

/// Infallible, clamped spacing-to-pixels conversion used by UI components
///
/// - Negative values clamp to 0
/// - Values above u16::MAX clamp to u16::MAX
/// - Values are rounded to nearest integer
#[must_use]
pub fn spacing_to_pixels_clamped(spacing: f32) -> u16 {
    spacing.round().clamp(0.0, u16::MAX as f32) as u16
}

/// Safe animation duration conversion: seconds (f64) to milliseconds (u32)
///
/// # Errors
/// Returns domain-specific UI errors for invalid duration values
pub fn safe_duration_secs_to_millis(duration_secs: f64) -> Result<u32, DomainCastError> {
    use crate::utils::casting::error::domain::UiCastError;

    let millis = (duration_secs * 1000.0).round();
    
    if !millis.is_finite() {
        return Err(UiCastError::InvalidDuration(duration_secs as f32).into());
    }

    if millis < 0.0 {
        return Err(CastError::NegativeValue(duration_secs.to_string()).into());
    }

    if millis > f64::from(u32::MAX) {
        return Err(CastError::ValueTooLarge(duration_secs.to_string(), u32::MAX.to_string()).into());
    }

    Ok(millis as u32)
}

/// Infallible animation duration conversion: seconds (f64) to milliseconds (u32), clamped
///
/// This is a convenience function that handles edge cases gracefully:
/// - Negative values clamp to 0
/// - Very large values clamp to u32::MAX
/// - Rounds to nearest millisecond  
/// - Non-finite values (NaN, infinity) are treated as 0 for UI safety
///
/// For error-reporting behavior, use [`safe_duration_secs_to_millis`] instead.
#[must_use]
pub fn duration_secs_to_millis_clamped(duration_secs: f64) -> u32 {
    let millis = (duration_secs * 1000.0).round();
    if !millis.is_finite() {
        return 0; // Treat non-finite values as 0 duration for UI safety
    }
    millis.clamp(0.0, f64::from(u32::MAX)) as u32
}

/// Infallible opacity to alpha conversion (0.0..=1.0 -> 0..=255)
#[must_use]
pub fn opacity_to_alpha_clamped(opacity: f32) -> u8 {
    let clamped = opacity.clamp(0.0, 1.0);
    (clamped * 255.0).round() as u8
}

/// Infallible color component conversion (0.0..=1.0 -> 0..=255)
#[must_use]
pub fn color_component_to_u8_clamped(component: f32) -> u8 {
    let clamped = component.clamp(0.0, 1.0);
    (clamped * 255.0).round() as u8
}

/// Infallible level-to-spacing conversion used by nested UI elements
///
/// Uses 24.0 logical pixels per level, clamps to a reasonable max depth (50)
#[must_use]
pub fn level_to_spacing_clamped(level: usize) -> f32 {
    const SPACING_PER_LEVEL: f32 = 24.0;
    const MAX_LEVELS: usize = 50;
    let clamped = level.min(MAX_LEVELS);
    (clamped as f32) * SPACING_PER_LEVEL
}

/// Infallible progress percentage calculation as 0.0..=100.0
#[must_use]
pub fn progress_percentage_clamped(current: usize, total: usize) -> f32 {
    if total == 0 {
        return 0.0;
    }
    let pct = (current as f64 / total as f64) * 100.0;
    pct.clamp(0.0, 100.0) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_to_pixels_clamped() {
        assert_eq!(spacing_to_pixels_clamped(0.0), 0);
        assert_eq!(spacing_to_pixels_clamped(10.5), 11);
        assert_eq!(spacing_to_pixels_clamped(-5.0), 0);
        assert_eq!(spacing_to_pixels_clamped(u16::MAX as f32 + 10.0), u16::MAX);
    }

    #[test]
    fn test_safe_duration_secs_to_millis() {
        assert_eq!(safe_duration_secs_to_millis(1.0).unwrap(), 1000);
        assert_eq!(safe_duration_secs_to_millis(1.5).unwrap(), 1500);
        assert!(safe_duration_secs_to_millis(-1.0).is_err());
        assert!(safe_duration_secs_to_millis(f64::INFINITY).is_err());
        assert!(safe_duration_secs_to_millis(f64::NAN).is_err());
    }

    #[test]
    fn test_duration_secs_to_millis_clamped() {
        assert_eq!(duration_secs_to_millis_clamped(0.0), 0);
        assert_eq!(duration_secs_to_millis_clamped(1.0), 1000);
        assert_eq!(duration_secs_to_millis_clamped(1.5), 1500);
        assert_eq!(duration_secs_to_millis_clamped(-1.0), 0);
        assert_eq!(duration_secs_to_millis_clamped(f64::INFINITY), 0); // non-finite -> 0
    }

    #[test]
    fn test_opacity_and_color_component_clamped() {
        assert_eq!(opacity_to_alpha_clamped(0.0), 0);
        assert_eq!(opacity_to_alpha_clamped(1.0), 255);
        assert_eq!(opacity_to_alpha_clamped(0.5), 128);
        assert_eq!(opacity_to_alpha_clamped(-0.1), 0);
        assert_eq!(opacity_to_alpha_clamped(1.1), 255);

        assert_eq!(color_component_to_u8_clamped(0.0), 0);
        assert_eq!(color_component_to_u8_clamped(1.0), 255);
        assert_eq!(color_component_to_u8_clamped(0.5), 128);
        assert_eq!(color_component_to_u8_clamped(-0.1), 0);
        assert_eq!(color_component_to_u8_clamped(1.1), 255);
    }

    #[test]
    fn test_level_and_progress_clamped() {
        assert_eq!(level_to_spacing_clamped(0), 0.0);
        assert_eq!(level_to_spacing_clamped(1), 24.0);
        assert_eq!(level_to_spacing_clamped(2), 48.0);
        assert_eq!(level_to_spacing_clamped(100), level_to_spacing_clamped(50));

        assert_eq!(progress_percentage_clamped(0, 100), 0.0);
        assert_eq!(progress_percentage_clamped(50, 100), 50.0);
        assert_eq!(progress_percentage_clamped(100, 100), 100.0);
        assert_eq!(progress_percentage_clamped(10, 0), 0.0);
    }
}
