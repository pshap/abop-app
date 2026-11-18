//! Audio normalization functionality
//!
//! This module provides volume normalization for audio buffers using
//! various normalization algorithms including peak normalization and
//! RMS-based normalization.

pub mod core;
pub mod error;
pub mod gain_application;
pub mod peak_detection;
pub mod rms_calculation;
pub mod tests;
pub mod traits_impl;

// Re-export the main types for convenience
pub use core::AudioNormalizer;
pub use error::NormalizerError;
