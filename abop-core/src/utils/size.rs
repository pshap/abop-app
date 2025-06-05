//! File size formatting utilities

/// Formats bytes as a human-readable size string
///
/// # Arguments
/// * `bytes` - The number of bytes to format
///
/// # Returns
/// Formatted size string with appropriate unit
///
/// # Examples
/// ```
/// use abop_core::utils::size::format_bytes;
///
/// assert_eq!(format_bytes(500), "500 B");
/// assert_eq!(format_bytes(1024), "1 KB");
/// assert_eq!(format_bytes(1_048_576), "1 MB");
/// assert_eq!(format_bytes(1536), "1.5 KB")
/// ```
#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    // For small values, avoid floating point entirely
    if bytes < 1024 {
        return format!("{bytes} B");
    }

    // Safe conversion with explicit handling of precision loss
    // Note: u64 to f64 conversion may lose precision for very large values (> 2^53)
    // but is acceptable for display purposes
    #[allow(clippy::cast_precision_loss)]
    let mut size = bytes as f64;
    let mut unit_index = 0;

    let max_units = UNITS.len().saturating_sub(1);
    while size >= 1024.0 && unit_index < max_units {
        size /= 1024.0;
        unit_index = unit_index.saturating_add(1);
    }

    // Format with 1 decimal place for values >= 1KB, no decimal for bytes
    if size.fract() < f64::EPSILON * 10.0 && unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else if size.fract() < f64::EPSILON * 10.0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Formats bytes as megabytes (legacy compatibility function)
///
/// # Arguments
/// * `bytes` - The number of bytes to format
///
/// # Returns
/// Formatted size string in MB
///
/// # Examples
/// ```
/// use abop_core::utils::size::format_as_mb;
///
/// assert_eq!(format_as_mb(1_048_576), "1.0 MB");
/// ```
#[must_use]
pub fn format_as_mb(bytes: u64) -> String {
    // Convert to f64 with explicit handling of precision loss
    // Note: u64 to f64 conversion may lose precision for very large values (> 2^53)
    // but is acceptable for display purposes in MB format
    #[allow(clippy::cast_precision_loss)]
    let mb = bytes as f64 / 1_048_576.0;
    format!("{mb:.1} MB")
}
