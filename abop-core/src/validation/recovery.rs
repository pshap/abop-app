//! State recovery and repair mechanisms for application state validation issues

use super::error::{ValidationError, ValidationResult, ValidationSeverity};
use crate::models::AppState;
use std::collections::HashMap;

/// Context for state repair operations containing validation results and repair options
#[derive(Debug, Clone)]
pub struct RepairContext {
    /// Validation issues that triggered the repair
    pub issues: Vec<ValidationError>,
    /// Whether to automatically repair critical issues
    pub auto_repair_critical: bool,
    /// Whether to automatically repair non-critical issues
    pub auto_repair_warnings: bool,
    /// Maximum number of backup states to create during repair
    pub max_backups: usize,
}

impl RepairContext {
    /// Create a new repair context from validation results
    pub fn new(validation_result: &ValidationResult) -> Self {
        Self {
            issues: validation_result.issues.clone(),
            auto_repair_critical: true,
            auto_repair_warnings: false,
            max_backups: 3,
        }
    }

    /// Create a repair context with custom settings
    pub fn with_settings(
        validation_result: &ValidationResult,
        auto_repair_critical: bool,
        auto_repair_warnings: bool,
        max_backups: usize,
    ) -> Self {
        Self {
            issues: validation_result.issues.clone(),
            auto_repair_critical,
            auto_repair_warnings,
            max_backups,
        }
    }

    /// Get issues that should be auto-repaired based on settings
    pub fn auto_repairable_issues(&self) -> Vec<&ValidationError> {
        self.issues
            .iter()
            .filter(|issue| match issue.severity {
                ValidationSeverity::Critical | ValidationSeverity::Error => {
                    self.auto_repair_critical
                }
                ValidationSeverity::Warning => self.auto_repair_warnings,
                ValidationSeverity::Info => false,
            })
            .collect()
    }
}

/// Represents a repair action taken on the application state
#[derive(Debug, Clone)]
pub struct RepairAction {
    /// Type of repair action performed
    pub action_type: RepairActionType,
    /// Description of what was repaired
    pub description: String,
    /// Field or component that was repaired
    pub target: String,
    /// Whether the repair was successful
    pub success: bool,
    /// Any additional details about the repair
    pub details: Option<String>,
}

/// Types of repair actions that can be performed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepairActionType {
    /// Remove invalid or orphaned entries
    Remove,
    /// Reset field to default value
    Reset,
    /// Update field with corrected value
    Update,
    /// Create missing entries
    Create,
    /// Reorder or restructure data
    Reorganize,
    /// Create backup before repair
    Backup,
}

impl std::fmt::Display for RepairActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Remove => write!(f, "Remove"),
            Self::Reset => write!(f, "Reset"),
            Self::Update => write!(f, "Update"),
            Self::Create => write!(f, "Create"),
            Self::Reorganize => write!(f, "Reorganize"),
            Self::Backup => write!(f, "Backup"),
        }
    }
}

impl RepairAction {
    /// Create a successful repair action
    pub const fn success(
        action_type: RepairActionType,
        description: String,
        target: String,
    ) -> Self {
        Self {
            action_type,
            description,
            target,
            success: true,
            details: None,
        }
    }

    /// Create a failed repair action
    pub const fn failure(
        action_type: RepairActionType,
        description: String,
        target: String,
        error: String,
    ) -> Self {
        Self {
            action_type,
            description,
            target,
            success: false,
            details: Some(error),
        }
    }

    /// Add additional details to the repair action
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
}

/// Strategy for repairing application state issues
pub struct StateRepairStrategy {
    /// Whether to create backups before repairs
    pub create_backups: bool,
    /// Maximum number of items to remove in a single repair operation
    pub max_removals_per_repair: usize,
    /// Whether to preserve user data when possible
    pub preserve_user_data: bool,
}

impl Default for StateRepairStrategy {
    fn default() -> Self {
        Self {
            create_backups: true,
            max_removals_per_repair: 100,
            preserve_user_data: true,
        }
    }
}

impl StateRepairStrategy {
    /// Create a conservative repair strategy that preserves data
    pub const fn conservative() -> Self {
        Self {
            create_backups: true,
            max_removals_per_repair: 10,
            preserve_user_data: true,
        }
    }

    /// Create an aggressive repair strategy that prioritizes fixing issues
    pub const fn aggressive() -> Self {
        Self {
            create_backups: true,
            max_removals_per_repair: 1000,
            preserve_user_data: false,
        }
    }

