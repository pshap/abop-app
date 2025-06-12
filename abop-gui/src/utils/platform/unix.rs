//! Unix-specific default directory resolution

use std::path::PathBuf;

/// Get the default audiobook directory on Unix-like systems
///
/// Unix-specific preferences following XDG Base Directory Specification:
/// 1. $XDG_DATA_HOME/audiobooks or ~/.local/share/audiobooks
/// 2. ~/Documents/Audiobooks (common user expectation)
/// 3. ~/Music/Audiobooks (audio content convention)
/// 4. ~ (home directory fallback)
/// 5. Current directory (ultimate fallback)
pub fn get_default_audiobook_directory() -> PathBuf {
    // Try XDG data directory first (proper Unix convention)
    if let Some(data_dir) = dirs::data_dir() {
        return data_dir.join("audiobooks");
    }

    // Try Documents folder as secondary option
    if let Some(docs_dir) = dirs::document_dir() {
        return docs_dir.join("Audiobooks");
    }

    // Try Music folder as tertiary option
    if let Some(music_dir) = dirs::audio_dir() {
        return music_dir.join("Audiobooks");
    }

    // Try home directory
    if let Some(home_dir) = dirs::home_dir() {
        return home_dir;
    }

    // Ultimate fallback
    PathBuf::from(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_audiobook_directory() {
        let dir = get_default_audiobook_directory();

        // Should not be empty and should be a valid path
        assert!(!dir.as_os_str().is_empty());

        // On Unix, should prefer XDG data directory if available
        if let Some(data_dir) = dirs::data_dir() {
            let expected = data_dir.join("audiobooks");
            assert_eq!(dir, expected);
        }
    }
}
