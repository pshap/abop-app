//! Core chip implementation with state management and validation
//!
//! This module contains the fundamental Chip type and its core functionality,
//! including state transitions, validation, and trait implementations.

use super::super::builder::{Chip, ChipBuilder};
use super::super::common::*;

use std::time::Duration;

// ============================================================================
// Constants
// ============================================================================

/// Maximum allowed length for chip labels
pub const MAX_CHIP_LABEL_LENGTH: usize = 100;

/// Default animation duration for chip state transitions
pub const DEFAULT_ANIMATION_DURATION: Duration = Duration::from_millis(150);

// ============================================================================
// Core Chip Implementation Extensions
// ============================================================================

impl Chip {
    /// Create a new chip with the specified label and variant
    #[must_use]
    pub fn new<S: Into<String>>(label: S, variant: ChipVariant) -> ChipBuilder {
        ChipBuilder::new(label, variant)
    }

    /// Create a filter chip
    #[must_use]
    pub fn filter<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::filter(label)
    }

    /// Create an assist chip
    #[must_use]
    pub fn assist<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::assist(label)
    }

    /// Create an input chip
    #[must_use]
    pub fn input<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::input(label)
    }

    /// Create a suggestion chip
    #[must_use]
    pub fn suggestion<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::suggestion(label)
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Create a new chip builder
#[must_use]
pub fn chip<S: Into<String>>(label: S, variant: ChipVariant) -> ChipBuilder {
    ChipBuilder::new(label, variant)
}

/// Create a filter chip builder
#[must_use]
pub fn filter_chip<S: Into<String>>(label: S) -> ChipBuilder {
    ChipBuilder::filter(label)
}

/// Create an assist chip builder
#[must_use]
pub fn assist_chip<S: Into<String>>(label: S) -> ChipBuilder {
    ChipBuilder::assist(label)
}

/// Create an input chip builder
#[must_use]
pub fn input_chip<S: Into<String>>(label: S) -> ChipBuilder {
    ChipBuilder::input(label)
}

/// Create a suggestion chip builder
#[must_use]
pub fn suggestion_chip<S: Into<String>>(label: S) -> ChipBuilder {
    ChipBuilder::suggestion(label)
}
