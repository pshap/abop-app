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

// ================================================================================================
// POSITION, SEEKING AND SAVED PROGRESS
// ================================================================================================

/// Books are restarted instead of resumed when this close to the end
const RESTART_NEAR_END: std::time::Duration = std::time::Duration::from_secs(30);

/// Where to start a book given its saved progress
#[must_use]
pub fn resume_position(
    progress: Option<&abop_core::models::Progress>,
    duration_seconds: Option<u64>,
) -> std::time::Duration {
    let Some(progress) = progress.filter(|p| !p.completed) else {
        return std::time::Duration::ZERO;
    };
    let position = std::time::Duration::from_secs(progress.position_seconds);
    match duration_seconds.map(std::time::Duration::from_secs) {
        Some(total) if position + RESTART_NEAR_END >= total => std::time::Duration::ZERO,
        _ => position,
    }
}

/// Play an audiobook, resuming from its saved position
///
/// # Errors
///
/// Returns an error message if the file is missing or cannot be played.
pub async fn play_audiobook(audiobook: abop_core::models::Audiobook) -> Result<String, String> {
    if !audiobook.path.exists() {
        return Err(format!(
            "Audio file not found: {}",
            audiobook.path.display()
        ));
    }
    tokio::task::spawn_blocking(move || {
        let saved = crate::database::app_database()
            .and_then(|db| Ok(db.progress_repository().find_by_audiobook(&audiobook.id)?))
            .unwrap_or_else(|e| {
                log::warn!("Could not load saved progress: {e}");
                None
            });
        let start = resume_position(saved.as_ref(), audiobook.duration_seconds);
        let title = audiobook.title.as_deref().unwrap_or("Unknown");
        get_audio_player()
            .play_from(&audiobook.path, start)
            .map(|()| {
                if start.is_zero() {
                    format!("Started playing: {title}")
                } else {
                    format!("Resumed {title} at {}s", start.as_secs())
                }
            })
            .map_err(|e| format!("Failed to play audio: {e}"))
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Jump to `position` in the current audiobook
pub fn seek_audio(position: std::time::Duration) {
    get_audio_player().seek(position);
}

/// Current playback position
#[must_use]
pub fn get_position() -> std::time::Duration {
    get_audio_player().get_position()
}

/// Duration of the current audiobook, if known
#[must_use]
pub fn get_duration() -> Option<std::time::Duration> {
    get_audio_player().get_duration()
}

/// Save listening progress for an audiobook
///
/// # Errors
///
/// Returns an error message if the database cannot be updated.
pub async fn save_progress(
    audiobook_id: String,
    position_seconds: u64,
    completed: bool,
) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let db = crate::database::app_database().map_err(|e| e.to_string())?;
        let repository = db.progress_repository();
        let mut progress = repository
            .find_by_audiobook(&audiobook_id)
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| abop_core::models::Progress::new(&audiobook_id, 0));
        progress.update_position(position_seconds);
        if completed {
            progress.mark_completed();
        } else {
            progress.completed = false;
        }
        repository.upsert(&progress).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Load saved listening progress for every audiobook
///
/// # Errors
///
/// Returns an error message if the database cannot be read.
pub async fn load_all_progress()
-> Result<std::collections::HashMap<String, crate::state::ListeningProgress>, String> {
    tokio::task::spawn_blocking(|| {
        let db = crate::database::app_database().map_err(|e| e.to_string())?;
        let all = db
            .progress_repository()
            .find_all()
            .map_err(|e| e.to_string())?;
        Ok(all
            .into_iter()
            .map(|p| {
                (
                    p.audiobook_id,
                    crate::state::ListeningProgress {
                        position_seconds: p.position_seconds,
                        completed: p.completed,
                    },
                )
            })
            .collect())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(test)]
mod resume_tests {
    use super::*;
    use abop_core::models::Progress;
    use std::time::Duration;

    #[test]
    fn resumes_unfinished_books_and_restarts_finished_ones() {
        let mut progress = Progress::new("b", 600);
        assert_eq!(resume_position(None, Some(3600)), Duration::ZERO);
        assert_eq!(
            resume_position(Some(&progress), Some(3600)),
            Duration::from_secs(600)
        );
        assert_eq!(
            resume_position(Some(&progress), None),
            Duration::from_secs(600)
        );

        // Within the last 30 seconds: start over
        progress.position_seconds = 3590;
        assert_eq!(resume_position(Some(&progress), Some(3600)), Duration::ZERO);

        progress.position_seconds = 600;
        progress.completed = true;
        assert_eq!(resume_position(Some(&progress), Some(3600)), Duration::ZERO);
    }
}
