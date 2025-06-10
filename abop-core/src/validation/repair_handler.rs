//! Trait-based repair handler system

use crate::models::AppState;
use super::error::ValidationError;
use super::{RepairAction, RepairActionType};

/// Trait for handling specific types of repairs
pub trait RepairHandler: std::fmt::Debug + Send + Sync {
    /// Check if this handler can process the given issue
    fn can_handle(&self, issue: &ValidationError) -> bool;
    
    /// Apply repairs for the given issues
    fn repair(&self, state: &mut AppState, issues: &[&ValidationError]) -> Vec<RepairAction>;
    
    /// Get the name of this repair handler
    fn name(&self) -> &'static str;
}

/// Helper function to create repair action results
pub fn create_repair_action_success(
    action_type: RepairActionType,
    count: usize,
    description: &str,
    target: &str,
) -> RepairAction {
    RepairAction::success(
        action_type,
        format!("{description} {count} items"),
        target.to_string(),
    )
}

/// Helper function to create single repair action
pub fn create_single_repair_action(
    action_type: RepairActionType,
    description: &str,
    target: &str,
) -> RepairAction {
    RepairAction::success(
        action_type,
        description.to_string(),
        target.to_string(),
    )
}
