//! Core application logic and state management for ABOP

mod message;
pub mod state;
mod update;

pub use message::Message;
pub use state::AppState;
pub use update::update;

/// Initializes the application state
#[must_use]
pub fn init() -> AppState {
    AppState::default()
}
