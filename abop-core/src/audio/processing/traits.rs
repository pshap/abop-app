//! Common traits for audio processors
//!
//! This module defines the core traits and interfaces that all audio processing
//! components must implement, ensuring consistent behavior and composability.

use super::super::AudioBuffer;
use super::error::Result;
use crate::utils::casting::domain::audio::safe_usize_to_f64_audio;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::AudioBuffer;
    use std::time::Duration;

    // Test implementation for AudioProcessor trait
    #[derive(Debug, Default)]
    struct MockProcessor {
        bypassed: bool,
        latency_samples: usize,
        is_valid: bool,
        is_ready: bool,
        is_processing: bool,
        memory_usage: usize,
        cpu_usage: f32,
        name: String,
    }

    impl AudioProcessor for MockProcessor {
        fn process(&mut self, _buffer: &mut AudioBuffer<f32>) -> Result<()> {
            if self.is_valid {
                Ok(())
            } else {
                Err(super::super::error::AudioProcessingError::Pipeline(
                    "Mock processor failed".to_string(),
                ))
            }
        }

        fn reset(&mut self) {
            self.is_processing = false;
        }
    }

    impl Configurable<String> for MockProcessor {
        fn configure(&mut self, config: String) -> Result<()> {
            if config == "invalid" {
                Err(super::super::error::AudioProcessingError::Configuration(
                    "Invalid configuration".to_string(),
                ))
            } else {
                self.name = config;
                Ok(())
            }
        }

        fn get_config(&self) -> &String {
            &self.name
        }
    }

    impl LatencyReporting for MockProcessor {
        fn get_latency_samples(&self) -> usize {
            self.latency_samples
        }
    }

    impl Validatable for MockProcessor {
        fn validate(&self) -> Result<()> {
            if self.is_valid {
                Ok(())
            } else {
                Err(super::super::error::AudioProcessingError::Configuration(
                    "Invalid processor state".to_string(),
                ))
            }
        }
    }

    impl ProcessorLifecycle for MockProcessor {
        fn initialize(&mut self) -> Result<()> {
            self.is_ready = true;
            Ok(())
        }

        fn cleanup(&mut self) -> Result<()> {
            self.is_ready = false;
            Ok(())
        }

        fn is_ready(&self) -> bool {
            self.is_ready
        }

        fn is_processing(&self) -> bool {
            self.is_processing
        }

        fn pause(&mut self) -> Result<()> {
            self.is_processing = false;
            Ok(())
        }

        fn resume(&mut self) -> Result<()> {
            self.is_processing = true;
            Ok(())
        }
    }

    impl ProcessorInfo for MockProcessor {
        fn name(&self) -> &str {
            if self.name.is_empty() {
                "MockProcessor"
            } else {
                &self.name
            }
        }
    }

    impl ProgressReporting for MockProcessor {
        type Progress = f32;

        fn report_progress(&self) -> Self::Progress {
            0.5
        }

        fn completion_percentage(&self) -> f32 {
            0.75
        }

        fn estimated_time_remaining(&self) -> Option<Duration> {
            Some(Duration::from_secs(10))
        }
    }

    impl ResourceEstimation for MockProcessor {
        fn estimate_memory_usage(&self, buffer_size: usize) -> usize {
            self.memory_usage + buffer_size * 4 // 4 bytes per f32 sample
        }

        fn estimate_cpu_usage(&self) -> f32 {
            self.cpu_usage
        }
    }

    impl StreamingProcessor for MockProcessor {
        fn process_chunk(&mut self, input: &[f32], output: &mut [f32]) -> Result<usize> {
            if input.len() != output.len() {
                return Err(super::super::error::AudioProcessingError::BufferValidation(
                    "Input and output buffer size mismatch".to_string(),
                ));
            }
            output.copy_from_slice(input);
            Ok(input.len())
        }

        fn preferred_chunk_size(&self) -> usize {
            512
        }

        fn streaming_latency(&self) -> usize {
            self.latency_samples
        }
    }

    impl Serializable for MockProcessor {
        fn serialize(&self) -> Result<Vec<u8>> {
            Ok(self.name.as_bytes().to_vec())
        }

        fn deserialize(&mut self, data: &[u8]) -> Result<()> {
            // Guard against unbounded memory allocation
            const MAX_INPUT_SIZE: usize = 1024 * 1024; // 1MB limit for safety

            if data.len() > MAX_INPUT_SIZE {
                return Err(super::super::error::AudioProcessingError::InvalidInput(
                    format!(
                        "Input data too large: {} bytes (max: {} bytes)",
                        data.len(),
                        MAX_INPUT_SIZE
                    ),
                ));
            }

            // Check for empty input
            if data.is_empty() {
                return Err(super::super::error::AudioProcessingError::InvalidInput(
                    "Empty input data provided".to_string(),
                ));
            }

            self.name = String::from_utf8(data.to_vec()).map_err(|e| {
                super::super::error::AudioProcessingError::InvalidInput(format!(
                    "Invalid UTF-8 data: {}",
                    e
                ))
            })?;
            Ok(())
        }

        fn type_id(&self) -> &'static str {
            "MockProcessor"
        }
    }

    impl Bypassable for MockProcessor {
        fn is_bypassed(&self) -> bool {
            self.bypassed
        }

        fn set_bypassed(&mut self, bypassed: bool) {
            self.bypassed = bypassed;
        }
    }

    #[test]
    fn test_audio_processor() {
        let mut processor = MockProcessor {
            is_valid: true,
            ..Default::default()
        };

        let mut buffer = AudioBuffer {
            data: vec![0.0f32; 100],
            format: crate::audio::SampleFormat::F32,
            channels: 2,
            sample_rate: 44100,
        };

        // Test successful processing
        assert!(processor.process(&mut buffer).is_ok());

        // Test reset
        processor.is_processing = true;
        processor.reset();
        assert!(!processor.is_processing);

        // Test failure case
        processor.is_valid = false;
        assert!(processor.process(&mut buffer).is_err());
    }

    #[test]
    fn test_configurable() {
        let mut processor = MockProcessor::default();

        // Test valid configuration
        assert!(processor.configure("test_config".to_string()).is_ok());
        assert_eq!(processor.get_config(), "test_config");

        // Test invalid configuration
        assert!(processor.configure("invalid".to_string()).is_err());
    }

    #[test]
    fn test_latency_reporting() {
        let processor = MockProcessor {
            latency_samples: 256,
            ..Default::default()
        };

        assert_eq!(processor.get_latency_samples(), 256);
        assert_eq!(processor.get_latency_seconds(44100), 256.0 / 44100.0);
    }

    #[test]
    fn test_validatable() {
        let processor = MockProcessor {
            is_valid: true,
            ..Default::default()
        };
        assert!(processor.validate().is_ok());

        let invalid_processor = MockProcessor {
            is_valid: false,
            ..Default::default()
        };
        assert!(invalid_processor.validate().is_err());
    }

    #[test]
    fn test_progress_reporting() {
        let processor = MockProcessor::default();

        assert_eq!(processor.report_progress(), 0.5);
        assert_eq!(processor.completion_percentage(), 0.75);
        assert_eq!(
            processor.estimated_time_remaining(),
            Some(Duration::from_secs(10))
        );
    }

    #[test]
    fn test_resource_estimation() {
        let processor = MockProcessor {
            memory_usage: 1000,
            cpu_usage: 0.5,
            ..Default::default()
        };

        let buffer_size = 1024;
        assert_eq!(
            processor.estimate_memory_usage(buffer_size),
            1000 + buffer_size * 4
        );
        assert_eq!(processor.estimate_cpu_usage(), 0.5);

        // Test processing time estimation
        let time = processor.estimate_processing_time(44100, 44100);
        assert_eq!(time, Duration::from_secs_f64(0.5)); // 1 second of audio * 0.5 CPU usage
    }

    #[test]
    fn test_processor_lifecycle() {
        let mut processor = MockProcessor::default();

        // Initial state
        assert!(!processor.is_ready());
        assert!(!processor.is_processing());

        // Initialize
        assert!(processor.initialize().is_ok());
        assert!(processor.is_ready());

        // Pause/Resume
        assert!(processor.resume().is_ok());
        assert!(processor.is_processing());

        assert!(processor.pause().is_ok());
        assert!(!processor.is_processing());

        // Cleanup
        assert!(processor.cleanup().is_ok());
        assert!(!processor.is_ready());
    }

    #[test]
    fn test_streaming_processor() {
        let mut processor = MockProcessor::default();

        assert_eq!(processor.preferred_chunk_size(), 512);
        assert_eq!(processor.streaming_latency(), 0);

        let input = vec![1.0, 2.0, 3.0, 4.0];
        let mut output = vec![0.0; 4];

        let processed = processor.process_chunk(&input, &mut output).unwrap();
        assert_eq!(processed, 4);
        assert_eq!(output, input);

        // Test size mismatch
        let mut output_wrong_size = vec![0.0; 2];
        assert!(
            processor
                .process_chunk(&input, &mut output_wrong_size)
                .is_err()
        );
    }

    #[test]
    fn test_serializable() {
        let processor = MockProcessor {
            name: "test_processor".to_string(),
            ..Default::default()
        };

        // Test serialization
        let data = processor.serialize().unwrap();
        assert_eq!(data, b"test_processor");

        // Test type_id
        assert_eq!(processor.type_id(), "MockProcessor");

        // Test deserialization
        let mut new_processor = MockProcessor::default();
        assert!(new_processor.deserialize(&data).is_ok());
        assert_eq!(new_processor.name, "test_processor");

        // Test invalid data
        assert!(new_processor.deserialize(&[0xFF, 0xFE]).is_err());
    }

    #[test]
    fn test_bypassable() {
        let mut processor = MockProcessor::default();

        assert!(!processor.is_bypassed());

        processor.set_bypassed(true);
        assert!(processor.is_bypassed());

        processor.toggle_bypass();
        assert!(!processor.is_bypassed());

        processor.toggle_bypass();
        assert!(processor.is_bypassed());
    }

    #[test]
    fn test_processor_info() {
        let processor = MockProcessor::default();

        assert_eq!(processor.name(), "MockProcessor");
        assert_eq!(processor.version(), "1.0.0");
        assert_eq!(processor.description(), "");
        assert_eq!(processor.author(), "ABOP");
        assert_eq!(processor.supported_input_formats(), vec!["f32".to_string()]);
        assert_eq!(
            processor.supported_output_formats(),
            vec!["f32".to_string()]
        );

        let named_processor = MockProcessor {
            name: "CustomName".to_string(),
            ..Default::default()
        };
        assert_eq!(named_processor.name(), "CustomName");
    }

    #[test]
    fn test_full_audio_processor() {
        let mut processor = MockProcessor {
            is_valid: true,
            ..Default::default()
        };

        let mut buffer = AudioBuffer {
            data: vec![0.0f32; 100],
            format: crate::audio::SampleFormat::F32,
            channels: 2,
            sample_rate: 44100,
        };

        // Test validate_and_process with valid processor
        assert!(processor.validate_and_process(&mut buffer).is_ok());

        // Test validate_and_process with invalid processor
        processor.is_valid = false;
        assert!(processor.validate_and_process(&mut buffer).is_err());
    }

    #[test]
    fn test_thread_safe_trait() {
        // Test that our mock processor is thread-safe
        fn assert_thread_safe<T: ThreadSafe>() {}
        assert_thread_safe::<MockProcessor>();
    }
}
