//! Message types for the application's message passing system

use super::state::View;

/// Main message type for the application
#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// Navigation messages
    Navigate(View),
    // Add other message variants as needed
}
