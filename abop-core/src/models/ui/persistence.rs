//! State persistence operations separated from core state management

use crate::error::{AppError, Result};
use std::path::PathBuf;
use std::sync::mpsc;

use super::constants::*;
use super::state::AppState;

/// Configuration options for save operations
#[derive(Debug, Clone, Default)]
pub struct SaveOptions {
    /// Whether to send progress updates during save
    pub progress_sender: Option<mpsc::Sender<f32>>,
    /// Whether to run in blocking mode (for async operations)
    pub blocking: bool,
    /// Whether to create a backup before saving
    pub create_backup: bool,
    /// Custom backup directory (if None, uses default)
    pub backup_dir: Option<PathBuf>,
}

impl SaveOptions {
    /// Creates new save options with default settings
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the progress sender for progress reporting
    #[must_use]
    pub fn with_progress(mut self, sender: mpsc::Sender<f32>) -> Self {
        self.progress_sender = Some(sender);
        self
    }

    /// Sets blocking mode
    #[must_use]
    pub const fn with_blocking(mut self, blocking: bool) -> Self {
        self.blocking = blocking;
        self
    }

    /// Sets backup creation
    #[must_use]
    pub const fn with_backup(mut self, create_backup: bool) -> Self {
        self.create_backup = create_backup;
        self
    }

    /// Sets custom backup directory
    #[must_use]
    pub fn with_backup_dir(mut self, backup_dir: PathBuf) -> Self {
        self.backup_dir = Some(backup_dir);
        self
    }
}

/// Handles state persistence operations with configurable options
#[derive(Debug, Clone)]
pub struct StatePersistence {
    state_path: PathBuf,
}

impl StatePersistence {
    /// Creates a new state persistence handler
    ///
    /// # Errors
    ///
    /// Returns an error if the state file path cannot be determined
    pub fn new() -> Result<Self> {
        let state_path = Self::get_state_path()?;
        Ok(Self { state_path })
    }

    /// Creates a state persistence handler with a custom path
    #[must_use]
    pub const fn with_path(state_path: PathBuf) -> Self {
        Self { state_path }
    }

    /// Gets the current state file path
    #[must_use]
    pub const fn state_path(&self) -> &PathBuf {
        &self.state_path
    }

    /// Loads application state from the file system
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The state file cannot be read
    /// - The state file contains invalid TOML
    /// - State validation fails critically
    pub fn load(&self) -> Result<AppState> {
        if !self.state_path.exists() {
            log::info!("State file does not exist, creating default state");
            let default_state = AppState::new();
            self.save(&default_state, &SaveOptions::new())?;
            return Ok(default_state);
        }

        log::info!("Loading state from: {}", self.state_path.display());
        let contents = std::fs::read_to_string(&self.state_path)?;
        let mut state: AppState = toml::from_str(&contents)?;

        // Validate and repair the loaded state
        let (validation_result, repair_actions) =
            crate::validation::validate_and_repair_app_state(&mut state);

        if !repair_actions.is_empty() {
            log::info!(
                "Applied {} repair actions to loaded state:",
                repair_actions.len()
            );
            for action in &repair_actions {
                if action.success {
                    log::info!("  ✓ {}: {}", action.action_type, action.description);
                    if let Some(details) = &action.details {
                        log::info!("    {details}");
                    }
                } else {
                    log::warn!("  ✗ {}: {}", action.action_type, action.description);
                }
            }

            // Save the repaired state back to disk
            self.save(&state, &SaveOptions::new())?;
            log::info!("Saved repaired state to disk");
        }

        if validation_result.has_critical_issues() {
            return Err(AppError::Config(format!(
                "Critical validation issues found: {}",
                validation_result.critical_issues().len()
            )));
        }

        if !validation_result.is_valid() {
            log::warn!(
                "State validation found {} issues:",
                validation_result.issues.len()
            );
            for issue in &validation_result.issues {
                log::warn!(
                    "  {} [{}]: {}",
                    issue.severity,
                    issue.category,
                    issue.message
                );
            }
        } else {
            log::info!("State validation passed successfully");
        }

        Ok(state)
    }

    /// Saves application state with the given options
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The parent directory cannot be created
    /// - The state cannot be serialized to TOML
    /// - The state file cannot be written
    pub fn save(&self, state: &AppState, options: &SaveOptions) -> Result<String> {
        if options.blocking {
            self.save_blocking(state.clone(), options)
        } else {
            self.save_immediate(state, options)
        }
    }
    /// Saves state immediately (non-blocking)
    fn save_immediate(&self, state: &AppState, _options: &SaveOptions) -> Result<String> {
        self.ensure_parent_directory()?;

        let contents = toml::to_string_pretty(state)?;
        std::fs::write(&self.state_path, contents)?;
        let message = format!(
            "State saved successfully with {} audiobooks",
            state.app_data.audiobooks.len()
        );

        log::info!("{message}");
        Ok(message)
    }

