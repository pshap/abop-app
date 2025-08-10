//! Core utility modules for the application
//!
//! This module contains various utility functions and types used throughout the application.
//!
//! # Public API
//!
//! The module provides specific exports instead of glob imports for better clarity:
//! - Casting utilities: [`CastingBuilder`], [`CastError`], [`CastResult`]
//! - Enhanced utilities: [`audio`], [`database`], [`ui`], [`file`] modules
//! - Size formatting: [`format_file_size_standard`], [`format_file_size_exact`], [`format_file_size`]
//! - Time utilities: [`format_seconds`], [`format_duration`], [`TimeFormat`]
//! - Timer: [`Timer`]

pub mod casting;
pub mod enhanced;
pub mod path;
pub mod time;
pub mod timer;

// Re-export commonly used utilities (specific items)
pub use casting::{
    CastError, CastResult, CastingBuilder, FileSizePrecision, format_file_size,
    format_file_size_exact, format_file_size_standard,
};
pub use enhanced::{audio, database, file, ui};
pub use path::{
    extension_matches, normalize_path_for_comparison, paths_equal, paths_equal_case_insensitive,
};
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
    fn test_file_size_formatting() {
        // Canonical helpers
        assert_eq!(format_file_size_standard(1024), "1.00 KB");
        assert_eq!(format_file_size_standard(1_048_576), "1.00 MB");
        assert_eq!(format_file_size_exact(500), "500 B");
        assert_eq!(format_file_size(1536, FileSizePrecision::Standard), "1.50 KB");
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
