//! Main application module
//!
//! This module contains the main application state and initialization logic.

use iced::{self, Element, Subscription, Task, keyboard, theme::Theme as IcedTheme};

// Import ThemeMode from the appropriate module

use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::{
    handlers,
    messages::Message,
    router::{self, Route},
    state::UiState,
    views,
};
use abop_core::services::ServiceContainer;

/// Messages for task management
#[derive(Debug)]
enum TaskMessage {
    /// Cancel a task by ID
    Cancel(u64),
    /// Request status of a task by ID
    Status(u64),
    /// Shutdown the task manager
    Shutdown,
}

/// Main application struct
#[derive(Debug)]
pub struct App {
    /// Current application state
    state: UiState,
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
                    TaskMessage::Status(task_id) => {
                        // Log task status - could be extended to return status via channel
                        debug!("Task status requested for task {task_id}");
                    }
                    TaskMessage::Shutdown => {
                        info!("Task manager received shutdown signal");
                        break;
                    }
                }
            }

            // Clean up all tasks when the manager exits
            if let Err(e) = services_clone.cancel_all_tasks() {
                error!("Failed to cancel all tasks during cleanup: {e}");
            }
        });

        Self {
            state: UiState::default(),
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

/// Application theme options
/// 
/// Defines the available visual themes for the application's user interface.
#[derive(Debug, Clone, Copy, Default)]
pub enum Theme {
    /// Light color scheme with dark text on light background
    #[default]
    Light,
    
    /// Dark color scheme with light text on dark background
    Dark,
}

impl App {
    /// Initialize the application with default settings
    pub fn initial() -> (Self, Task<Message>) {
        // Initialize the application with default state and router
        let app = Self::new();

        // Return the app with a no-op task
        (app, Task::none())
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

            // Settings messages
            Message::ShowSettings => self.router.navigate_to(Route::Settings),
            Message::CloseSettings => self.router.navigate_back(),

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
        // Create a default theme based on the current theme mode
        IcedTheme::default()
    }

    /// Create subscriptions for the application
    pub fn subscription(&self) -> Subscription<Message> {
        use keyboard::key::Key;

        iced::event::listen_with(|event, _status, _window| {
            if let iced::Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event {
                match key.as_ref() {
                    Key::Named(keyboard::key::Named::Space) => Some(Message::PlayPause),
                    Key::Named(keyboard::key::Named::ArrowRight) => Some(Message::Next),
                    Key::Named(keyboard::key::Named::ArrowLeft) => Some(Message::Previous),
                    Key::Named(keyboard::key::Named::Escape) => Some(Message::Stop),
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    /// Runs the application with the provided settings
    ///
    /// Note: This method is no longer used in iced 0.13.x.
    /// The application is now started using iced::application() in main.rs
    #[deprecated(note = "Use iced::application() in main.rs instead")]
    pub fn run(_settings: iced::Settings) -> iced::Result {
        // This method is no longer used with iced 0.13.x
        log::warn!("App::run is deprecated. Using iced::application() instead.");
        Ok(())
    }
}
