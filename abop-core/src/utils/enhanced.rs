//! Enhanced utility functions for common operations
//!
//! This module provides high-level utility functions that combine multiple
//! casting operations and add domain-specific logic for common use cases.

use crate::error::{AppError, Result};
use crate::utils::casting::CastingBuilder;

/// Audio processing utilities
pub mod audio {
    use super::*;

    /// Calculate the total duration of an audiobook from sample counts and rates
    pub fn calculate_total_duration(
        tracks: &[(usize, u32)], // (sample_count, sample_rate) pairs
    ) -> Result<f32> {
        let _builder = CastingBuilder::for_audiobook_processing();
        let mut total_duration = 0.0;
        for &(samples, rate) in tracks {
            let duration =
                crate::utils::casting::domain::audio::safe_samples_to_duration(samples, rate)
                    .map_err(|e| AppError::Audio(format!("Duration calculation failed: {e}")))?;
            total_duration += duration;
        }

        Ok(total_duration)
    }

    /// Calculate optimal buffer size for audio processing
    pub fn calculate_optimal_buffer_size(
        sample_rate: u32,
        target_latency_ms: f32,
    ) -> Result<usize> {
        let builder = CastingBuilder::for_realtime_audio();
        let latency_seconds = target_latency_ms / 1000.0;

        builder
            .time_to_samples(latency_seconds, sample_rate)
            .map_err(|e| AppError::Audio(format!("Buffer size calculation failed: {e}")))
    }

    /// Convert between different audio formats safely
    pub fn convert_audio_sample(
        value: f64,
        from_format: AudioSampleFormat,
        to_format: AudioSampleFormat,
    ) -> Result<i64> {
        let builder = CastingBuilder::for_audio();

        let from_bits = from_format.bit_depth();
        let to_bits = to_format.bit_depth();

        builder
            .convert_audio_value(value, from_bits, to_bits)
            .map_err(|e| AppError::Audio(format!("Sample format conversion failed: {e}")))
    }

    /// Audio sample format enumeration
    #[derive(Debug, Clone, Copy)]
    pub enum AudioSampleFormat {
        /// 16-bit signed integer format
        Int16,
        /// 24-bit signed integer format
        Int24,
        /// 32-bit signed integer format
        Int32,
        /// 32-bit floating point format
        Float32,
        /// 64-bit floating point format
        Float64,
    }

    impl AudioSampleFormat {
        /// Returns the bit depth of the audio sample format
        ///
        /// # Returns
        /// The number of bits used to represent each sample in this format
        #[must_use]
        pub const fn bit_depth(self) -> u8 {
            match self {
                Self::Int16 => 16,
                Self::Int24 => 24,
                Self::Int32 => 32,
                Self::Float32 => 32,
                Self::Float64 => 64,
            }
        }
    }
}

/// Database utilities
pub mod database {
    use super::*;

    /// Calculate pagination parameters safely
    pub fn calculate_pagination(
        total_items: i64,
        page_size: usize,
        current_page: usize,
    ) -> Result<PaginationInfo> {
        let builder = CastingBuilder::for_database();

        // Validate inputs
        if total_items < 0 {
            return Err(AppError::InvalidData(
                "Total items cannot be negative".to_string(),
            ));
        }
        if page_size == 0 {
            return Err(AppError::InvalidData(
                "Page size cannot be zero".to_string(),
            ));
        }

        let total_items_usize = builder.int_to_int::<i64, usize>(total_items).map_err(|e| {
            AppError::Database(crate::db::error::DatabaseError::ExecutionFailed {
                message: format!("Failed to convert total items: {e}"),
            })
        })?;

        let total_pages = total_items_usize.div_ceil(page_size);
        let offset = current_page.saturating_sub(1) * page_size;
        let has_next = current_page < total_pages;
        let has_prev = current_page > 1;

        Ok(PaginationInfo {
            total_items: total_items_usize,
            page_size,
            current_page,
            total_pages,
            offset,
            has_next,
            has_prev,
        })
    }

