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

/// Safe animation duration conversion: seconds (f64) to milliseconds (u32), clamped
///
/// - Negative values clamp to 0
/// - Very large values clamp to u32::MAX
/// - Rounds to nearest millisecond
#[must_use]
pub fn duration_secs_to_millis_clamped(duration_secs: f64) -> u32 {
    let millis = (duration_secs * 1000.0).round();
    if !millis.is_finite() {
        return 0;
    }
    if millis < 0.0 {
        0
    } else if millis > f64::from(u32::MAX) {
        u32::MAX
    } else {
        millis as u32
    }
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
