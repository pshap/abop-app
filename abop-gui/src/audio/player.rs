//! Audio player management and global player instance

use abop_core::{PlayerState, audio::player::ThreadSafeAudioPlayer};
use std::path::PathBuf;

// ================================================================================================
// GLOBAL AUDIO PLAYER
// ================================================================================================

/// Global audio player instance
static AUDIO_PLAYER: std::sync::LazyLock<ThreadSafeAudioPlayer> =
    std::sync::LazyLock::new(|| ThreadSafeAudioPlayer::new().expect("Failed to create audio player"));

/// Get a reference to the global audio player
#[must_use]
pub fn get_audio_player() -> &'static ThreadSafeAudioPlayer {
    &AUDIO_PLAYER
}

/// Play selected audio files
///
/// # Errors
///
/// Returns an error if:
/// - No audiobooks are selected for playback
/// - Selected audiobook is not found
/// - Audio file path is invalid or missing
/// - Audio playback initialization fails
pub async fn play_selected_audio(
    selected_ids: Vec<String>,
    audiobooks: Vec<abop_core::models::Audiobook>,
) -> Result<String, String> {
    if selected_ids.is_empty() {
        return Err("No audiobooks selected for playback".to_string());
    }

    // Find the first selected audiobook
    let audiobook = audiobooks
        .iter()
        .find(|ab| selected_ids.contains(&ab.id))
        .ok_or_else(|| "Selected audiobook not found".to_string())?;

    // Validate the file exists before trying to play
    if !std::path::Path::new(&audiobook.path).exists() {
        return Err(format!(
            "Audio file not found: {}",
            audiobook.path.display()
        ));
    }

    // Play the audio file using the global player
    match AUDIO_PLAYER.play(&audiobook.path) {
        Ok(()) => {
            let title = audiobook.title.as_deref().unwrap_or("Unknown");
            Ok(format!("Started playing: {title}"))
        }
        Err(e) => Err(format!("Failed to play audio: {e}")),
    }
}

/// Stop audio playback
pub fn stop_audio() {
    AUDIO_PLAYER.stop();
}

/// Get current player state
pub fn get_player_state() -> PlayerState {
    (*AUDIO_PLAYER).get_state()
}

/// Get currently playing file path
#[must_use]
pub fn get_current_playing_file() -> Option<PathBuf> {
    (*AUDIO_PLAYER).get_current_file()
}
