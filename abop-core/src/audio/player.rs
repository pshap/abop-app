//! Audio playback functionality using Rodio
//!
//! This module provides audio playback capabilities for the ABOP application.

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rodio::{Decoder, OutputStream, Sink};

use crate::error::{AppError, Result};

/// Audio player state
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PlayerState {
    /// Player is stopped
    #[default]
    Stopped,
    /// Player is playing
    Playing,
    /// Player is paused
    Paused,
}

/// Thread-safe audio player for playing audio files
///
/// This type is not `Send` or `Sync` by default. To share it between threads,
/// wrap it in an `Arc<Mutex<AudioPlayer>>` or use the provided `ThreadSafeAudioPlayer` type.
pub struct AudioPlayer {
    /// Current audio sink
    sink: Option<Sink>,
    /// Current player state
    state: PlayerState,
    /// Current volume (0.0 to 1.0)
    volume: f32,
    /// Current playing file path
    current_file: Option<PathBuf>,
}

/// Thread-safe wrapper around AudioPlayer
///
/// This type provides `Send` and `Sync` implementations by using internal
/// synchronization. It's the preferred way to share an AudioPlayer between threads.
#[derive(Clone)]
pub struct ThreadSafeAudioPlayer {
    inner: Arc<Mutex<AudioPlayer>>,
}

impl ThreadSafeAudioPlayer {
    /// Creates a new thread-safe audio player
    ///
    /// # Errors
    ///
    /// Returns `AppError::Audio` if audio system initialization fails.
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(AudioPlayer::new()?)),
        })
    }

    /// Plays an audio file
    ///
    /// See `AudioPlayer::play` for details.
    pub fn play<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        self.inner
            .lock()
            .map_err(|e| AppError::Audio(format!("Failed to acquire audio player lock: {e}")))?
            .play(file_path)
    }

    /// Stops audio playback
    ///
    /// See `AudioPlayer::stop` for details.
    pub fn stop(&self) {
        if let Ok(mut player) = self.inner.lock() {
            player.stop();
        }
    }

    /// Gets the current player state
    ///
    /// See `AudioPlayer::get_state` for details.
    #[must_use] pub fn get_state(&self) -> PlayerState {
        self.inner
            .lock()
            .map(|player| player.get_state())
            .unwrap_or(PlayerState::Stopped)
    }

    /// Gets the currently playing file path
    ///
    /// See `AudioPlayer::get_current_file` for details.
    #[must_use] pub fn get_current_file(&self) -> Option<PathBuf> {
        self.inner
            .lock()
            .map(|player| player.get_current_file())
            .unwrap_or(None)
    }
}

impl AudioPlayer {
    /// Creates a new audio player
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if audio system initialization fails.
    pub const fn new() -> Result<Self> {
        Ok(Self {
            sink: None,
            state: PlayerState::Stopped,
            volume: 0.7, // Default volume 70%
            current_file: None,
        })
    }

    /// Creates a new thread-safe audio player
    ///
    /// This is a convenience method that wraps the player in a `ThreadSafeAudioPlayer`.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Audio` if audio system initialization fails.
    pub fn new_thread_safe() -> Result<ThreadSafeAudioPlayer> {
        ThreadSafeAudioPlayer::new()
    }

    /// Plays an audio file
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if the audio output stream cannot be created,
    /// the file format is unsupported, or the audio sink creation fails.
    /// Returns [`AppError::Io`] if the file cannot be read.
    pub fn play<P: AsRef<Path>>(&mut self, file_path: P) -> Result<()> {
        let file_path = file_path.as_ref();

        // Stop any currently playing audio
        self.stop();

        // Create output stream for this playback session
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AppError::Audio(format!("Failed to create audio output stream: {e}")))?;

        // Open the audio file
        let file = File::open(file_path)?;
        let buf_reader = BufReader::new(file);

        // Create decoder
        let source = Decoder::new(buf_reader).map_err(|e| {
            AppError::Audio(format!(
                "Failed to decode audio file '{}': {}",
                file_path.display(),
                e
            ))
        })?;

        // Create new sink and set volume
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| AppError::Audio(format!("Failed to create audio sink: {e}")))?;
        sink.set_volume(self.volume);

        // Append the source and play
        sink.append(source);
        sink.play();

        // Update state
        self.sink = Some(sink);
        self.state = PlayerState::Playing;
        self.current_file = Some(file_path.to_path_buf());

        log::info!("Started playing audio file: {}", file_path.display());
        Ok(())
    }

    /// Stops audio playback
    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
            log::info!("Stopped audio playback");
        }

        self.state = PlayerState::Stopped;
        self.current_file = None;
    }
    /// Pauses audio playback
    pub fn pause(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.pause();
            self.state = PlayerState::Paused;
            log::info!("Paused audio playback");
        }
    }

    /// Resumes audio playback
    pub fn resume(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.play();
            self.state = PlayerState::Playing;
            log::info!("Resumed audio playback");
        }
    }

    /// Gets the current player state
    #[must_use]
    pub fn get_state(&self) -> PlayerState {
        self.state.clone()
    }

    /// Sets the volume (0.0 to 1.0)
    pub fn set_volume(&mut self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        self.volume = volume;

        if let Some(ref sink) = self.sink {
            sink.set_volume(volume);
        }
        log::debug!("Set volume to {:.1}%", volume * 100.0);
    }

    /// Gets the current volume (0.0 to 1.0)
    #[must_use]
    pub const fn get_volume(&self) -> f32 {
        self.volume
    }

    /// Checks if the player is currently playing
    #[must_use]
    pub fn is_playing(&self) -> bool {
        matches!(self.get_state(), PlayerState::Playing)
    }

    /// Checks if the player is paused
    #[must_use]
    pub fn is_paused(&self) -> bool {
        matches!(self.get_state(), PlayerState::Paused)
    }

    /// Checks if the player is stopped
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        matches!(self.get_state(), PlayerState::Stopped)
    }

    /// Gets the currently playing file path
    #[must_use]
    pub fn get_current_file(&self) -> Option<PathBuf> {
        self.current_file.clone()
    }

    /// Checks if the sink is empty (finished playing)
    pub fn is_finished(&self) -> bool {
        self.sink.as_ref().is_none_or(|s| s.empty())
    }

    /// Updates the player state based on sink status
    pub fn update_state(&mut self) {
        if let Some(sink) = self.sink.as_ref()
            && sink.empty()
        {
            self.state = PlayerState::Stopped;
        }
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        // Create a new instance with default values
        // This matches the field types in the struct definition exactly
        AudioPlayer {
            sink: None,                  // Option<Sink>
            state: PlayerState::Stopped, // PlayerState
            volume: 0.7,                 // f32
            current_file: None,          // Option<PathBuf>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_player_creation() {
        let player = AudioPlayer::new();
        assert!(player.is_ok());
    }

    #[test]
    fn test_initial_state() {
        let player = AudioPlayer::new().unwrap();
        assert_eq!(player.get_state(), PlayerState::Stopped);
        assert!(!player.is_playing());
        assert!(!player.is_paused());
        assert!(player.is_stopped());
        assert_eq!(player.get_volume(), 0.7);
        assert!(player.get_current_file().is_none());
    }

    #[test]
    fn test_volume_control() {
        let mut player = AudioPlayer::new().unwrap();

        player.set_volume(0.5);
        assert_eq!(player.get_volume(), 0.5);

        // Test clamping
        player.set_volume(1.5);
        assert_eq!(player.get_volume(), 1.0);

        player.set_volume(-0.5);
        assert_eq!(player.get_volume(), 0.0);
    }
}
