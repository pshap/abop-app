//! Base repair handler trait and helper functions

use crate::validation::error::ValidationError;
use crate::validation::recovery::{RepairAction, RepairActionType};
use crate::validation::repair_patterns::IssuePattern;
use crate::models::AppState;

/// Trait for handling specific types of validation issues
pub trait RepairHandler {
    /// Check if this handler can process the given issue pattern
    fn can_handle(&self, pattern: &IssuePattern) -> bool;

    /// Perform repair actions for the given validation error
    fn repair(&self, state: &mut AppState, error: &ValidationError) -> Vec<RepairAction>;

    /// Get the name of this repair handler
    fn name(&self) -> &'static str;
}

/// Helper function to create a successful repair action
pub fn create_repair_action_success(
    action_type: RepairActionType,
    description: String,
    target: String,
) -> RepairAction {
    RepairAction {
        action_type,
        description,
        target,
        success: true,
        details: None,
    }
}

/// Helper function to create a failed repair action
pub fn create_repair_action_failure(
    action_type: RepairActionType,
    description: String,
    target: String,
    error: String,
) -> RepairAction {
    RepairAction {
        action_type,
        description,
        target,
        success: false,
        details: Some(error),
    }
}

/// Helper function to create a simple successful repair action
pub fn create_single_repair_action(
    action_type: RepairActionType,
    description: &str,
    target: &str,
) -> RepairAction {
    RepairAction {
        action_type,
        description: description.to_string(),
        target: target.to_string(),
        success: true,
        details: None,
    }
}
