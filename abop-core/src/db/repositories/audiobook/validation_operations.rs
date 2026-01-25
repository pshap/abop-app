//! Input validation operations for audiobook repository

use crate::db::error::DatabaseError;
use crate::error::AppError;

/// Validate audiobook ID is not empty
pub fn validate_audiobook_id(id: &str) -> Result<(), AppError> {
    if id.is_empty() {
        return Err(AppError::Database(DatabaseError::validation_failed(
            "audiobook_id",
            "Audiobook ID cannot be empty",
        )));
    }
    Ok(())
}

/// Validate library ID is not empty
pub fn validate_library_id(library_id: &str) -> Result<(), AppError> {
    if library_id.is_empty() {
        return Err(AppError::Database(DatabaseError::validation_failed(
            "library_id",
            "Library ID cannot be empty",
        )));
    }
    Ok(())
}

/// Validate author name is not empty
pub fn validate_author(author: &str) -> Result<(), AppError> {
    if author.is_empty() {
        return Err(AppError::Database(DatabaseError::validation_failed(
            "author",
            "Author cannot be empty",
        )));
    }
    Ok(())
}

/// Validate audiobook path is not empty
pub fn validate_path(path: &str) -> Result<(), AppError> {
    if path.is_empty() {
        return Err(AppError::Database(DatabaseError::validation_failed(
            "path",
            "Audiobook path cannot be empty",
        )));
    }
    Ok(())
}

/// Validate ID for database operations (more generic validation)
pub fn validate_id(id: &str, field_name: &str) -> Result<(), DatabaseError> {
    if id.is_empty() {
        return Err(DatabaseError::validation_failed(
            field_name,
            &format!("{} cannot be empty", field_name),
        ));
    }
    Ok(())
}
