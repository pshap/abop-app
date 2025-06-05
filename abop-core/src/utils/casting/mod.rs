//! Unified safe casting utilities for numeric types
//!
//! This module provides safe conversion functions between numeric types with proper
//! bounds checking and error handling. The functionality is organized into several modules:
//! - `core`: Basic numeric type conversions
//! - `domain`: Domain-specific conversions (audio, database, UI, etc.)
//! - `builder`: Configurable casting operations
//! - `error`: Error types and handling

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

mod core;
pub use self::core::*;

mod error;
pub use self::error::*;

pub mod builder;
pub mod domain;

// Re-export commonly used items at the module root
pub use builder::CastingBuilder;
pub use domain::audio::*;
pub use domain::db::*;
pub use domain::file_size::{
    FileSizePrecision, format_file_size, format_file_size_exact, format_file_size_standard,
};
pub use domain::ui::*;

// Legacy re-exports for backward compatibility
pub mod audio_conversions {
    //! Audio-specific conversions (legacy module)
    //!
    //! This module is kept for backward compatibility. Prefer using the top-level module functions.

    pub use super::domain::audio::*;
}

pub mod db_conversions {
    //! Database-specific conversions (legacy module)
    //!
    //! This module is kept for backward compatibility. Prefer using the top-level module functions.

    pub use super::domain::db::max_safe_db_count;
    pub use super::domain::db::safe_db_count_to_usize;
    pub use super::domain::db::safe_usize_to_i64;
    pub use super::domain::db::validate_db_count;
}

pub mod ui_conversions {
    //! UI-specific conversions (legacy module)
    //!
    //! This module is kept for backward compatibility. Prefer using the top-level module functions.

    pub use super::domain::ui::*;
}

// For backward compatibility
#[deprecated(note = "Use `domain::file_size::FileSizePrecision` instead")]
pub use domain::file_size::FileSizePrecision as FileSizePrecisionCompat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_int_cast() {
        // Test u8 to i32
        let result: i32 = safe_int_cast::<u8, i32>(42u8, 100).unwrap();
        assert_eq!(result, 42i32);

        // Test i32 to u32 with negative value (should error)
        assert!(safe_int_cast::<i32, u32>(-1i32, 100).is_err());

        // Test u8 to u32
        let result: u32 = safe_int_cast::<u8, u32>(42u8, 100).unwrap();
        assert_eq!(result, 42u32);
    }

    #[test]
    fn test_float_to_int() {
        // Test f32 to i32 with whole number
        let result: i32 = float_to_int::<i32, f32>(42.0f32, 100.0).unwrap();
        assert_eq!(result, 42i32);

        // Test with fractional part (should error with PrecisionLoss)
        assert!(matches!(
            float_to_int::<i32, f32>(42.5f32, 100.0),
            Err(CastError::PrecisionLoss(_))
        ));

        // Test NaN (should error)
        assert!(float_to_int::<i32, f32>(f32::NAN, 100.0).is_err());

        // Test f64 to i64 with whole number
        let result: i64 = float_to_int::<i64, f64>(123.0f64, 200.0).unwrap();
        assert_eq!(result, 123i64);

        // Test with fractional part (should error with PrecisionLoss)
        assert!(matches!(
            float_to_int::<i64, f64>(123.7f64, 200.0),
            Err(CastError::PrecisionLoss(_))
        ));
    }
}
