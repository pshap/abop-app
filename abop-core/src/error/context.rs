//! Error context helpers

/// Helper trait for adding context to results
pub trait ErrorContext<T> {
    /// Add context to an error
    fn with_context<F>(self, f: F) -> crate::error::Result<T>
    where
        F: FnOnce() -> String;

    /// Add static context to an error
    fn context(self, context: &'static str) -> crate::error::Result<T>;
}

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: std::fmt::Display,
{
    fn with_context<F>(self, f: F) -> crate::error::Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| crate::error::AppError::Other(format!("{}: {}", f(), e)))
    }

    fn context(self, context: &'static str) -> crate::error::Result<T> {
        self.map_err(|e| crate::error::AppError::Other(format!("{context}: {e}")))
    }
}
