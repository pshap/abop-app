//! Platform-specific default directory resolution
//!
//! This module provides platform-appropriate default directories
//! for audiobook storage and application data.

#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub mod unix;

#[cfg(target_os = "macos")]
pub mod macos;

use std::path::PathBuf;

/// Get the default audiobook directory for the current platform
pub fn get_default_audiobook_directory() -> PathBuf {
    #[cfg(windows)]
    {
        windows::get_default_audiobook_directory()
    }
    
    #[cfg(target_os = "macos")]
    {
        macos::get_default_audiobook_directory()
    }
    
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        unix::get_default_audiobook_directory()
    }
    
    #[cfg(not(any(windows, unix)))]
    {
        // Ultimate fallback for unknown platforms
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}
