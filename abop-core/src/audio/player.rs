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
pub struct AudioPlayer {
    /// Current audio sink
    sink: Arc<Mutex<Option<Sink>>>,
    /// Current player state
    state: Arc<Mutex<PlayerState>>,
    /// Current volume (0.0 to 1.0)
    volume: Arc<Mutex<f32>>,
    /// Current playing file path
    current_file: Arc<Mutex<Option<PathBuf>>>,
}

// Implement Send + Sync for AudioPlayer to make it usable in static contexts
// This is safe because all fields are wrapped in Arc<Mutex<T>> which are Send + Sync
unsafe impl Send for AudioPlayer {}
unsafe impl Sync for AudioPlayer {}

impl AudioPlayer {
    /// Creates a new audio player
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if audio system initialization fails.
    pub fn new() -> Result<Self> {
        Ok(Self {
            sink: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(PlayerState::Stopped)),
            volume: Arc::new(Mutex::new(0.7)), // Default volume 70%
            current_file: Arc::new(Mutex::new(None)),
        })
    }

    /// Plays an audio file
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if the audio output stream cannot be created,
    /// the file format is unsupported, or the audio sink creation fails.
    /// Returns [`AppError::Io`] if the file cannot be read.
    pub fn play<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        let file_path = file_path.as_ref();

        // Stop any currently playing audio
        self.stop();

        // Create output stream for this playback session
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AppError::Audio(format!("Failed to create audio output stream: {e}")))?;

        // Open the audio file
        let file = File::open(file_path).map_err(AppError::Io)?;

        let buf_reader = BufReader::new(file);

        // Create decoder
        let source = Decoder::new(buf_reader).map_err(|e| {
            AppError::Audio(format!(
                "Failed to decode audio file '{}': {}",
                file_path.display(),
                e
            ))
        })?;

        // Create new sink
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| AppError::Audio(format!("Failed to create audio sink: {e}")))?;

        // Set volume
        if let Ok(volume) = self.volume.lock() {
            sink.set_volume(*volume);
        }

        // Append the source and play
        sink.append(source);
        sink.play();

        // Update state
        if let Ok(mut sink_opt) = self.sink.lock() {
            if let Some(sink) = sink_opt.take() {
                *sink_opt = Some(sink);
            }
        }
        if let Ok(mut state) = self.state.lock() {
            *state = PlayerState::Playing;
        }
        if let Ok(mut current_file) = self.current_file.lock() {
            *current_file = Some(file_path.to_path_buf());
        }

        log::info!("Started playing audio file: {}", file_path.display());
        Ok(())
    }

    /// Stops audio playback
    pub fn stop(&self) {
        if let Ok(mut sink_opt) = self.sink.lock() {
            if let Some(sink) = sink_opt.take() {
                sink.stop();
                log::info!("Stopped audio playback");
            }
        }

        if let Ok(mut state) = self.state.lock() {
            *state = PlayerState::Stopped;
        }
        if let Ok(mut current_file) = self.current_file.lock() {
            *current_file = None;
        }
    }
    /// Pauses audio playback
    pub fn pause(&self) {
        if let Ok(sink_opt) = self.sink.lock() {
            if let Some(ref sink) = *sink_opt {
                sink.pause();
                if let Ok(mut state) = self.state.lock() {
                    *state = PlayerState::Paused;
                }
                log::info!("Paused audio playback");
            }
        }
    }

    /// Resumes audio playback
    pub fn resume(&self) {
        if let Ok(sink_opt) = self.sink.lock() {
            if let Some(ref sink) = *sink_opt {
                sink.play();
                if let Ok(mut state) = self.state.lock() {
                    *state = PlayerState::Playing;
                }
                log::info!("Resumed audio playback");
            }
        }
    }

    /// Gets the current player state
    #[must_use]
    pub fn get_state(&self) -> PlayerState {
        self.state
            .lock()
            .map(|state| state.clone())
            .unwrap_or_default()
    }

    /// Sets the volume (0.0 to 1.0)
    pub fn set_volume(&self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        if let Ok(mut vol) = self.volume.lock() {
            *vol = volume;
        }

        if let Ok(sink_opt) = self.sink.lock() {
            if let Some(ref sink) = *sink_opt {
                sink.set_volume(volume);
            }
        }
        log::debug!("Set volume to {:.1}%", volume * 100.0);
    }

    /// Gets the current volume (0.0 to 1.0)
    #[must_use]
    pub fn get_volume(&self) -> f32 {
        self.volume.lock().map(|vol| *vol).unwrap_or(0.7)
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
        self.current_file.lock().ok().and_then(|file| file.clone())
    }

    /// Checks if the sink is empty (finished playing)
    #[must_use]
    pub fn is_finished(&self) -> bool {
        if let Ok(sink_opt) = self.sink.lock() {
            if let Some(ref sink) = *sink_opt {
                return sink.empty();
            }
        }
        true
    }

    /// Updates the player state based on sink status
    pub fn update_state(&self) {
        if self.is_finished() && self.get_state() == PlayerState::Playing {
            if let Ok(mut state) = self.state.lock() {
                *state = PlayerState::Stopped;
            }
            if let Ok(mut current_file) = self.current_file.lock() {
                *current_file = None;
            }
            log::info!("Audio playback finished");
        }
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            sink: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(PlayerState::Stopped)),
            volume: Arc::new(Mutex::new(0.7)),
            current_file: Arc::new(Mutex::new(None)),
        })
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
        let player = AudioPlayer::new().unwrap();

        player.set_volume(0.5);
        assert_eq!(player.get_volume(), 0.5);

        // Test clamping
        player.set_volume(1.5);
        assert_eq!(player.get_volume(), 1.0);

        player.set_volume(-0.5);
        assert_eq!(player.get_volume(), 0.0);
    }
}
