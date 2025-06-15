//! Common traits for audio processors
//!
//! This module defines the core traits and interfaces that all audio processing
//! components must implement, ensuring consistent behavior and composability.

use super::super::AudioBuffer;
use crate::utils::casting::domain::audio::safe_usize_to_f64_audio;
use super::error::Result;
use std::time::Duration;

/// Core trait for audio processing components
pub trait AudioProcessor {
    /// Process an audio buffer in place
    ///
    /// # Arguments
    /// * `buffer` - The audio buffer to process
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::ProcessingFailed`] if audio processing fails
    /// or [`AudioProcessingError::InvalidBuffer`] if the buffer is invalid.
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) -> Result<()>;

    /// Reset any internal state (useful for streaming processing)
    fn reset(&mut self);
}

/// Trait for processors that can be configured
pub trait Configurable<C> {
    /// Apply configuration to this processor
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if the configuration
    /// is invalid or cannot be applied.
    fn configure(&mut self, config: C) -> Result<()>;

    /// Get the current configuration
    fn get_config(&self) -> &C;
}

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

/// Trait for processors that can validate their parameters
pub trait Validatable {
    /// Validate the current configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if the current
    /// configuration is invalid.
    fn validate(&self) -> Result<()>;
}

/// Marker trait for processors that are thread-safe
pub trait ThreadSafe: Send + Sync {}

/// Automatically implement `ThreadSafe` for types that are Send + Sync
impl<T: Send + Sync> ThreadSafe for T {}

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

/// Resource estimation trait for processors
pub trait ResourceEstimation {
    /// Estimate memory usage in bytes for given buffer size
    fn estimate_memory_usage(&self, buffer_size: usize) -> usize;

    /// Estimate CPU usage as a factor (0.0 to 1.0+)
    fn estimate_cpu_usage(&self) -> f32;

    /// Estimate processing time for given buffer
    fn estimate_processing_time(&self, buffer_size: usize, sample_rate: u32) -> Duration {
        // Default implementation based on CPU usage estimate
        let samples_per_second = f64::from(sample_rate);
        // The cast from usize to f64 may lose precision for very large values (> 2^53)
        // but is acceptable for time estimation where slight precision loss is not critical
        #[allow(clippy::cast_precision_loss)]
        let buffer_size_f64 = buffer_size as f64;
        let buffer_duration = buffer_size_f64 / samples_per_second;
        let cpu_factor = f64::from(self.estimate_cpu_usage());
        Duration::from_secs_f64(buffer_duration * cpu_factor)
    }
}

/// Common processor lifecycle management
pub trait ProcessorLifecycle {
    /// Initialize the processor (allocate resources, etc.)
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InitializationFailed`] if the processor
    /// cannot be initialized or resources cannot be allocated.
    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup processor resources
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::CleanupFailed`] if resources cannot be
    /// properly cleaned up.
    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Check if processor is ready for processing
    fn is_ready(&self) -> bool {
        true
    }

    /// Check if processor is currently processing
    fn is_processing(&self) -> bool {
        false
    }
    /// Pause processing (if supported)
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::OperationNotSupported`] if pausing is not
    /// supported by this processor.
    fn pause(&mut self) -> Result<()> {
        Ok(())
    }
    /// Resume processing (if supported)
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::OperationNotSupported`] if resuming is not
    /// supported by this processor.
    fn resume(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Trait for processors that support streaming/real-time processing
pub trait StreamingProcessor: AudioProcessor {
    /// Process a chunk of streaming audio
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::ProcessingFailed`] if audio processing fails
    /// or [`AudioProcessingError::InvalidBuffer`] if the input/output buffers are invalid.
    fn process_chunk(&mut self, input: &[f32], output: &mut [f32]) -> Result<usize>;

    /// Get the preferred chunk size for streaming
    fn preferred_chunk_size(&self) -> usize {
        1024
    }

    /// Get processing latency for streaming
    fn streaming_latency(&self) -> usize {
        0
    }
}

/// Trait for processors that can be serialized/deserialized
pub trait Serializable {
    /// Serialize processor state to bytes
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::SerializationFailed`] if the processor
    /// state cannot be serialized.
    fn serialize(&self) -> Result<Vec<u8>>;

    /// Deserialize processor state from bytes
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::DeserializationFailed`] if the data
    /// cannot be deserialized or is invalid for this processor type.
    fn deserialize(&mut self, data: &[u8]) -> Result<()>;

    /// Get a unique identifier for this processor type
    fn type_id(&self) -> &'static str;
}

/// Trait for processors that support bypass/enable toggle
pub trait Bypassable {
    /// Check if processor is bypassed
    fn is_bypassed(&self) -> bool;

    /// Set bypass state
    fn set_bypassed(&mut self, bypassed: bool);

    /// Toggle bypass state
    fn toggle_bypass(&mut self) {
        let current = self.is_bypassed();
        self.set_bypassed(!current);
    }
}

/// Trait for processors that can provide detailed information
pub trait ProcessorInfo {
    /// Get processor name
    fn name(&self) -> &str;

    /// Get processor version
    fn version(&self) -> &'static str {
        "1.0.0"
    }

    /// Get processor description
    fn description(&self) -> &'static str {
        ""
    }

    /// Get processor author/vendor
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

/// Composite trait for full-featured audio processors
pub trait FullAudioProcessor:
    AudioProcessor + Validatable + LatencyReporting + ProcessorLifecycle + ProcessorInfo + ThreadSafe
{
    /// Convenience method to validate and process buffer
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if validation fails,
    /// or any error from the underlying `process` method.
    fn validate_and_process(&mut self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        self.validate()?;
        self.process(buffer)
    }
}

// Automatically implement FullAudioProcessor for types that implement all required traits
impl<T> FullAudioProcessor for T where
    T: AudioProcessor
        + Validatable
        + LatencyReporting
        + ProcessorLifecycle
        + ProcessorInfo
        + ThreadSafe
{
}
