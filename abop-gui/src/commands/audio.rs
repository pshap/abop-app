//! Audio command handlers for asynchronous audio operations

use iced::Task;
use std::collections::HashSet;

use crate::audio::{convert_selected_to_mono, play_selected_audio, stop_audio};
use crate::messages::{Command as GuiCommand, Message};
use crate::state::UiState;

/// Handles audio-related commands
#[must_use]
pub fn handle_audio_command(state: &mut UiState, command: GuiCommand) -> Option<Task<Message>> {
    match command {
        GuiCommand::ConvertToMono {
            selected_ids,
            audiobooks,
        } => {
            state.processing_audio = true;
            state.processing_status = Some("Converting selected audiobooks to mono...".to_string());
            state.processing_progress = Some(0.0);
            // Update cache for processing progress
            state.last_processing_progress = Some(0.0);
            state.cached_processing_progress_text = Some("0.0%".to_string());
            log::info!(
                "Executing ConvertToMono command for {} audiobooks",
                selected_ids.len()
            );
            let selected_set: HashSet<String> = selected_ids.into_iter().collect();
            Some(Task::perform(
                convert_selected_to_mono(selected_set, audiobooks),
                Message::AudioProcessingComplete,
            ))
        }
        GuiCommand::PlayAudio {
            selected_ids,
            audiobooks,
        } => {
            log::info!(
                "Executing PlayAudio command for {} audiobooks",
                selected_ids.len()
            );
            Some(Task::perform(
                play_selected_audio(selected_ids, audiobooks),
                Message::PlaybackStarted,
            ))
        }
        GuiCommand::StopAudio => {
            log::info!("Executing StopAudio command");
            stop_audio();
            // Return a task that will trigger the PlaybackStopped message
            Some(Task::perform(async {}, |()| Message::PlaybackStopped))
        }
        _ => None, // Not an audio command
    }
}
