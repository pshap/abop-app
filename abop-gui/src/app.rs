//! Main application module that ties together all the pieces

use iced::{Subscription, Task, keyboard};
use crate::messages::Message;
use crate::state::UiState;
use crate::update::update;
use crate::views::view;

/// The application's entry point and interface with the Iced runtime
pub struct App {
    /// The current GUI application state, including all user and view data
    pub state: UiState,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Creates a new application with initial state
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: UiState::default(),
        }
    }

    /// Static update function to be used with iced's `application()` function
    pub fn update(app: &mut Self, message: Message) -> Task<Message> {
        update(&mut app.state, message)
    }

    /// Static view function to be used with iced's `application()` function
    #[must_use]
    pub fn view(app: &Self) -> iced::Element<Message> {
        view(&app.state)
    }

    /// Static subscription function to handle keyboard events
    pub fn subscription(app: &Self) -> Subscription<Message> {
        // Only listen for escape key when settings dialog is open
        if app.state.settings_open {
            keyboard::on_key_press(|key, _modifiers| match key {
                keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::CloseSettings),
                _ => None,
            })
        } else {
            Subscription::none()
        }
    }

    /// Instance method for updating state
    pub fn update_state(&mut self, message: Message) -> Task<Message> {
        update(&mut self.state, message)
    }

    /// Instance method for rendering UI
    #[must_use]
    pub fn render(&self) -> iced::Element<Message> {
        view(&self.state)
    }
}
