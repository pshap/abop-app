//! Enhanced Material Design 3 Token System
//!
//! This module provides a comprehensive token system that integrates with unified
//! MaterialColors and provides additional semantic and component-specific tokens.
//!
//! ## Module Organization
//! - [`types`] - Main EnhancedMaterialTokens struct and factory methods
//! - [`typography`] - Typography and spacing token definitions
//! - [`tokens`] - Elevation, shape, and motion token definitions
//! - [`buttons`] - Button color and shape token definitions
//! - [`text_fields`] - Text field color and shape token definitions
//! - [`selection`] - Selection component token definitions
//! - [`builder`] - Fluent builder for creating token systems

// Re-export all enhanced token modules
pub mod builder;
pub mod buttons;
pub mod selection;
pub mod text_fields;
pub mod tokens;
pub mod types;
pub mod typography;

// Re-export all enhanced token types for convenient use
pub use builder::ThemeBuilder;
pub use buttons::{ButtonColorSet, ButtonShapes, ButtonTokens};
pub use selection::{SelectionColorSet, SelectionColors, SelectionTokens};
pub use text_fields::{TextFieldColors, TextFieldShapes, TextFieldTokens};
pub use tokens::{ElevationTokens, MotionTokens, ShapeTokens};
pub use types::EnhancedMaterialTokens;
pub use typography::{SpacingTokens, TypographyTokens};
