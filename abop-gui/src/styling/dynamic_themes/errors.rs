//! Error types for dynamic theme loading and validation

/// Theme loading errors
#[derive(Debug, Clone)]
pub enum ThemeLoadError {
    /// File not found or cannot be read
    FileError(String),
    /// Invalid JSON or TOML format
    ParseError(String),
    /// Invalid color format
    InvalidColor(String),
    /// Required field missing
    MissingField(String),
    /// Theme validation failed
    ValidationError(String),
}

impl std::fmt::Display for ThemeLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileError(msg) => write!(f, "File error: {msg}"),
            Self::ParseError(msg) => write!(f, "Parse error: {msg}"),
            Self::InvalidColor(msg) => write!(f, "Invalid color: {msg}"),
            Self::MissingField(field) => write!(f, "Missing field: {field}"),
            Self::ValidationError(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}

impl std::error::Error for ThemeLoadError {}
