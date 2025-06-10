//! Static test data and constants for chip testing
//!
//! This module provides all the static test data, constants, and reference
//! values used throughout the chip testing framework.

use crate::styling::material::components::selection::{
    ChipVariant, ComponentSize, ChipSelectionMode, ChipState
};
use crate::styling::material::components::selection::chip::core::MAX_CHIP_LABEL_LENGTH;

// ============================================================================
// Validation Constants
// ============================================================================

/// Maximum label length for chips (synchronized with core implementation)
pub const MAX_LABEL_LENGTH: usize = MAX_CHIP_LABEL_LENGTH;

/// Minimum recommended label length for UX
pub const MIN_RECOMMENDED_LABEL_LENGTH: usize = 2;

/// Length threshold for "very long" label warnings
pub const LONG_LABEL_WARNING_THRESHOLD: usize = 50;

// ============================================================================
// Test Label Sets
// ============================================================================

/// Standard valid labels for basic testing
pub const VALID_LABELS: &[&str] = &[
    "A",
    "Ok",
    "Tag",
    "File",
    "Short",
    "Filter",
    "Category",
    "Selection",
    "Medium Label",
    "A Longer Test Label",
    "Very Long Label Text Here",
];

/// Edge case labels for boundary testing  
pub const EDGE_CASE_LABELS: &[&str] = &[
    "A",                    // Minimum length
    "AB",                   // Two characters
    "😀",                   // Unicode emoji
    "Test 123",             // Mixed alphanumeric
    "Spec-ial_Chars",       // Special characters
    "Multi\nLine",          // Newline (should be handled)
    "Spaced  Out",          // Multiple spaces
    "   Padded   ",         // Leading/trailing spaces
];

/// Invalid labels for error testing
pub const INVALID_LABELS: &[&str] = &[
    "",                     // Empty string
    " ",                    // Whitespace only
    "    ",                 // Multiple whitespace
    "\t",                   // Tab character
    "\n",                   // Newline only
];

/// Performance test labels with varying lengths
pub fn performance_test_labels() -> Vec<String> {
    vec![
        "A".to_string(),
        "Short".to_string(),
        "A".repeat(10),
        "A".repeat(25),
        "A".repeat(50),
        "A".repeat(75),
        "A".repeat(MAX_LABEL_LENGTH),
    ]
}

/// Generate an oversized label for validation testing
pub fn oversized_label() -> String {
    "x".repeat(MAX_LABEL_LENGTH + 1)
}

/// Generate label with exact maximum length
pub fn max_length_label() -> String {
    "a".repeat(MAX_LABEL_LENGTH)
}

// ============================================================================
// Component Variants and Configurations
// ============================================================================

/// All chip variants for comprehensive testing
pub const ALL_CHIP_VARIANTS: &[ChipVariant] = &[
    ChipVariant::Assist,
    ChipVariant::Filter,
    ChipVariant::Input,
    ChipVariant::Suggestion,
];

/// All component sizes for testing
pub const ALL_COMPONENT_SIZES: &[ComponentSize] = &[
    ComponentSize::Small,
    ComponentSize::Medium,
    ComponentSize::Large,
];

/// All chip states for testing
pub const ALL_CHIP_STATES: &[ChipState] = &[
    ChipState::Unselected,
    ChipState::Selected,
    ChipState::Pressed,
];

/// All selection modes for collection testing
pub const ALL_SELECTION_MODES: &[ChipSelectionMode] = &[
    ChipSelectionMode::None,
    ChipSelectionMode::Single,
    ChipSelectionMode::Multiple,
];

// ============================================================================
// Test Scenarios and Use Cases
// ============================================================================

/// Real-world filter chip labels (e.g., for search interfaces)
pub const FILTER_CHIP_LABELS: &[&str] = &[
    "Category",
    "Price Range",  
    "Rating",
    "In Stock",
    "Brand",
    "Location",
    "Date Added",
    "Popularity",
    "Reviews",
    "Discount",
];

/// Input chip labels (e.g., for tags)
pub const INPUT_CHIP_LABELS: &[&str] = &[
    "rust",
    "programming",
    "web-dev",
    "frontend",
    "backend",
    "database",
    "api",
    "testing",
    "documentation",
    "performance",
];

