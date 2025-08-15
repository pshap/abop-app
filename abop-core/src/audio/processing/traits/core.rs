//! Core audio processing traits
//!
//! This module defines the fundamental traits that all audio processing
//! components must implement for basic functionality.

use super::super::error::Result;
use crate::audio::AudioBuffer;

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

/// Trait for processors that support validation of their state
pub trait Validatable {
    /// Validate the current processor state
    ///
    /// # Errors
    ///
    /// Returns an error if the processor is in an invalid state.
    fn validate(&self) -> Result<()>;
}

/// Marker trait for processors that are thread-safe
///
/// This is a marker trait that ensures processors can be safely used
/// across thread boundaries.
pub trait ThreadSafe: Send + Sync {}

/// Automatically implement `ThreadSafe` for types that are Send + Sync
impl<T: Send + Sync> ThreadSafe for T {}

/// Trait for processors that can be bypassed
pub trait Bypassable {
    /// Check if processing is currently bypassed
    fn is_bypassed(&self) -> bool;

    /// Set bypass state
    fn set_bypassed(&mut self, bypassed: bool);

    /// Toggle bypass state
    fn toggle_bypass(&mut self) {
        let current = self.is_bypassed();
        self.set_bypassed(!current);
    }
}

