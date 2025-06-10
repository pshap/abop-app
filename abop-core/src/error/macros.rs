//! Error handling macros and utilities
//!
//! This module provides ergonomic macros and helper functions for common
//! error handling patterns in the ABOP application.

/// Create an AppError::Config with context
#[macro_export]
macro_rules! config_error {
    ($msg:expr) => {
        $crate::error::AppError::Config($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::Config(format!($fmt, $($arg)*))
    };
}

/// Create an AppError::Audio with context
#[macro_export]
macro_rules! audio_error {
    ($msg:expr) => {
        $crate::error::AppError::Audio($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::Audio(format!($fmt, $($arg)*))
    };
}

/// Create an AppError::Database with context
#[macro_export]
macro_rules! database_error {
    ($msg:expr) => {
        $crate::error::AppError::Database($crate::db::error::DatabaseError::ExecutionFailed {
            message: $msg.to_string()
        })
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::Database($crate::db::error::DatabaseError::ExecutionFailed {
            message: format!($fmt, $($arg)*)
        })
    };
}

/// Create an AppError::ValidationFailed with context
#[macro_export]
macro_rules! validation_error {
    ($msg:expr) => {
        $crate::error::AppError::ValidationFailed($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::ValidationFailed(format!($fmt, $($arg)*))
    };
}

/// Create an AppError::InvalidData with context
#[macro_export]
macro_rules! invalid_data {
    ($msg:expr) => {
        $crate::error::AppError::InvalidData($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::InvalidData(format!($fmt, $($arg)*))
    };
}

/// Bail out early with an error (similar to anyhow::bail!)
#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::AppError::Other(format!($fmt, $($arg)*)))
    };
}

/// Ensure a condition is true, or return an error
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !($cond) {
            return Err($err.into());
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            return Err($crate::error::AppError::Other(format!($fmt, $($arg)*)));
        }
    };
}

/// Add context to an error result
#[macro_export]
macro_rules! with_context {
    ($result:expr, $context:expr) => {
        $result.map_err(|e| {
            $crate::error::AppError::Other(format!("{}: {}", $context, e))
        })
    };
    ($result:expr, $fmt:expr, $($arg:tt)*) => {
        $result.map_err(|e| {
            $crate::error::AppError::Other(format!("{}: {}", format!($fmt, $($arg)*), e))
        })
    };
}

/// Log and return an error
#[macro_export]
macro_rules! log_error {
    ($err:expr) => {{
        let error = $err;
        log::error!("{}", error);
        error
    }};
    ($err:expr, $context:expr) => {{
        let error = $err;
        log::error!("{}: {}", $context, error);
        error
    }};
}

/// Create a timeout error with operation context
#[macro_export]
macro_rules! timeout_error {
    ($operation:expr, $timeout_ms:expr, $elapsed_ms:expr) => {
        $crate::error::AppError::Timeout {
            operation: $operation.to_string(),
            timeout_ms: $timeout_ms,
            elapsed_ms: $elapsed_ms,
        }
    };
}

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

/// Helper for creating nested error chains
pub struct ErrorChain {
    errors: Vec<String>,
}

impl ErrorChain {
    /// Create a new error chain
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Add an error to the chain
    pub fn add<E: std::fmt::Display>(mut self, error: E) -> Self {
        self.errors.push(error.to_string());
        self
    }

    /// Add a context message to the chain
    pub fn context(mut self, context: &str) -> Self {
        self.errors.push(context.to_string());
        self
    }

    /// Convert to an AppError
    pub fn into_error(self) -> crate::error::AppError {
        crate::error::AppError::Other(self.errors.join(" -> "))
    }
}

impl Default for ErrorChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;

    #[test]
    fn test_config_error_macro() {
        let err = config_error!("test message");
        match err {
            AppError::Config(msg) => assert_eq!(msg, "test message"),
            _ => panic!("Wrong error type"),
        }

        let err = config_error!("test {} {}", "formatted", "message");
        match err {
            AppError::Config(msg) => assert_eq!(msg, "test formatted message"),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_error_context_trait() {
        let result: Result<(), std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));

        let with_context = result.context("Failed to read config file");
        assert!(with_context.is_err());
        assert!(
            with_context
                .unwrap_err()
                .to_string()
                .contains("Failed to read config file")
        );
    }

    #[test]
    fn test_error_chain() {
        let chain = ErrorChain::new()
            .context("Loading configuration")
            .add(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found",
            ))
            .context("Failed to initialize application");

        let error = chain.into_error();
        let error_string = error.to_string();
        assert!(error_string.contains("Loading configuration"));
        assert!(error_string.contains("file not found"));
        assert!(error_string.contains("Failed to initialize application"));
    }

    #[test]
    fn test_ensure_macro() {
        fn test_function(value: i32) -> crate::error::Result<i32> {
            ensure!(value > 0, "Value must be positive");
            Ok(value * 2)
        }

        assert!(test_function(5).is_ok());
        assert!(test_function(-1).is_err());
    }

    #[test]
    fn test_timeout_error_macro() {
        let err = timeout_error!("scan operation", 5000, 7500);
        match err {
            AppError::Timeout {
                operation,
                timeout_ms,
                elapsed_ms,
            } => {
                assert_eq!(operation, "scan operation");
                assert_eq!(timeout_ms, 5000);
                assert_eq!(elapsed_ms, 7500);
            }
            _ => panic!("Wrong error type"),
        }
    }
}
