//! Audio command handlers for asynchronous audio operations

use crate::audio::processing::{ProcessingEvent, convert_to_mono};
use crate::audio::{play_selected_audio, stop_audio};
use crate::messages::{Command as GuiCommand, Message};
use crate::state::AppState;
use iced::Task;

/// Handles audio-related commands
#[must_use]
pub fn handle_audio_command(state: &mut AppState, command: GuiCommand) -> Option<Task<Message>> {
    match command {
        GuiCommand::ConvertToMono {
            selected_ids,
            audiobooks,
        } => {
            if state.player.processing_audio {
                state.ui.notify_info("Audio processing is already running");
                return Some(Task::none());
            }
            let books: Vec<_> = audiobooks
                .into_iter()
                .filter(|book| selected_ids.contains(&book.id))
                .collect();
            log::info!("Converting {} audiobooks to mono", books.len());
            state
                .player
                .start_processing(Some("Preparing conversion…".to_string()));
            state.progress_cache.clear_processing_cache();
            Some(Task::run(convert_to_mono(books), |event| match event {
                ProcessingEvent::Progress(progress, status) => {
                    Message::ProcessingProgress(progress, status)
                }
                ProcessingEvent::Finished(result) => Message::AudioProcessingComplete(result),
            }))
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
            // Return a task that will trigger a Stop message to unify stop handling
            Some(Task::perform(async {}, |()| Message::Stop))
        }
        _ => None, // Not an audio command
    }
}