    /// Pagination information
    #[derive(Debug, Clone)]
    pub struct PaginationInfo {
        /// Total number of items across all pages
        pub total_items: usize,
        /// Maximum number of items per page
        pub page_size: usize,
        /// Current page number (1-based)
        pub current_page: usize,
        /// Total number of pages
        pub total_pages: usize,
        /// Offset for database queries
        pub offset: usize,
        /// Whether there is a next page
        pub has_next: bool,
        /// Whether there is a previous page
        pub has_prev: bool,
    }

    /// Convert database timestamps safely
    pub fn convert_timestamp(timestamp: i64) -> Result<std::time::SystemTime> {
        use std::time::{Duration, UNIX_EPOCH};

        if timestamp < 0 {
            return Err(AppError::InvalidData(
                "Timestamp cannot be negative".to_string(),
            ));
        }

        let duration = Duration::from_secs(timestamp as u64);
        Ok(UNIX_EPOCH + duration)
    }
}

/// UI utilities
pub mod ui {
    use super::*;

    /// Calculate responsive grid layout
    pub fn calculate_grid_layout(
        container_width: f32,
        item_min_width: f32,
        gap: f32,
    ) -> Result<GridLayout> {
        let builder = CastingBuilder::for_ui();

        if container_width <= 0.0 || item_min_width <= 0.0 {
            return Err(AppError::InvalidData(
                "Dimensions must be positive".to_string(),
            ));
        }

        // Calculate how many items can fit
        let available_width = container_width - gap;
        let item_width_with_gap = item_min_width + gap;

        let columns = (available_width / item_width_with_gap).floor().max(1.0);
        let columns_usize = builder
            .float_to_int::<usize>(columns.into())
            .map_err(|e| AppError::Other(format!("Grid calculation failed: {e}")))?;

        // Calculate actual item width
        let total_gap_width = gap * (columns - 1.0);
        let actual_item_width = (container_width - total_gap_width) / columns;

        Ok(GridLayout {
            columns: columns_usize,
            item_width: actual_item_width,
            gap,
        })
    }

    /// Grid layout information
    #[derive(Debug, Clone)]
    pub struct GridLayout {
        /// Number of columns in the grid
        pub columns: usize,
        /// Width of each grid item
        pub item_width: f32,
        /// Gap between grid items
        pub gap: f32,
    }
    /// Calculate animation progress with easing
    pub fn calculate_animation_progress(
        elapsed: f32,
        duration: f32,
        easing: EasingFunction,
    ) -> Result<f32> {
        if duration <= 0.0 {
            return Err(AppError::InvalidData(
                "Duration must be positive".to_string(),
            ));
        }

        let linear_progress = (elapsed / duration).clamp(0.0, 1.0);
        let eased_progress = easing.apply(linear_progress);

        Ok(eased_progress)
    }

    /// Easing function types for animations
    #[derive(Debug, Clone, Copy)]
    pub enum EasingFunction {
        /// Linear interpolation (no easing)
        Linear,
        /// Ease-in (starts slow, accelerates)
        EaseIn,
        /// Ease-out (starts fast, decelerates)
        EaseOut,
        /// Ease-in-out (starts slow, accelerates, then decelerates)
        EaseInOut,
    }

    impl EasingFunction {
        /// Applies the easing function to a normalized time value
        ///
        /// # Arguments
        /// * `t` - Normalized time value between 0.0 and 1.0
        ///
        /// # Returns
        /// The eased value at time `t`
        ///
        /// # Panics
        /// This function does not panic for any input, but values outside [0.0, 1.0]
        /// may produce unexpected results as they're outside the standard easing range
        #[must_use]
        pub fn apply(self, t: f32) -> f32 {
            match self {
                Self::Linear => t,
                Self::EaseIn => t * t,
                Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
                Self::EaseInOut => {
                    if t < 0.5 {
                        2.0 * t * t
                    } else {
                        1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                    }
                }
            }
        }
    }
}

