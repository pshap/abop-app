//! Advanced validation system for Material Design 3 selection component builders
//!
//! This module provides comprehensive validation capabilities including:
//! - Result aggregation for multiple validation issues
//! - Validation composition for complex scenarios
//! - Enhanced error reporting with context preservation
//! - Performance-optimized validation traits

use super::super::common::SelectionError;

// ============================================================================
// Validation Results and Context
// ============================================================================

/// Result type for validation operations that can aggregate multiple issues
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    /// Collection of validation errors
    pub errors: Vec<SelectionError>,
    /// Collection of validation warnings
    pub warnings: Vec<String>,
    /// Validation context for better error reporting
    pub context: ValidationContext,
}

impl ValidationResult {
    /// Create a new empty validation result
    #[must_use]
    pub fn new(context: ValidationContext) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            context,
        }
    }

    /// Create a successful validation result
    #[must_use]
    pub fn success(context: ValidationContext) -> Self {
        Self::new(context)
    }

    /// Add an error to the validation result
    pub fn add_error(&mut self, error: SelectionError) {
        self.errors.push(error);
    }

    /// Add a warning to the validation result
    pub fn add_warning<S: Into<String>>(&mut self, warning: S) {
        self.warnings.push(warning.into());
    }

    /// Check if validation passed (no errors)
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if there are warnings
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Combine this result with another
    pub fn combine(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }

    /// Convert to a simple Result type
    pub fn into_result(self) -> Result<(), SelectionError> {
        if self.is_valid() {
            Ok(())
        } else {
            // Return the first error, but in a real implementation you might
            // want to combine all errors into a composite error
            Err(self.errors.into_iter().next().unwrap_or_else(|| {
                SelectionError::ValidationError("Unknown validation error".to_string())
            }))
        }
    }
}

/// Context information for validation operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationContext {
    /// Component type being validated
    pub component_type: String,
    /// Field being validated (if applicable)
    pub field: Option<String>,
    /// Operation being performed
    pub operation: String,
}

impl ValidationContext {
    /// Create a new validation context
    #[must_use]
    pub fn new<S: Into<String>>(component_type: S, operation: S) -> Self {
        Self {
            component_type: component_type.into(),
            field: None,
            operation: operation.into(),
        }
    }

    /// Set the field being validated
    #[must_use]
    pub fn with_field<S: Into<String>>(mut self, field: S) -> Self {
        self.field = Some(field.into());
        self
    }
}

// ============================================================================
// Validation Traits
// ============================================================================

/// Trait for components that can validate themselves
pub trait BuilderValidation {
    /// Validate the component and return a detailed result
    fn validate_detailed(&self) -> ValidationResult;

    /// Validate the component (simple result)
    fn validate_simple(&self) -> Result<(), SelectionError> {
        self.validate_detailed().into_result()
    }

    /// Get the validation context for this component
    fn validation_context(&self) -> ValidationContext;
}

/// Trait for composing multiple validation operations
pub trait ValidationComposer {
    /// Run multiple validation functions and aggregate results
    fn compose_validations<T>(
        &self,
        target: &T,
        validators: Vec<Box<dyn Fn(&T) -> ValidationResult>>,
    ) -> ValidationResult;

    /// Validate a collection of items
    fn validate_collection<T: BuilderValidation>(
        &self,
        items: &[T],
        context: ValidationContext,
    ) -> ValidationResult;
}

/// Default implementation of validation composer
#[allow(dead_code)] // Public API struct - may not be used internally yet
pub struct DefaultValidationComposer;

impl ValidationComposer for DefaultValidationComposer {
    fn compose_validations<T>(
        &self,
        target: &T,
        validators: Vec<Box<dyn Fn(&T) -> ValidationResult>>,
    ) -> ValidationResult {
        let mut result = ValidationResult::new(ValidationContext::new(
            "Composite".to_string(),
            "validation".to_string(),
        ));

        for validator in validators {
            let validation_result = validator(target);
            result.combine(validation_result);
        }

        result
    }

    fn validate_collection<T: BuilderValidation>(
        &self,
        items: &[T],
        mut context: ValidationContext,
    ) -> ValidationResult {
        let mut result = ValidationResult::new(context.clone());

        for (index, item) in items.iter().enumerate() {
            context.field = Some(format!("item[{index}]"));
            let item_result = item.validate_detailed();
            result.combine(item_result);
        }

        result
    }
}

// ============================================================================
// Enhanced Error Handling
// ============================================================================

/// Enhanced error context for better debugging and user feedback
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorContext {
    /// The operation that failed
    pub operation: String,
    /// The component type involved
    pub component_type: String,
    /// The specific field that caused the error (if applicable)
    pub field: Option<String>,
    /// Suggested fix for the error
    pub suggestion: Option<String>,
    /// Error severity level
    pub severity: ErrorSeverity,
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Critical error that prevents operation
    Critical,
    /// Error that should be fixed but doesn't prevent operation
    Error,
    /// Warning that should be addressed
    Warning,
    /// Information for debugging
    Info,
}

