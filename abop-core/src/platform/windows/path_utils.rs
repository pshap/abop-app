//! Windows-specific path utilities
//!
//! This module provides Windows-specific path handling functionality,
//! including long path support and UNC path handling.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// The maximum path length in Windows without the extended-length prefix
const MAX_PATH: usize = 260;

/// The extended-length path prefix for Windows
const EXTENDED_PATH_PREFIX: &str = r"\\?\";

/// Converts a path to an extended-length path if it exceeds MAX_PATH
///
/// On Windows, paths longer than MAX_PATH (260 characters) require special handling.
/// This function adds the `\\?\` prefix to enable long path support.
pub fn to_extended_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();

    // If the path is already in extended format, return as is
    if is_extended(path) {
        return path.to_path_buf();
    }

    // Convert to absolute path if it's not already
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        // Get current directory and join with relative path
        std::env::current_dir().map_or_else(|_| path.to_path_buf(), |cwd| cwd.join(path))
    };

    // Convert to extended format if needed
    if needs_extended_prefix(&absolute_path) {
        let mut extended = String::with_capacity(EXTENDED_PATH_PREFIX.len() + 256);
        extended.push_str(EXTENDED_PATH_PREFIX);

        // Handle UNC paths specially
        if let Some(unc_path) = absolute_path.to_str().and_then(|s| s.strip_prefix(r"\\")) {
            extended.push_str(r"UNC\");
            extended.push_str(unc_path);
        } else {
            extended.push_str(absolute_path.to_str().unwrap_or(""));
        }

        PathBuf::from(extended)
    } else {
        absolute_path
    }
}

/// Checks if a path is in extended-length format
fn is_extended(path: &Path) -> bool {
    path.to_str()
        .is_some_and(|s| s.starts_with(EXTENDED_PATH_PREFIX))
}

/// Checks if a path needs the extended-length prefix
fn needs_extended_prefix(path: &Path) -> bool {
    // Check if the path is already in extended format
    if is_extended(path) {
        return false;
    }

    // Check if the path is a UNC path
    if let Some(unc) = path.to_str().and_then(|s| s.strip_prefix(r"\\")) {
        return unc.len() > 2 && unc.contains('\\');
    } // Check if the path is too long
    path.to_str().is_some_and(|s| s.len() >= MAX_PATH)
}

/// Converts a path to use the proper Windows path separators
pub fn normalize_path_separators<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let mut result = PathBuf::new();

    for component in path.components() {
        result.push(component.as_os_str());
    }

    result
}

/// Checks if a path is a valid Windows path
pub fn is_valid_windows_path<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    // Check for invalid characters in the path
    let invalid_chars: &[char] = &['<', '>', '"', '|', '?', '*'];

    path.to_str().is_some_and(|s| {
        !s.chars().any(|c| invalid_chars.contains(&c)) &&
        // Check for reserved device names (e.g., CON, PRN, AUX, etc.)
        !is_reserved_device_name(path)
    })
}

/// Checks if a path contains a reserved Windows device name
fn is_reserved_device_name(path: &Path) -> bool {
    const RESERVED_NAMES: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    path.file_stem()
        .and_then(OsStr::to_str)
        .is_some_and(|name| RESERVED_NAMES.contains(&name.to_uppercase().as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_to_extended_path() {
        let long_path = Path::new("C:").join("a".repeat(300));
        let extended = to_extended_path(&long_path);
        let extended_str = extended.to_str().unwrap();

        // More flexible assertion - check for \\?\ prefix and drive letter
        assert!(extended_str.starts_with(r"\\?\"));
        assert!(extended_str.contains("C:"));

        // Already extended path should remain unchanged
        let extended2 = to_extended_path(&extended);
        assert_eq!(extended, extended2);
    }

    #[test]
    fn test_unc_paths() {
        let unc_path = Path::new(r"\\server\share\file.txt");
        let extended = to_extended_path(unc_path);
        assert!(extended.to_str().unwrap().starts_with(r"\\?\UNC\"));
    }

    #[test]
    fn test_normalize_path_separators() {
        let path = Path::new("C:/some\\mixed//path\\");
        let normalized = normalize_path_separators(path);
        assert_eq!(normalized, Path::new(r"C:\some\mixed\path"));
    }

    #[test]
    fn test_is_valid_windows_path() {
        assert!(is_valid_windows_path("C:\\valid\\path.txt"));
        assert!(!is_valid_windows_path("C:\\invalid|path.txt"));
        assert!(!is_valid_windows_path("C:\\CON")); // Reserved device name
    }
}
