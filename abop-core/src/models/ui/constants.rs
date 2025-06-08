//! Constants for UI state management

use std::time::Duration;

// Window configuration constants
pub const DEFAULT_WINDOW_WIDTH: u32 = 1200;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 800;

// State validation and repair constants
pub const LARGE_DATASET_THRESHOLD: usize = 50;
pub const MAX_RECENT_DIRECTORIES: usize = 10;

// Progress reporting constants
pub const PROGRESS_UPDATE_DELAY: Duration = Duration::from_millis(50);
pub const LARGE_DATASET_PROGRESS_DELAY: Duration = Duration::from_millis(100);

// File size limits
pub const MAX_REASONABLE_WINDOW_SIZE: u32 = 10000;
pub const MIN_REASONABLE_WINDOW_SIZE: u32 = 100;

// Playback configuration constants
pub const DEFAULT_VOLUME: f32 = 0.8;
pub const DEFAULT_PLAYBACK_SPEED: f32 = 1.0;
pub const DEFAULT_SKIP_AMOUNT: u64 = 15;

// State backup configuration
pub const DEFAULT_MAX_BACKUPS: usize = 3;
pub const STATE_FILE_NAME: &str = "app_state.toml";
pub const APP_DATA_DIR: &str = "abop-iced";
