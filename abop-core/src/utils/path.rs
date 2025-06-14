//! Path handling utilities with Windows compatibility
//!
//! This module provides centralized path operations that are aware of Windows
//! path conventions including case-insensitive comparisons and proper path normalization.

use log;
use std::path::{Path, PathBuf};

/// Compare two paths for equality with platform-aware rules
///
/// On Windows, this performs case-insensitive comparison.
/// On other platforms, this performs case-sensitive comparison.
#[must_use] pub fn paths_equal(path1: &Path, path2: &Path) -> bool {
    #[cfg(windows)]
    {
        paths_equal_case_insensitive(path1, path2)
    }
    #[cfg(not(windows))]
    {
        path1 == path2
    }
}

/// Compare two paths for equality with case-insensitive comparison
#[must_use] pub fn paths_equal_case_insensitive(path1: &Path, path2: &Path) -> bool {
    normalize_path_for_comparison(path1) == normalize_path_for_comparison(path2)
}

/// Normalize a path for comparison purposes
///
/// This converts the path to a lowercase string representation for
/// consistent comparison across different path formats.
#[must_use] pub fn normalize_path_for_comparison(path: &Path) -> String {
    path.to_string_lossy().to_lowercase()
}

/// Normalize a path by resolving relative components and canonicalizing
///
/// This attempts to resolve `.` and `..` components and canonicalize the path
/// if possible, falling back to the original path on error.
#[must_use] pub fn normalize_path(path: &Path) -> PathBuf {
    match path.canonicalize() {
        Ok(canonical) => canonical,
        Err(_) => {
            // Fallback: manually resolve relative components
            let mut components = Vec::new();
            for component in path.components() {
                match component {
                    std::path::Component::CurDir => {
                        // Skip '.' components
                    }
                    std::path::Component::ParentDir => {
                        // Pop the last component for '..'
                        components.pop();
                    }
                    _ => {
                        components.push(component);
                    }
                }
            }
            components.into_iter().collect()
        }
    }
}

/// Check if a path exists with case-insensitive matching on Windows
///
/// This is useful for Windows where file names are case-insensitive
/// but the exact case might not match.
#[must_use] pub fn path_exists_case_insensitive(path: &Path) -> bool {
    #[cfg(windows)]
    {
        if path.exists() {
            return true;
        }

        // Try to find the path with different case
        if let Some(parent) = path.parent()
            && let Some(filename) = path.file_name()
            && let Ok(entries) = std::fs::read_dir(parent)
        {
            let target_name = filename.to_string_lossy().to_lowercase();
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let entry_name = entry.file_name().to_string_lossy().to_lowercase();
                        if entry_name == target_name {
                            return true;
                        }
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to read directory entry in {}: {}",
                            parent.display(),
                            e
                        );
                        // Continue processing other entries despite this error
                    }
                }
            }
        }
        false
    }
    #[cfg(not(windows))]
    {
        path.exists()
    }
}

/// Get the file extension in a case-insensitive manner
#[must_use] pub fn get_extension_case_insensitive(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

/// Compare file extensions case-insensitively
#[must_use] pub fn extension_matches(path: &Path, expected_ext: &str) -> bool {
    get_extension_case_insensitive(path).is_some_and(|ext| ext == expected_ext.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths_equal_case_insensitive() {
        let path1 = Path::new("C:\\Users\\Test\\file.txt");
        let path2 = Path::new("c:\\users\\test\\FILE.TXT");
        assert!(paths_equal_case_insensitive(path1, path2));
    }

    #[test]
    fn test_normalize_path_for_comparison() {
        let path = Path::new("C:\\Users\\Test\\File.TXT");
        let normalized = normalize_path_for_comparison(path);
        assert_eq!(normalized, "c:\\users\\test\\file.txt");
    }

    #[test]
    fn test_extension_matches() {
        let path = Path::new("test.MP3");
        assert!(extension_matches(path, "mp3"));
        assert!(extension_matches(path, "MP3"));
        assert!(!extension_matches(path, "wav"));
    }

    #[test]
    fn test_normalize_path() {
        let path = Path::new("./test/../file.txt");
        let normalized = normalize_path(path);
        assert!(!normalized.to_string_lossy().contains(".."));
    }
}
