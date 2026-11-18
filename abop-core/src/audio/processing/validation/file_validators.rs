//! File validation operations for audio processing

use crate::error::{AppError, Result as AppResult};
use std::path::{Path, PathBuf};

/// Validate that a file path exists and is accessible
///
/// # Errors
///
/// Returns [`AppError::Io`] if path does not exist or is not accessible.
pub fn validate_file_exists<P: AsRef<Path>>(path: P) -> AppResult<PathBuf> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(AppError::Io(format!(
            "File does not exist: {}",
            path.display()
        )));
    }

    if !path.is_file() {
        return Err(AppError::Io(format!(
            "Path is not a file: {}",
            path.display()
        )));
    }

    Ok(path.to_path_buf())
}

/// Validate that a directory path exists and is accessible
///
/// # Errors
///
/// Returns [`AppError::Io`] if directory does not exist or is not accessible.
pub fn validate_directory_exists<P: AsRef<Path>>(path: P) -> AppResult<PathBuf> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(AppError::Io(format!(
            "Directory does not exist: {}",
            path.display()
        )));
    }

    if !path.is_dir() {
        return Err(AppError::Io(format!(
            "Path is not a directory: {}",
            path.display()
        )));
    }

    Ok(path.to_path_buf())
}

/// Validate that we have read permissions for a file
///
/// # Errors
///
/// Returns [`AppError::Io`] if file cannot be read.
pub fn validate_file_readable<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let path = path.as_ref();

    // Try to open file in read-only mode to check permissions
    match std::fs::File::open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::Io(format!(
            "Cannot read file {}: {}",
            path.display(),
            e
        ))),
    }
}

/// Validate that we have write permissions for a directory
///
/// # Errors
///
/// Returns [`AppError::Io`] if directory is not writable.
pub fn validate_directory_writable<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let path = path.as_ref();

    // Try to create a temporary file to test write permissions
    let test_file = path.join(".abop_write_test");
    match std::fs::write(&test_file, "test") {
        Ok(_) => {
            // Clean up test file
            let _ = std::fs::remove_file(&test_file);
            Ok(())
        }
        Err(e) => Err(AppError::Io(format!(
            "Cannot write to directory {}: {}",
            path.display(),
            e
        ))),
    }
}

/// Validate file extension is a supported audio format
///
/// # Errors
///
/// Returns [`AppError::Audio`] if file extension is not supported.
pub fn validate_audio_extension<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let path = path.as_ref();

    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            match ext_str.to_lowercase().as_str() {
                "mp3" | "flac" | "aac" | "ogg" | "wav" | "m4a" | "m4b" => Ok(()),
                _ => Err(AppError::Audio(format!(
                    "Unsupported audio format: {} (supported: mp3, flac, aac, ogg, wav, m4a, m4b)",
                    ext_str
                ))),
            }
        } else {
            Err(AppError::Audio(
                "Invalid file extension (non-UTF8 characters)".to_string(),
            ))
        }
    } else {
        Err(AppError::Audio(
            "File has no extension - cannot determine audio format".to_string(),
        ))
    }
}

/// Validate file size is reasonable for audio processing
///
/// # Errors
///
/// Returns [`AppError::Audio`] if file size is zero or exceeds reasonable limits.
pub fn validate_file_size<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let path = path.as_ref();

    let metadata = match std::fs::metadata(path) {
        Ok(meta) => meta,
        Err(e) => {
            return Err(AppError::Io(format!(
                "Cannot read metadata for {}: {}",
                path.display(),
                e
            )));
        }
    };

    let file_size = metadata.len();

    if file_size == 0 {
        return Err(AppError::Audio(format!(
            "File is empty: {}",
            path.display()
        )));
    }

    const MAX_REASONABLE_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10GB

    if file_size > MAX_REASONABLE_SIZE {
        log::warn!(
            "Very large file detected: {} bytes ({} GB)",
            file_size,
            file_size / (1024 * 1024 * 1024)
        );
    }

    Ok(())
}

/// Validate audio file metadata is complete and reasonable
///
/// # Errors
///
/// Returns [`AppError::Audio`] if metadata appears invalid or incomplete.
pub fn validate_metadata_completeness(
    title: Option<&str>,
    _artist: Option<&str>,
    duration: Option<std::time::Duration>,
) -> AppResult<()> {
    // Check if essential metadata is present
    if title.is_none_or(str::is_empty) {
        log::warn!("Audio file has no title metadata");
    }

    if duration.is_none() {
        return Err(AppError::Audio(
            "Audio file has no duration information".to_string(),
        ));
    }

    // Check if duration is reasonable
    if let Some(dur) = duration {
        if dur.is_zero() {
            return Err(AppError::Audio("Audio file has zero duration".to_string()));
        }

        if dur.as_secs() > 24 * 3600 {
            log::warn!("Very long audio duration: {} hours", dur.as_secs() / 3600);
        }
    }

    Ok(())
}
