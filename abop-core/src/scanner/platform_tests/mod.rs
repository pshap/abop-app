//! Platform-specific tests for file discovery
//!
//! This module contains platform-specific test implementations
//! to ensure proper behavior across different operating systems.

#[cfg(windows)]
pub mod windows_tests;

#[cfg(unix)]
pub mod unix_tests;

#[cfg(test)]
mod shared_tests;
