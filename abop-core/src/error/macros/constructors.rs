//! Constructors for common `AppError` variants and simple guards.
//!
//! These macros provide concise ways to create strongly-typed application
//! errors throughout the codebase. Prefer these over ad‑hoc strings to keep
//! error handling consistent.

#[macro_export]
/// Create a `AppError::Config` from a string or format args.
///
/// Examples
/// - `config_error!("invalid setting")`
/// - `config_error!("invalid setting: {}", key)`
macro_rules! config_error {
    ($msg:expr) => {
        $crate::error::AppError::Config($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::Config(format!($fmt, $($arg)*))
    };
}

#[macro_export]
/// Create a `AppError::Audio` from a string or format args.
///
/// Examples
/// - `audio_error!("decoder failed")`
/// - `audio_error!("decoder {} failed", name)`
macro_rules! audio_error {
    ($msg:expr) => {
        $crate::error::AppError::Audio($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::Audio(format!($fmt, $($arg)*))
    };
}

#[macro_export]
/// Create a `AppError::Database` with an `ExecutionFailed` variant.
///
/// Examples
/// - `database_error!("unique constraint violated")`
/// - `database_error!("query failed: {}", sql)`
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

#[macro_export]
/// Create a `AppError::ValidationFailed` from a string or format args.
///
/// Examples
/// - `validation_error!("name cannot be empty")`
/// - `validation_error!("{} must be >= {}", field, min)`
macro_rules! validation_error {
    ($msg:expr) => {
        $crate::error::AppError::ValidationFailed($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::ValidationFailed(format!($fmt, $($arg)*))
    };
}

#[macro_export]
/// Create a `AppError::InvalidData` from a string or format args.
///
/// Examples
/// - `invalid_data!("unexpected file format")`
/// - `invalid_data!("bad header: {}", header)`
macro_rules! invalid_data {
    ($msg:expr) => {
        $crate::error::AppError::InvalidData($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::InvalidData(format!($fmt, $($arg)*))
    };
}

#[macro_export]
/// Create a `AppError::Timeout` capturing operation and timing details.
///
/// Example: `timeout_error!("scan library", 5_000, elapsed_ms)`
macro_rules! timeout_error {
    ($operation:expr, $timeout_ms:expr, $elapsed_ms:expr) => {
        $crate::error::AppError::Timeout {
            operation: $operation.to_string(),
            timeout_ms: $timeout_ms,
            elapsed_ms: $elapsed_ms,
        }
    };
}
