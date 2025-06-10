//! Advanced builder pattern implementation for Material Design 3 selection components
//!
//! This module provides sophisticated builder patterns with built-in validation,
//! fluent APIs, and comprehensive error handling for all selection components.
//!
//! ## Architecture
//!
//! The builder system is organized into several focused modules:
//!
//! - [`validation`] - Advanced validation system with trait-based patterns
//! - [`patterns`] - Builder composition and conditional building patterns  
//! - [`checkbox`] - Checkbox builder implementation
//! - [`radio`] - Radio button builder implementation
//! - [`switch`] - Switch builder implementation
//! - [`chip`] - Chip builder implementation
//! - [`components`] - Component struct definitions
//! - [`factory`] - Builder factory functions
//!
//! ## Phase 2 Enhancements
//! - Advanced validation system with result aggregation
//! - Builder composition and conditional building
//! - Enhanced error handling with context preservation
//! - Type safety improvements and performance optimizations

// Re-export all public types and traits for a clean API
pub use checkbox::CheckboxBuilder;
pub use chip::ChipBuilder;
pub use components::{Checkbox, Chip, Radio, Switch};
pub use factory::{
    assist_chip,
    checkbox,
    checkbox_from_bool,
    // Convenience factory functions
    checked_checkbox,
    chip,
    deletable_chip,
    filter_chip,
    indeterminate_checkbox,
    input_chip,
    labeled_checkbox,
    labeled_switch,
    radio,
    selected_filter_chip,
    suggestion_chip,
    switch,
    switch_from_bool,
    switch_off,
    switch_on,
    unchecked_checkbox,
};
pub use patterns::{
    AdvancedConditionalBuilder, BatchBuilder, BuilderComposer, ComponentBuilder,
    ConditionalBuilder, StatefulBuilder,
};
pub use radio::RadioBuilder;
pub use switch::SwitchBuilder;
pub use validation::{
    BuilderValidation, ConfigurationSummary, ErrorContext, ErrorReporter, ErrorSeverity,
    ValidationComposer, ValidationContext, ValidationResult,
};

// Internal modules
mod checkbox;
mod chip;
mod components;
mod factory;
pub mod patterns;
mod radio;
mod switch;
mod validation;

// Common imports used across all builder modules
mod common {}

#[cfg(test)]
mod tests;
