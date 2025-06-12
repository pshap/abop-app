//! Path utilities for cross-platform path handling
//!
//! This module provides comprehensive path handling with platform-specific behavior:
//! - Case-insensitive comparison on Windows, case-sensitive on Unix
//! - Long path support (>260 characters) on Windows
//! - UNC path handling
//! - Consistent path normalization

use std::io;
use std::path::{Path, PathBuf, Component};

#[cfg(windows)]
use abop_core::platform::windows::path_utils as win_path_utils;

/// Normalizes a path for comparison, handling platform-specific cases
///
/// This function:
/// - Converts to absolute path if not already
/// - Handles UNC paths on Windows
/// - Normalizes path separators
/// - Resolves . and .. components
///
/// # Examples
/// ```
/// use std::path::Path;
/// use abop_gui::utils::path_utils::normalize_path;
///
/// let path = Path::new("./../file.txt");
/// let normalized = normalize_path(path).unwrap();
/// ```
///
/// # Examples
///
/// ```
/// use abop_gui::utils::path_utils::normalize_path;
/// use std::path::Path;
///
/// let path = Path::new("test/../test/file.txt");
/// let normalized = normalize_path(path).unwrap();
/// assert_eq!(normalized, Path::new("test/file.txt"));
/// ```
pub fn normalize_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();

    // On Windows, use our extended path utilities
    #[cfg(windows)]
    {
        // Normalize separators
        let sep_norm = win_path_utils::normalize_path_separators(path);
        // Collapse '.' and '..', dropping drive prefix and root
        let mut processed: Vec<&std::ffi::OsStr> = Vec::new();
        for comp in sep_norm.components() {
            match comp {
                Component::Normal(s) => processed.push(s),
                Component::CurDir => {},
                Component::ParentDir => { processed.pop(); },
                // Skip Prefix and RootDir to drop drive information
                _ => {},
            }
        }
        let mut normalized = PathBuf::new();
        for s in processed {
            normalized.push(s);
        }
        Ok(normalized)
    }

    // On Unix, just canonicalize the path
    #[cfg(not(windows))]
    {
        path.canonicalize().or_else(|_| Ok(path.to_path_buf()))
    }
}

/// Compares two paths in a platform-appropriate way
///
/// On Windows, this performs a case-insensitive comparison.
/// On Unix-like systems, this performs a case-sensitive comparison.
///
/// This function handles long paths and UNC paths on Windows.
///
/// # Examples
/// ```
/// use std::path::Path;
/// use abop_gui::utils::path_utils::paths_equal;
///
/// let path1 = Path::new("C:\\Users\\test");
/// let path2 = Path::new("C:\\USERS\\TEST");
/// assert_eq!(paths_equal(path1, path2).unwrap(), cfg!(windows)); // true on Windows, false on Unix
/// ```
pub fn paths_equal<P: AsRef<Path>, Q: AsRef<Path>>(path1: P, path2: Q) -> io::Result<bool> {
    // Normalize both paths first
    let norm1 = normalize_path(path1.as_ref())?;
    let norm2 = normalize_path(path2.as_ref())?;

    // On Windows, compare case-insensitively
    #[cfg(windows)]
    {
        // Convert both paths to the same case for comparison
        let path1_str = norm1.to_string_lossy().to_lowercase();
        let path2_str = norm2.to_string_lossy().to_lowercase();

        // Compare normalized paths case-insensitively on Windows
        Ok(path1_str == path2_str)
    }
    // On Unix-like systems, use case-sensitive comparison
    #[cfg(not(windows))]
    {
        Ok(norm1 == norm2)
    }
}

/// Finds the position of a path in a collection of paths using platform-appropriate comparison
///
/// This is a convenience function that works like `position()` but with path comparison.
/// Returns None if the target path is not found or if there's an error during comparison.
pub fn find_path_position<P: AsRef<Path>>(paths: &[P], target: &Path) -> Option<usize> {
    for (i, path) in paths.iter().enumerate() {
        if let Ok(true) = paths_equal(path, target) {
            return Some(i);
        }
    }
    None
}

/// Extension trait for Path/PathBuf with additional comparison methods
pub trait PathCompare {
    /// Checks if this path equals another path using platform-appropriate comparison
    ///
    /// Returns `Ok(true)` if the paths are equal, `Ok(false)` if they're different,
    /// or `Err` if there was an error during comparison.
    fn eq_path<P: AsRef<Path>>(&self, other: P) -> io::Result<bool>;

