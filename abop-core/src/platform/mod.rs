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
        pub fn get_env(key: &str) -> Option<String> {
            env::var(key).ok()
        }
        
        /// Set environment variable (fallback implementation)
        pub fn set_env(key: &str, value: &str) {
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
