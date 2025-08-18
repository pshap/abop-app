//! Audio processing traits organized by functionality
//!
//! This module provides a comprehensive set of traits for audio processing
//! components, organized into logical groups for better maintainability.
//!
//! # Module Organization
//!
//! - [`core`] - Fundamental traits for basic audio processing
//! - [`reporting`] - Traits for monitoring and status reporting  
//! - [`lifecycle`] - Traits for processor lifecycle management
//! - [`specialized`] - Traits for specialized processing scenarios

pub mod core;
pub mod lifecycle;
pub mod reporting;
pub mod specialized;

// Re-export all traits for backward compatibility and convenience
pub use core::{AudioProcessor, Bypassable, Configurable, ThreadSafe, Validatable};
pub use lifecycle::{ProcessorLifecycle, Serializable};
pub use reporting::{LatencyReporting, ProcessorInfo, ProgressReporting, ResourceEstimation};
pub use specialized::{FileWriter, StreamingProcessor};

