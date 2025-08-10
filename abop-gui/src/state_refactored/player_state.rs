//! Audio player state management
//!
//! This module handles all audio playback related state.

use std::path::PathBuf;
use abop_core::audio::player::PlayerState as CorePlayerState;

/// Audio player state management
#[derive(Debug, Clone)]
pub struct PlayerState {
    /// Current audio player state from core
    pub player_state: CorePlayerState,
    /// Currently playing file path
    pub current_playing_file: Option<PathBuf>,
    /// Whether audio processing is in progress
    pub processing_audio: bool,
    /// Progress of the current audio processing (0.0 to 1.0)
    pub processing_progress: Option<f32>,
    /// Current audio processing status message
    pub processing_status: Option<String>,
    /// Flag to indicate player state needs UI redraw
    pub needs_redraw: bool,
}

impl PlayerState {
    /// Create new player state
    #[must_use]
    pub fn new() -> Self {
        Self {
            player_state: CorePlayerState::Stopped,
            current_playing_file: None,
            processing_audio: false,
            processing_progress: None,
            processing_status: None,
            needs_redraw: false,
        }
    }

    /// Set the current player state
    pub fn set_player_state(&mut self, state: CorePlayerState) {
        if self.player_state != state {
            self.player_state = state;
            self.needs_redraw = true;
        }
    }

    /// Set the currently playing file
    pub fn set_current_playing_file(&mut self, file: Option<PathBuf>) {
        if self.current_playing_file != file {
            self.current_playing_file = file;
            self.needs_redraw = true;
        }
    }

    /// Start audio processing
    pub fn start_processing(&mut self, status: Option<String>) {
        self.processing_audio = true;
        self.processing_progress = Some(0.0);
        self.processing_status = status;
        self.needs_redraw = true;
    }

    /// Update audio processing progress
    pub fn update_processing_progress(&mut self, progress: f32, status: Option<String>) {
        let progress = progress.clamp(0.0, 1.0);
        if self.processing_progress != Some(progress) {
            self.processing_progress = Some(progress);
            self.needs_redraw = true;
        }
        if let Some(new_status) = status
            && self.processing_status.as_ref() != Some(&new_status)
        {
            self.processing_status = Some(new_status);
            self.needs_redraw = true;
        }
    }

    /// Complete audio processing
    pub fn complete_processing(&mut self) {
        if self.processing_audio {
            self.processing_audio = false;
            self.processing_progress = None;
            self.processing_status = None;
            self.needs_redraw = true;
        }
    }

    /// Cancel audio processing
    pub fn cancel_processing(&mut self) {
        if self.processing_audio {
            self.processing_audio = false;
            self.processing_progress = None;
            self.processing_status = Some("Processing cancelled".to_string());
            self.needs_redraw = true;
        }
    }

    /// Check if audio is currently being processed
    pub fn is_processing(&self) -> bool {
        self.processing_audio
    }

    /// Check if player is currently playing
    pub fn is_playing(&self) -> bool {
        matches!(self.player_state, CorePlayerState::Playing)
    }

    /// Check if player is paused
    pub fn is_paused(&self) -> bool {
        matches!(self.player_state, CorePlayerState::Paused)
    }

    /// Check if player is stopped
    pub fn is_stopped(&self) -> bool {
        matches!(self.player_state, CorePlayerState::Stopped)
    }

    /// Check if the player state needs a redraw
    #[must_use]
    pub const fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    /// Clear the redraw flag (typically called after redraw is complete)
    pub fn clear_redraw_flag(&mut self) {
        self.needs_redraw = false;
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}