//! Unified message/event system for ABOP

use crate::ViewType;
use serde::{Deserialize, Serialize};

/// Core application messages for state updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppMessage {
    /// Application view was changed
    ViewChanged(ViewType),
    /// UI button was pressed with identifier
    ButtonPressed(String),
    /// Data loading operation completed
    DataLoaded(String),
    /// Data saving operation completed
    DataSaved,
    /// An error occurred during operation
    Error(String),
    /// Application configuration was modified
    ConfigChanged,
    /// Audio playback operation result
    PlaybackStarted(Result<String, String>),
    /// Audio-related message
    Audio(AudioMessage),
}

/// Audio processing options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioProcessingOption {
    /// Convert stereo audio to mono
    StereoToMono,
    /// Remove noise from audio
    NoiseRemoval,
    /// Normalize audio volume
    Normalization,
    /// Split audio file into multiple parts
    Split,
    /// Merge multiple audio files
    Merge,
}

/// Audio-specific messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioMessage {
    /// Process selected audio files
    ProcessAudio,
    /// Select a processing option
    SelectProcessingOption(AudioProcessingOption),
    /// Audio processing completed successfully
    AudioProcessingComplete,
    /// Audio processing encountered an error
    AudioError(String),
    /// Cancel audio processing
    Cancel,
}
