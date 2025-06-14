//! File size formatting utilities

/// Precision modes for file size formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSizePrecision {
    /// Exact precision (no decimal places)
    Exact,
    /// Standard precision (2 decimal places)
    Standard,
    /// High precision (3 decimal places)
    High,
    /// Custom number of decimal places
    Custom(usize),
}

impl Default for FileSizePrecision {
    fn default() -> Self {
        Self::Standard
    }
}

impl FileSizePrecision {
    /// Get the number of decimal places for this precision mode
    #[must_use]
    pub const fn decimal_places(self) -> usize {
        match self {
            Self::Exact => 0,
            Self::Standard => 2,
            Self::High => 3,
            Self::Custom(n) => n,
        }
    }
}

/// Format a byte count into a human-readable file size string
///
/// # Arguments
/// * `bytes` - The number of bytes to format
/// * `precision` - The precision mode for decimal places
///
/// # Returns
/// A formatted string like "1.23 MB" or "512 B"
#[must_use]
pub fn format_file_size(bytes: u64, precision: FileSizePrecision) -> String {
    const UNITS: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

    if bytes == 0 {
        return format!("0 {}", UNITS[0]);
    }

    let i = (bytes.checked_ilog(1024).unwrap_or(0) as usize).min(UNITS.len() - 1);
    let size = bytes as f64 / 1024f64.powi(i as i32);

    // For exact precision or when showing bytes, round to nearest integer
    if matches!(precision, FileSizePrecision::Exact) || i == 0 {
        format!("{} {}", size.round() as u64, UNITS[i])
    } else {
        let decimal_places = precision.decimal_places();
        format!("{:.*} {}", decimal_places, size, UNITS[i])
    }
}

/// Format a file size with standard precision (2 decimal places)
#[must_use]
pub fn format_file_size_standard(bytes: u64) -> String {
    format_file_size(bytes, FileSizePrecision::Standard)
}

/// Format a file size with exact precision (no decimals)
#[must_use]
pub fn format_file_size_exact(bytes: u64) -> String {
    format_file_size(bytes, FileSizePrecision::Exact)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0, FileSizePrecision::Standard), "0 B");
        assert_eq!(
            format_file_size(1023, FileSizePrecision::Standard),
            "1023 B"
        );
        assert_eq!(
            format_file_size(1024, FileSizePrecision::Standard),
            "1.00 KB"
        );
        assert_eq!(format_file_size(1536, FileSizePrecision::Exact), "2 KB");
        assert_eq!(format_file_size(1536, FileSizePrecision::High), "1.500 KB");
        assert_eq!(
            format_file_size(1024 * 1024, FileSizePrecision::Standard),
            "1.00 MB"
        );

        // Test custom precision
        assert_eq!(
            format_file_size(1536, FileSizePrecision::Custom(4)),
            "1.5000 KB"
        );

        // Test edge cases
        assert_eq!(format_file_size(999, FileSizePrecision::Standard), "999 B");
        assert_eq!(
            format_file_size(1000, FileSizePrecision::Standard),
            "1000 B"
        );
        assert_eq!(
            format_file_size(1023, FileSizePrecision::Standard),
            "1023 B"
        );
        assert_eq!(
            format_file_size(1024, FileSizePrecision::Standard),
            "1.00 KB"
        );
    }
}
