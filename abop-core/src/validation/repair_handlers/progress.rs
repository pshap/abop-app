//! Progress-specific repair operations

use std::collections::HashMap;
use crate::models::AppState;
use crate::validation::error::ValidationError;
use crate::validation::recovery::{RepairAction, RepairActionType};
use crate::validation::repair_patterns::IssuePattern;
use super::repair_handler::{RepairHandler, create_repair_action_success};

/// Handles repair operations for progress-related issues
#[derive(Debug, Default)]
pub struct ProgressRepairHandler;

impl RepairHandler for ProgressRepairHandler {
    fn can_handle(&self, pattern: &IssuePattern) -> bool {
        matches!(pattern,
            IssuePattern::Orphaned |
            IssuePattern::InvalidDuration
        )
    }
    
    fn name(&self) -> &'static str {
        "Progress Repair Handler"
    }
    
    fn repair(&self, state: &mut AppState, issue: &ValidationError) -> Vec<RepairAction> {
        let mut actions = Vec::new();
        
        // Convert error to pattern for processing
        if let Some(pattern) = IssuePattern::from_validation_error(issue) {
            match pattern {
                IssuePattern::Orphaned => {
                    let removed_count = Self::remove_orphaned_progress(state);
                    if removed_count > 0 {
                        actions.push(create_repair_action_success(
                            RepairActionType::Remove,
                            format!("Removed {} orphaned progress entries", removed_count),
                            "progress".to_string(),
                        ));
                    }                }
                IssuePattern::InvalidDuration => {
                    let capped_count = Self::cap_progress_at_duration(state);
                    if capped_count > 0 {
                        actions.push(create_repair_action_success(
                            RepairActionType::Update,
                            format!("Capped {} progress entries at audiobook duration", capped_count),
                            "progress.position_seconds".to_string(),
                        ));
                    }
                }
                _ => {
                    // Unknown pattern for progress category
                    actions.push(RepairAction::failure(
                        RepairActionType::Update,
                        format!("Unknown progress issue pattern: {}", issue.message),
                        "progress".to_string(),
                        "Pattern not recognized".to_string(),
                    ));
                }
            }
        }
        
        actions
    }
}

impl ProgressRepairHandler {
    fn remove_orphaned_progress(state: &mut AppState) -> usize {
        let valid_audiobook_ids: std::collections::HashSet<_> =
            state.app_data.audiobooks.iter().map(|ab| &ab.id).collect();

        let initial_count = state.app_data.progress.len();
        state.app_data.progress.retain(|progress| {
            valid_audiobook_ids.contains(&progress.audiobook_id)
        });
        
        initial_count - state.app_data.progress.len()
    }

    fn cap_progress_at_duration(state: &mut AppState) -> usize {
        let mut count = 0;
        let audiobook_durations: HashMap<_, _> = state
            .app_data
            .audiobooks
            .iter()
            .filter_map(|ab| ab.duration_seconds.map(|d| (&ab.id, d)))
            .collect();

        for progress in &mut state.app_data.progress {
            if let Some(&duration) = audiobook_durations.get(&progress.audiobook_id)
                && progress.position_seconds > duration
            {
                progress.position_seconds = duration;
                count += 1;
            }
        }
        count
    }
}
