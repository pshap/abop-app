//! Integration tests for enhanced error handling
//!
//! These tests verify that the new error handling macros and utilities
//! work correctly and provide good developer experience.

use abop_core::{
    error::AppError,
    config_error, audio_error, database_error, validation_error, 
    invalid_data, bail, ensure, with_context, timeout_error,
    error::{ErrorContext, ErrorChain},
};

#[test]
fn test_error_macros() {
    // Test config_error macro
    let err = config_error!("Invalid setting");
    match err {
        AppError::Config(msg) => assert_eq!(msg, "Invalid setting"),
        _ => panic!("Wrong error type"),
    }

    // Test formatted config_error
    let err = config_error!("Invalid value {} for setting {}", 42, "max_items");
    match err {
        AppError::Config(msg) => assert_eq!(msg, "Invalid value 42 for setting max_items"),
        _ => panic!("Wrong error type"),
    }

    // Test audio_error macro
    let err = audio_error!("Unsupported format: {}", "mp5");
    match err {
        AppError::Audio(msg) => assert_eq!(msg, "Unsupported format: mp5"),
        _ => panic!("Wrong error type"),
    }

    // Test validation_error macro
    let err = validation_error!("Field '{}' is required", "name");
    match err {
        AppError::ValidationFailed(msg) => assert_eq!(msg, "Field 'name' is required"),
        _ => panic!("Wrong error type"),
    }

    // Test invalid_data macro
    let err = invalid_data!("Negative value not allowed: {}", -5);
    match err {
        AppError::InvalidData(msg) => assert_eq!(msg, "Negative value not allowed: -5"),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_timeout_error_macro() {
    let err = timeout_error!("database query", 5000, 7500);
    match err {
        AppError::Timeout { operation, timeout_ms, elapsed_ms } => {
            assert_eq!(operation, "database query");
            assert_eq!(timeout_ms, 5000);
            assert_eq!(elapsed_ms, 7500);
        }
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_bail_macro() {
    fn test_function(value: i32) -> Result<i32, AppError> {
        if value < 0 {
            bail!("Negative values not allowed: {}", value);
        }
        Ok(value * 2)
    }

    assert!(test_function(5).is_ok());
    assert_eq!(test_function(5).unwrap(), 10);
    
    let err = test_function(-1).unwrap_err();
    match err {
        AppError::Other(msg) => assert_eq!(msg, "Negative values not allowed: -1"),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_ensure_macro() {
    fn test_function(value: i32) -> Result<i32, AppError> {
        ensure!(value > 0, AppError::InvalidData("Value must be positive".to_string()));
        ensure!(value < 100, "Value {} is too large", value);
        Ok(value)
    }

    assert!(test_function(50).is_ok());
    
    let err = test_function(-1).unwrap_err();
    match err {
        AppError::InvalidData(msg) => assert_eq!(msg, "Value must be positive"),
        _ => panic!("Wrong error type"),
    }

    let err = test_function(150).unwrap_err();
    match err {
        AppError::Other(msg) => assert_eq!(msg, "Value 150 is too large"),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_with_context_macro() {
    fn failing_operation() -> Result<(), std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"))
    }

    let result = with_context!(failing_operation(), "Failed to load configuration");
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    match err {
        AppError::Other(msg) => {
            assert!(msg.contains("Failed to load configuration"));
            assert!(msg.contains("file not found"));
        }
        _ => panic!("Wrong error type"),
    }

    // Test formatted context
    let result = with_context!(failing_operation(), "Failed to load file {}", "config.toml");
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    match err {
        AppError::Other(msg) => {
            assert!(msg.contains("Failed to load file config.toml"));
        }
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_error_context_trait() {
    fn io_operation() -> Result<String, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied"))
    }

    // Test context method
    let result = io_operation().context("Reading config file");
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    match err {
        AppError::Other(msg) => {
            assert!(msg.contains("Reading config file"));
            assert!(msg.contains("access denied"));
        }
        _ => panic!("Wrong error type"),
    }

    // Test with_context method
    let result = io_operation().with_context(|| "Dynamic context message".to_string());
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    match err {
        AppError::Other(msg) => {
            assert!(msg.contains("Dynamic context message"));
        }
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_error_chain() {
    let chain = ErrorChain::new()
        .context("Loading application")
        .add(std::io::Error::new(std::io::ErrorKind::NotFound, "config.toml not found"))
        .context("Configuration initialization failed")
        .add("Database connection error");

    let error = chain.into_error();
    let error_string = error.to_string();
    
    assert!(error_string.contains("Loading application"));
    assert!(error_string.contains("config.toml not found"));
    assert!(error_string.contains("Configuration initialization failed"));
    assert!(error_string.contains("Database connection error"));
    assert!(error_string.contains(" -> ")); // Chain separator
}

#[test]
fn test_complex_error_scenario() {
    fn complex_operation() -> Result<String, AppError> {
        // Simulate a complex operation with multiple potential failure points
        
        // Validation step
        let input = -1;
        ensure!(input >= 0, validation_error!("Input must be non-negative"));
        
        // File operation step
        let file_result: Result<String, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file missing"
        ));
        
        let _content = file_result
            .context("Reading configuration file")
            .map_err(|e| {
                ErrorChain::new()
                    .context("Configuration loading")
                    .add(e)
                    .context("Application initialization failed")
                    .into_error()
            })?;

        Ok("success".to_string())
    }

    let result = complex_operation();
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    let error_string = err.to_string();
    assert!(error_string.contains("Input must be non-negative"));
}

#[test]
fn test_error_propagation() {
    fn level_3() -> Result<(), AppError> {
        bail!("Level 3 error");
    }

    fn level_2() -> Result<(), AppError> {
        level_3().with_context(|| "Level 2 context".to_string())
    }

    fn level_1() -> Result<(), AppError> {
        level_2().context("Level 1 context")
    }

    let result = level_1();
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    let error_string = err.to_string();
    assert!(error_string.contains("Level 1 context"));
    assert!(error_string.contains("Level 2 context"));
    assert!(error_string.contains("Level 3 error"));
}

#[test]
fn test_database_error_macro() {
    let err = database_error!("Connection timeout after {}ms", 5000);
    match err {
        AppError::Database(db_err) => {
            match db_err {
                abop_core::db::error::DatabaseError::ExecutionFailed { message } => {
                    assert_eq!(message, "Connection timeout after 5000ms");
                }
                _ => panic!("Wrong database error type"),
            }
        }
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_error_display_formatting() {
    let errors = vec![
        config_error!("Invalid configuration"),
        audio_error!("Unsupported format"),
        validation_error!("Missing required field"),
        timeout_error!("scan", 1000, 1500),
    ];

    for error in errors {
        let display_string = error.to_string();
        assert!(!display_string.is_empty());
        assert!(display_string.len() > 10); // Should have meaningful content
        
        // All errors should have proper formatting
        match error {
            AppError::Config(_) => assert!(display_string.contains("Configuration error")),
            AppError::Audio(_) => assert!(display_string.contains("Audio processing error")),
            AppError::ValidationFailed(_) => assert!(display_string.contains("Validation failed")),
            AppError::Timeout { .. } => assert!(display_string.contains("timed out")),
            _ => {}
        }
    }
}

#[test]
fn test_error_from_conversions() {
    // Test that standard error types can be converted to AppError
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
    let app_error: AppError = io_error.into();
    match app_error {
        AppError::Io(_) => {} // Expected
        _ => panic!("Wrong conversion"),
    }

    // Test string conversion
    let app_error: AppError = "simple error".into();
    match app_error {
        AppError::Other(msg) => assert_eq!(msg, "simple error"),
        _ => panic!("Wrong conversion"),
    }
}
