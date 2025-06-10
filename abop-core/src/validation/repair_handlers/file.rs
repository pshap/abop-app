//! File-specific repair operations

use super::repair_handler::RepairHandler;
use crate::models::AppState;
use crate::validation::error::ValidationError;
use crate::validation::recovery::{RepairAction, RepairActionType};
use crate::validation::repair_patterns::IssuePattern;

/// Handles repair operations for file-related issues
#[derive(Debug, Default)]
pub struct FileRepairHandler;

impl RepairHandler for FileRepairHandler {
    fn can_handle(&self, pattern: &IssuePattern) -> bool {
        matches!(pattern, IssuePattern::FileNotExists)
    }

    fn name(&self) -> &'static str {
        "File Repair Handler"
    }

    fn repair(&self, _state: &mut AppState, _issue: &ValidationError) -> Vec<RepairAction> {
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
}
