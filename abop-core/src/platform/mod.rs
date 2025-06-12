//! Platform-specific functionality
//!
//! This module provides platform-specific implementations for different operating systems.
//! Currently supports Windows with fallback implementations for other platforms.

#[cfg(windows)]
pub mod windows;

// Fallback module for non-Windows platforms
#[cfg(not(windows))]
pub mod fallback {
    //! Fallback implementations for non-Windows platforms

    pub mod env_utils {
        //! Environment utilities fallback for non-Windows platforms
        use std::env;

        /// Get environment variable (fallback implementation)
        ///
        /// # Safety
        /// This function provides read-only access to environment variables.
        /// Use with caution as environment variables may contain sensitive information.
        pub fn get_env(key: &str) -> Option<std::ffi::OsString> {
            env::var_os(key)
        }

        /// Set environment variable (fallback implementation)
        ///
        /// # Safety
        /// This function allows modification of environment variables.
        /// Use with extreme caution as this affects the entire process.
        /// Consider restricting usage to specific, validated keys only.
        ///
        /// # Security Note
        /// Environment variables can affect process behavior and security.
        /// Validate input and restrict to known-safe variables only.
        pub fn set_env(key: &str, value: &str) {
            // In production, consider restricting to specific allowed keys
            // For now, we allow all but add documentation warnings
            env::set_var(key, value);
        }
    }
}

/// Re-export platform-specific functionality
#[cfg(windows)]
pub use windows::path_utils;

#[cfg(not(windows))]
pub use fallback::*;

/// Environment variable utilities with platform-specific optimizations
#[cfg(windows)]
pub use windows::env_utils;

#[cfg(not(windows))]
pub use fallback::env_utils;
