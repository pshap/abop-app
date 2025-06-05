//! Specific validators for different aspects of application state

use super::error::{ValidationError, ValidationResult};
use super::state_validator::ValidationConfig;
use crate::models::{AppState, Audiobook};
use std::collections::HashSet;
use std::path::Path;

/// Validates file and directory references
/// Validates file paths and file system related validations
pub struct FileValidator {
    /// Configuration for file validation
    _config: ValidationConfig,
}

impl FileValidator {
    /// Create a new FileValidator with the given configuration
    pub fn new(config: &ValidationConfig) -> Self {
        Self {
            _config: config.clone(),
        }
    }

    /// Validate that a directory path exists and is accessible
    pub fn validate_directory_path(
        &self,
        path: &Path,
        context: &str,
        result: &mut ValidationResult,
    ) {
        if !path.exists() {
            result.add_issue(
                ValidationError::error(context, "Directory does not exist")
                    .with_file_path(path.to_path_buf())
                    .with_suggestion("Update the path or create the directory"),
            );
        } else if !path.is_dir() {
            result.add_issue(
                ValidationError::error(context, "Path exists but is not a directory")
                    .with_file_path(path.to_path_buf())
                    .with_suggestion("Ensure the path points to a directory"),
            );
        } else {
            // Check if directory is accessible
            match std::fs::read_dir(path) {
                Ok(_) => {
                    // Directory is accessible
                }
                Err(e) => {
                    result.add_issue(
                        ValidationError::error(
                            context,
                            &format!("Directory is not accessible: {e}"),
                        )
                        .with_file_path(path.to_path_buf())
                        .with_suggestion("Check directory permissions"),
                    );
                }
            }
        }
    }

    /// Validate that an audio file path exists and is accessible
    pub fn validate_audio_file_path(&self, path: &Path, result: &mut ValidationResult) {
        if !path.exists() {
            result.add_issue(
                ValidationError::error("audiobook", "Audio file does not exist")
                    .with_file_path(path.to_path_buf())
                    .with_suggestion("Remove reference or restore missing file"),
            );
        } else if !path.is_file() {
            result.add_issue(
                ValidationError::error("audiobook", "Path exists but is not a file")
                    .with_file_path(path.to_path_buf())
                    .with_suggestion("Ensure the path points to an audio file"),
            );
        } else {
            // Check file accessibility
            match std::fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.len() == 0 {
                        result.add_issue(
                            ValidationError::warning("audiobook", "Audio file is empty")
                                .with_file_path(path.to_path_buf())
                                .with_suggestion("Verify the file is not corrupted"),
                        );
                    }
                }
                Err(e) => {
                    result.add_issue(
                        ValidationError::error(
                            "audiobook",
                            &format!("Cannot access file metadata: {e}"),
                        )
                        .with_file_path(path.to_path_buf())
                        .with_suggestion("Check file permissions"),
                    );
                }
            }

            // Validate audio file extension
            if self._config.check_audio_formats {
                self.validate_audio_file_extension(path, result);
            }
        }
    }

    /// Validate that a file has a supported audio extension
    fn validate_audio_file_extension(&self, path: &Path, result: &mut ValidationResult) {
        let supported_extensions = &["mp3", "m4a", "m4b", "flac", "ogg", "wav", "aac"];

        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            let ext_lower = extension.to_lowercase();
            if !supported_extensions.contains(&ext_lower.as_str()) {
                result.add_issue(
                    ValidationError::warning("audiobook", "File has unsupported audio extension")
                        .with_file_path(path.to_path_buf())
                        .with_field("extension")
                        .with_suggestion(&format!(
                            "Consider converting to a supported format: {}",
                            supported_extensions.join(", ")
                        )),
                );
            }
        } else {
            result.add_issue(
                ValidationError::warning("audiobook", "Audio file has no extension")
                    .with_file_path(path.to_path_buf())
                    .with_suggestion("Add an appropriate audio file extension"),
            );
        }
    }
}

/// Validates audiobook metadata consistency and integrity
/// Validates metadata consistency and integrity
pub struct MetadataValidator {
    /// Configuration for metadata validation
    _config: ValidationConfig,
}

