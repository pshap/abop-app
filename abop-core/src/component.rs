//! Trait-based component system for ABOP
//!
//! This module provides generic component traits that can be implemented
//! by UI framework-specific components in the GUI layer.

use crate::{AppMessage, Result};

/// Trait for components that can be updated with messages
///
/// This trait allows a component to receive and process messages, returning any resulting messages
/// to be handled by the application. Used for Elm-style update patterns.
///
/// # Type Parameters
/// * `Message` - The type of message the component can handle
pub trait Component<Message> {
    /// Update the component with a message and return any resulting messages
    ///
    /// # Arguments
    /// * `message` - The message to process
    ///
    /// # Returns
    /// A result containing a vector of new `AppMessage`s or an error
    ///
    /// # Errors
    ///
    /// Returns [`AppError`] if the message cannot be processed due to invalid
    /// state, invalid message content, or other component-specific errors.
    fn update(&mut self, message: Message) -> Result<Vec<AppMessage>>;
}

/// Trait for components that have state and can be rendered
///
/// This trait provides a method to retrieve a unique identifier or description for a component,
/// useful for debugging, logging, or dynamic UI management.
pub trait Renderable {
    /// Get a description or identifier for this component (for debugging/logging)
    ///
    /// # Returns
    /// A string slice representing the component's identifier
    fn component_id(&self) -> &str;
}

/// Trait for components that can handle app-level messages
///
/// This trait allows a component to process high-level application messages and return any resulting messages.
/// Used for message-driven architectures.
pub trait Updatable {
    /// Handle an app message and return any resulting messages
    ///
    /// # Arguments
    /// * `message` - The application message to process
    ///
    /// # Returns
    /// A result containing a vector of new `AppMessage`s or an error
    ///
    /// # Errors
    ///
    /// Returns an error if message processing fails or if the component
    /// encounters an unrecoverable state during message handling.
    fn handle_message(&mut self, message: AppMessage) -> Result<Vec<AppMessage>>;
}
