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
