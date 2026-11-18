//! Main application module
//!
//! This module contains the main application state and initialization logic.

use iced::{self, Element, Subscription, Task, keyboard, theme::Theme as IcedTheme};

// Import ThemeMode from the appropriate module

use log::{error, info};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::preferences::GuiPreferences;
use crate::{handlers, messages::Message, router, state::AppState, views};
use abop_core::services::ServiceContainer;

/// Messages for task management
#[derive(Debug)]
enum TaskMessage {
    /// Cancel a task by ID
    Cancel(u64),
}

/// Main application struct
#[derive(Debug)]
pub struct App {
    /// Current application state
    state: AppState,
    /// Application router for view navigation
    router: router::Router,
    /// Service container for managing services and background tasks
    services: Arc<ServiceContainer>,
    /// Sender for task management messages
    task_tx: Option<UnboundedSender<TaskMessage>>,
    /// Where preferences are saved; `None` disables persistence (e.g. in tests)
    preferences_path: Option<std::path::PathBuf>,
    /// Preferences as last written, to save only when they change
    saved_preferences: Option<GuiPreferences>,
}

impl App {
    /// Create a new application instance with default settings
    pub fn new() -> Self {
        let services = Arc::new(ServiceContainer::new());

        // Set up task management channel
        let (task_tx, mut task_rx) = mpsc::unbounded_channel();

        // Clone services for the task manager
        let services_clone = services.clone();

        // Spawn the task manager
        tokio::spawn(async move {
            while let Some(message) = task_rx.recv().await {
                match message {
                    TaskMessage::Cancel(task_id) => {
                        if let Err(e) = services_clone.cancel_task(task_id) {
                            error!("Failed to cancel task {task_id}: {e}");
                        }
                    }
                }
            }

            // Clean up all tasks when the manager exits
            if let Err(e) = services_clone.cancel_all_tasks() {
                error!("Failed to cancel all tasks during cleanup: {e}");
            }
        });

        Self {
            state: AppState::default(),
            router: router::Router::new(),
            services,
            task_tx: Some(task_tx),
            preferences_path: None,
            saved_preferences: None,
        }
    }

    /// Spawn a new background task
    pub fn spawn_task<F, Fut>(&self, name: impl Into<String>, task: F) -> Option<u64>
    where
        F: FnOnce(&ServiceContainer) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let name = name.into();
        let services = Arc::clone(&self.services);

        // Simplified task wrapper - directly return the future result
        let wrapped_task = move || async move {
            task(&services).await;
            Ok(())
        };

        // Directly use ServiceContainer::spawn_named to get the task ID synchronously
        match self.services.spawn_named(name, wrapped_task) {
            Ok(task_id) => Some(task_id),
            Err(e) => {
                error!("Failed to spawn task: {e}");
                None
            }
        }
    }

    /// Cancel a running task
    pub fn cancel_task(&self, task_id: u64) -> bool {
        if let Some(tx) = &self.task_tx {
            match tx.send(TaskMessage::Cancel(task_id)) {
                Ok(()) => true,
                Err(e) => {
                    error!("Failed to send cancel task: {e}");
                    false
                }
            }
        } else {
            error!("Task manager not initialized");
            false
        }
    }

    /// Get a reference to the service container
    pub fn services(&self) -> &Arc<ServiceContainer> {
        &self.services
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // Close the task channel to signal the task manager to shut down
        drop(self.task_tx.take());

        // Log application shutdown
        info!("Application shutting down");
    }
}

// Theme is defined in `crate::theme`; avoid duplicating here.
// Removed duplicate Theme enum - use crate::theme::ThemeMode instead.

impl App {
    /// Initialize the application with default settings
    pub fn initial() -> (Self, Task<Message>) {
        Self::with_library(None)
    }

    /// Initialize the application, optionally opening and scanning a library folder
    ///
    /// Restores saved preferences. A folder passed on the command line is scanned;
    /// otherwise the last open library is reloaded from the database.
    pub fn with_library(library_path: Option<std::path::PathBuf>) -> (Self, Task<Message>) {
        let mut app = Self::new();
        let preferences_path = GuiPreferences::default_path();
        let preferences = preferences_path
            .as_deref()
            .map(GuiPreferences::load_from)
            .unwrap_or_default();
        preferences.apply_to(&mut app.state);

        let fonts = crate::assets::register_fonts().map(|_| Message::NoOp);
        let open_library = match (library_path, &preferences.last_library) {
            (Some(path), _) => Task::done(Message::DirectorySelected(Some(path))),
            (None, Some(last)) if last.is_dir() => {
                let last = last.clone();
                Task::perform(
                    crate::commands::library::load_library_from_db(last.clone()),
                    move |result| Message::LibraryLoaded(last.clone(), result),
                )
            }
            (None, _) => Task::none(),
        };

        app.saved_preferences = Some(preferences);
        app.preferences_path = preferences_path;
        (app, Task::batch([fonts, open_library]))
    }

    /// Save preferences if they changed since the last save
    fn persist_preferences(&mut self) {
        let Some(path) = &self.preferences_path else {
            return;
        };
        let current = GuiPreferences::from_state(&self.state);
        if self.saved_preferences.as_ref() == Some(&current) {
            return;
        }
        match current.save_to(path) {
            Ok(()) => log::debug!("Saved preferences to {}", path.display()),
            Err(error) => log::warn!("Could not save preferences to {}: {error}", path.display()),
        }
        // Remember even on failure so a broken config dir doesn't retry every update
        self.saved_preferences = Some(current);
    }