impl MetadataValidator {
    /// Create a new MetadataValidator with the given configuration
    pub fn new(config: &ValidationConfig) -> Self {
        Self {
            _config: config.clone(),
        }
    }

    /// Validate audiobook metadata fields for consistency
    pub fn validate_audiobook_metadata(
        &self,
        audiobook: &Audiobook,
        result: &mut ValidationResult,
    ) {
        // Check for missing critical metadata
        if audiobook.title.is_none()
            || audiobook
                .title
                .as_ref()
                .map(|t| t.trim().is_empty())
                .unwrap_or(true)
        {
            result.add_issue(
                ValidationError::info("metadata", "Audiobook is missing title")
                    .with_file_path(audiobook.path.clone())
                    .with_field("title")
                    .with_suggestion("Extract title from filename or metadata"),
            );
        }

        if audiobook.author.is_none() {
            result.add_issue(
                ValidationError::info("metadata", "Audiobook is missing author information")
                    .with_file_path(audiobook.path.clone())
                    .with_field("author")
                    .with_suggestion("Add author metadata if available"),
            );
        }

        if audiobook.duration_seconds.is_none() {
            result.add_issue(
                ValidationError::warning("metadata", "Audiobook is missing duration information")
                    .with_file_path(audiobook.path.clone())
                    .with_field("duration_seconds")
                    .with_suggestion("Re-scan file to extract duration metadata"),
            );
        }

        // Validate timestamp consistency
        if audiobook.updated_at < audiobook.created_at {
            result.add_issue(
                ValidationError::error("metadata", "Updated timestamp is before created timestamp")
                    .with_file_path(audiobook.path.clone())
                    .with_field("updated_at")
                    .with_suggestion("Fix timestamp inconsistency"),
            );
        }

        // Validate ID format
        if audiobook.id.trim().is_empty() {
            result.add_issue(
                ValidationError::critical("metadata", "Audiobook ID is empty")
                    .with_file_path(audiobook.path.clone())
                    .with_field("id")
                    .with_suggestion("Regenerate audiobook ID"),
            );
        }

        // Check for reasonable title length
        if let Some(ref title) = audiobook.title
            && title.len() > 500
        {
            result.add_issue(
                ValidationError::warning("metadata", "Audiobook title is unusually long")
                    .with_file_path(audiobook.path.clone())
                    .with_field("title")
                    .with_suggestion("Consider truncating the title"),
            );
        }
    }
}

/// Validates referential integrity between different data entities
/// Validates referential integrity between entities
pub struct IntegrityValidator {
    /// Configuration for integrity validation
    _config: ValidationConfig,
}

impl IntegrityValidator {
    /// Create a new IntegrityValidator with the given configuration
    pub fn new(config: &ValidationConfig) -> Self {
        Self {
            _config: config.clone(),
        }
    }

    /// Validate cross-references between all state entities
    pub fn validate_cross_references(&self, state: &AppState, result: &mut ValidationResult) {
        self.validate_library_audiobook_references(state, result);
        self.validate_duplicate_ids(state, result);
    }

    /// Validate that audiobooks reference existing libraries
    fn validate_library_audiobook_references(
        &self,
        state: &AppState,
        result: &mut ValidationResult,
    ) {
        let library_ids: HashSet<_> = state.data.libraries.iter().map(|lib| &lib.id).collect();

        for audiobook in &state.data.audiobooks {
            if !library_ids.contains(&audiobook.library_id) {
                result.add_issue(
                    ValidationError::error(
                        "integrity",
                        "Audiobook references non-existent library",
                    )
                    .with_file_path(audiobook.path.clone())
                    .with_field("library_id")
                    .with_suggestion("Create missing library or update library reference"),
                );
            }
        }
    }

