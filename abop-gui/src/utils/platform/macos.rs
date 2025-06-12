//! macOS-specific default directory resolution

use std::path::PathBuf;

/// Get the default audiobook directory on macOS
///
/// macOS-specific preferences following Apple's conventions:
/// 1. ~/Music/Audiobooks (macOS standard location for audiobooks)
/// 2. ~/Documents/Audiobooks (documents fallback)
/// 3. ~/Downloads/Audiobooks (common download location)
/// 4. ~ (home directory fallback)
/// 5. Current directory (ultimate fallback)
pub fn get_default_audiobook_directory() -> PathBuf {
    // Try Music folder first (macOS standard for audiobooks)
    if let Some(music_dir) = dirs::audio_dir() {
        return music_dir.join("Audiobooks");
    }

    // Try Documents folder as secondary option
    if let Some(docs_dir) = dirs::document_dir() {
        return docs_dir.join("Audiobooks");
    }

    // Try Downloads folder (common for audiobook files)
    if let Some(download_dir) = dirs::download_dir() {
        return download_dir.join("Audiobooks");
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

        // On macOS, should prefer Music\Audiobooks if available
        if let Some(music_dir) = dirs::audio_dir() {
            let expected = music_dir.join("Audiobooks");
            assert_eq!(dir, expected);
        }
    }
}
