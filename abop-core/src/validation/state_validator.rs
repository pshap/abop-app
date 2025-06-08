//! Main state validator implementation

use super::error::{ValidationError, ValidationResult};
use super::validators::{FileValidator, IntegrityValidator, MetadataValidator, SchemaValidator};
use crate::models::{AppState, Audiobook, Library, Progress};

/// Configuration for state validation behavior
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Whether to validate file existence for referenced paths
    pub check_file_existence: bool,
    /// Whether to validate audiobook metadata consistency
    pub check_metadata_consistency: bool,
    /// Whether to validate database referential integrity
    pub check_referential_integrity: bool,
    /// Whether to validate schema version compatibility
    pub check_schema_version: bool,
    /// Maximum file size to consider valid (in bytes)
    pub max_file_size: Option<u64>,
    /// Whether to perform deep validation (slower but more thorough)
    pub deep_validation: bool,
    /// Whether to validate audio file formats
    pub check_audio_formats: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            check_file_existence: true,
            check_metadata_consistency: true,
            check_referential_integrity: true,
            check_schema_version: true,
            max_file_size: Some(10 * 1024 * 1024 * 1024), // 10GB
            deep_validation: false,
            check_audio_formats: true,
        }
    }
}

impl ValidationConfig {    /// Create a fast validation configuration (minimal checks)
    pub const fn fast() -> Self {
        Self {
            check_file_existence: false,
            check_metadata_consistency: true,
            check_referential_integrity: true,
            check_schema_version: true,
            max_file_size: None,
            deep_validation: false,
            check_audio_formats: false,
        }
    }

    /// Create a thorough validation configuration (all checks enabled)
    pub const fn thorough() -> Self {
        Self {
            check_file_existence: true,
            check_metadata_consistency: true,
            check_referential_integrity: true,
            check_schema_version: true,
            max_file_size: Some(10 * 1024 * 1024 * 1024), // 10GB
            deep_validation: true,
            check_audio_formats: true,
        }
    }
}

/// Main state validator that orchestrates all validation checks
pub struct StateValidator {
    config: ValidationConfig,
    file_validator: FileValidator,
    metadata_validator: MetadataValidator,
    integrity_validator: IntegrityValidator,
    schema_validator: SchemaValidator,
}

impl StateValidator {
    /// Create a new state validator with the given configuration
    pub fn new(config: ValidationConfig) -> Self {
        Self {
            file_validator: FileValidator::new(&config),
            metadata_validator: MetadataValidator::new(&config),
            integrity_validator: IntegrityValidator::new(&config),
            schema_validator: SchemaValidator::new(&config),
            config,
        }
    }

    /// Validate an entire application state
    pub fn validate(&self, state: &AppState) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Schema version validation (always first)
        if self.config.check_schema_version {
            self.schema_validator
                .validate_schema_version(state, &mut result);
        }

        // Validate libraries
        for library in &state.data.libraries {
            self.validate_library(library, &mut result);
        }

        // Validate audiobooks
        for audiobook in &state.data.audiobooks {
            self.validate_audiobook(audiobook, &mut result);
        }

        // Validate progress entries
        for progress in &state.data.progress {
            self.validate_progress(progress, &state.data.audiobooks, &mut result);
        }

        // Cross-referential integrity checks
        if self.config.check_referential_integrity {
            self.integrity_validator
                .validate_cross_references(state, &mut result);
        }

        // User preferences validation
        self.validate_user_preferences(state, &mut result);

