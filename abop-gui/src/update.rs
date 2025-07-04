//! Update logic for the application
//!
//! This module contains the update logic for the application, handling state transitions
//! in response to messages.

use iced::Task;

use crate::handlers;
use crate::messages::Message;
use crate::state::UiState;

/// Updates application state in response to messages
pub fn update(state: &mut UiState, message: Message) -> Task<Message> {
    handlers::handle_message(state, message)
}
