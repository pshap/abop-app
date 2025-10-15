//! Flow-control, guard, and logging macros for error handling.
//!
//! These helpers make early-returns and error annotation concise and uniform.

#[macro_export]
/// Early-return with an error. Equivalent to `return Err(err.into())`.
macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::AppError::Other(format!($fmt, $($arg)*)))
    };
}

#[macro_export]
/// Ensure a condition holds, otherwise early-return with an error.
///
/// Examples
/// - `ensure!(x > 0, AppError::InvalidData("x must be > 0".into()))`
/// - `ensure!(ok, "operation {} failed", name)`
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

#[macro_export]
/// Map an error by attaching additional context to its message.
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

#[macro_export]
/// Log an error (with optional context) and yield it back unchanged.
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
