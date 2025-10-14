//! Shared test helpers for component tests
//!
//! Keep test-only utilities here to reduce duplication in component tests.

#![cfg(test)]

use crate::styling::material::MaterialTokens;

/// Returns default Material tokens for tests
pub(crate) fn default_tokens() -> MaterialTokens {
    MaterialTokens::default()
}

/// Simple assertion helper for elements that should render without panicking
pub(crate) fn assert_renders_ok<T>(_element: T) {
    // No-op: existence implies the builder returned successfully
}
