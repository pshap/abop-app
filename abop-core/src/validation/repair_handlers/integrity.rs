//! Integrity-specific repair operations

use crate::models::AppState;
use crate::validation::error::ValidationError;
use crate::validation::recovery::{RepairAction, RepairActionType};
use crate::validation::repair_patterns::IssuePattern;
use super::repair_handler::{RepairHandler, create_repair_action_success};

/// Handles repair operations for integrity-related issues
#[derive(Debug, Default)]
pub struct IntegrityRepairHandler;

impl RepairHandler for IntegrityRepairHandler {    fn can_handle(&self, pattern: &IssuePattern) -> bool {
        matches!(pattern,
            IssuePattern::Duplicate |
            IssuePattern::Orphaned
        )
    }
    
    fn name(&self) -> &'static str {
        "Integrity Repair Handler"
    }
    
    fn repair(&self, state: &mut AppState, issue: &ValidationError) -> Vec<RepairAction> {
        let mut actions = Vec::new();
        
        // Convert error to pattern for processing
        if let Some(pattern) = IssuePattern::from_validation_error(issue) {
            match pattern {                IssuePattern::Duplicate => {
                    let removed_count = Self::remove_duplicates(state);
                    if removed_count > 0 {
                        actions.push(create_repair_action_success(
                            RepairActionType::Remove,
                            format!("Removed {} duplicate entries", removed_count),
                            "duplicates".to_string(),
                        ));
                    }
                }
                _ => {
                    // Unknown pattern for integrity category
                    actions.push(RepairAction::failure(
                        RepairActionType::Update,
                        format!("Unknown integrity issue pattern: {}", issue.message),
                        "integrity".to_string(),
                        "Pattern not recognized".to_string(),
                    ));
                }
            }
        }
        
        actions
    }
}

impl IntegrityRepairHandler {
    fn remove_duplicates(state: &mut AppState) -> usize {
        let mut removed_count = 0;

        // Remove duplicate libraries (by path)
        let mut seen_library_paths = std::collections::HashSet::new();
        state.app_data.libraries.retain(|library| {
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
        state.app_data.audiobooks.retain(|audiobook| {
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
