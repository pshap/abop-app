use super::*;
use crate::error::{AppError, ErrorChain, ErrorContext};

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

// =============================================================================
// Tests for Error Conversion Unification Macros
// =============================================================================

mod error_conversion_tests {
    use super::*;

    #[test]
    fn test_impl_error_conversions_macro() {
        // Test error type for conversions
        #[allow(dead_code)]
        #[derive(Debug, PartialEq)]
        enum TestError {
            Custom(String),
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
        #[allow(dead_code)]
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
        let WrapperError::Source(SourceError::Network(msg)) = wrapped;
        assert_eq!(msg, "connection failed");
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
        }

        impl_conditional_conversions! {
            ConditionalTestError, SourceError => {
                SourceError::Network(msg) => ConditionalTestError::Network(msg),
                SourceError::Parse(msg) => ConditionalTestError::Parse(msg),
            }
        }

        let network_error = SourceError::Network("timeout".to_string());
        let converted: ConditionalTestError = network_error.into();
        assert_eq!(
            converted,
            ConditionalTestError::Network("timeout".to_string())
        );

        let parse_error = SourceError::Parse("invalid format".to_string());
        let converted: ConditionalTestError = parse_error.into();
        assert_eq!(
            converted,
            ConditionalTestError::Parse("invalid format".to_string())
        );
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
        let ContextualTestError::Network(msg) = converted;
        assert!(msg.contains("Network operation failed"));
        assert!(msg.contains("connection timeout"));
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

        #[allow(dead_code)]
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
