//! Core AppState implementation with refactored architecture
//!
//! This module contains the main AppState struct with improved separation of concerns,
//! delegating specific responsibilities to specialized components.

use crate::error::Result;
use crate::validation::{StateValidator, ValidationConfig};
use super::{
    data_repository::DataRepository,
    persistence::{SaveOptions, StatePersistence},
    types::*,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Main application state with improved architecture
///
/// This refactored AppState delegates responsibilities to specialized components:
/// - Data operations → DataRepository
/// - Persistence → StatePersistence
/// - Core state → AppState (this struct)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    // Core UI state
    /// Current active view in the application
    pub current_view: ViewType,
    /// ID of the currently selected audiobook, if any
    pub selected_audiobook_id: Option<String>,
    /// User-specific preferences and settings
    pub user_preferences: UserPreferences,
    /// Window size, position, and display configuration
    pub window_config: WindowConfig,
    /// Theme and color scheme configuration
    pub theme_config: ThemeConfig,
    /// Audio playback settings and configuration
    pub playback_config: PlaybackConfig,
    
    // Application data (managed by DataRepository)
    /// Core application data including libraries, audiobooks, and progress
    #[serde(flatten)]
    pub app_data: AppData,
    
    // Internal state
    #[serde(skip)]
    data_repository: Option<DataRepository>,
    #[serde(skip)]
    persistence: Option<StatePersistence>,
    #[serde(skip)]
    validator: Option<StateValidator>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_view: ViewType::default(),
            selected_audiobook_id: None,
            user_preferences: UserPreferences::default(),
            window_config: WindowConfig::default(),
            theme_config: ThemeConfig::default(),
            playback_config: PlaybackConfig::default(),
            app_data: AppData::default(),
            data_repository: None,
            persistence: None,
            validator: None,
        }
    }
}

impl AppState {
    /// Create a new AppState with all dependencies
    pub fn new() -> Self {
        let mut state = Self::default();
        state.initialize_components();
        state
    }    /// Create AppState from a specific state file
    pub fn from_file(state_file: PathBuf) -> Result<Self> {
        let persistence = StatePersistence::with_path(state_file);
        let mut state = persistence.load()?;
        state.initialize_components();
        Ok(state)    }
    
    /// Initialize internal components
    fn initialize_components(&mut self) {
        self.data_repository = Some(DataRepository::new());
        self.persistence = Some(StatePersistence::new().unwrap_or_else(|_| {
            StatePersistence::with_path(PathBuf::from("state.toml"))
        }));
        self.validator = Some(StateValidator::new(ValidationConfig::default()));
        
        debug!("AppState components initialized");
    }
    
    /// Ensure components are initialized (lazy initialization)
    fn ensure_initialized(&mut self) {
        if self.data_repository.is_none() {
            self.initialize_components();
        }
    }
    
    // === Core State Operations ===
    
    /// Set the current view
    pub fn set_current_view(&mut self, view: ViewType) {
        debug!("Changing view from {:?} to {:?}", self.current_view, view);
        self.current_view = view;
    }
    
    /// Set the selected audiobook
    pub fn select_audiobook(&mut self, audiobook_id: Option<String>) {
        debug!("Selected audiobook changed to: {:?}", audiobook_id);
        self.selected_audiobook_id = audiobook_id;
    }
    
    /// Update user preferences
    pub fn update_preferences(&mut self, preferences: UserPreferences) {
        debug!("Updating user preferences");
        self.user_preferences = preferences;
    }
    
    /// Update window configuration
    pub fn update_window_config(&mut self, config: WindowConfig) {
        debug!("Updating window configuration");
        self.window_config = config;
    }
    
    /// Update theme configuration
    pub fn update_theme_config(&mut self, config: ThemeConfig) {
        debug!("Updating theme configuration");
        self.theme_config = config;
    }
    
    /// Update playback configuration
    pub fn update_playback_config(&mut self, config: PlaybackConfig) {
        debug!("Updating playback configuration");
        self.playback_config = config;
    }
      // === Data Operations (delegated to DataRepository) ===
    /// Refresh all data from the repository
    pub fn refresh_data(&mut self) -> Result<()> {
        self.ensure_initialized();
        if let Some(ref repo) = self.data_repository {
            self.app_data.libraries = repo.libraries().to_vec();
            self.app_data.audiobooks = repo.audiobooks().to_vec();
            self.app_data.progress = repo.progress().to_vec();
            info!("All application data refreshed");
        }
        Ok(())
    }
    
    /// Refresh libraries data
    pub fn refresh_libraries(&mut self) -> Result<()> {
        self.ensure_initialized();        if let Some(ref repo) = self.data_repository {
            self.app_data.libraries = repo.libraries().to_vec();
            info!("Libraries data refreshed");
        }
        Ok(())
    }
    
    /// Refresh audiobooks data
    pub fn refresh_audiobooks(&mut self) -> Result<()> {
        self.ensure_initialized();        if let Some(ref repo) = self.data_repository {
            self.app_data.audiobooks = repo.audiobooks().to_vec();
            info!("Audiobooks data refreshed");
        }
        Ok(())
    }
    
