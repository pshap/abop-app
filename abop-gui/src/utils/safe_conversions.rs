//! UI-specific safe conversions for Iced GUI components
//!
//! This module provides safe conversion utilities for UI-related casting
//! operations, particularly for Material Design spacing and animation timing.

/// UI-specific safe conversion utilities
pub mod ui_conversions {
    /// Convert Material Design spacing (f32) to pixel units (u16)
    ///
    /// Material Design uses logical spacing units that need to be converted
    /// to actual pixel coordinates. This function ensures the conversion
    /// is safe and doesn't overflow.
    ///
    /// # Arguments
    /// * `spacing` - Spacing value in logical units
    ///
    /// # Returns
    /// * `u16` - Pixel value, clamped to valid range
    #[must_use]
    pub fn safe_spacing_to_pixels(spacing: f32) -> u16 {
        spacing.round().clamp(0.0, f32::from(u16::MAX)).max(0.0) as u16
    }

    /// Safe animation duration conversion with bounds checking
    ///
    /// # Arguments
    /// * `duration` - Duration in seconds (f64)
    ///
    /// # Returns
    /// * `u32` - Duration in milliseconds, clamped to valid range
    #[must_use]
    pub fn safe_duration_to_millis(duration: f64) -> u32 {
        let millis = duration * 1000.0;
        millis.clamp(0.0, f64::from(u32::MAX)).round() as u32
    }

    /// Safe thickness conversion for UI borders and dividers
    ///
    /// # Arguments
    /// * `thickness` - Thickness in logical pixels (f32)
    ///
    /// # Returns
    /// * `u16` - Pixel thickness, clamped to valid range
    #[must_use]
    pub fn safe_thickness_to_pixels(thickness: f32) -> u16 {
        thickness.round().clamp(0.0, f32::from(u16::MAX)).max(0.0) as u16
    }

    /// Safe level to spacing conversion for nested UI elements
    ///
    /// # Arguments
    /// * `level` - Nesting level (0-based)
    ///
    /// # Returns
    /// * `f32` - Spacing in logical pixels
    #[must_use]
    pub fn safe_level_to_spacing(level: usize) -> f32 {
        const SPACING_PER_LEVEL: f32 = 24.0;
        const MAX_LEVELS: usize = 50; // Reasonable UI depth limit

        let clamped_level = level.min(MAX_LEVELS);
        (clamped_level as f32) * SPACING_PER_LEVEL
    }

    /// Safe progress percentage calculation for UI progress bars
    ///
    /// # Arguments
    /// * `current` - Current progress value
    /// * `total` - Total progress value
    ///
    /// # Returns
    /// * `f32` - Progress percentage (0.0 to 100.0), or 0.0 if total is zero
    #[must_use]
    pub fn safe_progress_percentage(current: usize, total: usize) -> f32 {
        if total == 0 {
            return 0.0;
        }

        // Use f64 for intermediate calculation to preserve precision
        let percentage = (current as f64 / total as f64) * 100.0;
        percentage.clamp(0.0, 100.0) as f32
    }

    /// Safe file size formatting for UI display
    ///
    /// # Arguments
    /// * `bytes` - File size in bytes
    ///
    /// # Returns
    /// * `String` - Human-readable file size (e.g., "1.2 MB")
    #[must_use]
    pub fn format_file_size_for_ui(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        const THRESHOLD: f64 = 1024.0;

        if bytes == 0 {
            return "0 B".to_string();
        }

        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
            size /= THRESHOLD;
            unit_index += 1;
        }

        if size >= 100.0 {
            format!("{:.0} {}", size, UNITS[unit_index])
        } else if size >= 10.0 {
            format!("{:.1} {}", size, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }

    /// Safe opacity conversion (f32 to u8) for alpha channels
    ///
    /// # Arguments
    /// * `opacity` - Opacity value (0.0 to 1.0)
    ///
    /// # Returns
    /// * `u8` - Alpha value (0 to 255)
    #[must_use]
    pub fn safe_opacity_to_alpha(opacity: f32) -> u8 {
        let clamped = opacity.clamp(0.0, 1.0);
        (clamped * 255.0).round() as u8
    }

    /// Safe color component conversion (f32 to u8)
    ///
    /// # Arguments
    /// * `component` - Color component (0.0 to 1.0)
    ///
    /// # Returns
    /// * `u8` - Color component (0 to 255)
    #[must_use]
    pub fn safe_color_component_to_u8(component: f32) -> u8 {
        let clamped = component.clamp(0.0, 1.0);
        (clamped * 255.0).round() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::ui_conversions::*;

    #[test]
    fn test_safe_spacing_to_pixels() {
        assert_eq!(safe_spacing_to_pixels(0.0), 0);
        assert_eq!(safe_spacing_to_pixels(10.5), 11); // Rounds to nearest
        assert_eq!(safe_spacing_to_pixels(-5.0), 0); // Clamps negative to 0
        assert_eq!(safe_spacing_to_pixels(f32::from(u16::MAX) + 1.0), u16::MAX);
    }

    #[test]
    fn test_safe_duration_to_millis() {
        assert_eq!(safe_duration_to_millis(0.0), 0);
        assert_eq!(safe_duration_to_millis(1.0), 1000);
        assert_eq!(safe_duration_to_millis(1.5), 1500);
        assert_eq!(safe_duration_to_millis(-1.0), 0); // Clamps negative
    }

    #[test]
    fn test_safe_level_to_spacing() {
        assert_eq!(safe_level_to_spacing(0), 0.0);
        assert_eq!(safe_level_to_spacing(1), 24.0);
        assert_eq!(safe_level_to_spacing(2), 48.0);
        // Test clamping
        assert_eq!(safe_level_to_spacing(100), safe_level_to_spacing(50));
    }

    #[test]
    fn test_safe_progress_percentage() {
        assert_eq!(safe_progress_percentage(0, 100), 0.0);
        assert_eq!(safe_progress_percentage(50, 100), 50.0);
        assert_eq!(safe_progress_percentage(100, 100), 100.0);
        assert_eq!(safe_progress_percentage(10, 0), 0.0); // Handle division by zero
    }

    #[test]
    fn test_format_file_size_for_ui() {
        assert_eq!(format_file_size_for_ui(0), "0 B");
        assert_eq!(format_file_size_for_ui(512), "512 B");
        assert_eq!(format_file_size_for_ui(1024), "1.00 KB");
        assert_eq!(format_file_size_for_ui(1536), "1.50 KB");
        assert!(format_file_size_for_ui(1048576).contains("MB"));
    }

    #[test]
    fn test_safe_opacity_to_alpha() {
        assert_eq!(safe_opacity_to_alpha(0.0), 0);
        assert_eq!(safe_opacity_to_alpha(1.0), 255);
        assert_eq!(safe_opacity_to_alpha(0.5), 128);
        assert_eq!(safe_opacity_to_alpha(-0.1), 0); // Clamps negative
        assert_eq!(safe_opacity_to_alpha(1.1), 255); // Clamps over 1.0
    }
}
