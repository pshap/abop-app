//! Configuration types for the casting builder
//!
//! This module defines the configuration types used by the `CastingBuilder` to control
//! how numeric conversions are performed. These settings allow fine-tuning of precision,
//! overflow handling, rounding behavior, and validation levels for different use cases.

/// Precision handling mode for numeric conversions
///
/// Controls how strictly numeric precision is maintained during conversions.
/// Different modes are suitable for different use cases:
///
/// - `Strict`: Best for financial calculations and database operations
/// - `Tolerant`: Good for audio processing where minor precision loss is acceptable
/// - `Adaptive`: Ideal for UI calculations where performance is important
#[derive(Debug, Clone, Copy)]
pub enum PrecisionMode {
    /// Strict - fail on any precision loss
    ///
    /// This mode ensures that no precision is lost during conversions.
    /// It is suitable for financial calculations and database operations
    /// where exact values must be preserved.
    Strict,
    /// Tolerant - allow minor precision loss within epsilon
    ///
    /// This mode allows small precision errors up to the specified epsilon value.
    /// It is useful for audio processing and scientific calculations where
    /// minor precision loss is acceptable.
    Tolerant {
        /// Maximum allowed precision error
        ///
        /// Values smaller than this epsilon are considered equal.
        /// For example, an epsilon of 1e-6 means that values within
        /// ±0.000001 of each other are considered equal.
        epsilon: f64,
    },
    /// Adaptive - use best available precision for target type
    ///
    /// This mode automatically adjusts precision requirements based on
    /// the target type's capabilities. It is ideal for UI calculations
    /// where performance is important and exact precision is not critical.
    Adaptive,
}

impl PartialEq for PrecisionMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Strict, Self::Strict) => true,
            (Self::Adaptive, Self::Adaptive) => true,
            (Self::Tolerant { epsilon: e1 }, Self::Tolerant { epsilon: e2 }) => {
                (e1 - e2).abs() < f64::EPSILON
            }
            _ => false,
        }
    }
}

impl Eq for PrecisionMode {}

/// Overflow handling behavior for numeric conversions
///
/// Controls how numeric overflow is handled during conversions.
/// Different behaviors are suitable for different use cases:
///
/// - `Fail`: Best for financial calculations and database operations
/// - `Clamp`: Good for UI calculations and audio processing
/// - `Saturate`: Useful for signal processing and real-time applications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowBehavior {
    /// Fail on overflow
    ///
    /// Returns an error if the value would overflow the target type.
    /// This is the safest option but may require additional error handling.
    Fail,
    /// Clamp to maximum valid value
    ///
    /// Clamps the value to the maximum valid value for the target type.
    /// This is useful for UI calculations where graceful degradation is desired.
    Clamp,
    /// Saturate at type boundaries
    ///
    /// Similar to clamp but preserves the sign of the value.
    /// Useful for signal processing where saturation is a valid behavior.
    Saturate,
}

/// Rounding mode for float to integer conversions
///
/// Controls how floating-point values are rounded to integers.
/// Different modes are suitable for different use cases:
///
/// - `Nearest`: Best for general-purpose calculations
/// - `Floor`: Useful for financial calculations (rounding down)
/// - `Ceiling`: Useful for resource allocation (rounding up)
/// - `Truncate`: Good for database operations (rounding toward zero)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    /// Round to nearest integer
    ///
    /// Rounds to the closest integer, with ties going to the even number.
    /// This is the most commonly used rounding mode for general calculations.
    Nearest,
    /// Always round down (floor)
    ///
    /// Always rounds toward negative infinity.
    /// Useful for financial calculations where rounding down is required.
    Floor,
    /// Always round up (ceiling)
    ///
    /// Always rounds toward positive infinity.
    /// Useful for resource allocation where rounding up is required.
    Ceiling,
    /// Truncate towards zero
    ///
    /// Removes the fractional part, effectively rounding toward zero.
    /// Useful for database operations and integer division.
    Truncate,
}

/// Validation level for input checking
///
/// Controls how strictly input values are validated during conversions.
/// Different levels are suitable for different use cases:
///
/// - `None`: Best for performance-critical real-time processing
/// - `Basic`: Good for most general-purpose calculations
/// - `Full`: Best for financial calculations and database operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationLevel {
    /// No validation (fastest)
    ///
    /// Skips all validation checks for maximum performance.
    /// Use this only when you are certain the input values are valid.
    None,
    /// Basic validation (finite, range)
    ///
    /// Performs essential validation like checking for finite values
    /// and basic range checks. This is a good balance of safety and performance.
    Basic,
    /// Full validation (all checks)
    ///
    /// Performs all possible validation checks including precision,
    /// range, and domain-specific validations. This is the safest option
    /// but may impact performance.
    Full,
}

impl Default for ValidationLevel {
    fn default() -> Self {
        Self::Full
    }
}
