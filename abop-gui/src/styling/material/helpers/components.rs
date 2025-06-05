//! Component helper trait for Material Design token system
//!
//! This trait provides helper methods for creating pre-configured Material Design components
//! with proper styling and behavior according to Material Design 3 specifications.

/// Helper trait for component creation functionality
///
/// This trait provides convenient methods for creating common Material Design components
/// with default configurations that follow Material Design 3 guidelines.
pub trait ComponentHelpers {
    /// Create a Material Design card with proper elevation
    ///
    /// Returns a pre-configured `MaterialCard` with default elevation and styling
    /// according to Material Design card specifications.
    ///
    /// # Returns
    /// A `MaterialCard` component with Level 1 elevation and proper styling
    fn card(&self) -> crate::styling::material::components::containers::MaterialCard {
        crate::styling::material::components::containers::MaterialCard::default()
    }

    /// Create a Material Design progress indicator
    ///
    /// Returns a pre-configured `MaterialProgressIndicator` with default styling
    /// for showing loading and progress states.
    ///
    /// # Returns
    /// A `MaterialProgressIndicator` with standard Material Design styling
    fn progress_indicator(
        &self,
    ) -> crate::styling::material::components::feedback::MaterialProgressIndicator {
        crate::styling::material::components::feedback::MaterialProgressIndicator::default()
    }

    /// Create a Material Design notification
    ///
    /// Returns a pre-configured `MaterialNotification` with toast styling
    /// for displaying temporary messages to users.
    ///
    /// # Arguments
    /// * `message` - The message content to display in the notification
    ///
    /// # Returns
    /// A `MaterialNotification` configured as a toast with the provided message
    fn notification(
        &self,
        message: impl Into<String>,
    ) -> crate::styling::material::components::feedback::MaterialNotification {
        crate::styling::material::components::feedback::MaterialNotification::toast(message)
    }

    /// Create a Material Design badge
    ///
    /// Returns a pre-configured `MaterialBadge` with default styling
    /// for displaying status indicators and counters.
    ///
    /// # Returns
    /// A `MaterialBadge` with standard Material Design styling
    fn badge(&self) -> crate::styling::material::components::feedback::MaterialBadge {
        crate::styling::material::components::feedback::MaterialBadge::default()
    }

    /// Create a Material Design status indicator
    ///
    /// Returns a pre-configured `MaterialStatusIndicator` with default styling
    /// for showing system and application states.
    ///
    /// # Returns
    /// A `MaterialStatusIndicator` with standard Material Design styling
    fn status_indicator(
        &self,
    ) -> crate::styling::material::components::feedback::MaterialStatusIndicator {
        crate::styling::material::components::feedback::MaterialStatusIndicator::default()
    }
}