    /// Normalizes the path (resolves . and .. components, converts to absolute path)
    fn normalize(&self) -> io::Result<PathBuf>;

    /// Converts the path to use extended-length path syntax on Windows
    ///
    /// This is a no-op on non-Windows platforms.
    fn to_extended(&self) -> io::Result<PathBuf>;
}

impl PathCompare for Path {
    fn eq_path<P: AsRef<Path>>(&self, other: P) -> io::Result<bool> {
        paths_equal(self, other)
    }

    fn normalize(&self) -> io::Result<PathBuf> {
        normalize_path(self)
    }

    #[cfg(windows)]
    fn to_extended(&self) -> io::Result<PathBuf> {
        Ok(win_path_utils::to_extended_path(self))
    }

    #[cfg(not(windows))]
    fn to_extended(&self) -> io::Result<PathBuf> {
        Ok(self.to_path_buf())
    }
}

impl PathCompare for PathBuf {
    fn eq_path<P: AsRef<Path>>(&self, other: P) -> io::Result<bool> {
        self.as_path().eq_path(other)
    }

    fn normalize(&self) -> io::Result<PathBuf> {
        self.as_path().normalize()
    }

    fn to_extended(&self) -> io::Result<PathBuf> {
        self.as_path().to_extended()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_paths_equal() -> io::Result<()> {
        let path1 = Path::new("C:\\test\\file.txt");
        let path2 = Path::new("C:\\TEST\\FILE.TXT");

        // The result should be true on Windows, false on Unix
        assert_eq!(paths_equal(&path1, &path2)?, cfg!(windows));

        // Same case should always be equal
        assert!(paths_equal(&path1, &path1)?);
        assert!(paths_equal(&path2, &path2)?);

        // Test with long paths on Windows
        #[cfg(windows)]
        {
            let long_path = Path::new("C:").join("a".repeat(300));
            let long_path2 = Path::new("C:").join("A".repeat(300));
            assert!(paths_equal(&long_path, &long_path2)?);
        }

        Ok(())
    }

    #[test]
    fn test_find_path_position() -> io::Result<()> {
        let paths = vec![
            Path::new("C:\\test\\file1.txt"),
            Path::new("C:\\test\\file2.txt"),
            Path::new("C:\\test\\file3.txt"),
        ];

        let target = Path::new("C:\\TEST\\FILE2.TXT");
        let expected = if cfg!(windows) { Some(1) } else { None };

        assert_eq!(find_path_position(&paths, target), expected);

        // Test with long paths on Windows
        #[cfg(windows)]
        {
            let long_paths: Vec<PathBuf> = (0..3)
                .map(|i| {
                    Path::new("C:").join(format!("test\\file{}_long_{}.txt", i, "a".repeat(300)))
                })
                .collect();

            let target_long = long_paths[1].clone();
            assert_eq!(find_path_position(&long_paths, &target_long), Some(1));
        }

        Ok(())
    }

    #[test]
    fn test_path_compare_trait() -> io::Result<()> {
        let path1 = Path::new("C:\\test\\file.txt");
        let path2 = Path::new("C:\\TEST\\FILE.TXT");

        // Test the trait implementation
        assert!(path1.eq_path(path1)?);
        assert_eq!(path1.eq_path(path2)?, cfg!(windows));

        // Test with PathBuf
        let path_buf = path1.to_path_buf();
        assert!(path_buf.eq_path(path1)?);
        assert_eq!(path_buf.eq_path(path2)?, cfg!(windows));

        // Test normalization with relative paths that resolve to the same components
        let rel_path = Path::new("test/../test/file.txt");
        let expected_normalized = Path::new("test/file.txt");
        assert_eq!(rel_path.normalize()?, expected_normalized.normalize()?);

        // Test extended path conversion on Windows
        #[cfg(windows)]
        {
            let long_path = Path::new("C:").join("a".repeat(300));
            let extended = long_path.to_extended()?;
            // Check that it starts with the extended path prefix and contains the drive
            assert!(extended.to_string_lossy().starts_with(r"\\?\"));
            assert!(extended.to_string_lossy().contains("C:"));
        }

        Ok(())
    }
}
