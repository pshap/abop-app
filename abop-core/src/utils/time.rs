//! Time formatting utilities for the application

use std::time::Duration;

/// Time formatting options
#[derive(Debug, Clone, Copy)]
pub enum TimeFormat {
    /// Always show hours with leading zeros (HH:MM:SS)
    AlwaysHours,
    /// Show hours only when > 0, with leading zeros (HH:MM:SS or MM:SS)
    HoursWhenNonZero,
    /// Show hours only when > 0, no leading zeros on hours (H:MM:SS or MM:SS)
    CompactHours,
}

/// Formats seconds as a time string with the specified format
///
/// # Arguments
/// * `seconds` - The number of seconds to format
/// * `format` - The formatting style to use
///
/// # Returns
/// Formatted time string
///
/// # Examples
/// ```
/// use abop_core::utils::time::{format_seconds, TimeFormat};
///
/// assert_eq!(format_seconds(3661, TimeFormat::AlwaysHours), "01:01:01");
/// assert_eq!(format_seconds(125, TimeFormat::HoursWhenNonZero), "02:05");
/// assert_eq!(format_seconds(3661, TimeFormat::CompactHours), "1:01:01");
/// ```
#[must_use]
pub fn format_seconds(seconds: u64, format: TimeFormat) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    match format {
        TimeFormat::AlwaysHours => format!("{hours:02}:{minutes:02}:{secs:02}"),
        TimeFormat::HoursWhenNonZero => {
            if hours > 0 {
                format!("{hours:02}:{minutes:02}:{secs:02}")
            } else {
                format!("{minutes:02}:{secs:02}")
            }
        }
        TimeFormat::CompactHours => {
            if hours > 0 {
                format!("{hours}:{minutes:02}:{secs:02}")
            } else {
                format!("{minutes}:{secs:02}")
            }
        }
    }
}

/// Formats a floating-point duration in seconds with the specified format
///
/// # Arguments
/// * `seconds` - The number of seconds to format (as f64)
/// * `format` - The formatting style to use
///
/// # Returns
/// Formatted time string
///
/// # Examples
/// ```
/// use abop_core::utils::time::{format_seconds_f64, TimeFormat};
///
/// assert_eq!(format_seconds_f64(3661.5, TimeFormat::CompactHours), "1:01:01");
/// assert_eq!(format_seconds_f64(125.7, TimeFormat::HoursWhenNonZero), "02:05");
/// ```
#[must_use]
pub fn format_seconds_f64(seconds: f64, format: TimeFormat) -> String {
    // Safely convert f64 to u64 with bounds checking and explicit handling of precision loss
    // Note: f64 to u64 conversion may lose precision for very large values (> 2^53)
    // but is acceptable for time formatting purposes
    let seconds_u64 = if seconds < 0.0 {
        0
    } else if seconds > f64::from(u32::MAX) {
        // For very large values, cap at u32::MAX to avoid precision issues
        u64::from(u32::MAX)
    } else {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let secs = seconds.floor() as u64;
        secs
    };
    format_seconds(seconds_u64, format)
}

/// Formats a Duration with the specified format
///
/// # Arguments
/// * `duration` - The Duration to format
/// * `format` - The formatting style to use
///
/// # Returns
/// Formatted time string
///
/// # Examples
/// ```
/// use std::time::Duration;
/// use abop_core::utils::time::{format_duration, TimeFormat};
///
/// let duration = Duration::from_secs(3661);
/// assert_eq!(format_duration(duration, TimeFormat::AlwaysHours), "01:01:01");
/// ```
#[must_use]
pub fn format_duration(duration: Duration, format: TimeFormat) -> String {
    // Using as_secs() is safe here as it returns u64 directly
    let seconds = duration.as_secs();
    format_seconds(seconds, format)
}
