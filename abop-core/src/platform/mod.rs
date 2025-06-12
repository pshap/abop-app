//! Platform-specific functionality

#[cfg(windows)]
pub mod windows;

/// Re-export platform-specific modules
#[cfg(windows)]
pub use windows::*;

/// Platform-agnostic environment variable utilities
#[cfg(windows)]
pub mod env_utils {
    pub use super::windows::env_utils::*;
}
