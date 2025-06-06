//! Main application module
//!
//! This module contains the main application state and initialization logic.

use iced::{Element, Subscription, Task, keyboard};

use crate::handlers;
use crate::messages::Message;
use crate::state::UiState;
use crate::update::update;
use crate::views;

/// Main application struct
#[derive(Debug)]
pub struct App {
    /// Current application state
    pub state: UiState,
}

impl App {
    /// Creates a new application instance
    pub fn new() -> Self {
        Self {
            state: UiState::default(),
        }
    }

    /// Updates application state in response to messages
    pub fn update_state(&mut self, message: Message) -> Task<Message> {
        handlers::handle_message(&mut self.state, message)
    }

    /// Returns the current view of the application
    pub fn render(&self) -> iced::Element<'_, Message> {
        views::view(&self.state)
    }

    /// Returns the application theme
    pub fn get_theme(&self) -> iced::Theme {
        self.state.theme_mode.theme()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Static update function to be used with iced's `application()` function
    pub fn update(app: &mut Self, message: Message) -> Task<Message> {
        update(&mut app.state, message)
    }

    /// Static view function to be used with iced's `application()` function
    #[must_use]
    pub fn view(app: &Self) -> Element<'_, Message> {
        views::view(&app.state)
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
}
