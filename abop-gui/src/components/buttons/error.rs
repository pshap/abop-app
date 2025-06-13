//! Error types for button operations

use std::fmt;

/// Errors that can occur during button building or operations
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ButtonError {
    /// The button is missing a required field
    MissingField(&'static str),

    /// The button configuration is invalid
    InvalidConfiguration(&'static str),

    /// The requested icon was not found
    IconNotFound(&'static str),

    /// The button variant is not supported for the current operation
    UnsupportedVariant(&'static str),

    /// The button is missing both label and icon
    MissingLabelAndIcon,

    /// The button is missing an on_press handler
    MissingOnPress,

    /// The button state is invalid
    InvalidState(&'static str),

    /// The icon position is invalid for the current button configuration
    InvalidIconPosition,
}

impl std::error::Error for ButtonError {}

impl fmt::Display for ButtonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ButtonError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ButtonError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
            ButtonError::IconNotFound(icon) => write!(f, "Icon not found: {}", icon),
            ButtonError::UnsupportedVariant(variant) => {
                write!(f, "Unsupported variant: {}", variant)
            }
            ButtonError::MissingLabelAndIcon => {
                write!(f, "Button must have either a label or an icon")
            }
            ButtonError::MissingOnPress => write!(f, "Button must have an on_press handler"),
            ButtonError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            ButtonError::InvalidIconPosition => {
                write!(
                    f,
                    "Invalid icon position: IconPosition::Only cannot be used with both label and icon"
                )
            }
        }
    }
}

/// A specialized `Result` type for button operations
///
/// This is a convenience type alias that represents the result of button-related operations.
/// The success type `T` contains the expected value, while the error type is always `ButtonError`.
///
/// # Examples
///
/// ```rust
/// use abop_gui::components::buttons::{ButtonResult, ButtonError};
///
/// fn create_button_element() -> ButtonResult<SomeButtonType> {
///     // ... button creation logic
///     if icon_exists {
///         Ok(button)
///     } else {
///         Err(ButtonError::IconNotFound("invalid_icon"))
///     }
/// }
/// ```
pub type ButtonResult<T> = Result<T, ButtonError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            ButtonError::MissingField("label").to_string(),
            "Missing required field: label"
        );

        assert_eq!(
            ButtonError::InvalidConfiguration("invalid size").to_string(),
            "Invalid configuration: invalid size"
        );

        assert_eq!(
            ButtonError::IconNotFound("missing_icon").to_string(),
            "Icon not found: missing_icon"
        );

        assert_eq!(
            ButtonError::UnsupportedVariant("for operation").to_string(),
            "Unsupported variant: for operation"
        );

        assert_eq!(
            ButtonError::InvalidState("button is disabled").to_string(),
            "Invalid state: button is disabled"
        );
    }
}