/// File system utilities
pub mod file {
    use super::*;
    use std::path::Path;
    /// Calculate safe file copy buffer size based on file size
    /// 
    /// Buffer size rationale:
    /// - Small files (<1MB): 8KB buffer to minimize memory overhead
    /// - Medium files (<100MB): 64KB buffer for balanced performance/memory
    /// - Large files (≥100MB): 1MB buffer for maximum throughput
    pub const fn calculate_copy_buffer_size(file_size: u64) -> Result<usize> {
        // Use different buffer sizes based on file size
        let buffer_size = if file_size < 1024 * 1024 {
            // < 1MB
            8192 // 8KB
        } else if file_size < 100 * 1024 * 1024 {
            // < 100MB  
            65536 // 64KB
        } else {
            1024 * 1024 // 1MB
        };

        Ok(buffer_size)
    }

    /// Validate file path and return normalized version
    pub fn normalize_path<P: AsRef<Path>>(path: P) -> Result<std::path::PathBuf> {
        let path = path.as_ref();

        // Convert to absolute path
        let absolute = path.canonicalize().map_err(|e| {
            AppError::Io(format!(
                "Failed to canonicalize path {}: {}",
                path.display(),
                e
            ))
        })?;

        Ok(absolute)
    }

    /// Calculate directory size safely
    pub fn calculate_directory_size<P: AsRef<Path>>(path: P) -> Result<u64> {
        use std::fs;

        let path = path.as_ref();
        if !path.is_dir() {
            return Err(AppError::InvalidData("Path is not a directory".to_string()));
        }

        let mut total_size = 0u64;

        fn visit_dir(dir: &Path, total: &mut u64) -> Result<()> {
            let entries = fs::read_dir(dir).map_err(|e| {
                AppError::Io(format!("Failed to read directory {}: {}", dir.display(), e))
            })?;

            for entry in entries {
                let entry = entry
                    .map_err(|e| AppError::Io(format!("Failed to read directory entry: {e}")))?;
                let path = entry.path();

                if path.is_dir() {
                    visit_dir(&path, total)?;
                } else {
                    let metadata = entry.metadata().map_err(|e| {
                        AppError::Io(format!(
                            "Failed to read metadata for {}: {}",
                            path.display(),
                            e
                        ))
                    })?;
                    *total = total.saturating_add(metadata.len());
                }
            }
            Ok(())
        }

        visit_dir(path, &mut total_size)?;
        Ok(total_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_duration_calculation() {
        let tracks = vec![
            (44100, 44100), // 1 second at 44.1kHz
            (48000, 48000), // 1 second at 48kHz
        ];

        let total = audio::calculate_total_duration(&tracks).unwrap();
        assert!((total - 2.0).abs() < 0.01); // Should be ~2 seconds
    }

    #[test]
    fn test_database_pagination() {
        let pagination = database::calculate_pagination(100, 10, 3).unwrap();
        assert_eq!(pagination.total_pages, 10);
        assert_eq!(pagination.offset, 20);
        assert!(pagination.has_next);
        assert!(pagination.has_prev);
    }

    #[test]
    fn test_ui_grid_layout() {
        let layout = ui::calculate_grid_layout(800.0, 150.0, 10.0).unwrap();
        assert!(layout.columns >= 1);
        assert!(layout.item_width >= 150.0);
    }

    #[test]
    fn test_ui_animation_progress() {
        let progress =
            ui::calculate_animation_progress(0.5, 1.0, ui::EasingFunction::Linear).unwrap();
        assert_eq!(progress, 0.5);

        let progress =
            ui::calculate_animation_progress(0.5, 1.0, ui::EasingFunction::EaseIn).unwrap();
        assert_eq!(progress, 0.25); // 0.5^2
    }

    #[test]
    fn test_file_buffer_size() {
        let size = file::calculate_copy_buffer_size(500).unwrap();
        assert_eq!(size, 8192);

        let size = file::calculate_copy_buffer_size(50 * 1024 * 1024).unwrap();
        assert_eq!(size, 65536);

        let size = file::calculate_copy_buffer_size(200 * 1024 * 1024).unwrap();
        assert_eq!(size, 1024 * 1024);
    }
}
