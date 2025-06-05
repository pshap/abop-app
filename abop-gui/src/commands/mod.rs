//! Command handling module
//!
//! This module coordinates the execution of asynchronous commands by routing them
//! to the appropriate specialized command handlers.

pub mod audio;
pub mod library;

use iced::Task;

use crate::messages::{Command, Message};
use crate::state::UiState;

/// Routes commands to appropriate specialized handlers
pub fn handle_command(state: &mut UiState, command: Command) -> Task<Message> {
    // Try audio commands first
    if let Some(task) = audio::handle_audio_command(state, command.clone()) {
        return task;
    }

    // Try library commands
    if let Some(task) = library::handle_library_command(state, command.clone()) {
        return task;
    }

    // If no handler matches, log and return empty task
    log::warn!("No handler found for command: {command:?}");
    Task::none()
}
