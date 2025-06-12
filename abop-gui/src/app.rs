//! Main application module
//!
//! This module contains the main application state and initialization logic.

use iced::{
    self,
    Element,
    theme::Theme as IcedTheme,
    keyboard,
    Subscription, Task,
};

// Import ThemeMode from the appropriate module
use crate::theme::ThemeMode;

use log::{error, info};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};

use abop_core::services::ServiceContainer;
use crate::{
    handlers,
    messages::Message,
    router::{self, Route},
    state::UiState,
    views,
};

/// Messages for task management
enum TaskMessage {
    /// Spawn a new task
    Spawn {
        name: String,
        task: Box<dyn FnOnce(&ServiceContainer) -> tokio::task::JoinHandle<()> + Send + 'static>,
    },
    /// Cancel a task by ID
    Cancel(u64),
}

impl std::fmt::Debug for TaskMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskMessage::Spawn { name, .. } => f.debug_struct("Spawn").field("name", name).finish(),
            TaskMessage::Cancel(id) => f.debug_tuple("Cancel").field(id).finish(),
        }
    }
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
            let mut task_handles = std::collections::HashMap::new();
            
            while let Some(message) = task_rx.recv().await {
                match message {
                    TaskMessage::Spawn { name, task } => {
                        let task_name = name.clone();
                        let handle = task(&services_clone);
                        
                        if let Ok(task_id) = services_clone.spawn_named(&task_name, || async move {
                            handle.await.ok();
                            Ok(())
                        }) {
                            task_handles.insert(task_id, task_name);
                        } else {
                            error!("Failed to spawn task: {}", name);
                        }
                    }
                    TaskMessage::Cancel(task_id) => {
                        if let Err(e) = services_clone.cancel_task(task_id) {
                            error!("Failed to cancel task {}: {}", task_id, e);
                        } else {
                            task_handles.remove(&task_id);
                        }
                    }
                }
            }
            
            // Clean up all tasks when the manager exits
            if let Err(e) = services_clone.cancel_all_tasks() {
                error!("Failed to cancel all tasks during cleanup: {}", e);
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
        let task = move |services: &ServiceContainer| {
            let future = task(services);
            tokio::spawn(future)
        };
        
        if let Some(tx) = &self.task_tx {
            match tx.send(TaskMessage::Spawn { name, task: Box::new(task) }) {
                Ok(()) => None, // Task ID will be handled by the task manager
                Err(e) => {
                    error!("Failed to send spawn task: {}", e);
                    None
                }
            }
        } else {
            error!("Task manager not initialized");
            None
        }
    }
    
    /// Cancel a running task
    pub fn cancel_task(&self, task_id: u64) -> bool {
        if let Some(tx) = &self.task_tx {
            match tx.send(TaskMessage::Cancel(task_id)) {
                Ok(()) => true,
                Err(e) => {
                    error!("Failed to send cancel task: {}", e);
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

#[derive(Debug, Clone, Copy, Default)]
pub enum Theme {
    #[default]
    Light,
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
        log::debug!("Processing message: {:?}", message);

        match message {
            // Navigation messages
            Message::Navigate(route) => {
                self.router.navigate_to(route);
                Task::none()
            },
            Message::NavigateBack => {
                self.router.navigate_back();
                Task::none()
            },

            // Settings messages
            Message::ShowSettings => {
                self.router.navigate_to(Route::Settings);
                Task::none()
            },
            Message::CloseSettings => {
                self.router.navigate_back();
                Task::none()
            },
            Message::ToggleTheme => {
                self.state.theme_mode = match self.state.theme_mode {
                    ThemeMode::Light => ThemeMode::Dark,
                    ThemeMode::Dark => ThemeMode::Light,
                    _ => ThemeMode::System, // Default to System if other variants exist
                };
                Task::none()
            },
            Message::ToggleAutoSaveLibrary => {
                self.state.auto_save_library = !self.state.auto_save_library;
                Task::none()
            },
            Message::ToggleScanSubdirectories => {
                self.state.scan_subdirectories = !self.state.scan_subdirectories;
                Task::none()
            },
            Message::SetTheme(theme) => {
                self.state.theme_mode = theme;
                Task::none()
            },

            // Library management messages
            Message::DirectorySelected(path) => {
                if let Some(path) = path {
                    // Create a dummy database and library for now
                    // TODO: Use proper database and library from application state
                    let db = match abop_core::db::Database::open(":memory:") {
                        Ok(db) => db,
                        Err(e) => {
                            error!("Failed to create database: {}", e);
                            return Task::none();
                        }
                    };
                    let library = abop_core::models::Library::new("GUI Library", &path);
                    
                    Task::perform(
                        crate::library::scan_library(db, library),
                        |result| Message::ScanComplete(result.map_err(|e| e.to_string())),
                    )
                } else {
                    Task::none()
                }
            },
            Message::SelectRecentDirectory(path) => {
                // Create a dummy database and library for now
                // TODO: Use proper database and library from application state
                let db = match abop_core::db::Database::open(":memory:") {
                    Ok(db) => db,
                    Err(e) => {
                        error!("Failed to create database: {}", e);
                        return Task::none();
                    }
                };
                let library = abop_core::models::Library::new("GUI Library", &path);
                
                Task::perform(
                    crate::library::scan_library(db, library),
                    |result| Message::ScanComplete(result.map_err(|e| e.to_string())),
                )
            },
            Message::ShowRecentDirectories => {
                // Implementation for showing recent directories
                Task::none()
            },
            Message::QuickScanDirectory(path) => {
                // TODO: Implement quick scan functionality
                error!("Quick scan directory not yet implemented for: {:?}", path);
                Task::none()
            },
            Message::QuickScanComplete(result) => {
                match result {
                    Ok(directory_info) => {
                        // Handle successful scan
                        info!("Quick scan complete: {:?}", directory_info);
                    }
                    Err(e) => {
                        error!("Quick scan failed: {}", e);
                    }
                }
                Task::none()
            },
            Message::ScanComplete(result) => {
                match result {
                    Ok(scan_result) => {
                        // Handle successful scan
                        info!("Scan complete: {:?} audiobooks found", scan_result.audiobooks.len());
                    }
                    Err(e) => {
                        error!("Scan failed: {}", e);
                    }
                }
                Task::none()
            },
            Message::ScanProgress(progress) => {
                // Update progress in the UI state
                self.state.scan_progress = Some(progress);
                Task::none()
            },
            Message::ScanProgressEnhanced(progress) => {
                // Handle enhanced progress updates
                self.state.enhanced_scan_progress = Some(progress);
                Task::none()
            },

            // Audiobook selection messages
            Message::SelectAudiobook(id) => {
                // Store the selected audiobook ID in core state
                self.state.core_state.selected_audiobook_id = Some(id);
                Task::none()
            },
            Message::ToggleAudiobookSelection(id) => {
                if self.state.selected_audiobooks.contains(&id) {
                    self.state.selected_audiobooks.remove(&id);
                } else {
                    self.state.selected_audiobooks.insert(id);
                }
                Task::none()
            },
            Message::ToggleSelectAll => {
                // Toggle select all audiobooks
                if self.state.selected_audiobooks.len() == self.state.audiobooks.len() {
                    self.state.selected_audiobooks.clear();
                } else {
                    self.state.selected_audiobooks = self.state.audiobooks.iter().map(|ab| ab.id.clone()).collect();
                }
                Task::none()
            },
            Message::DeselectAll => {
                self.state.selected_audiobooks.clear();
                Task::none()
            },
            Message::SortBy(field) => {
                // Sort audiobooks by the specified field
                self.state.table_state.sort_column = field;
                // Trigger a resort of the audiobooks
                crate::utils::sort_audiobooks(&mut self.state);
                Task::none()
            },

            // Playback control messages
            Message::StartPlayback => {
                // Start playback logic
                Task::none()
            },
            Message::StopPlayback => {
                // Stop playback logic
                Task::none()
            },
            Message::PlayPause => {
                // Toggle play/pause logic
                Task::none()
            },
            Message::Previous => {
                // Previous track logic
                Task::none()
            },
            Message::Next => {
                // Next track logic
                Task::none()
            },
            Message::Stop => {
                // Stop playback
                Task::none()
            },

            // Handle command execution
            Message::ExecuteCommand(command) => {
                log::info!("Executing command: {:?}", command);
                // Handle command execution - for now just return none
                // TODO: Implement proper command handling
                Task::none()
            },

            // No operation
            Message::NoOp => Task::none(),

            // Delegate other messages to appropriate handlers
            _ => {
                // Try UI state handlers first
                if let Some(task) = handlers::ui_state::handle_ui_message(&mut self.state, message.clone()) {
                    return task;
                }

                // Then try data update handlers
                if let Some(task) = handlers::data_updates::handle_gui_message(&mut self.state, message.clone()) {
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
        match current_route {
            Route::Library => views::view(&self.state),
            Route::Player => views::audio_processing_view(&self.state),
            Route::Settings => views::settings_view(&self.state),
            Route::About => views::about_view(&self.state),
        }
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
