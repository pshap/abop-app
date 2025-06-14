//! Preferences-specific repair operations

use super::repair_handler::{RepairHandler, create_repair_action_success};
use crate::models::AppState;
use crate::validation::error::ValidationError;
use crate::validation::recovery::{RepairAction, RepairActionType};
use crate::validation::repair_constants::defaults;
use crate::validation::repair_patterns::IssuePattern;

/// Handles repair operations for preferences-related issues
#[derive(Debug, Default)]
pub struct PreferencesRepairHandler;

impl RepairHandler for PreferencesRepairHandler {
    fn can_handle(&self, pattern: &IssuePattern) -> bool {
        matches!(
            pattern,
            IssuePattern::NoLongerExists | IssuePattern::TooSmall
        )
    }

    fn name(&self) -> &'static str {
        "Preferences Repair Handler"
    }

    fn repair(&self, state: &mut AppState, issue: &ValidationError) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        // Convert error to pattern for processing
        if let Some(pattern) = IssuePattern::from_validation_error(issue) {
            match pattern {
                IssuePattern::NoLongerExists => {
                    let removed_count = Self::clean_recent_directories(state);
                    if removed_count > 0 {
                        actions.push(create_repair_action_success(
                            RepairActionType::Remove,
                            format!("Removed {removed_count} non-existent recent directories"),
                            "preferences.recent_directories".to_string(),
                        ));
                    }
                }
                IssuePattern::TooSmall => {
                    if Self::reset_window_size(state) {
                        actions.push(create_repair_action_success(
                            RepairActionType::Reset,
                            "Reset window size to default values".to_string(),
                            "preferences.window_config".to_string(),
                        ));
                    }
                }
                _ => {
                    // Unknown pattern for preferences category
                    actions.push(RepairAction::failure(
                        RepairActionType::Update,
                        format!("Unknown preferences issue pattern: {}", issue.message),
                        "preferences".to_string(),
                        "Pattern not recognized".to_string(),
                    ));
                }
            }
        }

        actions
    }
}

impl PreferencesRepairHandler {
    fn clean_recent_directories(state: &mut AppState) -> usize {
        let initial_count = state.user_preferences.recent_directories.len();
        state
            .user_preferences
            .recent_directories
            .retain(|path| path.exists());
        initial_count - state.user_preferences.recent_directories.len()
    }

    const fn reset_window_size(state: &mut AppState) -> bool {
        let window_config = &mut state.user_preferences.window_config;
        if window_config.width < defaults::MIN_WINDOW_SIZE
            || window_config.height < defaults::MIN_WINDOW_SIZE
        {
            window_config.width = defaults::DEFAULT_WINDOW_WIDTH;
            window_config.height = defaults::DEFAULT_WINDOW_HEIGHT;
            true
        } else {
            false
        }
    }
}
