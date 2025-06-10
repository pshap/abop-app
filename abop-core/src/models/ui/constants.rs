//! Constants for UI state management

use std::time::Duration;

// Window configuration constants
/// Default width for the application window in pixels
pub const DEFAULT_WINDOW_WIDTH: u32 = 1200;
/// Default height for the application window in pixels
pub const DEFAULT_WINDOW_HEIGHT: u32 = 800;

// State validation and repair constants
/// Threshold for determining if a dataset is considered large for UI performance optimization
pub const LARGE_DATASET_THRESHOLD: usize = 50;
/// Maximum number of recent directories to keep in the application state
pub const MAX_RECENT_DIRECTORIES: usize = 10;

// Progress reporting constants
/// Delay between progress updates for normal datasets
pub const PROGRESS_UPDATE_DELAY: Duration = Duration::from_millis(50);
/// Delay between progress updates for large datasets to reduce UI load
pub const LARGE_DATASET_PROGRESS_DELAY: Duration = Duration::from_millis(100);

// File size limits
/// Maximum reasonable window size in pixels for validation
pub const MAX_REASONABLE_WINDOW_SIZE: u32 = 10000;
/// Minimum reasonable window size in pixels for validation
pub const MIN_REASONABLE_WINDOW_SIZE: u32 = 100;

// Playback configuration constants
/// Default audio volume level (0.0 to 1.0)
pub const DEFAULT_VOLUME: f32 = 0.8;
/// Default playback speed multiplier
pub const DEFAULT_PLAYBACK_SPEED: f32 = 1.0;
/// Default skip amount in seconds for forward/backward navigation
pub const DEFAULT_SKIP_AMOUNT: u64 = 15;

// State backup configuration
/// Default maximum number of backup files to maintain
pub const DEFAULT_MAX_BACKUPS: usize = 3;
/// Filename for the application state file
pub const STATE_FILE_NAME: &str = "app_state.toml";
/// Application data directory name
pub const APP_DATA_DIR: &str = "abop-iced";
