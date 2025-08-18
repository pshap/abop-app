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

// ============================================================================
// Error Conversion Unification Macros
// ============================================================================

/// Generate multiple `From` implementations for an error type
///
/// This macro reduces boilerplate when implementing conversions from multiple
/// error types into a single target error type. It's particularly useful for
/// consolidating common error conversion patterns across the codebase.
///
/// # Examples
///
/// ```rust
/// use abop_core::impl_error_conversions;
/// 
/// // Generate conversions from std::io::Error and String to MyError
/// impl_error_conversions! {
///     MyError => {
///         std::io::Error => |e| MyError::Io(e.to_string()),
///         String => |s| MyError::Custom(s),
///         &str => |s| MyError::Custom(s.to_string()),
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_error_conversions {
    ($target:ty => {
        $($source:ty => |$param:ident| $conversion:expr),+ $(,)?
    }) => {
        $(
            impl From<$source> for $target {
                fn from($param: $source) -> Self {
                    $conversion
                }
            }
        )+
    };
}

/// Generate bidirectional error conversions between two error types
///
/// This macro creates `From` implementations in both directions between two error types,
/// useful when you need symmetric error conversion capabilities.
///
/// # Examples
///
/// ```rust
/// use abop_core::impl_bidirectional_conversions;
/// 
/// impl_bidirectional_conversions! {
///     DatabaseError, AppError => {
///         DatabaseError => AppError: |e| AppError::Database(e),
///         AppError => DatabaseError: |e| match e {
///             AppError::Database(db_err) => db_err,
///             other => DatabaseError::Custom(other.to_string()),
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_bidirectional_conversions {
    ($type_a:ty, $type_b:ty => {
        $type_a_ident:ident => $type_b_ident:ident: |$param_a:ident| $conversion_a:expr,
        $type_b_ident2:ident => $type_a_ident2:ident: |$param_b:ident| $conversion_b:expr $(,)?
    }) => {
        impl From<$type_a> for $type_b {
            fn from($param_a: $type_a) -> Self {
                $conversion_a
            }
        }
        
        impl From<$type_b> for $type_a {
            fn from($param_b: $type_b) -> Self {
                $conversion_b
            }
        }
    };
}

/// Generate error conversions with string formatting
///
/// This macro simplifies creating error conversions that primarily involve converting
/// the source error to a string and wrapping it in a target error variant.
///
/// # Examples
///
/// ```rust
/// use abop_core::impl_string_conversions;
/// 
/// impl_string_conversions! {
///     AppError => {
///         std::io::Error => Io,
///         rusqlite::Error => Database,
///         toml::de::Error => Config,
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_string_conversions {
    ($target:ty => {
        $($source:ty => $variant:ident),+ $(,)?
    }) => {
        $(
            impl From<$source> for $target {
                fn from(err: $source) -> Self {
                    Self::$variant(err.to_string())
                }
            }
        )+
    };
}

/// Generate wrapped error conversions
///
/// This macro creates conversions where the source error is wrapped directly
/// in a target error variant without string conversion, preserving the original error.
///
/// # Examples
///
/// ```rust
/// use abop_core::impl_wrapped_conversions;
/// 
/// impl_wrapped_conversions! {
///     DomainError => {
///         AudioError => Audio,
///         DatabaseError => Database,
///         ScanError => Scan,
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_wrapped_conversions {
    ($target:ty => {
        $($source:ty => $variant:ident),+ $(,)?
    }) => {
        $(
            impl From<$source> for $target {
                fn from(err: $source) -> Self {
                    Self::$variant(err)
                }
            }
        )+
    };
}

