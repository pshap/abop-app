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

        // Update database
        let mut updated_audiobook = audiobook.clone();
        updated_audiobook.path = new_path;
        // Also update the title to the new name if it was set
        updated_audiobook.title = Some(new_name.to_string());
        updated_audiobook.updated_at = chrono::Utc::now();

        self.audiobook_repo.upsert(&updated_audiobook)?;

        Ok(updated_audiobook)
    }
}
