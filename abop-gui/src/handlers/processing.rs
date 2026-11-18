//! Audio processing message handlers (convert selected audiobooks to mono)

use iced::Task;

use crate::messages::{Command, Message};
use crate::state::AppState;

/// Handles processing messages; returns `None` for anything else
pub fn handle_processing_message(state: &mut AppState, message: &Message) -> Option<Task<Message>> {
    let task = match message {
        Message::ProcessSelected => {
            if state.player.processing_audio {
                state.ui.notify_info("Audio processing is already running");
            } else if state.library.selected_audiobooks.is_empty() {
                state.ui.notify_info("Select audiobooks to process");
            } else {
                state.ui.confirm_process = true;
            }
            Task::none()
        }
        Message::CancelProcess => {
            state.ui.confirm_process = false;
            Task::none()
        }
        Message::ConfirmProcess => {
            state.ui.confirm_process = false;
            let selected_ids = state.library.selected_audiobooks.iter().cloned().collect();
            Task::done(Message::command(Command::ConvertToMono {
                selected_ids,
                audiobooks: state.library.audiobooks.clone(),
            }))
        }
        Message::ProcessingProgress(progress, status) => {
            state
                .player
                .update_processing_progress(*progress, Some(status.clone()));
            Task::none()
        }
        Message::AudioProcessingComplete(result) => {
            state.player.complete_processing();
            match result {
                Ok(summary) => state.ui.notify_info(summary.clone()),
                Err(error) => state.ui.notify_error(error.clone()),
            }
            Task::none()
        }
        _ => return None,
    };
    Some(task)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::NoticeKind;

    #[test]
    fn process_requires_selection_then_confirms() {
        let mut state = AppState::default();
        let _ = handle_processing_message(&mut state, &Message::ProcessSelected);
        assert!(!state.ui.confirm_process);
        assert!(state.ui.notice.is_some());

        state.library.selected_audiobooks.insert("b1".into());
        let _ = handle_processing_message(&mut state, &Message::ProcessSelected);
        assert!(state.ui.confirm_process);

        let _ = handle_processing_message(&mut state, &Message::CancelProcess);
        assert!(!state.ui.confirm_process);
    }

    #[test]
    fn does_not_start_twice() {
        let mut state = AppState::default();
        state.library.selected_audiobooks.insert("b1".into());
        state.player.start_processing(None);
        let _ = handle_processing_message(&mut state, &Message::ProcessSelected);
        assert!(!state.ui.confirm_process);
    }

    #[test]
    fn progress_and_completion_update_state() {
        let mut state = AppState::default();
        state.player.start_processing(None);
        let _ = handle_processing_message(
            &mut state,
            &Message::ProcessingProgress(0.5, "Converting “A” (1/2)".into()),
        );
        assert_eq!(state.player.processing_progress, Some(0.5));
        assert_eq!(
            state.player.processing_status.as_deref(),
            Some("Converting “A” (1/2)")
        );

        let _ = handle_processing_message(
            &mut state,
            &Message::AudioProcessingComplete(Err("Conversion failed".into())),
        );
        assert!(!state.player.processing_audio);
        assert_eq!(
            state.ui.notice.as_ref().map(|n| n.kind),
            Some(NoticeKind::Error)
        );
    }
}
