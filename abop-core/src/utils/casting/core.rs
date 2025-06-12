//! Core numeric type casting utilities with bounds and precision checking.

use std::fmt::Display;

use crate::utils::casting::error::CastError;

/// Result type for casting operations
pub type CastResult<T> = Result<T, CastError>;

/// Precision limits and casting constants
pub mod precision_limits {
    /// Maximum integer value that can be exactly represented in f32 (2^24)
    pub const MAX_EXACT_F32_INT: f32 = 16_777_216.0;

    /// Maximum integer value that can be exactly represented in f64 (2^53)
    pub const MAX_EXACT_F64_INT: f64 = 9_007_199_254_740_992.0;

    /// Epsilon for floating-point comparisons (f32)
    pub const EPSILON_F32: f32 = 1e-6;

    /// Epsilon for floating-point comparisons (f64)
    pub const EPSILON_F64: f64 = 1e-12;
}

/// Safely convert between integer types with bounds checking
///
/// # Errors
/// Returns `CastError` if:
/// - The value is negative and the target type is unsigned
/// - The value exceeds the maximum allowed value
/// - The conversion to the target type fails
pub fn safe_int_cast<T, U>(value: T, max_allowed: u128) -> CastResult<U>
where
    T: TryInto<i128> + Copy + Display,
    U: TryFrom<i128> + Display,
{
    let signed_value: i128 = value
        .try_into()
        .map_err(|_| CastError::ValueTooLarge(value.to_string(), max_allowed.to_string()))?;

    if signed_value < 0 {
        return Err(CastError::NegativeValue(value.to_string()));
    }

    let unsigned_value = signed_value as u128;
    if unsigned_value > max_allowed {
        return Err(CastError::ValueTooLarge(
            value.to_string(),
            max_allowed.to_string(),
        ));
    }

    U::try_from(signed_value)
        .map_err(|_| CastError::ValueTooLarge(value.to_string(), "target type".to_string()))
}

/// Safely convert a floating-point value to an integer type
///
/// # Errors
/// Returns `CastError` if:
/// - The value is not finite (NaN or infinity)
/// - The value is negative and the target type is unsigned
/// - The value exceeds the maximum allowed value
/// - Precision would be lost in the conversion
pub fn float_to_int<T, F>(value: F, max_allowed: f64) -> CastResult<T>
where
    T: TryFrom<i128> + Display,
    F: Into<f64>,
{
    let value_f64 = value.into();

    if !value_f64.is_finite() {
        return Err(CastError::NotFinite(value_f64));
    }

    if value_f64 < 0.0 {
        return Err(CastError::NegativeValue(value_f64.to_string()));
    }

    if value_f64 > max_allowed {
        return Err(CastError::ValueTooLarge(
            value_f64.to_string(),
            max_allowed.to_string(),
        ));
    }

    // Check for fractional part
    if value_f64.fract() != 0.0 {
        return Err(CastError::PrecisionLoss(value_f64));
    }

    // Check round-trip conversion
    let value_i128 = value_f64 as i128;
    if (value_i128 as f64 - value_f64).abs() > f64::EPSILON {
        return Err(CastError::PrecisionLoss(value_f64));
    }

    T::try_from(value_i128).map_err(|_| {
        CastError::ValueTooLarge(value_f64.to_string(), "target integer type".to_string())
    })
}

/// Safely convert between floating-point types with precision checking
///
/// # Errors
/// Returns `CastError` if:
/// - The value is not finite (NaN or infinity)
/// - The absolute value exceeds the maximum allowed value
/// - Precision would be lost in the conversion to the target type
pub fn float_to_float<T, F>(value: F, max_abs: f64) -> CastResult<T>
where
    T: TryFrom<f64> + Display,
    F: Into<f64>,
{
    let value_f64 = value.into();

    if !value_f64.is_finite() {
        return Err(CastError::NotFinite(value_f64));
    }

    if value_f64.abs() > max_abs {
        return Err(CastError::ValueTooLarge(
            value_f64.to_string(),
            max_abs.to_string(),
        ));
    }

    // Check if the value can be exactly represented in the target type
    let value_f32 = value_f64 as f32;
    if (value_f32 as f64 - value_f64).abs() > f64::EPSILON {
        return Err(CastError::PrecisionLoss(value_f64));
    }

    T::try_from(value_f64).map_err(|_| {
        CastError::ValueTooLarge(value_f64.to_string(), "target float type".to_string())
    })
}
