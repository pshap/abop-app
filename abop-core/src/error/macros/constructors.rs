// Macros for constructing AppError variants and simple guards

#[macro_export]
macro_rules! config_error {
    ($msg:expr) => {
        $crate::error::AppError::Config($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::Config(format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! audio_error {
    ($msg:expr) => {
        $crate::error::AppError::Audio($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::Audio(format!($fmt, $($arg)*))
    };
}

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

#[macro_export]
macro_rules! validation_error {
    ($msg:expr) => {
        $crate::error::AppError::ValidationFailed($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::ValidationFailed(format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! invalid_data {
    ($msg:expr) => {
        $crate::error::AppError::InvalidData($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::AppError::InvalidData(format!($fmt, $($arg)*))
    };
}

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
