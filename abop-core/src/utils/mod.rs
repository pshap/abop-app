//! Core utility modules for the application
//!
//! This module contains various utility functions and types used throughout the application.

pub mod casting;
pub mod size;
pub mod time;
pub mod timer;

// Re-export commonly used utilities
pub use casting::*;
pub use size::*;
pub use time::*;
pub use timer::*;

// Legacy re-exports for backward compatibility
pub mod time_utils {
    //! Time formatting utilities (legacy module)
    //!
    //! This module is kept for backward compatibility. Prefer using the top-level time module.
    //!
    //! # Examples
    //! ```
    //! use abop_core::utils::time::*;
    //! ```
    pub use super::time::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_seconds() {
        assert_eq!(format_seconds(65, TimeFormat::HoursWhenNonZero), "01:05");
        assert_eq!(format_seconds(3665, TimeFormat::CompactHours), "1:01:05");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1 KB");
        assert_eq!(format_bytes(1_048_576), "1 MB");
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1536), "1.5 KB");
    }
}
