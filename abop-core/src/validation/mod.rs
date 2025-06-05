//! State validation and recovery system for ABOP
//!
//! This module provides comprehensive validation for application state loaded from disk,
//! including data integrity checks, schema version compatibility, file reference validation,
//! and state repair strategies.

mod error;
mod recovery;
mod state_validator;
mod validators;

pub use error::{ValidationError, ValidationResult, ValidationSeverity};
pub use recovery::{RepairAction, RepairContext, StateRepairStrategy};
pub use state_validator::{StateValidator, ValidationConfig};
pub use validators::*;

/// Default validation configuration for typical use cases
pub fn default_validation_config() -> ValidationConfig {
    ValidationConfig::default()
}

/// Validate an application state with default configuration
pub fn validate_app_state(state: &crate::models::AppState) -> ValidationResult {
    let validator = StateValidator::new(default_validation_config());
    validator.validate(state)
}

/// Validate and attempt to repair an application state
pub fn validate_and_repair_app_state(
    state: &mut crate::models::AppState,
) -> (ValidationResult, Vec<RepairAction>) {
    let validator = StateValidator::new(default_validation_config());
    let validation_result = validator.validate(state);

    if validation_result.has_critical_issues() {
        let repair_context = RepairContext::new(&validation_result);
        let repair_actions = StateRepairStrategy::default().repair(state, &repair_context);
        (validation_result, repair_actions)
    } else {
        (validation_result, Vec::new())
    }
}
