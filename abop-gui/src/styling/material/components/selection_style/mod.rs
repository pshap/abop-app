//! Material Design 3 Selection Component Styling System
//!
//! This module provides a sophisticated styling system for all selection components,
//! implementing the strategy pattern used throughout the Material Design system.

mod lib;
pub mod strategy;

// Re-export everything from lib
pub use lib::*;
pub use strategy::*;
