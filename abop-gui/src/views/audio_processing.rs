//! Audio processing view module

use iced::Length;
use iced::widget::{column, container};

use crate::components::audio_controls::AudioControls;
use crate::components::status::StatusDisplay;
use crate::messages::Message;
use crate::state::UiState;
use crate::styling::container::LayoutContainerStyles;

/// Creates the audio processing view with conversion and playback controls
#[must_use]
pub fn audio_processing_view(state: &UiState) -> iced::Element<'_, Message> {
    // Use the StatusDisplay component for processing status
    let status_display = StatusDisplay::view(
        state.scanning,
        state.scan_progress,
        state.processing_audio,
        state.processing_progress,
        state.processing_status.as_deref(),
        &state.player_state,
        state.current_playing_file.as_ref(),
        state.selected_audiobooks.len(),
        state.audiobooks.len(),
        state.theme_mode,
        &state.material_tokens,
    );

    // Use the AudioControls component for audio processing controls
    let audio_controls = AudioControls::view(
        &state.selected_audiobooks,
        &state.audiobooks,
        state.player_state.clone(),
        &state.material_tokens,
    );
    // Combine components into the audio mixdown view with consistent spacing
    let content =
        column![status_display, audio_controls].spacing(state.material_tokens.spacing().md);
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(LayoutContainerStyles::content(state.theme_mode))
        .padding(state.material_tokens.spacing().md)
        .into()
}
