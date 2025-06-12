//! Windows-specific environment variable utilities
//!
//! This module provides utilities for working with Windows environment variables,
//! including path expansion and system directory resolution.

use std::env;
use std::ffi::OsString;
use std::fmt;
use std::path::{Path, PathBuf};

/// Error type for environment variable operations
#[derive(Debug)]
pub enum EnvVarError {
    /// Environment variable not found
    NotFound(String),
    /// Invalid Unicode in environment variable
    InvalidUnicode(OsString),
    /// Path conversion error
    PathConversion(String),
}

impl std::error::Error for EnvVarError {}

impl fmt::Display for EnvVarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvVarError::NotFound(var) => write!(f, "Environment variable '{var}' not found"),
            EnvVarError::InvalidUnicode(os_str) => write!(
                f,
                "Environment variable contains invalid Unicode: {os_str:?}"
            ),
            EnvVarError::PathConversion(msg) => write!(f, "Path conversion error: {msg}"),
        }
    }
}

/// Expands environment variables in a path string
///
/// On Windows, this handles both `%VAR%` and `$env:VAR` syntax for compatibility.
/// Returns the expanded path with all environment variables resolved.
pub fn expand_env_vars(path: &str) -> Result<String, EnvVarError> {
    let mut result = String::with_capacity(path.len());
    let mut chars = path.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            // Handle %VAR% syntax
            let mut var_name = String::new();
            for c in chars.by_ref() {
                if c == '%' {
                    break;
                }
                var_name.push(c);
            }

            let value = env::var(&var_name).map_err(|_| EnvVarError::NotFound(var_name))?;
            result.push_str(&value);
        } else if c == '$' && chars.peek() == Some(&'e') {
            // Handle $env:VAR syntax (PowerShell style)
            let prefix: String = chars.by_ref().take(4).collect();
            if prefix == "env:" {
                let mut var_name = String::new();
                let mut next_char = None;
                for c in chars.by_ref() {
                    if !c.is_alphanumeric() && c != '_' {
                        next_char = Some(c);
                        break;
                    }
                    var_name.push(c);
                }

                let value = env::var(&var_name).map_err(|_| EnvVarError::NotFound(var_name))?;
                result.push_str(&value);

                // Push the next character if it's not a valid variable name character
                if let Some(next_c) = next_char {
                    result.push(next_c);
                }
            } else {
                result.push('$');
                result.push_str(&prefix);
            }
        } else {
            result.push(c);
        }
    }

    Ok(result)
}

/// Gets the application data directory, respecting Windows conventions
///
/// Returns paths in this order:
/// 1. %APPDATA%\<app_name> for roaming app data
/// 2. %LOCALAPPDATA%\<app_name> for local app data
/// 3. Fallback to a path in the user's home directory
pub fn get_app_data_dir(app_name: &str) -> Result<PathBuf, EnvVarError> {
    // Try APPDATA first (roaming)
    if let Ok(app_data) = env::var("APPDATA") {
        let path = Path::new(&app_data).join(app_name);
        return Ok(path);
    }

    // Then try LOCALAPPDATA (non-roaming)
    if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
        let path = Path::new(&local_app_data).join(app_name);
        return Ok(path);
    }

    // Fallback to home directory
    dirs::home_dir()
        .map(|mut path| {
            path.push("AppData");
            path.push("Local");
            path.push(app_name);
            path
        })
        .ok_or_else(|| {
            EnvVarError::PathConversion("Could not determine home directory".to_string())
        })
}

/// Expands environment variables in a Path
///
/// Converts the path to a string, expands any environment variables, and converts back to a Path.
/// This is useful for paths that might contain environment variables like %USERPROFILE%.
pub fn expand_path_env_vars(path: &Path) -> Result<PathBuf, EnvVarError> {
    let path_str = path
        .to_str()
        .ok_or_else(|| EnvVarError::PathConversion("Path contains invalid UTF-8".to_string()))?;

    let expanded = expand_env_vars(path_str)?;
    Ok(PathBuf::from(expanded))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // Use a static mutex to prevent environment variable tests from interfering with each other
    static ENV_MUTEX: std::sync::LazyLock<std::sync::Mutex<()>> =
        std::sync::LazyLock::new(|| std::sync::Mutex::new(()));

    #[test]
    fn test_expand_env_vars() {
        let _lock = ENV_MUTEX.lock().unwrap();

        // Set up test environment variables
        unsafe {
            env::set_var("TEST_VAR", "test_value");
            env::set_var("USERNAME", "testuser");
        }

        // Test %VAR% syntax
        assert_eq!(
            expand_env_vars("C:\\Users\\%USERNAME%\\Documents").unwrap(),
            "C:\\Users\\testuser\\Documents"
        );

        // Test $env:VAR syntax
        assert_eq!(
            expand_env_vars("C:\\Users\\$env:USERNAME\\Documents").unwrap(),
            "C:\\Users\\testuser\\Documents"
        );

        // Test unknown variable
        assert!(matches!(
            expand_env_vars("%NONEXISTENT%"),
            Err(EnvVarError::NotFound(_))
        ));
    }

    #[test]
    fn test_get_app_data_dir() {
        let _lock = ENV_MUTEX.lock().unwrap();

        // Test with APPDATA set
        unsafe {
            env::set_var("APPDATA", "C:\\Users\\testuser\\AppData\\Roaming");
        }
        let path = get_app_data_dir("testapp").unwrap();
        assert!(path.ends_with("testapp"));

        // Test with LOCALAPPDATA set
        unsafe {
            env::remove_var("APPDATA");
            env::set_var("LOCALAPPDATA", "C:\\Users\\testuser\\AppData\\Local");
        }
        let path = get_app_data_dir("testapp").unwrap();
        assert!(path.ends_with("testapp"));
    }
}
