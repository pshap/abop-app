//! Tests for the utils module.
//!
//! This module contains unit tests for utility functions.

use abop_core::utils::*;
use std::time::Duration;

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(
            format_duration(Duration::from_secs(0), TimeFormat::CompactHours),
            "0:00"
        );
        assert_eq!(
            format_duration(Duration::from_secs(1), TimeFormat::CompactHours),
            "0:01"
        );
        assert_eq!(
            format_duration(Duration::from_secs(60), TimeFormat::CompactHours),
            "1:00"
        );
        assert_eq!(
            format_duration(Duration::from_secs(61), TimeFormat::CompactHours),
            "1:01"
        );
        assert_eq!(
            format_duration(Duration::from_secs(3600), TimeFormat::CompactHours),
            "1:00:00"
        );
        assert_eq!(
            format_duration(Duration::from_secs(3661), TimeFormat::CompactHours),
            "1:01:01"
        );
    }

    #[test]
    fn test_format_duration_fractional() {
        assert_eq!(
            format_duration(Duration::from_secs_f64(0.0), TimeFormat::CompactHours),
            "0:00"
        );
        assert_eq!(
            format_duration(Duration::from_secs_f64(1.0), TimeFormat::CompactHours),
            "0:01"
        );
        assert_eq!(
            format_duration(Duration::from_secs_f64(60.0), TimeFormat::CompactHours),
            "1:00"
        );
        assert_eq!(
            format_duration(Duration::from_secs_f64(61.0), TimeFormat::CompactHours),
            "1:01"
        );
        assert_eq!(
            format_duration(Duration::from_secs_f64(3600.0), TimeFormat::CompactHours),
            "1:00:00"
        );
        assert_eq!(
            format_duration(Duration::from_secs_f64(3661.0), TimeFormat::CompactHours),
            "1:01:01"
        );
    }
}

#[cfg(test)]
mod file_tests {
    use std::path::Path;

    fn file_stem(path: &str) -> &str {
        Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
    }

    fn file_extension(path: &str) -> &str {
        Path::new(path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
    }

    #[test]
    fn test_file_stem() {
        assert_eq!(file_stem("file.txt"), "file");
        assert_eq!(file_stem("path/to/file.txt"), "file");
        assert_eq!(file_stem("file"), "file");
        assert_eq!(file_stem("file.tar.gz"), "file.tar");
    }

    #[test]
    fn test_file_extension() {
        assert_eq!(file_extension("file.txt"), "txt");
        assert_eq!(file_extension("path/to/file.txt"), "txt");
        assert_eq!(file_extension("file"), "");
        assert_eq!(file_extension("file.tar.gz"), "gz");
    }
}
