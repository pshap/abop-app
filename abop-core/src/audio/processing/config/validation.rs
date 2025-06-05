//! Validation utilities for audio processing configuration.
//!
//! This module provides reusable validation functions for checking configuration
//! parameters across different components. These utilities help ensure consistent
//! validation behavior and error messages throughout the configuration system.

use std::fmt::Display;
use std::path::Path;

use crate::audio::processing::error::{AudioProcessingError, Result};

/// Validates that a value is within a specified inclusive range
///
/// # Arguments
///
/// * `value` - The value to validate
/// * `min` - The minimum allowed value (inclusive)
/// * `max` - The maximum allowed value (inclusive)
/// * `name` - The name of the parameter for error messages
///
/// # Returns
///
/// * `Ok(())` if the value is within the range
/// * `Err(AudioProcessingError)` if the value is outside the range
///
/// # Errors
///
/// Returns [`AudioProcessingError::InvalidConfiguration`] if the value is less than
/// the minimum or greater than the maximum allowed value.
///
/// # Examples
///
/// ```
/// use abop_core::audio::processing::config::validation;
///
/// // Validate that a sample rate is between 8000 and 192000 Hz
/// let result = validation::range(&44100, &8000, &192000, "sample rate");
/// assert!(result.is_ok());
///
/// // This will fail because the value is outside the range
/// let result = validation::range(&200000, &8000, &192000, "sample rate");
/// assert!(result.is_err());
/// ```
pub fn range<T>(value: &T, min: &T, max: &T, name: &str) -> Result<()>
where
    T: PartialOrd + Display,
{
    if value < min {
        return Err(AudioProcessingError::config(format!(
            "{name} must be at least {min} (got {value})"
        )));
    }
    if value > max {
        return Err(AudioProcessingError::config(format!(
            "{name} must be at most {max} (got {value})"
        )));
    }
    Ok(())
}

/// Validates that a value is less than a specified maximum
///
/// # Arguments
///
/// * `value` - The value to validate
/// * `max` - The maximum allowed value (exclusive)
/// * `name` - The name of the parameter for error messages
///
/// # Returns
///
/// * `Ok(())` if the value is less than the maximum
/// * `Err(AudioProcessingError)` if the value is greater than or equal to the maximum
///
/// # Errors
///
/// Returns [`AudioProcessingError::InvalidConfiguration`] if the value is greater
/// than or equal to the maximum allowed value.
pub fn less_than<T>(value: &T, max: &T, name: &str) -> Result<()>
where
    T: PartialOrd + Display,
{
    if value >= max {
        return Err(AudioProcessingError::config(format!(
            "{name} must be less than {max} (got {value})"
        )));
    }
    Ok(())
}

/// Validates that a value is positive (greater than zero)
///
/// # Arguments
///
/// * `value` - The value to validate
/// * `name` - The name of the parameter for error messages
///
/// # Returns
///
/// * `Ok(())` if the value is positive
/// * `Err(AudioProcessingError)` if the value is zero or negative
///
/// # Errors
///
/// Returns [`AudioProcessingError::InvalidConfiguration`] if the value is zero
/// or negative.
pub fn positive<T>(value: &T, name: &str) -> Result<()>
where
    T: PartialOrd + Display + Default,
{
    if value <= &T::default() {
        return Err(AudioProcessingError::config(format!(
            "{name} must be positive (got {value})"
        )));
    }
    Ok(())
}

/// Validates that a value is negative (less than zero)
///
/// # Arguments
///
/// * `value` - The value to validate
/// * `name` - The name of the parameter for error messages
///
/// # Returns
///
/// * `Ok(())` if the value is negative
/// * `Err(AudioProcessingError)` if the value is zero or positive
///
/// # Errors
///
/// Returns [`AudioProcessingError::InvalidConfiguration`] if the value is zero
/// or positive.
pub fn negative<T>(value: &T, name: &str) -> Result<()>
where
    T: PartialOrd + Display + Default,
{
    if value >= &T::default() {
        return Err(AudioProcessingError::config(format!(
            "{name} must be negative (got {value})"
        )));
    }
    Ok(())
}

/// Validates that a directory exists
///
/// # Arguments
///
/// * `path` - The path to validate
/// * `name` - The name of the parameter for error messages
///
/// # Returns
///
/// * `Ok(())` if the directory exists
/// * `Err(AudioProcessingError)` if the directory does not exist or is not a directory
///
/// # Errors
///
/// Returns [`AudioProcessingError::InvalidConfiguration`] if the directory does not
/// exist or if the path exists but is not a directory.
pub fn directory_exists(path: &Path, name: &str) -> Result<()> {
    if !path.exists() {
        return Err(AudioProcessingError::config(format!(
            "{} does not exist: {}",
            name,
            path.display()
        )));
    }
    if !path.is_dir() {
        return Err(AudioProcessingError::config(format!(
            "{} is not a directory: {}",
            name,
            path.display()
        )));
    }
    Ok(())
}

/// Validates that a string is not empty
///
/// # Arguments
///
/// * `value` - The string to validate
/// * `name` - The name of the parameter for error messages
///
/// # Returns
///
/// * `Ok(())` if the string is not empty
/// * `Err(AudioProcessingError)` if the string is empty
///
/// # Errors
///
/// Returns [`AudioProcessingError::InvalidConfiguration`] if the string is empty.
pub fn non_empty_string(value: &str, name: &str) -> Result<()> {
    if value.is_empty() {
        return Err(AudioProcessingError::config(format!(
            "{name} cannot be empty"
        )));
    }
    Ok(())
}
