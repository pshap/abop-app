//! Test module organization for selection components

// Legacy test modules (maintained for compatibility)
pub mod checkbox_tests;
pub mod chip_tests;
pub mod chip_test_helpers;
pub mod chip_validation_tests;
pub mod chip_integration_tests;
pub mod switch_tests;

// New comprehensive test framework modules
pub mod basic_functionality;
pub mod builder_patterns;
pub mod collection_behavior;
pub mod validation_rules;
pub mod integration_scenarios;
pub mod performance_benchmarks;
pub mod fixtures;

// Test modules are private by default, only the test runner needs to access them
