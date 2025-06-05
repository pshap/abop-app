//! Configuration types for the casting builder

/// Precision handling mode for numeric conversions
#[derive(Debug, Clone, Copy)]
pub enum PrecisionMode {
    /// Strict - fail on any precision loss
    Strict,
    /// Tolerant - allow minor precision loss within epsilon
    Tolerant {
        /// Maximum allowed precision error
        epsilon: f64,
    },
    /// Adaptive - use best available precision for target type
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowBehavior {
    /// Fail on overflow
    Fail,
    /// Clamp to maximum valid value
    Clamp,
    /// Saturate at type boundaries
    Saturate,
}

/// Rounding mode for float to integer conversions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    /// Round to nearest integer
    Nearest,
    /// Always round down (floor)
    Floor,
    /// Always round up (ceiling)
    Ceiling,
    /// Truncate towards zero
    Truncate,
}

/// Validation level for input checking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationLevel {
    /// No validation (fastest)
    None,
    /// Basic validation (finite, range)
    Basic,
    /// Full validation (all checks)
    Full,
}

impl Default for ValidationLevel {
    fn default() -> Self {
        Self::Full
    }
}