    /// Refresh playback progress data
    pub fn refresh_progress(&mut self) -> Result<()> {
        self.ensure_initialized();        if let Some(ref repo) = self.data_repository {
            self.app_data.progress = repo.progress().to_vec();
            info!("Playback progress data refreshed");
        }
        Ok(())
    }
    
    // === Persistence Operations (delegated to StatePersistence) ===
      /// Save state with default options
    pub fn save(&self) -> Result<()> {
        self.save_with_options(SaveOptions::default())
    }
    
    /// Save state with background processing
    pub fn save_async(&self) -> Result<()> {
        let options = SaveOptions {
            blocking: false,
            ..Default::default()
        };
        self.save_with_options(options)
    }
    
    /// Save state with backup
    pub fn save_with_backup(&self) -> Result<()> {
        let options = SaveOptions {
            create_backup: true,
            ..Default::default()
        };
        self.save_with_options(options)
    }
    
    /// Save state with custom options (replaces the three separate save methods)
    pub fn save_with_options(&self, options: SaveOptions) -> Result<()> {        if let Some(ref persistence) = self.persistence {
            persistence.save(self, &options)?;
            debug!("State saved with options: {:?}", options);
        } else {            warn!("Persistence component not initialized, using default");
            let persistence = StatePersistence::new().map_err(|_| {
                crate::error::AppError::Io("Failed to create default persistence".to_string())
            })?;
            persistence.save(self, &options)?;
        }
        Ok(())
    }
      /// Load state from file
    pub fn load(&mut self) -> Result<()> {        if let Some(ref persistence) = self.persistence {
            let loaded_state = persistence.load()?;
            *self = loaded_state;
            self.initialize_components();
            info!("State loaded successfully");
        } else {
            warn!("Persistence component not initialized");
        }
        Ok(())
    }
      // === Validation Operations ===
    /// Validate current state
    pub fn validate(&self) -> Result<()> {
        if let Some(ref validator) = self.validator {
            let validation_result = validator.validate(self);
            if !validation_result.is_valid() {
                return Err(crate::error::AppError::ValidationFailed(format!(
                    "State validation failed with {} issues", 
                    validation_result.issues.len()
                )));
            }
            debug!("State validation passed");
        }
        Ok(())
    }
    
    /// Perform state recovery if needed
    pub fn recover(&mut self) -> Result<()> {
        if let Some(ref validator) = self.validator {
            let validation_result = validator.validate(self);
            if validation_result.has_critical_issues() {
                // For now, just log the issues - recovery logic would go here
                warn!("State has critical issues that need recovery: {}", validation_result);
                // TODO: Implement actual recovery logic
            }
        }
        Ok(())
    }
    
    // === Utility Methods ===
    
    /// Check if the state has been modified since last save
    pub fn is_dirty(&self) -> bool {
        // This would be implemented with change tracking in a full implementation
        // For now, we'll always return false to maintain compatibility
        false
    }
    
    /// Get a summary of the current state
    pub fn get_summary(&self) -> String {
        format!(
            "AppState: view={:?}, audiobooks={}, libraries={}, selected={:?}",
            self.current_view,
            self.app_data.audiobooks.len(),
            self.app_data.libraries.len(),
            self.selected_audiobook_id
        )
    }
    
    /// Reset to default state
    pub fn reset(&mut self) {
        *self = Self::new();
        info!("State reset to defaults");
    }
}

// === Getters for read-only access ===
impl AppState {
    /// Get current view
    pub fn current_view(&self) -> &ViewType {
        &self.current_view
    }
    
    /// Get selected audiobook ID
    pub fn selected_audiobook_id(&self) -> Option<&String> {
        self.selected_audiobook_id.as_ref()
    }
    
    /// Get user preferences
    pub fn user_preferences(&self) -> &UserPreferences {
        &self.user_preferences
    }
    
    /// Get window configuration
    pub fn window_config(&self) -> &WindowConfig {
        &self.window_config
    }
    
    /// Get theme configuration
    pub fn theme_config(&self) -> &ThemeConfig {
        &self.theme_config
    }
    
    /// Get playback configuration
    pub fn playback_config(&self) -> &PlaybackConfig {
        &self.playback_config
    }
    
    /// Get application data
    pub fn app_data(&self) -> &AppData {
        &self.app_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        assert_eq!(state.current_view, ViewType::default());
        assert!(state.selected_audiobook_id.is_none());
    }
    
    #[test]
    fn test_view_change() {
        let mut state = AppState::new();
        state.set_current_view(ViewType::Library);
        assert_eq!(state.current_view, ViewType::Library);
    }
    
    #[test]
    fn test_state_summary() {
        let state = AppState::new();
        let summary = state.get_summary();
        assert!(summary.contains("AppState"));
        assert!(summary.contains("view="));
    }
    
    #[test]
    fn test_validation() {
        let state = AppState::new();
        // Should not panic
        let _ = state.validate();
    }
}