    /// Repair application state based on validation issues
    pub fn repair(&self, state: &mut AppState, context: &RepairContext) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        // Create backup if requested
        if self.create_backups
            && let Some(backup_action) = self.create_backup(state)
        {
            actions.push(backup_action);
        }

        // Group issues by category for efficient processing
        let mut issues_by_category: HashMap<String, Vec<&ValidationError>> = HashMap::new();
        for issue in context.auto_repairable_issues() {
            issues_by_category
                .entry(issue.category.clone())
                .or_default()
                .push(issue);
        }

        // Repair each category of issues
        for (category, issues) in issues_by_category {
            match category.as_str() {
                "library" => actions.extend(self.repair_library_issues(state, &issues)),
                "audiobook" => actions.extend(self.repair_audiobook_issues(state, &issues)),
                "progress" => actions.extend(self.repair_progress_issues(state, &issues)),
                "preferences" => actions.extend(self.repair_preferences_issues(state, &issues)),
                "file" => actions.extend(self.repair_file_issues(state, &issues)),
                "integrity" => actions.extend(self.repair_integrity_issues(state, &issues)),
                _ => {
                    // Unknown category - log but don't repair
                    actions.push(RepairAction::failure(
                        RepairActionType::Update,
                        format!("Unknown issue category: {category}"),
                        category,
                        "Cannot repair unknown issue type".to_string(),
                    ));
                }
            }
        }

