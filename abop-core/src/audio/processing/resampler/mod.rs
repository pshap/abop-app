//! Audio resampling functionality
//!
//! This module provides sample rate conversion for audio buffers using
//! a simple linear resampling algorithm suitable for testing and basic
//! audio processing tasks.

pub mod core;
pub mod error;
pub mod scalar;
pub mod simd;
pub mod tests;
pub mod traits_impl;
pub mod types;

// Re-export the main types for convenience
pub use core::LinearResampler;
pub use error::ResamplerError;
pub use types::ResampleBufferFn;
