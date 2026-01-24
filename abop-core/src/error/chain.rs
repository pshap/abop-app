//! Error chain helper to accumulate context messages

/// Helper for creating nested error chains
pub struct ErrorChain {
    errors: Vec<String>,
}

impl ErrorChain {
    /// Create a new error chain
    #[must_use]
    pub const fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Add an error to the chain
    pub fn push_error<E: std::fmt::Display>(mut self, error: E) -> Self {
        self.errors.push(error.to_string());
        self
    }

    /// Add a context message to the chain
    #[must_use]
    pub fn context(mut self, context: &str) -> Self {
        self.errors.push(context.to_string());
        self
    }

    /// Convert to an AppError
    #[must_use]
    pub fn into_error(self) -> crate::error::AppError {
        crate::error::AppError::Other(self.errors.join(" -> "))
    }
}

impl Default for ErrorChain {
    fn default() -> Self {
        Self::new()
    }
}
