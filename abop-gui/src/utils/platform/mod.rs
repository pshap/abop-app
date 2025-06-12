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
    return windows::get_default_audiobook_directory();
    
    #[cfg(target_os = "macos")]
    return macos::get_default_audiobook_directory();
    
    #[cfg(all(unix, not(target_os = "macos")))]
    return unix::get_default_audiobook_directory();
      #[cfg(not(any(windows, unix)))]
    return PathBuf::from(".");
}