    /// Get the application title
    pub fn title(&self) -> String {
        const APP_NAME: &str = "ABOP — Audiobook Organizer & Processor";
        let player = &self.state.player;
        if player.player_state == abop_core::PlayerState::Stopped {
            return APP_NAME.to_string();
        }
        let playing = player.playing_audiobook_id.as_ref().and_then(|id| {
            self.state
                .library
                .audiobooks
                .iter()
                .find(|book| &book.id == id)
                .and_then(|book| book.title.clone())
        });
        playing.map_or_else(|| APP_NAME.to_string(), |title| format!("{title} — ABOP"))
    }

    /// Update the application state based on messages
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let task = self.handle_message(message);
        self.persist_preferences();
        task
    }

    /// Route a message to the matching handler
    fn handle_message(&mut self, message: Message) -> Task<Message> {
        log::debug!("Processing message: {message:?}");

        match message {
            // Navigation messages
            Message::Navigate(route) => self.router.navigate_to(route),
            Message::NavigateBack => self.router.navigate_back(),

            // Settings open as a modal dialog; handled by the UI state handlers below

            // Escape dismisses the topmost dialog, otherwise stops playback
            Message::Escape => {
                let next = if self.state.ui.renaming_id.is_some() {
                    Message::CancelRename
                } else if self.state.ui.confirm_process {
                    Message::CancelProcess
                } else if self.state.ui.settings_open {
                    Message::CloseSettings
                } else {
                    Message::Stop
                };
                self.handle_message(next)
            }

            // Handle command execution
            Message::ExecuteCommand(command) => {
                log::info!("Executing command: {command:?}");
                // Use the command handler infrastructure
                crate::commands::handle_command(&mut self.state, command)
            }

            Message::FocusSearch => {
                iced::widget::text_input::focus(crate::views::library::search_input_id())
            }

            // No operation
            Message::NoOp => Task::none(),

            // Delegate other messages to appropriate handlers
            _ => {
                if let Some(task) =
                    handlers::playback::handle_playback_message(&mut self.state, &message)
                {
                    return task;
                }
                if let Some(task) =
                    handlers::processing::handle_processing_message(&mut self.state, &message)
                {
                    return task;
                }

                // Try UI state handlers first
                if let Some(task) =
                    handlers::ui_state::handle_ui_message(&mut self.state, message.clone())
                {
                    return task;
                }

                // Then try data update handlers
                if let Some(task) =
                    handlers::data_updates::handle_gui_message(&mut self.state, message.clone())
                {
                    return task;
                }

                // Finally, try core operations
                handlers::data_updates::handle_core_operation(&mut self.state, message)
            }
        }
    }

    /// Create the view for the application
    pub fn view(&self) -> Element<'_, Message> {
        // Get the current route to determine which view to show
        let current_route = self.router.current_route();

        // Use the unified view function that includes toolbar and handles routing
        views::view(&self.state, current_route)
    }

    /// Get the theme for the application
    pub fn theme(&self) -> IcedTheme {
        // Resolve theme from UI state's ThemeMode
        self.state.ui.theme_mode.theme()
    }

    /// Create subscriptions for the application
    pub fn subscription(&self) -> Subscription<Message> {
        use keyboard::key::Key;

        // Poll the player while playing so the UI notices when a book ends
        let player_tick = matches!(
            self.state.player.player_state,
            abop_core::PlayerState::Playing
        )
        .then(|| {
            iced::time::every(std::time::Duration::from_millis(500)).map(|_| Message::PlayerTick)
        });

        let keyboard = iced::event::listen_with(|event, status, _window| {
            let iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event
            else {
                return None;
            };

            // Application shortcuts work everywhere
            if modifiers.command() {
                return match key.as_ref() {
                    Key::Character("q") => Some(Message::command(crate::messages::Command::Quit)),
                    Key::Character("o") => {
                        Some(Message::command(crate::messages::Command::BrowseDirectory))
                    }
                    Key::Character(",") => Some(Message::ShowSettings),
                    Key::Character("f") => Some(Message::FocusSearch),
                    _ => None,
                };
            }

            if matches!(key.as_ref(), Key::Named(keyboard::key::Named::Escape)) {
                return Some(Message::Escape);
            }

            // Playback shortcuts must not steal keys from focused widgets (e.g. search input)
            if status == iced::event::Status::Captured {
                return None;
            }
            match key.as_ref() {
                Key::Named(keyboard::key::Named::Space) => Some(Message::PlayPause),
                Key::Named(keyboard::key::Named::ArrowRight) => Some(Message::Next),
                Key::Named(keyboard::key::Named::ArrowLeft) => Some(Message::Previous),
                _ => None,
            }
        });

        let close_requests = iced::window::close_requests()
            .map(|_| Message::command(crate::messages::Command::Quit));

        Subscription::batch([keyboard, close_requests].into_iter().chain(player_tick))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn title_shows_playing_audiobook() {
        let mut app = App::new();
        assert!(app.title().starts_with("ABOP"));

        let mut book = abop_core::models::Audiobook::new("lib", "/books/martian.m4b");
        book.title = Some("The Martian".into());
        app.state.player.playing_audiobook_id = Some(book.id.clone());
        app.state.library.audiobooks.push(book);
        app.state.player.player_state = abop_core::PlayerState::Playing;
        assert_eq!(app.title(), "The Martian — ABOP");
    }
}
