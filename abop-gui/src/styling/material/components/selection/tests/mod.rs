//! Test module organization for selection components
//!
//! This module organizes all test functionality for the selection component system,
//! including chips, checkboxes, switches, and their collections.

// Legacy test modules (maintained for compatibility)

/// Checkbox component tests
pub mod checkbox_tests;

/// Basic chip component tests  
pub mod chip_tests;

/// Chip validation and error handling tests
pub mod chip_validation_tests;

/// Chip integration tests across different scenarios
pub mod chip_integration_tests;

/// Switch component tests
pub mod switch_tests;

// New comprehensive test framework modules

/// Builder pattern tests and validation
pub mod builder_patterns;

/// Collection behavior and interaction tests
pub mod collection_behavior;

/// Validation rules and error handling tests
pub mod validation_rules;

// TODO: Create these modules when needed
// pub mod integration_scenarios;
// pub mod performance_benchmarks;

/// Test fixtures, factories, and shared test data
pub mod fixtures;

// Test modules are private by default, only the test runner needs to access them