    /// Check for duplicate IDs across all entities
    fn validate_duplicate_ids(&self, state: &AppState, result: &mut ValidationResult) {
        // Check library ID duplicates
        let mut library_ids = HashSet::new();
        for library in &state.data.libraries {
            if !library_ids.insert(&library.id) {
                result.add_issue(
                    ValidationError::critical("integrity", "Duplicate library ID found")
                        .with_field("library.id")
                        .with_suggestion("Regenerate duplicate library IDs"),
                );
            }
        }

        // Check audiobook ID duplicates
        let mut audiobook_ids = HashSet::new();
        for audiobook in &state.data.audiobooks {
            if !audiobook_ids.insert(&audiobook.id) {
                result.add_issue(
                    ValidationError::critical("integrity", "Duplicate audiobook ID found")
                        .with_file_path(audiobook.path.clone())
                        .with_field("audiobook.id")
                        .with_suggestion("Regenerate duplicate audiobook IDs"),
                );
            }
        }

        // Check progress ID duplicates
        let mut progress_ids = HashSet::new();
        for progress in &state.data.progress {
            let progress_key = &progress.audiobook_id; // Progress is unique per audiobook
            if !progress_ids.insert(progress_key) {
                result.add_issue(
                    ValidationError::error("integrity", "Duplicate progress entry found")
                        .with_field("progress.audiobook_id")
                        .with_suggestion("Remove duplicate progress entries"),
                );
            }
        }
    }
}

/// Validates schema version compatibility
/// Validates schema version compatibility
pub struct SchemaValidator {
    /// Configuration for schema validation
    _config: ValidationConfig,
}

impl SchemaValidator {
    /// Create a new SchemaValidator with the given configuration
    pub fn new(config: &ValidationConfig) -> Self {
        Self {
            _config: config.clone(),
        }
    }

    /// Validate schema version compatibility
    pub fn validate_schema_version(&self, _state: &AppState, result: &mut ValidationResult) {
        // For now, we don't have explicit schema versioning in the AppState
        // This is a placeholder for future schema evolution

        // In the future, this would check something like:
        // if let Some(version) = state.schema_version {
        //     if !self.is_compatible_version(version) {
        //         result.add_issue(
        //             ValidationError::critical("schema", "Incompatible schema version")
        //                 .with_field("schema_version")
        //                 .with_suggestion("Migrate data to current schema version")
        //         );
        //     }
        // }

        // For now, we'll add an info message suggesting schema versioning
        result.add_issue(
            ValidationError::info("schema", "No explicit schema version found")
                .with_suggestion("Consider adding schema versioning for future compatibility"),
        );
    }

    /// Check if a schema version is compatible (placeholder)
    #[allow(dead_code)]
    fn is_compatible_version(&self, _version: &str) -> bool {
        // Placeholder for version compatibility logic
        true
    }
}

#[cfg(test)]
mod tests {
    use super::super::error::ValidationSeverity;
    use super::*;
    use crate::models::{Audiobook, Library};
    use chrono::Utc;
    use std::path::PathBuf;

    #[test]
    fn test_file_validator_with_nonexistent_file() {
        let config = ValidationConfig::default();
        let validator = FileValidator::new(&config);
        let mut result = ValidationResult::new();

        let nonexistent_path = PathBuf::from("/definitely/does/not/exist.mp3");
        validator.validate_audio_file_path(&nonexistent_path, &mut result);

        assert!(!result.is_valid());
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_metadata_validator_empty_title() {
        let config = ValidationConfig::default();
        let validator = MetadataValidator::new(&config);
        let mut result = ValidationResult::new();

        let audiobook = Audiobook {
            id: "test".to_string(),
            library_id: "lib".to_string(),
            path: PathBuf::from("/test.mp3"),
            title: Some("".to_string()), // Empty title
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ..Default::default()
        };

        validator.validate_audiobook_metadata(&audiobook, &mut result);

        assert!(
            result
                .issues
                .iter()
                .any(|issue| issue.category == "metadata" && issue.message.contains("title"))
        );
    }

    #[test]
    fn test_integrity_validator_duplicate_ids() {
        let config = ValidationConfig::default();
        let validator = IntegrityValidator::new(&config);
        let mut result = ValidationResult::new();

        let mut state = AppState::default();

        // Add duplicate library IDs
        state
            .data
            .libraries
            .push(Library::new("Library 1", "/path1"));
        state
            .data
            .libraries
            .push(Library::new("Library 2", "/path2"));
        // Manually set duplicate ID
        state.data.libraries[1].id = state.data.libraries[0].id.clone();

        validator.validate_cross_references(&state, &mut result);

        assert!(!result.is_valid());
        assert!(
            result
                .issues
                .iter()
                .any(|issue| issue.severity == ValidationSeverity::Critical
                    && issue.message.contains("Duplicate"))
        );
    }
}