        result
    }

    /// Validate a single library
    fn validate_library(&self, library: &Library, result: &mut ValidationResult) {
        // Basic library validation
        if library.name.trim().is_empty() {
            result.add_issue(
                ValidationError::error("library", "Library name cannot be empty")
                    .with_field("name")
                    .with_suggestion("Provide a meaningful library name"),
            );
        }

        if library.name.len() > 255 {
            result.add_issue(
                ValidationError::warning("library", "Library name is very long")
                    .with_field("name")
                    .with_suggestion("Consider shortening the library name"),
            );
        }

        // Path validation
        if self.config.check_file_existence {
            self.file_validator
                .validate_directory_path(&library.path, "library", result);
        }
    }

    /// Validate a single audiobook
    fn validate_audiobook(&self, audiobook: &Audiobook, result: &mut ValidationResult) {
        // File existence validation
        if self.config.check_file_existence {
            self.file_validator
                .validate_audio_file_path(&audiobook.path, result);
        }

        // Metadata validation
        if self.config.check_metadata_consistency {
            self.metadata_validator
                .validate_audiobook_metadata(audiobook, result);
        }

        // Basic field validation
        if let Some(ref title) = audiobook.title
            && title.trim().is_empty()
        {
            result.add_issue(
                ValidationError::warning("audiobook", "Audiobook title is empty")
                    .with_file_path(audiobook.path.clone())
                    .with_field("title"),
            );
        }

        // Duration validation
        if let Some(duration) = audiobook.duration_seconds {
            if duration == 0 {
                result.add_issue(
                    ValidationError::warning("audiobook", "Audiobook duration is zero")
                        .with_file_path(audiobook.path.clone())
                        .with_field("duration_seconds"),
                );
            } else if duration > 24 * 60 * 60 * 30 {
                // 30 days worth of seconds
                result.add_issue(
                    ValidationError::warning(
                        "audiobook",
                        "Audiobook duration seems unusually long",
                    )
                    .with_file_path(audiobook.path.clone())
                    .with_field("duration_seconds")
                    .with_suggestion("Verify the duration is correct"),
                );
            }
        }

        // File size validation
        if let Some(size) = audiobook.size_bytes
            && let Some(max_size) = self.config.max_file_size
            && size > max_size
        {
            result.add_issue(
                ValidationError::warning("audiobook", "Audiobook file size exceeds maximum")
                    .with_file_path(audiobook.path.clone())
                    .with_field("size_bytes")
                    .with_suggestion("Consider compressing the audio file"),
            );
        }
    }

    /// Validate a progress entry
    fn validate_progress(
        &self,
        progress: &Progress,
        audiobooks: &[Audiobook],
        result: &mut ValidationResult,
    ) {
        // Check if referenced audiobook exists
        if !audiobooks
            .iter()
            .any(|book| book.id == progress.audiobook_id)
        {
            result.add_issue(
                ValidationError::error("progress", "Progress references non-existent audiobook")
                    .with_field("audiobook_id")
                    .with_suggestion("Remove orphaned progress entry or restore missing audiobook"),
            );
        }

        // Validate position
        if let Some(audiobook) = audiobooks
            .iter()
            .find(|book| book.id == progress.audiobook_id)
            && let Some(duration) = audiobook.duration_seconds
            && progress.position_seconds > duration
        {
            result.add_issue(
                ValidationError::error("progress", "Progress position exceeds audiobook duration")
                    .with_field("position_seconds")
                    .with_suggestion("Reset progress position to valid range"),
            );
        }
    }

    /// Validate user preferences
    fn validate_user_preferences(&self, state: &AppState, result: &mut ValidationResult) {
        // Validate recent directories
        if self.config.check_file_existence {
            for (i, path) in state.user_preferences.recent_directories.iter().enumerate() {
                if !path.exists() {
                    result.add_issue(
                        ValidationError::warning(
                            "preferences",
                            "Recent directory no longer exists",
                        )
                        .with_file_path(path.clone())
                        .with_field(&format!("recent_directories[{i}]"))
                        .with_suggestion("Remove non-existent directories from recent list"),
                    );
                }
            }
        }

        // Validate window configuration
        let window_config = &state.user_preferences.window_config;
        if window_config.width < 100 || window_config.height < 100 {
            result.add_issue(
                ValidationError::error("preferences", "Window size is too small")
                    .with_field("window_config")
                    .with_suggestion("Reset window size to reasonable defaults"),
            );
        }

        if window_config.width > 10000 || window_config.height > 10000 {
            result.add_issue(
                ValidationError::warning("preferences", "Window size is unusually large")
                    .with_field("window_config")
                    .with_suggestion("Verify window size is appropriate for your display"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AppState, Progress};

    #[test]
    fn test_validation_config_defaults() {
        let config = ValidationConfig::default();
        assert!(config.check_file_existence);
        assert!(config.check_metadata_consistency);
        assert!(config.check_referential_integrity);
    }

    #[test]
    fn test_fast_config() {
        let config = ValidationConfig::fast();
        assert!(!config.check_file_existence);
        assert!(!config.deep_validation);
    }

    #[test]
    fn test_empty_state_validation() {
        let state = AppState::default();
        let validator = StateValidator::new(ValidationConfig::fast());
        let result = validator.validate(&state);

        assert!(result.is_valid());
        assert!(!result.has_critical_issues());
    }

    #[test]
    fn test_library_name_validation() {
        let mut state = AppState::default();
        let library = Library::new("", "/non/existent/path");
        state.data.libraries.push(library);

        let validator = StateValidator::new(ValidationConfig::fast());
        let result = validator.validate(&state);

        assert!(!result.is_valid());
        assert!(
            result
                .issues
                .iter()
                .any(|issue| issue.category == "library")
        );
    }

    #[test]
    fn test_orphaned_progress_detection() {
        let mut state = AppState::default();

        // Add progress for non-existent audiobook
        let progress = Progress::new("non-existent-id", 100);
        state.data.progress.push(progress);

        let validator = StateValidator::new(ValidationConfig::fast());
        let result = validator.validate(&state);

        assert!(!result.is_valid());
        assert!(result.issues.iter().any(|issue|
            issue.category == "progress" && issue.message.contains("non-existent")
        ));
    }
}
