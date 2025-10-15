// Flow/guard/logging macros for error handling

#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::AppError::Other(format!($fmt, $($arg)*)))
    };
}

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
