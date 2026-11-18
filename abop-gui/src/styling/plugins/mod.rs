//! Plugin-based Style System
//!
//! This module provides a plugin architecture for extending the styling system
//! with custom components, themes, and behaviors.
//!
//! ## Module Organization
//! - [`types`] - Plugin types, errors, and common definitions
//! - [`registry`] - Plugin registry for managing style plugins
//! - [`customization`] - Advanced customization API with theme overrides
//! - [`builder`] - Custom style builder with fluent interface

// Re-export all plugin modules
pub mod types;
pub mod registry;
pub mod customization;
pub mod builder;

// Re-export all public types for convenient use
pub use types::{
    CustomComponentStyle, PluginInfo, StylePlugin, StylePluginError, StyleError,
    StyleProperty, StyleResult, read_lock, write_lock,
};
pub use registry::StylePluginRegistry;
pub use customization::StyleCustomizationAPI;
pub use builder::CustomStyleBuilder;