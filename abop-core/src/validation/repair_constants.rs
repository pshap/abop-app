//! Constants for repair operations

/// Category constants for validation issues
pub mod categories {
    pub const LIBRARY: &str = "library";
    pub const AUDIOBOOK: &str = "audiobook";
    pub const PROGRESS: &str = "progress";
    pub const PREFERENCES: &str = "preferences";
    pub const FILE: &str = "file";
    pub const INTEGRITY: &str = "integrity";
}

/// Error pattern constants for string matching
pub mod error_patterns {
    pub const EMPTY_NAME: &str = "empty name";
    pub const DOES_NOT_EXIST: &str = "does not exist";
    pub const INVALID_DURATION: &str = "invalid duration";
    pub const NON_EXISTENT: &str = "non-existent";
    pub const ORPHANED: &str = "orphaned";
    pub const EXCEEDS_DURATION: &str = "exceeds duration";
    pub const NO_LONGER_EXISTS: &str = "no longer exists";
    pub const TOO_SMALL: &str = "too small";
    pub const DUPLICATE: &str = "duplicate";
}

/// Default values for repair operations
pub mod defaults {
    pub const DEFAULT_WINDOW_WIDTH: u32 = 800;
    pub const DEFAULT_WINDOW_HEIGHT: u32 = 600;
    pub const MIN_WINDOW_SIZE: u32 = 100;
}
