//! Library data model and operations

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Represents an audiobook library
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Library {
    /// Unique identifier for the library
    pub id: String,
    /// Display name of the library
    pub name: String,
    /// Filesystem path to the library
    pub path: PathBuf,
}

impl Library {
    /// Creates a new library with the given name and path
    #[must_use]
    pub fn new<P: AsRef<Path>>(name: &str, path: P) -> Self {
        // Ensure the path is canonicalized to handle platform-specific path issues
        let path_buf = if path.as_ref().is_absolute() {
            path.as_ref().to_path_buf()
        } else {
            // For relative paths, convert to absolute using current directory
            match std::env::current_dir() {
                Ok(current_dir) => current_dir.join(path.as_ref()),
                Err(_) => path.as_ref().to_path_buf(), // Fallback to original path if current_dir fails
            }
        };

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            path: path_buf,
        }
    }

    /// Checks if the library path exists on the filesystem
    #[must_use]
    pub fn path_exists(&self) -> bool {
        self.path.exists()
    }

    /// Gets the display name for the library
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_library_creation() {
        let test_path = if cfg!(windows) {
            r"C:\test\library"
        } else {
            "/test/library"
        };

        let library = Library::new("Test Library", test_path);
        assert_eq!(library.name, "Test Library");
        assert_eq!(library.path, Path::new(test_path));
        assert!(!library.id.is_empty());
    }

    #[test]
    fn test_library_display_name() {
        let test_path = if cfg!(windows) {
            r"C:\some\path"
        } else {
            "/some/path"
        };

        let library = Library::new("My Library", test_path);
        assert_eq!(library.display_name(), "My Library");
    }
}
