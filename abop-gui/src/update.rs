//! Update logic for the application

use iced::Task;

use crate::handlers;
use crate::messages::Message;
use crate::state::UiState;

/// Update function that handles messages and updates application state
pub fn update(state: &mut UiState, message: Message) -> Task<Message> {
    handlers::handle_message(state, message)
}
