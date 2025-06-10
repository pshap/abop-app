//! Core utility modules for the application
//!
//! This module contains various utility functions and types used throughout the application.
//!
//! # Public API
//!
//! The module provides specific exports instead of glob imports for better clarity:
//! - Casting utilities: [`CastingBuilder`], [`CastError`], [`CastResult`]
//! - Enhanced utilities: [`audio`], [`database`], [`ui`], [`file`] modules
//! - Size formatting: [`format_bytes`]
//! - Time utilities: [`format_seconds`], [`format_duration`], [`TimeFormat`]
//! - Timer: [`Timer`]

pub mod casting;
pub mod enhanced;
pub mod size;
pub mod time;
pub mod timer;

// Re-export commonly used utilities (specific items)
pub use casting::{CastError, CastResult, CastingBuilder};
pub use enhanced::{audio, database, file, ui};
pub use size::format_bytes;
pub use time::{TimeFormat, format_duration, format_seconds};
pub use timer::Timer;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::time::TimeFormat;

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

    #[test]
    fn test_enhanced_utilities() {
        // Test audio utilities
        let tracks = vec![(44100, 44100), (48000, 48000)];
        let duration = enhanced::audio::calculate_total_duration(&tracks).unwrap();
        assert!((duration - 2.0).abs() < 0.01);

        // Test database utilities
        let pagination = enhanced::database::calculate_pagination(100, 10, 3).unwrap();
        assert_eq!(pagination.total_pages, 10);

        // Test UI utilities
        let layout = enhanced::ui::calculate_grid_layout(800.0, 150.0, 10.0).unwrap();
        assert!(layout.columns >= 1);
    }
}
