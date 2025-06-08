//! State validation errors and result types

use std::fmt;
use std::path::PathBuf;

/// Severity level of a validation issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationSeverity {
    /// Informational - state is valid but could be optimized
    Info,
    /// Warning - state is valid but has minor issues
    Warning,
    /// Error - state has issues that should be addressed
    Error,
    /// Critical - state is corrupted and needs immediate repair
    Critical,
}

impl fmt::Display for ValidationSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Warning => write!(f, "WARNING"),
            Self::Error => write!(f, "ERROR"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// A specific validation error or issue
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Severity of the validation issue
    pub severity: ValidationSeverity,
    /// Category of the validation error
    pub category: String,
    /// Human-readable description of the issue
    pub message: String,
    /// Optional file path associated with the issue
    pub file_path: Option<PathBuf>,
    /// Optional field name or identifier associated with the issue
    pub field: Option<String>,
    /// Optional suggested fix or action
    pub suggestion: Option<String>,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(severity: ValidationSeverity, category: &str, message: &str) -> Self {
        Self {
            severity,
            category: category.to_string(),
            message: message.to_string(),
            file_path: None,
            field: None,
            suggestion: None,
        }
    }

    /// Create a critical validation error
    pub fn critical(category: &str, message: &str) -> Self {
        Self::new(ValidationSeverity::Critical, category, message)
    }

    /// Create an error-level validation issue
    pub fn error(category: &str, message: &str) -> Self {
        Self::new(ValidationSeverity::Error, category, message)
    }

    /// Create a warning-level validation issue
    pub fn warning(category: &str, message: &str) -> Self {
        Self::new(ValidationSeverity::Warning, category, message)
    }

    /// Create an info-level validation issue
    pub fn info(category: &str, message: &str) -> Self {
        Self::new(ValidationSeverity::Info, category, message)
    }

    /// Set the file path associated with this error
    pub fn with_file_path(mut self, path: PathBuf) -> Self {
        self.file_path = Some(path);
        self
    }

    /// Set the field name associated with this error
    pub fn with_field(mut self, field: &str) -> Self {
        self.field = Some(field.to_string());
        self
    }

    /// Set a suggested fix for this error
    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }

    /// Check if this is a critical error
    pub fn is_critical(&self) -> bool {
        self.severity == ValidationSeverity::Critical
    }

    /// Check if this is an error (not just warning or info)
    pub const fn is_error(&self) -> bool {
        matches!(
            self.severity,
            ValidationSeverity::Error | ValidationSeverity::Critical
        )
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.category, self.message)?;

        if let Some(ref field) = self.field {
            write!(f, " (field: {field})")?;
        }

        if let Some(ref path) = self.file_path {
            write!(f, " (file: {})", path.display())?;
        }

        if let Some(ref suggestion) = self.suggestion {
            write!(f, " - Suggestion: {suggestion}")?;
        }

        Ok(())
    }
}

/// Aggregated result of state validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// All validation issues found
    pub issues: Vec<ValidationError>,
    /// Whether the state can be safely used
    pub is_valid: bool,
    /// Whether critical issues require immediate attention
    pub has_critical_issues: bool,
    /// Summary statistics
    pub summary: ValidationSummary,
}

/// Summary statistics for validation results
#[derive(Debug, Clone, Default)]
pub struct ValidationSummary {
    /// Number of critical issues
    pub critical_count: usize,
    /// Number of error-level issues
    pub error_count: usize,
    /// Number of warning-level issues
    pub warning_count: usize,
    /// Number of info-level issues
    pub info_count: usize,
    /// Total number of issues
    pub total_count: usize,
}

impl ValidationResult {
    /// Create a new empty validation result
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            is_valid: true,
            has_critical_issues: false,
            summary: ValidationSummary::default(),
        }
    }

    /// Add a validation issue to the result
    pub fn add_issue(&mut self, issue: ValidationError) {
        // Update state validity based on issue severity
        if issue.is_critical() {
            self.has_critical_issues = true;
            self.is_valid = false;
        } else if issue.is_error() {
            self.is_valid = false;
        }

        // Update summary statistics
        match issue.severity {
            ValidationSeverity::Critical => self.summary.critical_count += 1,
            ValidationSeverity::Error => self.summary.error_count += 1,
            ValidationSeverity::Warning => self.summary.warning_count += 1,
            ValidationSeverity::Info => self.summary.info_count += 1,
        }
        self.summary.total_count += 1;

        self.issues.push(issue);
    }

    /// Check if there are any critical issues
    pub const fn has_critical_issues(&self) -> bool {
        self.has_critical_issues
    }

    /// Check if the state is considered valid for use
    pub const fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Get all issues of a specific severity
    pub fn issues_by_severity(&self, severity: ValidationSeverity) -> Vec<&ValidationError> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == severity)
            .collect()
    }

    /// Get all critical issues
    pub fn critical_issues(&self) -> Vec<&ValidationError> {
        self.issues_by_severity(ValidationSeverity::Critical)
    }

    /// Get all error-level issues
    pub fn error_issues(&self) -> Vec<&ValidationError> {
        self.issues_by_severity(ValidationSeverity::Error)
    }

    /// Get issues by category
    pub fn issues_by_category(&self, category: &str) -> Vec<&ValidationError> {
        self.issues
            .iter()
            .filter(|issue| issue.category == category)
            .collect()
    }

    /// Create a summary report of all issues
    pub fn summary_report(&self) -> String {
        if self.issues.is_empty() {
            return "No validation issues found.".to_string();
        }

        let mut report = format!(
            "Validation Summary: {} total issues\n",
            self.summary.total_count
        );

        if self.summary.critical_count > 0 {
            report.push_str(&format!(
                "  {} critical issues\n",
                self.summary.critical_count
            ));
        }
        if self.summary.error_count > 0 {
            report.push_str(&format!("  {} errors\n", self.summary.error_count));
        }
        if self.summary.warning_count > 0 {
            report.push_str(&format!("  {} warnings\n", self.summary.warning_count));
        }
        if self.summary.info_count > 0 {
            report.push_str(&format!("  {} info items\n", self.summary.info_count));
        }

        report.push('\n');
        for issue in &self.issues {
            report.push_str(&format!("  {issue}\n"));
        }

        report
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary_report())
    }
}