/// Assist chip labels (e.g., for help actions)
pub const ASSIST_CHIP_LABELS: &[&str] = &[
    "Help",
    "Tutorial",
    "Get Started",
    "Learn More",
    "Documentation",
    "Contact Support",
    "FAQ",
    "Quick Tips",
];

/// Suggestion chip labels (e.g., for quick actions)
pub const SUGGESTION_CHIP_LABELS: &[&str] = &[
    "Save",
    "Share",
    "Export",
    "Download",
    "Copy Link",
    "Add to Favorites",
    "Schedule",
    "Remind Me",
];

// ============================================================================
// Performance Testing Constants
// ============================================================================

/// Number of chips for small collection performance tests
pub const SMALL_COLLECTION_SIZE: usize = 10;

/// Number of chips for medium collection performance tests
pub const MEDIUM_COLLECTION_SIZE: usize = 100;

/// Number of chips for large collection performance tests  
pub const LARGE_COLLECTION_SIZE: usize = 1_000;

/// Number of chips for stress testing
pub const STRESS_TEST_COLLECTION_SIZE: usize = 10_000;

/// Maximum acceptable time for chip creation (microseconds)
pub const MAX_CHIP_CREATION_TIME_US: u128 = 100;

/// Maximum acceptable time for collection operations (milliseconds)
pub const MAX_COLLECTION_OPERATION_TIME_MS: u128 = 50;

// ============================================================================
// Material Design 3 Compliance Constants
// ============================================================================

/// Material Design 3 minimum touch target size (dp)
pub const MD3_MIN_TOUCH_TARGET_SIZE: f32 = 48.0;

/// Material Design 3 chip height specifications (dp)
pub const MD3_CHIP_HEIGHT_SMALL: f32 = 32.0;
pub const MD3_CHIP_HEIGHT_MEDIUM: f32 = 40.0;
pub const MD3_CHIP_HEIGHT_LARGE: f32 = 48.0;

/// Material Design 3 spacing recommendations (dp)
pub const MD3_CHIP_SPACING_COMPACT: f32 = 4.0;
pub const MD3_CHIP_SPACING_STANDARD: f32 = 8.0;
pub const MD3_CHIP_SPACING_COMFORTABLE: f32 = 16.0;

// ============================================================================
// Error Message Patterns
// ============================================================================

/// Expected patterns in error messages for validation
pub const ERROR_PATTERNS: &[&str] = &[
    "empty",
    "too long",
    "invalid",
    "must have",
    "required",
    "exceeds maximum",
];

// ============================================================================
// Property-Based Testing Generators (if fake/rand deps available)
// ============================================================================

#[cfg(feature = "fake")]
/// Generate random valid label within constraints
pub fn random_valid_label() -> String {
    use fake::{Fake, faker::lorem::en::*};
    
    // Generate word between 1-3 words, 2-MAX_LABEL_LENGTH chars total
    let words: Vec<String> = (1..=3).fake::<Vec<usize>>()
        .into_iter()
        .map(|_| Word().fake::<String>())
        .collect();
    
    let label = words.join(" ");
    
    // Truncate if too long
    if label.len() > MAX_LABEL_LENGTH {
        label.chars().take(MAX_LABEL_LENGTH).collect()
    } else if label.is_empty() {
        "Test".to_string() // Fallback for empty generated strings
    } else {
        label
    }
}

#[cfg(not(feature = "fake"))]
/// Generate deterministic valid label for testing (fallback)
pub fn random_valid_label() -> String {
    "Generated Test Label".to_string()
}

/// Generate random chip variant
pub fn random_chip_variant() -> ChipVariant {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // Simple deterministic "random" selection based on current time
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    let index = (hasher.finish() as usize) % ALL_CHIP_VARIANTS.len();
    ALL_CHIP_VARIANTS[index]
}

/// Generate random component size
pub fn random_component_size() -> ComponentSize {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    let index = (hasher.finish() as usize) % ALL_COMPONENT_SIZES.len();
    ALL_COMPONENT_SIZES[index]
}

/// Generate random selection mode
pub fn random_selection_mode() -> ChipSelectionMode {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    let index = (hasher.finish() as usize) % ALL_SELECTION_MODES.len();
    ALL_SELECTION_MODES[index]
}
