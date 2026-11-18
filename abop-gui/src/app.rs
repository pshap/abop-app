//! Main application module
//!
//! This module contains the main application state and initialization logic.

use iced::{self, Element, Subscription, Task, keyboard, theme::Theme as IcedTheme};

// Import ThemeMode from the appropriate module

use log::{error, info};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};

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
    pub fn with_library(library_path: Option<std::path::PathBuf>) -> (Self, Task<Message>) {
        let app = Self::new();
        let fonts = crate::assets::register_fonts().map(|_| Message::NoOp);

        let task = match library_path {
            Some(path) => Task::batch([fonts, Task::done(Message::DirectorySelected(Some(path)))]),
            None => fonts,
        };
        (app, task)
    }

    /// Get the application title
    pub fn title(&self) -> String {
        "ABOP - Audiobook Organizer & Processor".to_string()
    }

    /// Update the application state based on messages
    pub fn update(&mut self, message: Message) -> Task<Message> {
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
                self.update(next)
            }

            // Handle command execution
            Message::ExecuteCommand(command) => {
                log::info!("Executing command: {command:?}");
                // Use the command handler infrastructure
                crate::commands::handle_command(&mut self.state, command)
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

        Subscription::batch(std::iter::once(keyboard).chain(player_tick))
    }
}