/// Enhanced error reporting with context preservation
pub struct ErrorReporter {
    context_stack: Vec<ErrorContext>,
}

impl ErrorReporter {
    /// Create a new error reporter
    #[must_use]
    pub fn new() -> Self {
        Self {
            context_stack: Vec::new(),
        }
    }

    /// Push an error context onto the stack
    pub fn push_context(&mut self, context: ErrorContext) {
        self.context_stack.push(context);
    }

    /// Pop the last error context
    pub fn pop_context(&mut self) -> Option<ErrorContext> {
        self.context_stack.pop()
    }

    /// Report an error with full context
    pub fn report_error(&self, error: SelectionError) -> SelectionError {
        if let Some(context) = self.context_stack.last() {
            self.enhance_error_with_context(error, context)
        } else {
            error
        }
    }

    /// Enhance an error with additional context
    fn enhance_error_with_context(
        &self,
        error: SelectionError,
        context: &ErrorContext,
    ) -> SelectionError {
        match error {
            SelectionError::ValidationError(msg) => {
                let enhanced_msg = format!(
                    "{} in {}.{}: {}",
                    context.operation,
                    context.component_type,
                    context.field.as_deref().unwrap_or("unknown"),
                    msg
                );
                SelectionError::ValidationError(enhanced_msg)
            }
            SelectionError::InvalidLabel { reason } => SelectionError::InvalidLabel {
                reason: format!("{}: {}", context.component_type, reason),
            },
            SelectionError::InvalidState { details } => SelectionError::InvalidState {
                details: format!(
                    "{} ({}): {}",
                    context.component_type, context.operation, details
                ),
            },
            other => other,
        }
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Configuration Summary
// ============================================================================

/// Configuration summary for debugging and introspection
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationSummary {
    /// Whether the component is disabled
    pub disabled: bool,
    /// Component size
    pub size: super::super::common::ComponentSize,
    /// Whether the component has a label
    pub has_label: bool,
    /// Length of the label in characters
    pub label_length: usize,
    /// Whether the component has an error state
    pub has_error: bool,
    /// Whether animations are enabled
    pub animation_enabled: bool,
}

// ============================================================================
// Validation Helper Functions
// ============================================================================

/// Enhanced validation with better error context and performance optimization
#[inline]
pub fn validate_with_context<T>(
    _builder: &T,
    component_type: &str,
    validate_fn: impl FnOnce() -> Result<(), SelectionError>,
) -> Result<(), SelectionError> {
    validate_fn().map_err(|e| enhance_error_with_context(e, component_type))
}

/// Enhanced validation that returns a detailed ValidationResult
#[allow(dead_code)] // Public API function - may not be used internally yet
pub fn validate_with_detailed_context<T>(
    _builder: &T,
    context: ValidationContext,
    validate_fn: impl FnOnce() -> Result<(), SelectionError>,
) -> ValidationResult {
    let mut result = ValidationResult::new(context);

    match validate_fn() {
        Ok(()) => result,
        Err(error) => {
            result.add_error(error);
            result
        }
    }
}

/// Batch validation for multiple components
#[allow(dead_code)] // Public API function - may not be used internally yet
pub fn validate_batch<T>(
    items: &[T],
    component_type: &str,
    validate_fn: impl Fn(&T) -> Result<(), SelectionError>,
) -> ValidationResult {
    let context =
        ValidationContext::new(component_type.to_string(), "batch_validation".to_string());
    let mut result = ValidationResult::new(context);

    for (index, item) in items.iter().enumerate() {
        match validate_fn(item) {
            Ok(()) => {}
            Err(error) => {
                let enhanced_error =
                    enhance_error_with_context(error, &format!("{component_type}[{index}]"));
                result.add_error(enhanced_error);
            }
        }
    }

    result
}

/// Enhanced error context application with performance optimization
#[inline]
fn enhance_error_with_context(error: SelectionError, component_type: &str) -> SelectionError {
    match error {
        SelectionError::ValidationError(msg) => {
            SelectionError::ValidationError(format!("{component_type}: {msg}"))
        }
        SelectionError::InvalidLabel { reason } => SelectionError::InvalidLabel {
            reason: format!("{component_type}: {reason}"),
        },
        SelectionError::LabelTooLong { len, max } => SelectionError::LabelTooLong { len, max },
        SelectionError::EmptyLabel => SelectionError::EmptyLabel,
        SelectionError::InvalidState { details } => SelectionError::InvalidState {
            details: format!("{component_type}: {details}"),
        },
        SelectionError::ConflictingStates { details } => SelectionError::ConflictingStates {
            details: format!("{component_type}: {details}"),
        },
        SelectionError::CustomRule { rule, message } => SelectionError::CustomRule {
            rule,
            message: format!("{component_type}: {message}"),
        },
    }
}
