//! Component properties and configuration types for Material Design 3 selection components
//!
//! This module contains shared property definitions, size variants, and component types
//! used across all selection components.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::constants;

// ============================================================================
// Component Size System
// ============================================================================

/// Size variants for consistent sizing across all selection components
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ComponentSize {
    /// Small size (16px) - for dense layouts and compact spaces
    Small,
    /// Medium size (20px) - default size for most use cases
    Medium,
    /// Large size (24px) - for accessibility and prominent placement
    Large,
}

impl Default for ComponentSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl ComponentSize {
    /// Get the pixel size for the selection component
    #[must_use]
    pub const fn size_px(self) -> f32 {
        match self {
            Self::Small => constants::sizes::SMALL_SIZE_PX,
            Self::Medium => constants::sizes::MEDIUM_SIZE_PX,
            Self::Large => constants::sizes::LARGE_SIZE_PX,
        }
    }

    /// Get the appropriate touch target size (Material Design minimum 48px)
    #[must_use]
    pub const fn touch_target_size(self) -> f32 {
        // Material Design minimum touch target size is 48px
        constants::ui::MIN_TOUCH_TARGET_SIZE
    }

    /// Get the appropriate border width for the size
    #[must_use]
    pub const fn border_width(self) -> f32 {
        // Default border width based on size
        match self {
            Self::Small => 1.0,
            Self::Medium => 1.5,
            Self::Large => 2.0,
        }
    }

    /// Get the appropriate text size for labels
    #[must_use]
    pub const fn text_size(self) -> f32 {
        match self {
            Self::Small => 12.0,
            Self::Medium => 14.0,
            Self::Large => 16.0,
        }
    }

    /// Get all available sizes
    #[must_use]
    pub const fn all() -> [Self; 3] {
        [Self::Small, Self::Medium, Self::Large]
    }

    /// Check if this size meets Material Design touch target requirements
    #[must_use]
    pub const fn meets_touch_target_requirements(self) -> bool {
        self.touch_target_size() >= constants::ui::MIN_TOUCH_TARGET_SIZE
    }
}

// ============================================================================
// Chip Variants
// ============================================================================

/// Material Design 3 chip variants for different use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChipVariant {
    /// Action chips for common tasks and quick actions
    Assist,
    /// Filter chips for filtering content and making selections  
    Filter,
    /// Input chips for user-generated content and tags
    Input,
    /// Suggestion chips for suggested actions or completions
    Suggestion,
}

impl Default for ChipVariant {
    fn default() -> Self {
        Self::Filter
    }
}

// ============================================================================
// Component Properties
// ============================================================================

/// Common properties shared across selection components
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ComponentProps {
    /// Optional text label displayed with the component
    pub label: Option<String>,
    /// Whether the component is disabled (non-interactive)
    pub disabled: bool,
    /// The size variant of the component
    pub size: ComponentSize,
    /// Metadata storage for extended properties (icons, badges, layout, etc.)
    pub metadata: HashMap<String, String>,
}

impl ComponentProps {
    /// Create new component properties
    #[must_use]
    pub fn new() -> Self {
        Self {
            label: None,
            disabled: false,
            size: ComponentSize::Medium,
            metadata: HashMap::new(),
        }
    }

    /// Set the label (builder pattern)
    #[must_use]
    pub fn with_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set disabled state (builder pattern)
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set size (builder pattern)
    #[must_use]
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    /// Add metadata key-value pair (builder pattern)
    ///
    /// This method allows storing arbitrary metadata for enhanced features
    /// like icons, badges, layout preferences, etc. It validates that only
    /// known metadata keys are used to prevent typos and maintain consistency.
    ///
    /// # Arguments
    /// * `key` - The metadata key (should use predefined constants)
    /// * `value` - The metadata value (automatically converted to String)
    ///
    /// # Examples
    /// ```rust,no_run
    /// use abop_gui::styling::material::components::selection::properties::*;
    /// use abop_gui::styling::material::components::selection::constants;
    ///
    /// let props = ComponentProps::new()
    ///     .with_metadata(constants::metadata_keys::LEADING_ICON, "star")
    ///     .with_metadata(constants::metadata_keys::BADGE, "5");
    /// ```
    #[must_use]
    pub fn with_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        let key_string = key.into();

        // Use const lookup for better performance in release builds
        let is_known_key = constants::metadata_keys::ALL_SUPPORTED
            .iter()
            .any(|&k| k == key_string);

        if is_known_key {
            self.metadata.insert(key_string, value.into());
        } else {
            // Log unknown keys in all builds for better debugging
            log::debug!("Unknown metadata key '{key_string}'. Consider using predefined constants from constants::metadata_keys.");
            // Allow unknown keys for extensibility 
            self.metadata.insert(key_string, value.into());
        }
        self
    }

    /// Insert metadata key-value pair mutably (more efficient than with_metadata for single updates)
    pub fn insert_metadata<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        let key_string = key.into();

        // Use const lookup for better performance in release builds
        let is_known_key = constants::metadata_keys::ALL_SUPPORTED
            .iter()
            .any(|&k| k == key_string);

        if is_known_key {
            self.metadata.insert(key_string, value.into());
        } else {
            // Log unknown keys in all builds for better debugging
            log::debug!("Unknown metadata key '{key_string}'. Consider using predefined constants from constants::metadata_keys.");
            // Allow unknown keys for extensibility 
            self.metadata.insert(key_string, value.into());
        }
    }

    /// Get metadata value by key
    ///
    /// # Arguments
    /// * `key` - The metadata key to look up
    ///
    /// # Returns
    /// Optional reference to the metadata value as a string slice for better performance
    #[must_use]
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Check if metadata contains a specific key
    ///
    /// # Arguments
    /// * `key` - The metadata key to check
    ///
    /// # Returns
    /// True if the key exists in metadata
    #[must_use]
    pub fn has_metadata(&self, key: &str) -> bool {
        self.metadata.contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_size_properties() {
        assert_eq!(ComponentSize::Small.size_px(), 16.0);
        assert_eq!(ComponentSize::Medium.size_px(), 20.0);
        assert_eq!(ComponentSize::Large.size_px(), 24.0);

        assert!(ComponentSize::Small.touch_target_size() >= 32.0);
        assert!(ComponentSize::Large.touch_target_size() >= 48.0);
    }

    #[test]
    fn test_component_props_builder() {
        let props = ComponentProps::new()
            .with_label("Test Label")
            .disabled(true)
            .size(ComponentSize::Large);

        assert_eq!(props.label, Some("Test Label".to_string()));
        assert!(props.disabled);
        assert_eq!(props.size, ComponentSize::Large);
    }

    #[test]
    fn test_component_props_metadata() {
        let props = ComponentProps::new()
            .with_metadata("leading_icon", "filter")
            .with_metadata("badge_count", "5")
            .with_metadata("layout", "wrap");

        assert_eq!(props.get_metadata("leading_icon"), Some("filter"));
        assert_eq!(props.get_metadata("badge_count"), Some("5"));
        assert_eq!(props.get_metadata("layout"), Some("wrap"));
        assert_eq!(props.get_metadata("nonexistent"), None);

        assert!(props.has_metadata("leading_icon"));
        assert!(props.has_metadata("badge_count"));
        assert!(!props.has_metadata("nonexistent"));
    }
}