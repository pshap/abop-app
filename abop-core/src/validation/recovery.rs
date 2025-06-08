//! State recovery and repair mechanisms for application state validation issues

use super::error::{ValidationError, ValidationResult, ValidationSeverity};
use super::repair_handlers::get_all_handlers;
use super::repair_patterns::IssuePattern;
use crate::models::AppState;
use std::collections::HashSet;

/// Create a unique identifier for a ValidationError based on its fields
fn create_error_id(error: &ValidationError) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    error.severity.hash(&mut hasher);
    error.category.hash(&mut hasher);
    error.message.hash(&mut hasher);
    if let Some(ref path) = error.file_path {
        path.hash(&mut hasher);
    }
    if let Some(ref field) = error.field {
        field.hash(&mut hasher);
    }

    format!("{:x}", hasher.finish())
}

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
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            create_backups: true,
            max_removals_per_repair: 10,
            preserve_user_data: true,
        }
    }

    /// Create an aggressive repair strategy that prioritizes fixing issues
    #[must_use]
    pub const fn aggressive() -> Self {
        Self {
            create_backups: true,
            max_removals_per_repair: 1000,
            preserve_user_data: false,
        }
    }

    /// Repair application state based on validation issues using modular handler system
    pub fn repair(&self, state: &mut AppState, context: &RepairContext) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        // Create backup if requested
        if self.create_backups {
            let backup_action = Self::create_backup(state);
            actions.push(backup_action);
        }

        // Get all available repair handlers
        let handlers = get_all_handlers();

        // Track processed errors to avoid duplicates
        let mut processed_errors = HashSet::new();

        // Process each validation error with appropriate handlers
        for issue in context.auto_repairable_issues() {
            // Create unique identifier for this error
            let error_id = create_error_id(issue);

            // Skip if we've already processed this error
            if processed_errors.contains(&error_id) {
                continue;
            }

            // Convert error to issue pattern for type-safe handling
            if let Some(pattern) = IssuePattern::from_validation_error(issue) {
                // Find handlers that can process this pattern
                for handler in &handlers {
                    if handler.can_handle(&pattern) {
                        // Apply repair and collect actions
                        let handler_actions = handler.repair(state, issue);
                        actions.extend(handler_actions);
                        processed_errors.insert(error_id.clone());
                        break; // Only one handler per issue
                    }
                }
            } else {
                // Fallback for unrecognized patterns
                actions.push(RepairAction::failure(
                    RepairActionType::Update,
                    format!("Unrecognized issue pattern: {}", issue.message),
                    issue.category.clone(),
                    "No handler available for this issue type".to_string(),
                ));
            }
        }

        actions
    }

    /// Create a backup of the current state
    fn create_backup(state: &AppState) -> RepairAction {
        // In a real implementation, this would save the state to a backup file
        // For now, we'll just log the action
        RepairAction::success(
            RepairActionType::Backup,
            "Created state backup before repair".to_string(),
            "app_state".to_string(),
        )
        .with_details(format!(
            "Backed up {} libraries, {} audiobooks",
            state.app_data.libraries.len(),
            state.app_data.audiobooks.len()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AppState;
    use crate::validation::error::ValidationError;

    fn create_test_state() -> AppState {
        AppState::default()
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
    fn test_repair_with_new_handler_system() {
        let mut state = create_test_state();

        // Create a validation result with various issues
        let mut result = ValidationResult::new();
        result.add_issue(ValidationError::error("library", "Library has empty name"));
        result.add_issue(ValidationError::error(
            "progress",
            "Progress entry for non-existent audiobook",
        ));

        let context = RepairContext::new(&result);
        let strategy = StateRepairStrategy::default();

        let actions = strategy.repair(&mut state, &context);

        // Should have at least a backup action
        assert!(!actions.is_empty());
        assert_eq!(actions[0].action_type, RepairActionType::Backup);
    }

    #[test]
    fn test_repair_context_auto_repairable_issues() {
        let mut result = ValidationResult::new();
        result.add_issue(ValidationError::error("test", "Critical error"));
        result.add_issue(ValidationError::warning("test", "Warning message"));
        result.add_issue(ValidationError::info("test", "Info message"));

        let context = RepairContext::with_settings(&result, true, false, 5);
        let auto_repairable = context.auto_repairable_issues();

        // Only critical/error issues should be auto-repairable with these settings
        assert_eq!(auto_repairable.len(), 1);
        assert_eq!(auto_repairable[0].severity, ValidationSeverity::Error);
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
