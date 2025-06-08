//! Material Design 3 Elevation System
//!
//! Implements the complete Material Design 3 elevation system including:
//! - Six elevation levels (0-5) with corresponding shadow values
//! - Shadow calculations and color blending
//! - Integration with Iced shadow system
//! - Strong type safety with newtypes
//! - Trait-based extensibility
//! - Cache-optimized theme-aware context
//! - Serialization support

// Re-export core types from the new modules
pub use self::{
    builder::ElevationStyleBuilder, 
    constants::*, 
    context::ElevationContext,
    level::ElevationLevel,
    performance::{ElevationCache, FastElevation},
    registry::ElevationRegistry, 
    style::{ElevationStyle, ShadowParams},
    system::MaterialElevation,
    types::*,
    utils::ComponentType,
};

// Module declarations
pub mod builder;
pub mod color_blending;
pub mod constants;
pub mod context;
pub mod level;
pub mod performance;
pub mod registry;
pub mod shadow_calculations;
pub mod serde_impl;
pub mod style;
pub mod system;
pub mod types;
pub mod utils;

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
mod tests;

/// Float comparison epsilon for elevation calculations
#[allow(dead_code)]
const FLOAT_EPSILON: f32 = f32::EPSILON * 4.0;

/// Error type for elevation system
#[derive(Debug, thiserror::Error)]
pub enum ElevationError {
    /// Invalid elevation level provided
    #[error("Invalid elevation level: {0}")]
    InvalidLevel(u8),
    /// Custom elevation not found in registry
    #[error("Custom elevation not found: {0}")]
    CustomNotFound(String),
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Trait for extensible elevation support
pub trait Elevatable {
    /// Get the elevation level for this component
    fn elevation_level(&self) -> ElevationLevel;

    /// Get optional custom elevation key
    fn custom_elevation_key(&self) -> Option<&'static str> {
        None
    }
}

/// Example Elevatable implementation for a custom component
pub struct ExampleComponent;

impl Elevatable for ExampleComponent {
    fn elevation_level(&self) -> ElevationLevel {
        ElevationLevel::Level2
    }
}






