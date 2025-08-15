//! Traits for processor lifecycle management and state handling
//!
//! This module provides traits for managing the lifecycle of audio processors,
//! including initialization, cleanup, and state persistence.

use super::super::error::Result;

/// Trait for processors that have a managed lifecycle
pub trait ProcessorLifecycle {
    /// Initialize the processor
    ///
    /// This method is called when the processor is first created or needs
    /// to be reset to a clean state.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails due to resource constraints
    /// or invalid configuration.
    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Prepare the processor for processing
    ///
    /// This method is called before processing begins, allowing the processor
    /// to allocate resources, validate configuration, and prepare internal state.
    ///
    /// # Arguments
    /// * `sample_rate` - The sample rate that will be used for processing
    /// * `max_buffer_size` - The maximum buffer size that will be processed
    ///
    /// # Errors
    ///
    /// Returns an error if preparation fails.
    fn prepare(&mut self, sample_rate: u32, max_buffer_size: usize) -> Result<()> {
        let _ = (sample_rate, max_buffer_size);
        Ok(())
    }

    /// Clean up resources when processing is complete
    ///
    /// This method is called when processing is finished, allowing the processor
    /// to release resources and perform cleanup operations.
    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Check if the processor is currently active/ready
    fn is_active(&self) -> bool {
        true
    }

    /// Force stop the processor (emergency cleanup)
    ///
    /// This method should perform immediate cleanup without waiting for
    /// graceful shutdown. Used in error conditions or forced termination.
    fn force_stop(&mut self) -> Result<()> {
        self.cleanup()
    }
}

/// Trait for processors that support state serialization
pub trait Serializable {
    /// Serialize the processor state to bytes
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails due to memory constraints
    /// or serialization format issues.
    fn serialize(&self) -> Result<Vec<u8>>;

    /// Deserialize processor state from bytes
    ///
    /// # Arguments
    /// * `data` - The serialized state data
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails due to corrupted data,
    /// version incompatibility, or format issues.
    fn deserialize(&mut self, data: &[u8]) -> Result<()>;

    /// Get a unique identifier for this processor type
    fn type_id(&self) -> &'static str;
}

