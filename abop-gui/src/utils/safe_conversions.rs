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
    #[deprecated(since = "0.1.0", note = "Use abop_core::utils::casting::domain::ui::spacing_to_pixels_clamped")]
    #[must_use]
    pub fn safe_spacing_to_pixels(spacing: f32) -> u16 {
        abop_core::utils::casting::domain::ui::spacing_to_pixels_clamped(spacing)
    }

    /// Safe animation duration conversion with bounds checking
    ///
    /// # Arguments
    /// * `duration` - Duration in seconds (f64)
    ///
    /// # Returns
    /// * `u32` - Duration in milliseconds, clamped to valid range
    #[deprecated(since = "0.1.0", note = "Use abop_core::utils::casting::domain::ui::duration_secs_to_millis_clamped")]
    #[must_use]
    pub fn safe_duration_to_millis(duration: f64) -> u32 {
        abop_core::utils::casting::domain::ui::duration_secs_to_millis_clamped(duration)
    }

    /// Safe thickness conversion for UI borders and dividers
    ///
    /// # Arguments
    /// * `thickness` - Thickness in logical pixels (f32)
    ///
    /// # Returns
    /// * `u16` - Pixel thickness, clamped to valid range
    #[deprecated(since = "0.1.0", note = "Use abop_core::utils::casting::domain::ui::spacing_to_pixels_clamped")]
    #[must_use]
    pub fn safe_thickness_to_pixels(thickness: f32) -> u16 {
        abop_core::utils::casting::domain::ui::spacing_to_pixels_clamped(thickness)
    }

    /// Safe level to spacing conversion for nested UI elements
    ///
    /// # Arguments
    /// * `level` - Nesting level (0-based)
    ///
    /// # Returns
    /// * `f32` - Spacing in logical pixels
    #[deprecated(since = "0.1.0", note = "Use abop_core::utils::casting::domain::ui::level_to_spacing_clamped")]
    #[must_use]
    pub fn safe_level_to_spacing(level: usize) -> f32 {
        abop_core::utils::casting::domain::ui::level_to_spacing_clamped(level)
    }

    /// Safe progress percentage calculation for UI progress bars
    ///
    /// # Arguments
    /// * `current` - Current progress value
    /// * `total` - Total progress value
    ///
    /// # Returns
    /// * `f32` - Progress percentage (0.0 to 100.0), or 0.0 if total is zero
    #[deprecated(since = "0.1.0", note = "Use abop_core::utils::casting::domain::ui::progress_percentage_clamped")]
    #[must_use]
    pub fn safe_progress_percentage(current: usize, total: usize) -> f32 {
        abop_core::utils::casting::domain::ui::progress_percentage_clamped(current, total)
    }

    /// Safe file size formatting for UI display (deprecated; use abop_core canonical helpers)
    #[deprecated(since = "0.1.0", note = "Use abop_core::utils::casting::format_file_size_* instead")]
    #[must_use]
    pub fn format_file_size_for_ui(bytes: u64) -> String {
        abop_core::utils::casting::format_file_size_standard(bytes)
    }

    /// Safe opacity conversion (f32 to u8) for alpha channels
    ///
    /// # Arguments
    /// * `opacity` - Opacity value (0.0 to 1.0)
    ///
    /// # Returns
    /// * `u8` - Alpha value (0 to 255)
    #[deprecated(since = "0.1.0", note = "Use abop_core::utils::casting::domain::ui::opacity_to_alpha_clamped")]
    #[must_use]
    pub fn safe_opacity_to_alpha(opacity: f32) -> u8 {
        abop_core::utils::casting::domain::ui::opacity_to_alpha_clamped(opacity)
    }

    /// Safe color component conversion (f32 to u8)
    ///
    /// # Arguments
    /// * `component` - Color component (0.0 to 1.0)
    ///
    /// # Returns
    /// * `u8` - Color component (0 to 255)
    #[deprecated(since = "0.1.0", note = "Use abop_core::utils::casting::domain::ui::color_component_to_u8_clamped")]
    #[must_use]
    pub fn safe_color_component_to_u8(component: f32) -> u8 {
        abop_core::utils::casting::domain::ui::color_component_to_u8_clamped(component)
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