    /// Saves state in blocking mode with optional progress reporting
    fn save_blocking(&self, state: AppState, options: &SaveOptions) -> Result<String> {
        let send_progress = |progress: f32| {
            if let Some(ref sender) = options.progress_sender {
                let _ = sender.send(progress);
                std::thread::sleep(PROGRESS_UPDATE_DELAY);
            }
        };

        log::info!(
            "save_blocking: Saving to path: {}",
            self.state_path.display()
        );
        send_progress(0.1); // 10% - Started

        self.ensure_parent_directory()?;
        send_progress(0.2); // 20% - Directory created

        let audiobook_count = state.app_data.audiobooks.len();
        let is_large_dataset = audiobook_count > LARGE_DATASET_THRESHOLD;

        log::info!(
            "save_blocking: About to serialize AppState ({audiobook_count} audiobooks) to TOML."
        );
        send_progress(0.3); // 30% - Starting serialization

        if is_large_dataset {
            log::info!(
                "save_blocking: Large dataset detected ({audiobook_count} audiobooks), will show serialization progress"
            );
            send_progress(0.4); // 40%
            std::thread::sleep(LARGE_DATASET_PROGRESS_DELAY);
            send_progress(0.5); // 50%
            std::thread::sleep(LARGE_DATASET_PROGRESS_DELAY);
            send_progress(0.6); // 60%
        }

        let contents = match toml::to_string_pretty(&state) {
            Ok(c) => {
                log::info!(
                    "save_blocking: TOML serialization successful. Content length: {}",
                    c.len()
                );
                c
            }
            Err(e) => {
                log::error!("save_blocking: TOML serialization FAILED: {e}");
                return Err(AppError::TomlSer(e.to_string()));
            }
        };

        send_progress(0.8); // 80% - Serialization complete

        log::info!(
            "save_blocking: About to write file at {}",
            self.state_path.display()
        );

        if let Err(e) = std::fs::write(&self.state_path, &contents) {
            log::error!(
                "save_blocking: File write FAILED for {}: {}",
                self.state_path.display(),
                e
            );
            return Err(AppError::Io(e.to_string()));
        }

        send_progress(1.0); // 100% - Complete

        let message = format!("State saved successfully with {audiobook_count} audiobooks");

        log::info!(
            "save_blocking: File write successful to {}",
            self.state_path.display()
        );
        Ok(message)
    }

    /// Ensures the parent directory exists
    fn ensure_parent_directory(&self) -> Result<()> {
        if let Some(parent) = self.state_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    /// Determines the state file path
    fn get_state_path() -> Result<PathBuf> {
        let mut path = dirs::data_dir()
            .ok_or_else(|| AppError::Config("Could not find data directory".to_string()))?;
        path.push(APP_DATA_DIR);
        path.push(STATE_FILE_NAME);
        Ok(path)
    }

    /// Creates a backup of the current state file
    ///
    /// # Errors
    ///
    /// Returns an error if the backup cannot be created
    pub fn create_backup(&self, backup_dir: Option<&PathBuf>) -> Result<PathBuf> {
        if !self.state_path.exists() {
            return Err(AppError::Config(
                "No state file exists to backup".to_string(),
            ));
        }

        let backup_dir = backup_dir
            .cloned()
            .unwrap_or_else(|| self.state_path.parent().unwrap().to_path_buf());

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_filename = format!("app_state_backup_{timestamp}.toml");
        let backup_path = backup_dir.join(backup_filename);

        std::fs::copy(&self.state_path, &backup_path)?;
        log::info!("Created backup at: {}", backup_path.display());

        Ok(backup_path)
    }

    /// Restores state from a backup file
    ///
    /// # Errors
    ///
    /// Returns an error if the backup cannot be restored
    pub fn restore_from_backup(&self, backup_path: &PathBuf) -> Result<AppState> {
        if !backup_path.exists() {
            return Err(AppError::Config(format!(
                "Backup file does not exist: {}",
                backup_path.display()
            )));
        }

        let contents = std::fs::read_to_string(backup_path)?;
        let state: AppState = toml::from_str(&contents)?;

        log::info!("Restored state from backup: {}", backup_path.display());
        Ok(state)
    }
}

impl Default for StatePersistence {
    fn default() -> Self {
        Self::new().expect("Failed to create default StatePersistence")
    }
}
