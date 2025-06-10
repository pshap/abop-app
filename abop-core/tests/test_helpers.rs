//! Test helpers for ABOP-Core tests.
//!
//! This module provides common utilities and fixtures for tests.
//! All helpers are designed to be thread-safe and can be used in parallel tests.

use abop_core::AppState;

/// Creates a default AppState for testing.
///
/// # Examples
/// ```
/// use abop_core::tests::test_helpers::create_test_app_state;
///
/// let app_state = create_test_app_state();
/// assert_eq!(app_state.libraries().len(), 0);
/// ```
pub fn create_test_app_state() -> AppState {
    AppState::default()
}
