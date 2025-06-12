//! Windows-specific default directory resolution

use std::path::PathBuf;

/// Get the default audiobook directory on Windows
///
/// Windows-specific preferences:
/// 1. %USERPROFILE%\Music\Audiobooks (following Windows media conventions)
/// 2. %USERPROFILE%\Documents\Audiobooks (documents fallback)
/// 3. %USERPROFILE% (home directory fallback)
/// 4. Current directory (ultimate fallback)
pub fn get_default_audiobook_directory() -> PathBuf {
    // Try Music folder first (Windows convention for audio content)
    if let Some(music_dir) = dirs::audio_dir() {
        return music_dir.join("Audiobooks");
    }
    
    // Try Documents folder as secondary option
    if let Some(docs_dir) = dirs::document_dir() {
        return docs_dir.join("Audiobooks");
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
        
        // On Windows, should prefer Music\Audiobooks if available
        if let Some(music_dir) = dirs::audio_dir() {
            let expected = music_dir.join("Audiobooks");
            assert_eq!(dir, expected);
        }
    }
}
