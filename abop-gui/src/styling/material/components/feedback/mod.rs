//! Material Design 3 Feedback Components
//!
//! This module provides Material Design 3 feedback components including:
//! - Progress indicators (linear, circular, determinate/indeterminate)
//! - Badges (notification badges, status indicators)
//! - Status indicators (success, warning, error, info states)
//! - Dialogs (alert, confirmation, form, bottom sheet, full screen)
//! - Modal overlays
//! - Notifications (toast, banner, inline alert, snackbar)

pub mod badge;
pub mod dialog;
pub mod modal;
pub mod notification;
pub mod progress;
pub mod status;
pub mod style_utils;

// Re-export all components for easier access
pub use badge::*;
pub use dialog::*;
pub use modal::*;
pub use notification::*;
pub use progress::*;
pub use status::*;

// Re-export utility functions
pub use style_utils::*;
