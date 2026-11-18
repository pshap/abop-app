//! Common utilities for audio processing

// Re-export commonly used functions at module level
pub use buffer::validate_buffer;
pub use performance::{estimate_memory_usage, estimate_processing_time};
pub use timing::Timer;

/// Common buffer validation utilities
pub mod buffer;

/// Sample rate validation utilities
pub mod sample_rate;

/// Channel count validation utilities
pub mod channels;

/// Memory estimation utilities
pub mod memory;

/// Performance timing utilities
pub mod timing;

/// Performance utilities for timing and estimation
pub mod performance;

/// Common audio processing constants
pub mod constants;
