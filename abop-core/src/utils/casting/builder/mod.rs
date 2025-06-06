//! Builder pattern for configurable numeric conversions
//!
//! This module provides a flexible builder pattern for numeric type conversions with
//! configurable precision, overflow handling, rounding, and validation settings.
//! The builder allows creating specialized conversion configurations for different
//! use cases like audio processing, UI calculations, and database operations.
//!
//! # Examples
//!
//! ```rust
//! use abop_core::utils::casting::CastingBuilder;
//!
//! // Create a builder for audio processing
//! let builder = CastingBuilder::for_audio();
//!
//! // Convert a float to integer with audio-optimized settings
//! let result = builder.float_to_int::<i32>(42.5);
//!
//! // Create a custom builder with specific settings
//! let custom = CastingBuilder::new()
//!     .with_precision(PrecisionMode::Strict)
//!     .with_overflow_behavior(OverflowBehavior::Fail)
//!     .with_rounding(RoundingMode::Nearest)
//!     .with_validation(ValidationLevel::Full);
//! ```

mod config;
mod impls;

pub use config::*;
pub use impls::CastingBuilder;
