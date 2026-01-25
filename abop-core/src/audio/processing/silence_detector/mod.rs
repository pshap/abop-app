//! Audio silence detection and removal functionality
//!
//! This module provides silence detection and removal capabilities for audio buffers,
//! including configurable thresholds and minimum duration requirements.

pub mod core;
pub mod helpers;
pub mod tests;
pub mod traits_impl;
pub mod types;

// Re-export the main types for easier use
pub use core::SilenceDetector;
pub use types::{SilenceDetectorError, SilenceSegment};
