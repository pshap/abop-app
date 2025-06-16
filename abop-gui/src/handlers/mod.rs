//! Message handler coordination module
//!
//! This module coordinates message handling by routing messages to the appropriate
//! specialized handlers based on message type and category.

pub mod data_updates;
pub mod ui_state;

#[cfg(test)]
mod tests;

use iced::Task;

use crate::commands;
use crate::messages::Message;
use crate::state::UiState;

/// Routes messages to appropriate specialized handlers
pub fn handle_message(state: &mut UiState, message: Message) -> Task<Message> {
    // Handle command execution messages
    if let Message::ExecuteCommand(command) = message {
        return commands::handle_command(state, command);
    }

    // Try UI state handlers first
    if let Some(task) = ui_state::handle_ui_message(state, message.clone()) {
        return task;
    }

    // Try GUI message handlers
    if let Some(task) = data_updates::handle_gui_message(state, message.clone()) {
        return task;
    }

    // Handle core operations
    data_updates::handle_core_operation(state, message)
}
