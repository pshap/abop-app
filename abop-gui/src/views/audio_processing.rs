//! Audio processing view module

use iced::Length;
use iced::widget::{column, container};

use crate::components::audio_controls::AudioControls;
use crate::components::status::StatusDisplay;
use crate::messages::Message;
use crate::state::AppState;
use crate::styling::container::LayoutContainerStyles;

/// Creates the audio processing view with conversion and playback controls
#[must_use]
pub fn audio_processing_view(state: &AppState) -> iced::Element<'_, Message> {
    // Use the StatusDisplay component for processing status
    let status_display = StatusDisplay::view(
        state.library.scanning,
        state.library.scan_progress,
        state.player.processing_audio,
        state.player.processing_progress,
        state.player.processing_status.as_deref(),
        &state.player.player_state,
        state.player.current_playing_file.as_ref(),
        state.library.selected_audiobooks.len(),
        state.library.audiobooks.len(),
        state.ui.theme_mode,
        &state.ui.material_tokens,
    );

    // Use the AudioControls component for audio processing controls
    let audio_controls = AudioControls::view(
        &state.library.selected_audiobooks,
        &state.library.audiobooks,
        state.player.player_state.clone(),
        &state.ui.material_tokens,
    );
    // Combine components into the audio mixdown view with consistent spacing
    let content =
        column![status_display, audio_controls].spacing(state.ui.material_tokens.spacing().md);
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(LayoutContainerStyles::content(state.ui.theme_mode))
        .padding(state.ui.material_tokens.spacing().md)
        .into()
}
