//! DEPRECATED: Test helpers for chip component testing
//!
//! ⚠️  **Migration Notice**: This module has been consolidated into the unified factory pattern.
//! 
//! All functionality has been moved to `fixtures::chip_factory` with improved organization:
//! - `chip_factory::basic::*` - Basic chip creation functions
//! - `chip_factory::states::*` - State-specific variants 
//! - `chip_factory::variants::*` - Variant-specific convenience constructors
//! - `chip_factory::collections::*` - Collection creation functions
//! - `chip_factory::assertions::*` - Test assertion helpers
//! - `chip_factory::generators::*` - Data generation utilities
//!
//! **Migration Guide:**
//! ```rust
//! // Old usage:
//! use super::chip_test_helpers::*;
//! 
//! // New usage:
//! use super::fixtures::chip_factory::{
//!     assertions::*, basic::*, variants::*, collections::*
//! };
//! ```
//!
//! This module provides backward compatibility re-exports. Update your imports to use
//! the new unified factory for better organization and maintainability.

use crate::styling::material::components::selection::{
    Chip, ChipVariant,
};

// Re-export everything from the unified factory for backward compatibility
pub use super::fixtures::chip_factory::{
    // Basic creation functions
    test_chip,
    sized_chip as sized_test_chip,
    
    // State functions  
    selected_chip as selected_test_chip,
    
    // Collection functions
    collections::*,
    
    // Assertion functions
    assertions::*,
};

// Specific re-exports to avoid conflicts
pub use super::fixtures::chip_factory::generators::{
    MAX_LABEL_LENGTH, generate_test_labels, all_component_sizes, all_selection_modes,
    all_chip_variants, max_length_label,
};

// Legacy constants for backward compatibility
pub use super::fixtures::test_data::ALL_CHIP_VARIANTS;

// Deprecated wrapper functions for backward compatibility
/// Legacy function - use `chip_factory::batch::all_state_chips()` instead
#[deprecated(note = "Use chip_factory::batch::all_state_chips() instead")]
pub fn all_state_chips(label: &str, variant: ChipVariant) -> Vec<Chip> {
    super::fixtures::chip_factory::batch::all_state_chips(label, variant)
}

/// Legacy function - use `chip_factory::batch::all_variant_chips()` instead
#[deprecated(note = "Use chip_factory::batch::all_variant_chips() instead")]
pub fn all_variant_chips(label: &str) -> Vec<Chip> {
    super::fixtures::chip_factory::batch::all_variant_chips(label)
}

/// Legacy function - use `chip_factory::variants::selected_filter_chip()` instead
#[deprecated(note = "Use chip_factory::variants::selected_filter_chip() instead")]
pub fn selected_filter_chip(label: &str) -> Chip {
    super::fixtures::chip_factory::variants::selected_filter_chip(label)
}