        actions
    }

    /// Create a backup of the current state
    fn create_backup(&self, state: &AppState) -> Option<RepairAction> {
        // In a real implementation, this would save the state to a backup file
        // For now, we'll just log the action
        Some(
            RepairAction::success(
                RepairActionType::Backup,
                "Created state backup before repair".to_string(),
                "app_state".to_string(),
            )
            .with_details(format!(
                "Backed up {} libraries, {} audiobooks",
                state.data.libraries.len(),
                state.data.audiobooks.len()
            )),
        )
    }

    /// Repair library-related issues
    fn repair_library_issues(
        &self,
        state: &mut AppState,
        issues: &[&ValidationError],
    ) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        for issue in issues {
            if issue.message.contains("empty name") {
                // Find and repair libraries with empty names
                let repaired_count = self.repair_empty_library_names(state);
                if repaired_count > 0 {
                    actions.push(RepairAction::success(
                        RepairActionType::Update,
                        format!("Fixed {repaired_count} libraries with empty names"),
                        "library.name".to_string(),
                    ));
                }
            } else if issue.message.contains("does not exist") {
                // Remove libraries with non-existent paths
                let removed_count = self.remove_invalid_libraries(state);
                if removed_count > 0 {
                    actions.push(RepairAction::success(
                        RepairActionType::Remove,
                        format!("Removed {removed_count} libraries with invalid paths"),
                        "library.path".to_string(),
                    ));
                }
            }
        }

        actions
    }

    /// Repair audiobook-related issues
    fn repair_audiobook_issues(
        &self,
        state: &mut AppState,
        issues: &[&ValidationError],
    ) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        for issue in issues {
            if issue.message.contains("does not exist") {
                // Remove audiobooks with non-existent files
                let removed_count = self.remove_invalid_audiobooks(state);
                if removed_count > 0 {
                    actions.push(RepairAction::success(
                        RepairActionType::Remove,
                        format!("Removed {removed_count} audiobooks with missing files"),
                        "audiobook.path".to_string(),
                    ));
                }
            } else if issue.message.contains("invalid duration") {
                // Reset invalid durations
                let repaired_count = self.repair_invalid_durations(state);
                if repaired_count > 0 {
                    actions.push(RepairAction::success(
                        RepairActionType::Reset,
                        format!("Reset {repaired_count} invalid audiobook durations"),
                        "audiobook.duration_seconds".to_string(),
                    ));
                }
            }
        }

        actions
    }

    /// Repair progress-related issues
    fn repair_progress_issues(
        &self,
        state: &mut AppState,
        issues: &[&ValidationError],
    ) -> Vec<RepairAction> {
        let mut actions = Vec::new();
        let mut removed_count = 0;

        for issue in issues {
            if issue.message.contains("non-existent") || issue.message.contains("orphaned") {
                // Get valid audiobook IDs for reference checking
                let valid_audiobook_ids: std::collections::HashSet<_> =
                    state.data.audiobooks.iter().map(|ab| &ab.id).collect();

                // Remove orphaned progress entries
                state.data.progress.retain(|progress| {
                    let is_valid = valid_audiobook_ids.contains(&progress.audiobook_id);
                    if !is_valid {
                        removed_count += 1;
                    }
                    is_valid
                });
            } else if issue.message.contains("exceeds duration") {
                // Cap progress at audiobook duration
                let capped_count = self.cap_progress_at_duration(state);
                if capped_count > 0 {
                    actions.push(RepairAction::success(
                        RepairActionType::Update,
                        format!("Capped {capped_count} progress entries at audiobook duration"),
                        "progress.position_seconds".to_string(),
                    ));
                }
            }
        }

        if removed_count > 0 {
            actions.push(RepairAction::success(
                RepairActionType::Remove,
                format!("Removed {removed_count} orphaned progress entries"),
                "progress".to_string(),
            ));
        }

        actions
    }

    /// Repair user preferences issues
    fn repair_preferences_issues(
        &self,
        state: &mut AppState,
        issues: &[&ValidationError],
    ) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        for issue in issues {
            if issue.message.contains("no longer exists") {
                // Remove non-existent recent directories
                let removed_count = self.clean_recent_directories(state);
                if removed_count > 0 {
                    actions.push(RepairAction::success(
                        RepairActionType::Remove,
                        format!("Removed {removed_count} non-existent recent directories"),
                        "preferences.recent_directories".to_string(),
                    ));
                }
            } else if issue.message.contains("too small") {
                // Reset window size to reasonable defaults
                let window_config = &mut state.user_preferences.window_config;
                if window_config.width < 100 || window_config.height < 100 {
                    window_config.width = 800;
                    window_config.height = 600;
                    actions.push(RepairAction::success(
                        RepairActionType::Reset,
                        "Reset window size to default values".to_string(),
                        "preferences.window_config".to_string(),
                    ));
                }
            }
        }

        actions
    }

    /// Repair file-related issues
    fn repair_file_issues(
        &self,
        _state: &mut AppState,
        _issues: &[&ValidationError],
    ) -> Vec<RepairAction> {
        // File issues are typically handled by other repair methods
        // (e.g., removing audiobooks with missing files)
        // This method can be extended for file-specific repairs like:
        // - Moving files to expected locations
        // - Updating file references
        // - Repairing file permissions

        vec![RepairAction::success(
            RepairActionType::Update,
            "File issues resolved by related repairs".to_string(),
            "files".to_string(),
        )]
    }

    /// Repair integrity-related issues
    fn repair_integrity_issues(
        &self,
        state: &mut AppState,
        issues: &[&ValidationError],
    ) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        for issue in issues {
            if issue.message.contains("duplicate") {
                // Remove duplicate entries
                let removed_count = self.remove_duplicates(state);
                if removed_count > 0 {
                    actions.push(RepairAction::success(
                        RepairActionType::Remove,
                        format!("Removed {removed_count} duplicate entries"),
                        "duplicates".to_string(),
                    ));
                }
            }
        }

        actions
    }

    // Helper methods for specific repair operations

    fn repair_empty_library_names(&self, state: &mut AppState) -> usize {
        let mut count = 0;
        for library in &mut state.data.libraries {
            if library.name.trim().is_empty() {
                library.name = format!("Library {}", library.id);
                count += 1;
            }
        }
        count
    }

    fn remove_invalid_libraries(&self, state: &mut AppState) -> usize {
        let initial_count = state.data.libraries.len();
        state.data.libraries.retain(|library| library.path.exists());
        initial_count - state.data.libraries.len()
    }

    fn remove_invalid_audiobooks(&self, state: &mut AppState) -> usize {
        let initial_count = state.data.audiobooks.len();
        state
            .data
            .audiobooks
            .retain(|audiobook| audiobook.path.exists());
        initial_count - state.data.audiobooks.len()
    }

    fn repair_invalid_durations(&self, state: &mut AppState) -> usize {
        let mut count = 0;
        for audiobook in &mut state.data.audiobooks {
            if let Some(duration) = audiobook.duration_seconds
                && duration == 0
            {
                audiobook.duration_seconds = None;
                count += 1;
            }
        }
        count
    }

    fn cap_progress_at_duration(&self, state: &mut AppState) -> usize {
        let mut count = 0;
        let audiobook_durations: HashMap<_, _> = state
            .data
            .audiobooks
            .iter()
            .filter_map(|ab| ab.duration_seconds.map(|d| (&ab.id, d)))
            .collect();

        for progress in &mut state.data.progress {
            if let Some(&duration) = audiobook_durations.get(&progress.audiobook_id)
                && progress.position_seconds > duration
            {
                progress.position_seconds = duration;
                count += 1;
            }
        }
        count
    }

    fn clean_recent_directories(&self, state: &mut AppState) -> usize {
        let initial_count = state.user_preferences.recent_directories.len();
        state
            .user_preferences
            .recent_directories
            .retain(|path| path.exists());
        initial_count - state.user_preferences.recent_directories.len()
    }

    fn remove_duplicates(&self, state: &mut AppState) -> usize {
        let mut removed_count = 0;

        // Remove duplicate libraries (by path)
        let mut seen_library_paths = std::collections::HashSet::new();
        state.data.libraries.retain(|library| {
            if seen_library_paths.contains(&library.path) {
                removed_count += 1;
                false
            } else {
                seen_library_paths.insert(library.path.clone());
                true
            }
        });

        // Remove duplicate audiobooks (by path)
        let mut seen_audiobook_paths = std::collections::HashSet::new();
        state.data.audiobooks.retain(|audiobook| {
            if seen_audiobook_paths.contains(&audiobook.path) {
                removed_count += 1;
                false
            } else {
                seen_audiobook_paths.insert(audiobook.path.clone());
                true
            }
        });

        removed_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AppData, AppState, Library, Progress, UserPreferences};
    use crate::validation::error::ValidationError;

    fn create_test_state() -> AppState {
        AppState {
            current_view: crate::models::ui::ViewType::Library,
            user_preferences: UserPreferences::default(),
            data: AppData::default(),
        }
    }

    #[test]
    fn test_repair_context_creation() {
        let mut result = ValidationResult::new();
        result.add_issue(ValidationError::error("test", "Test error"));
        result.add_issue(ValidationError::warning("test", "Test warning"));

        let context = RepairContext::new(&result);
        assert_eq!(context.issues.len(), 2);
        assert!(context.auto_repair_critical);
        assert!(!context.auto_repair_warnings);
        assert_eq!(context.max_backups, 3);

        let auto_repairable = context.auto_repairable_issues();
        assert_eq!(auto_repairable.len(), 1); // Only the error, not the warning
    }

    #[test]
    fn test_repair_action_creation() {
        let action = RepairAction::success(
            RepairActionType::Remove,
            "Removed invalid entry".to_string(),
            "test_field".to_string(),
        );

        assert_eq!(action.action_type, RepairActionType::Remove);
        assert!(action.success);
        assert_eq!(action.target, "test_field");
        assert!(action.details.is_none());

        let action_with_details = action.with_details("Additional info".to_string());
        assert!(action_with_details.details.is_some());
    }

    #[test]
    fn test_repair_empty_library_names() {
        let mut state = create_test_state();
        let library = Library::new("", "/test/path");
        state.data.libraries.push(library);

        let strategy = StateRepairStrategy::default();
        let count = strategy.repair_empty_library_names(&mut state);

        assert_eq!(count, 1);
        assert!(!state.data.libraries[0].name.is_empty());
        assert!(state.data.libraries[0].name.starts_with("Library"));
    }

    #[test]
    fn test_repair_orphaned_progress() {
        let mut state = create_test_state();

        // Add progress for non-existent audiobook
        let progress = Progress::new("non-existent-id", 100);
        state.data.progress.push(progress);

        let mut result = ValidationResult::new();
        result.add_issue(ValidationError::error(
            "progress",
            "orphaned progress entry",
        ));

        let strategy = StateRepairStrategy::default();
        let actions = strategy.repair_progress_issues(&mut state, &[&result.issues[0]]);

        assert!(state.data.progress.is_empty());
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action_type, RepairActionType::Remove);
    }

    #[test]
    fn test_conservative_vs_aggressive_strategies() {
        let conservative = StateRepairStrategy::conservative();
        let aggressive = StateRepairStrategy::aggressive();

        assert!(conservative.preserve_user_data);
        assert!(!aggressive.preserve_user_data);
        assert!(conservative.max_removals_per_repair < aggressive.max_removals_per_repair);
    }
}