/// Generate conditional error conversions with pattern matching
///
/// This macro creates error conversions that use pattern matching on the source
/// error to determine the appropriate target error variant.
///
/// # Examples
///
/// ```rust
/// use abop_core::impl_conditional_conversions;
/// 
/// impl_conditional_conversions! {
///     AppError, DatabaseError => {
///         DatabaseError::Sqlite(e) => AppError::Database(DatabaseError::Sqlite(e)),
///         DatabaseError::NotFound { entity, id } => AppError::InvalidData(
///             format!("Missing {entity} with id {id}")
///         ),
///         other => AppError::Other(other.to_string()),
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_conditional_conversions {
    ($target:ty, $source:ty => {
        $($pattern:pat => $conversion:expr),+ $(,)?
    }) => {
        impl From<$source> for $target {
            fn from(err: $source) -> Self {
                match err {
                    $($pattern => $conversion,)+
                }
            }
        }
    };
}

/// Generate error conversions with context information
///
/// This macro creates conversions that add contextual information to errors,
/// useful for providing more descriptive error messages in nested error scenarios.
///
/// # Examples
///
/// ```rust
/// use abop_core::impl_contextual_conversions;
/// 
/// impl_contextual_conversions! {
///     ProcessingError => {
///         std::io::Error => FileIo: "File operation failed",
///         serde_json::Error => Serialization: "JSON processing failed",
///         ValidationError => Validation: "Input validation failed",
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_contextual_conversions {
    ($target:ty => {
        $($source:ty => $variant:ident: $context:expr),+ $(,)?
    }) => {
        $(
            impl From<$source> for $target {
                fn from(err: $source) -> Self {
                    Self::$variant(format!("{}: {}", $context, err))
                }
            }
        )+
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
            .push_error(std::io::Error::new(
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

    // ============================================================================
    // Tests for Error Conversion Unification Macros
    // ============================================================================
    
    mod error_conversion_tests {
        use super::*;

        #[test]
        fn test_impl_error_conversions_macro() {
            // Test error type for conversions
            #[derive(Debug, PartialEq)]
            enum TestError {
                Custom(String),
                Io(String),
            }
            
            // Test the impl_error_conversions! macro
            impl_error_conversions! {
                TestError => {
                    String => |s| TestError::Custom(s),
                    &str => |s| TestError::Custom(s.to_string()),
                }
            }
            
            let from_string: TestError = "test message".to_string().into();
            assert_eq!(from_string, TestError::Custom("test message".to_string()));
            
            let from_str: TestError = "test str".into();
            assert_eq!(from_str, TestError::Custom("test str".to_string()));
        }

        #[test]
        fn test_impl_string_conversions_macro() {
            // Create a test error type
            #[derive(Debug, PartialEq)]
            enum StringTestError {
                Io(String),
                Parse(String),
            }
            
            // Test the impl_string_conversions! macro
            impl_string_conversions! {
                StringTestError => {
                    std::io::Error => Io,
                    std::num::ParseIntError => Parse,
                }
            }
            
            let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
            let converted: StringTestError = io_error.into();
            match converted {
                StringTestError::Io(msg) => assert!(msg.contains("file not found")),
                _ => panic!("Unexpected error type"),
            }
            
            let parse_err = "not_a_number".parse::<i32>().unwrap_err();
            let converted: StringTestError = parse_err.into();
            assert!(matches!(converted, StringTestError::Parse(_)));
        }

        #[test]
        fn test_impl_wrapped_conversions_macro() {
            // Source error type
            #[derive(Debug, PartialEq)]
            enum SourceError {
                Network(String),
            }
            
            // Wrapper error type  
            #[derive(Debug, PartialEq)]
            enum WrapperError {
                Source(SourceError),
            }
            
            impl_wrapped_conversions! {
                WrapperError => {
                    SourceError => Source,
                }
            }
            
            let source_error = SourceError::Network("connection failed".to_string());
            let wrapped: WrapperError = source_error.into();
            match wrapped {
                WrapperError::Source(SourceError::Network(msg)) => {
                    assert_eq!(msg, "connection failed");
                }
                _ => panic!("Unexpected error type"),
            }
        }

        #[test]
        fn test_impl_conditional_conversions_macro() {
            // Source error type
            #[derive(Debug, PartialEq)]
            enum SourceError {
                Network(String),
                Parse(String),
            }
            
            // Target error type
            #[derive(Debug, PartialEq)]
            enum ConditionalTestError {
                Network(String),
                Parse(String),
                Other(String),
            }
            
            impl_conditional_conversions! {
                ConditionalTestError, SourceError => {
                    SourceError::Network(msg) => ConditionalTestError::Network(msg),
                    SourceError::Parse(msg) => ConditionalTestError::Parse(msg),
                }
            }
            
            let network_error = SourceError::Network("timeout".to_string());
            let converted: ConditionalTestError = network_error.into();
            assert_eq!(converted, ConditionalTestError::Network("timeout".to_string()));
            
            let parse_error = SourceError::Parse("invalid format".to_string());
            let converted: ConditionalTestError = parse_error.into();
            assert_eq!(converted, ConditionalTestError::Parse("invalid format".to_string()));
        }

        #[test]
        fn test_impl_contextual_conversions_macro() {
            // Source error type
            #[derive(Debug, PartialEq)]
            enum SourceError {
                Network(String),
            }
            
            impl std::fmt::Display for SourceError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        SourceError::Network(s) => write!(f, "Network: {s}"),
                    }
                }
            }
            
            // Target error type
            #[derive(Debug, PartialEq)]
            enum ContextualTestError {
                Network(String),
            }
            
            impl_contextual_conversions! {
                ContextualTestError => {
                    SourceError => Network: "Network operation failed",
                }
            }
            
            let source_error = SourceError::Network("connection timeout".to_string());
            let converted: ContextualTestError = source_error.into();
            match converted {
                ContextualTestError::Network(msg) => {
                    assert!(msg.contains("Network operation failed"));
                    assert!(msg.contains("connection timeout"));
                }
                _ => panic!("Unexpected error type"),
            }
        }

        #[test]
        fn test_impl_bidirectional_conversions_macro() {
            // Two error types for bidirectional conversion
            #[derive(Debug, PartialEq, Clone)]
            enum ErrorA {
                Message(String),
            }
            
            #[derive(Debug, PartialEq, Clone)]
            enum ErrorB {
                Content(String),
            }
            
            impl_bidirectional_conversions! {
                ErrorA, ErrorB => {
                    ErrorA => ErrorB: |e| match e {
                        ErrorA::Message(msg) => ErrorB::Content(msg),
                    },
                    ErrorB => ErrorA: |e| match e {
                        ErrorB::Content(content) => ErrorA::Message(content),
                    }
                }
            }
            
            let error_a = ErrorA::Message("test".to_string());
            let converted_to_b: ErrorB = error_a.clone().into();
            assert_eq!(converted_to_b, ErrorB::Content("test".to_string()));
            
            let converted_back_to_a: ErrorA = converted_to_b.into();
            assert_eq!(converted_back_to_a, error_a);
        }

        #[test]
        fn test_macro_reduces_boilerplate() {
            // This test demonstrates how the macros reduce boilerplate code
            // by implementing multiple conversions at once
            
            #[derive(Debug, PartialEq)]
            enum MultiConversionError {
                Io(String),
                Parse(String),
                Custom(String),
            }
            
            // Single macro call replaces multiple separate impl blocks
            impl_string_conversions! {
                MultiConversionError => {
                    std::io::Error => Io,
                    std::num::ParseIntError => Parse,
                }
            }
            
            impl_error_conversions! {
                MultiConversionError => {
                    String => |s| MultiConversionError::Custom(s),
                    &str => |s| MultiConversionError::Custom(s.to_string()),
                }
            }
            
            // Test all conversions work correctly
            let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
            let converted: MultiConversionError = io_err.into();
            assert!(matches!(converted, MultiConversionError::Io(_)));
            
            let parse_err = "not_a_number".parse::<i32>().unwrap_err();
            let converted: MultiConversionError = parse_err.into();
            assert!(matches!(converted, MultiConversionError::Parse(_)));
            
            let string_err: MultiConversionError = "test".to_string().into();
            assert_eq!(string_err, MultiConversionError::Custom("test".to_string()));
            
            let str_err: MultiConversionError = "test".into();
            assert_eq!(str_err, MultiConversionError::Custom("test".to_string()));
        }
    }
}