//! Audiobook-specific repair operations

use super::repair_handler::{RepairHandler, create_repair_action_success};
use crate::models::AppState;
use crate::validation::error::ValidationError;
use crate::validation::recovery::{RepairAction, RepairActionType};
use crate::validation::repair_patterns::IssuePattern;

/// Handles repair operations for audiobook-related issues
#[derive(Debug, Default)]
pub struct AudiobookRepairHandler;

impl RepairHandler for AudiobookRepairHandler {
    fn can_handle(&self, pattern: &IssuePattern) -> bool {
        matches!(
            pattern,
            IssuePattern::FileNotExists | IssuePattern::InvalidDuration
        )
    }

    fn name(&self) -> &'static str {
        "Audiobook Repair Handler"
    }

    fn repair(&self, state: &mut AppState, issue: &ValidationError) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        // Convert error to pattern for processing
        if let Some(pattern) = IssuePattern::from_validation_error(issue) {
            match pattern {
                IssuePattern::FileNotExists => {
                    let removed_count = Self::remove_invalid_audiobooks(state);
                    if removed_count > 0 {
                        actions.push(create_repair_action_success(
                            RepairActionType::Remove,
                            format!("Removed {removed_count} audiobooks with missing files"),
                            "audiobook.path".to_string(),
                        ));
                    }
                }
                IssuePattern::InvalidDuration => {
                    let repaired_count = Self::repair_invalid_durations(state);
                    if repaired_count > 0 {
                        actions.push(create_repair_action_success(
                            RepairActionType::Reset,
                            format!("Reset {repaired_count} invalid audiobook durations"),
                            "audiobook.duration_seconds".to_string(),
                        ));
                    }
                }
                _ => {
                    // Unknown pattern for audiobook category
                    actions.push(RepairAction::failure(
                        RepairActionType::Update,
                        format!("Unknown audiobook issue pattern: {}", issue.message),
                        "audiobook".to_string(),
                        "Pattern not recognized".to_string(),
                    ));
                }
            }
        }

        actions
    }
}

impl AudiobookRepairHandler {
    fn remove_invalid_audiobooks(state: &mut AppState) -> usize {
        let initial_count = state.app_data.audiobooks.len();
        state
            .app_data
            .audiobooks
            .retain(|audiobook| audiobook.path.exists());
        initial_count - state.app_data.audiobooks.len()
    }

    fn repair_invalid_durations(state: &mut AppState) -> usize {
        let mut count = 0;
        for audiobook in &mut state.app_data.audiobooks {
            if let Some(duration) = audiobook.duration_seconds
                && duration == 0
            {
                audiobook.duration_seconds = None;
                count += 1;
            }
        }
        count
    }
}
