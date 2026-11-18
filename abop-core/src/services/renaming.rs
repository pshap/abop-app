//! Renaming service for audiobooks and libraries
//!
//! This service provides atomic operations for renaming audiobook files and
//! library directories on disk, while keeping the database in sync.

use std::fs;
use std::path::Path;
use std::sync::Arc;
use tracing::{info, instrument};

use crate::db::repositories::AudiobookRepository;
use crate::error::{AppError, Result};
use crate::models::Audiobook;

/// Service for handling renaming operations
pub struct RenamingService {
    audiobook_repo: Arc<AudiobookRepository>,
}

impl RenamingService {
    /// Create a new renaming service
    #[must_use]
    pub fn new(audiobook_repo: Arc<AudiobookRepository>) -> Self {
        Self { audiobook_repo }
    }

    /// Rename an audiobook file on disk and update the database
    ///
    /// # Arguments
    /// * `audiobook` - The audiobook to rename
    /// * `new_name` - The new filename (without extension if extension should be preserved)
    ///
    /// # Errors
    /// Returns an error if the disk operation or database update fails.
    #[instrument(skip(self, audiobook))]
    pub fn rename_audiobook(&self, audiobook: &Audiobook, new_name: &str) -> Result<Audiobook> {
        let old_path = &audiobook.path;
        let new_name = validate_file_name(new_name)?;

        // Ensure the old path exists
        if !old_path.exists() {
            return Err(AppError::Io(format!(
                "Source file not found: {}",
                old_path.display()
            )));
        }

        // Determine new path while preserving extension
        let extension = old_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let mut new_path = old_path.parent().map(Path::to_path_buf).unwrap_or_default();

        let filename_with_ext = if extension.is_empty() {
            new_name.to_string()
        } else {
            format!("{new_name}.{extension}")
        };

        new_path.push(filename_with_ext);

        if new_path == *old_path {
            return Ok(audiobook.clone());
        }

        // Don't overwrite existing files
        if new_path.exists() {
            return Err(AppError::Other(format!(
                "Destination already exists: {}",
                new_path.display()
            )));
        }

        info!(
            "Renaming audiobook file from {} to {}",
            old_path.display(),
            new_path.display()
        );

        // Perform disk operation
        fs::rename(old_path, &new_path)?;

        // Update database; the title comes from the file's tags and is left unchanged
        let mut updated_audiobook = audiobook.clone();
        updated_audiobook.path = new_path;
        updated_audiobook.updated_at = chrono::Utc::now();

        if let Err(db_error) = self.audiobook_repo.upsert(&updated_audiobook) {
            // Keep disk and database consistent: undo the rename
            if let Err(undo_error) = fs::rename(&updated_audiobook.path, old_path) {
                tracing::error!(
                    "Failed to restore {} after database error: {undo_error}",
                    old_path.display()
                );
            }
            return Err(db_error);
        }

        Ok(updated_audiobook)
    }
}

/// Validate a user-supplied file name (without extension)
///
/// Rejects names that are empty, contain path separators or would refer to a
/// different directory, so a rename can never move a file elsewhere.
fn validate_file_name(name: &str) -> Result<&str> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::InvalidData(
            "File name cannot be empty".to_string(),
        ));
    }
    if name == "." || name == ".." {
        return Err(AppError::InvalidData(format!("Invalid file name: {name}")));
    }
    if let Some(bad) = name
        .chars()
        .find(|c| matches!(c, '/' | '\\' | '\0') || c.is_control())
    {
        return Err(AppError::InvalidData(format!(
            "File name cannot contain {bad:?}"
        )));
    }
    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{TestDataFactory, TestDatabase};

    struct Fixture {
        _db: TestDatabase,
        dir: tempfile::TempDir,
        service: RenamingService,
        book: Audiobook,
    }

    fn fixture() -> Fixture {
        let db = TestDatabase::new();
        db.insert_test_library("lib", "Library", "/library");
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Original Name.mp3");
        fs::write(&path, b"audio").unwrap();
        let book =
            TestDataFactory::audiobook_with_path("book", "lib", &path, "Tag Title", "Author");
        let repo = Arc::new(AudiobookRepository::new(db.connection.clone()));
        repo.upsert(&book).unwrap();
        Fixture {
            _db: db,
            dir,
            service: RenamingService::new(repo),
            book,
        }
    }

    #[test]
    fn renames_file_keeps_extension_and_title() {
        let f = fixture();
        let updated = f.service.rename_audiobook(&f.book, "  New Name  ").unwrap();
        assert_eq!(updated.path, f.dir.path().join("New Name.mp3"));
        assert!(updated.path.exists());
        assert!(!f.book.path.exists());
        assert_eq!(updated.title.as_deref(), Some("Tag Title"));
    }

    #[test]
    fn rejects_names_that_escape_the_directory() {
        let f = fixture();
        for name in [
            "",
            "   ",
            "..",
            "../escaped",
            "sub/dir",
            "back\\slash",
            "nul\0byte",
        ] {
            assert!(
                f.service.rename_audiobook(&f.book, name).is_err(),
                "{name:?} should be rejected"
            );
        }
        assert!(f.book.path.exists(), "original file must be untouched");
    }

    #[test]
    fn refuses_to_overwrite_existing_files() {
        let f = fixture();
        fs::write(f.dir.path().join("Taken.mp3"), b"other").unwrap();
        assert!(f.service.rename_audiobook(&f.book, "Taken").is_err());
        assert!(f.book.path.exists());
        assert_eq!(fs::read(f.dir.path().join("Taken.mp3")).unwrap(), b"other");
    }

    #[test]
    fn renaming_to_same_name_is_a_no_op() {
        let f = fixture();
        let updated = f
            .service
            .rename_audiobook(&f.book, "Original Name")
            .unwrap();
        assert_eq!(updated.path, f.book.path);
        assert!(f.book.path.exists());
    }
}
