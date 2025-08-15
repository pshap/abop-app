//! Traits for reporting and monitoring audio processing operations
//!
//! This module provides traits for processors that can report on their
//! performance, resource usage, and operational status.

use crate::utils::casting::domain::audio::safe_usize_to_f64_audio;
use std::time::Duration;

/// Trait for processors that can report their processing latency
pub trait LatencyReporting {
    /// Get the processing latency in samples
    fn get_latency_samples(&self) -> usize {
        0
    }

    /// Get the processing latency in seconds for a given sample rate
    fn get_latency_seconds(&self, sample_rate: u32) -> f64 {
        safe_usize_to_f64_audio(self.get_latency_samples()) / f64::from(sample_rate)
    }
}

/// Progress reporting trait for long-running operations
pub trait ProgressReporting {
    /// Progress information type
    type Progress;

    /// Report current progress
    fn report_progress(&self) -> Self::Progress;

    /// Get estimated completion percentage (0.0 to 1.0)
    fn completion_percentage(&self) -> f32 {
        0.0
    }

    /// Get estimated time remaining
    fn estimated_time_remaining(&self) -> Option<Duration> {
        None
    }
}

/// Trait for processors that can estimate resource requirements
pub trait ResourceEstimation {
    /// Estimate memory usage in bytes for the given buffer size
    fn estimate_memory_usage(&self, buffer_size: usize) -> usize {
        buffer_size * std::mem::size_of::<f32>()
    }

    /// Estimate CPU usage as a percentage (0.0 to 1.0)
    fn estimate_cpu_usage(&self) -> f32 {
        0.1
    }

    /// Estimate processing time for a given duration
    fn estimate_processing_time(&self, input_duration: Duration) -> Duration {
        input_duration
    }
}

/// Trait for processors that provide operational information
pub trait ProcessorInfo {
    /// Get the processor name
    fn name(&self) -> &str;

    /// Get the processor version
    fn version(&self) -> &'static str {
        "1.0.0"
    }

    /// Get a description of what this processor does
    fn description(&self) -> &'static str {
        ""
    }

    /// Get the processor author/vendor
    fn author(&self) -> &'static str {
        "ABOP"
    }

    /// Get supported input formats
    fn supported_input_formats(&self) -> Vec<String> {
        vec!["f32".to_string()]
    }

    /// Get supported output formats
    fn supported_output_formats(&self) -> Vec<String> {
        vec!["f32".to_string()]
    }
}

