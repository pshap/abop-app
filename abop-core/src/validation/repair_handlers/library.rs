//! Library-specific repair operations

use super::repair_handler::{RepairHandler, create_repair_action_success};
use crate::models::AppState;
use crate::validation::error::ValidationError;
use crate::validation::recovery::{RepairAction, RepairActionType};
use crate::validation::repair_patterns::IssuePattern;

/// Handles repair operations for library-related issues
#[derive(Debug, Default)]
pub struct LibraryRepairHandler;

impl RepairHandler for LibraryRepairHandler {
    fn can_handle(&self, pattern: &IssuePattern) -> bool {
        matches!(
            pattern,
            IssuePattern::EmptyName | IssuePattern::FileNotExists | IssuePattern::Duplicate
        )
    }

    fn name(&self) -> &'static str {
        "Library Repair Handler"
    }
    fn repair(&self, state: &mut AppState, issue: &ValidationError) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        // Convert error to pattern for processing
        if let Some(pattern) = IssuePattern::from_validation_error(issue) {
            match pattern {
                IssuePattern::EmptyName => {
                    let repaired_count = Self::repair_empty_library_names(state);
                    if repaired_count > 0 {
                        actions.push(create_repair_action_success(
                            RepairActionType::Update,
                            format!("Fixed {repaired_count} libraries with empty names"),
                            "library.name".to_string(),
                        ));
                    }
                }
                IssuePattern::FileNotExists => {
                    let removed_count = Self::remove_invalid_libraries(state);
                    if removed_count > 0 {
                        actions.push(create_repair_action_success(
                            RepairActionType::Remove,
                            format!("Removed {removed_count} libraries with invalid paths"),
                            "library.path".to_string(),
                        ));
                    }
                }
                _ => {
                    // Unknown pattern for library category
                    actions.push(RepairAction::failure(
                        RepairActionType::Update,
                        format!("Unknown library issue pattern: {}", issue.message),
                        "library".to_string(),
                        "Pattern not recognized".to_string(),
                    ));
                }
            }
        }

        actions
    }
}

impl LibraryRepairHandler {
    fn repair_empty_library_names(state: &mut AppState) -> usize {
        let mut count = 0;
        for library in &mut state.app_data.libraries {
            if library.name.trim().is_empty() {
                library.name = format!("Library {}", library.id);
                count += 1;
            }
        }
        count
    }

    fn remove_invalid_libraries(state: &mut AppState) -> usize {
        let initial_count = state.app_data.libraries.len();
        state
            .app_data
            .libraries
            .retain(|library| library.path.exists());
        initial_count - state.app_data.libraries.len()
    }
}
