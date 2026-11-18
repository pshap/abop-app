//! Audio player management and global player instance

use std::path::PathBuf;

use abop_core::PlayerState;
use abop_core::audio::player::ThreadSafeAudioPlayer;

// ================================================================================================
// GLOBAL AUDIO PLAYER
// ================================================================================================

/// Global audio player instance using std::sync::OnceLock for thread safety
static AUDIO_PLAYER: std::sync::OnceLock<ThreadSafeAudioPlayer> = std::sync::OnceLock::new();

/// Initialize audio player
fn init_audio_player() -> ThreadSafeAudioPlayer {
    ThreadSafeAudioPlayer::new().expect("Failed to create audio player")
}

/// Get a reference to the global audio player
#[must_use]
pub fn get_audio_player() -> &'static ThreadSafeAudioPlayer {
    AUDIO_PLAYER.get_or_init(init_audio_player)
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
    let player = get_audio_player();
    match player.play(&audiobook.path) {
        Ok(()) => {
            let title = audiobook.title.as_deref().unwrap_or("Unknown");
            Ok(format!("Started playing: {title}"))
        }
        Err(e) => Err(format!("Failed to play audio: {e}")),
    }
}

/// Stop audio playback
pub fn stop_audio() {
    let player = get_audio_player();
    player.stop();
}

/// Pause audio playback, keeping the position
pub fn pause_audio() {
    get_audio_player().pause();
}

/// Resume paused audio playback
pub fn resume_audio() {
    get_audio_player().resume();
}

/// Get current player state
pub fn get_player_state() -> PlayerState {
    let player = get_audio_player();
    player.get_state()
}

/// Get currently playing file path
#[must_use]
pub fn get_current_playing_file() -> Option<PathBuf> {
    let player = get_audio_player();
    player.get_current_file()
}
