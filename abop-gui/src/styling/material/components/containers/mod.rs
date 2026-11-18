//! Material Design 3 Container Components
//!
//! This module provides Material Design 3 container components including cards,
//! surfaces, dividers, and layout containers that integrate with Iced framework.
//!
//! ## Module Organization
//! - [`card`] - Material Design 3 Card component
//! - [`surface`] - Surface component with elevation variants
//! - [`divider`] - Divider component with orientation options
//! - [`container`] - General container with alignment and styling
//! - [`tokens`] - MaterialTokens integration module

// Re-export all container modules
pub mod card;
pub mod container;
pub mod divider;
pub mod surface;
pub mod tokens;

// Re-export all container types for convenient use
pub use card::{CardVariant, ElevationLevel, MaterialCard};
pub use container::{ContainerAlignment, MaterialContainer};
pub use divider::{DividerOrientation, MaterialDivider};
pub use surface::{MaterialSurface, SurfaceVariant};
