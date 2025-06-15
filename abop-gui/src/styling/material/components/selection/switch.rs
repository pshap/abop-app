//! Modern Material Design 3 Switch Implementation
//!
//! This module provides a completely redesigned switch component with:
//! - State-based design using SwitchState enum
//! - Built-in validation and error handling
//! - Animation support for smooth transitions
//! - Modern builder pattern with fluent API
//! - Preparation for custom switch widget implementation (Phase 4)

use super::builder::components::LIGHT_TOKENS;
use super::builder::{Switch, SwitchBuilder};
use super::common::prelude::*;
use crate::styling::material::MaterialColors;
use crate::styling::material::components::selection_style::{
    SelectionSize as LegacySelectionSize, SelectionStyleBuilder, SelectionVariant,
};

use iced::{
    Element,
    Renderer,
    theme::Theme,
    widget::Checkbox as IcedCheckbox, // Temporary: Phase 4 will implement custom widget
};

// ============================================================================
// Component Implementation
// ============================================================================

impl Switch {
    /// Create a new switch builder with the specified state
    #[must_use]
    pub fn builder(state: SwitchState) -> SwitchBuilder {
        SwitchBuilder::new(state)
    }

    /// Create a switch in the on state
    #[must_use]
    pub fn on() -> SwitchBuilder {
        SwitchBuilder::on()
    }

    /// Create a switch in the off state
    #[must_use]
    pub fn off() -> SwitchBuilder {
        SwitchBuilder::off()
    }

    /// Create switch from boolean value
    #[must_use]
    pub fn from_bool(enabled: bool) -> SwitchBuilder {
        SwitchBuilder::from_bool(enabled)
    }

    /// Create the Iced widget element for this switch
    ///
    /// # Arguments
    /// * `on_toggle` - Callback function called when switch state changes
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to the UI
    ///    /// # Note
    /// Currently implemented as styled checkbox. Phase 4 will replace this
    /// with a proper custom switch widget implementation.
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        on_toggle: impl Fn(SwitchState) -> Message + 'a,
        _color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        // TODO: Phase 4 - Replace with custom switch widget
        // For now, use styled checkbox as placeholder

        // Convert switch state to boolean for checkbox compatibility
        let is_enabled = self.state().is_on();

        // Create the switch label
        let default_label = String::new();
        let label = self.props().label.as_ref().unwrap_or(&default_label);

        // Use static tokens to avoid lifetime issues
        let tokens = &*LIGHT_TOKENS; // Default to light tokens for now

        // Create the style function with the tokens
        let style_fn = {
            let selection_size = match self.props().size {
                ComponentSize::Small => LegacySelectionSize::Small,
                ComponentSize::Medium => LegacySelectionSize::Medium,
                ComponentSize::Large => LegacySelectionSize::Large,
            };

            let builder = SelectionStyleBuilder::new(tokens, SelectionVariant::Switch)
                .size(selection_size)
                .error(self.has_error());

            // Create the style function
            builder.checkbox_style()
        };

        // Create the checkbox widget with the style function
        let mut switch_widget = IcedCheckbox::new(label, is_enabled).style(style_fn);

        // Only add on_toggle handler if the switch is not disabled
        if !self.props().disabled {
            // Convert boolean toggle to state-based toggle
            let current_state = self.state();
            switch_widget =
                switch_widget.on_toggle(move |_enabled| on_toggle(current_state.toggle()));
        }

        switch_widget.into()
    }
    /// Create a simplified view that handles state changes automatically
    ///
    /// This is a convenience method for cases where you want the switch to
    /// manage its own state internally.
    pub fn view_with_state<'a, Message: Clone + 'a>(
        &self,
        on_change: impl Fn(SwitchState) -> Message + 'a,
        _color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        self.view(on_change, _color_scheme)
    }
}

// ============================================================================
// Future: Custom Switch Widget (Phase 4 Implementation)
// ============================================================================

/// Configuration for Material Design 3 switch dimensions
///
/// TODO: Phase 4 - Use these for custom switch widget implementation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwitchDimensions {
    /// Track width
    pub track_width: f32,
    /// Track height
    pub track_height: f32,
    /// Thumb diameter
    pub thumb_diameter: f32,
    /// Track border radius (always height/2 for fully rounded)
    pub track_radius: f32,
    /// Thumb shadow elevation
    pub thumb_elevation: f32,
    /// Thumb travel distance
    pub thumb_travel: f32,
}

impl SwitchDimensions {
    /// Get Material Design 3 switch dimensions for the given size
    #[must_use]
    pub const fn for_size(size: ComponentSize) -> Self {
        match size {
            ComponentSize::Small => Self {
                track_width: 48.0,
                track_height: 28.0,
                thumb_diameter: 20.0,
                track_radius: 14.0, // height/2
                thumb_elevation: 1.0,
                thumb_travel: 20.0, // track_width - thumb_diameter - padding
            },
            ComponentSize::Medium => Self {
                track_width: 52.0,
                track_height: 32.0,
                thumb_diameter: 24.0,
                track_radius: 16.0, // height/2
                thumb_elevation: 1.0,
                thumb_travel: 20.0,
            },
            ComponentSize::Large => Self {
                track_width: 56.0,
                track_height: 36.0,
                thumb_diameter: 28.0,
                track_radius: 18.0, // height/2
                thumb_elevation: 1.0,
                thumb_travel: 20.0,
            },
        }
    }
}

/// Custom switch widget that follows Material Design 3 specifications
///
/// TODO: Phase 4 - Implement this as a proper Iced widget
/// This would provide native switch appearance with:
/// - Rounded track with proper dimensions
/// - Animated thumb that slides between positions
/// - State-based track and thumb colors
/// - Proper shadow and elevation effects
/// - Smooth animations with Material Design timing
#[allow(dead_code)]
struct CustomSwitchWidget {
    state: SwitchState,
    label: String,
    disabled: bool,
    error_state: bool,
    size: ComponentSize,
    dimensions: SwitchDimensions,
    animation_config: AnimationConfig,
}

#[allow(dead_code)]
impl CustomSwitchWidget {
    /// Create a new custom switch widget
    fn new(state: SwitchState, label: String) -> Self {
        let size = ComponentSize::Medium;
        Self {
            state,
            label,
            disabled: false,
            error_state: false,
            size,
            dimensions: SwitchDimensions::for_size(size),
            animation_config: AnimationConfig::default(),
        }
    }

    /// Set disabled state
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set error state
    fn error(mut self, error: bool) -> Self {
        self.error_state = error;
        self
    }

    /// Set size
    fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self.dimensions = SwitchDimensions::for_size(size);
        self
    }

    /// Set animation configuration
    fn animation(mut self, config: AnimationConfig) -> Self {
        self.animation_config = config;
        self
    }

    /// Calculate thumb position based on state and animation progress
    fn thumb_position(&self, animation_progress: f32) -> f32 {
        match self.state {
            SwitchState::Off => {
                // Thumb starts at left side
                (self.dimensions.track_height - self.dimensions.thumb_diameter) / 2.0
            }
            SwitchState::On => {
                // Thumb ends at right side
                let base_position =
                    (self.dimensions.track_height - self.dimensions.thumb_diameter) / 2.0;
                base_position + (self.dimensions.thumb_travel * animation_progress)
            }
        }
    }

    /// Get track color based on state and theme
    fn track_color(&self, colors: &MaterialColors) -> iced::Color {
        match (self.state, self.disabled, self.error_state) {
            (_, true, _) => colors.outline_variant, // Disabled
            (_, _, true) => colors.error.base,      // Error state
            (SwitchState::On, false, false) => colors.primary.base, // On state
            (SwitchState::Off, false, false) => colors.outline, // Off state
        }
    }
    /// Get thumb color based on state and theme
    fn thumb_color(&self, colors: &MaterialColors) -> iced::Color {
        match (self.state, self.disabled, self.error_state) {
            (_, true, _) => {
                // Disabled - create color with alpha
                let base = colors.on_surface;
                iced::Color::from_rgba(base.r, base.g, base.b, 0.38)
            }
            (_, _, true) => colors.on_error(), // Error state
            (SwitchState::On, false, false) => colors.on_primary(), // On state
            (SwitchState::Off, false, false) => colors.outline, // Off state
        }
    }
}

// TODO: Phase 4 - Implement Widget trait for CustomSwitchWidget
// This would involve:
// 1. Implementing iced::Widget trait
// 2. Custom drawing using Canvas or primitive shapes
// 3. Animation state management
// 4. Event handling for mouse/touch interaction
// 5. Accessibility support

// ============================================================================
// Convenience Functions
// ============================================================================

/// Create a new switch builder
#[must_use]
pub fn switch(state: SwitchState) -> SwitchBuilder {
    SwitchBuilder::new(state)
}

/// Create a switch builder in the on state
#[must_use]
pub fn switch_on() -> SwitchBuilder {
    SwitchBuilder::on()
}

/// Create a switch builder in the off state
#[must_use]
pub fn switch_off() -> SwitchBuilder {
    SwitchBuilder::off()
}

/// Create switch from boolean value
#[must_use]
pub fn switch_from_bool(enabled: bool) -> SwitchBuilder {
    SwitchBuilder::from_bool(enabled)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::super::builder::ComponentBuilder;
    use super::*;

    #[test]
    fn test_switch_creation() {
        let switch = Switch::on()
            .label("Test Switch")
            .size(ComponentSize::Large)
            .build()
            .expect("Should create valid switch");

        assert_eq!(switch.state(), SwitchState::On);
        assert_eq!(switch.props().label, Some("Test Switch".to_string()));
        assert_eq!(switch.props().size, ComponentSize::Large);
        assert!(switch.is_on());
        assert!(switch.to_bool());
    }

    #[test]
    fn test_switch_state_transitions() {
        let mut switch = Switch::off().build().expect("Should create valid switch");

        assert_eq!(switch.state(), SwitchState::Off);
        assert!(switch.is_off());
        assert!(!switch.to_bool());

        // Toggle to on
        let (previous_state, new_state) = switch.toggle().expect("Should toggle successfully");
        assert_eq!(previous_state, SwitchState::Off);
        assert_eq!(new_state, SwitchState::On);
        assert_eq!(switch.state(), SwitchState::On);
        assert!(switch.is_on());
        assert!(switch.to_bool());

        // Toggle back to off
        let (previous_state, new_state) = switch.toggle().expect("Should toggle successfully");
        assert_eq!(previous_state, SwitchState::On);
        assert_eq!(new_state, SwitchState::Off);
        assert_eq!(switch.state(), SwitchState::Off);
        assert!(switch.is_off());
        assert!(!switch.to_bool());
    }

    #[test]
    fn test_switch_validation() {
        // Valid switch
        let valid_switch = Switch::on().label("Valid Label").build();
        assert!(valid_switch.is_ok());

        // Invalid switch - label too long
        let long_label = "x".repeat(201);
        let invalid_switch = Switch::off().label(long_label).build();
        assert!(invalid_switch.is_err());
    }

    #[test]
    fn test_switch_error_state() {
        let mut switch = Switch::on()
            .error(true)
            .build()
            .expect("Should create switch with error state");

        assert!(switch.has_error());

        switch.set_error(false);
        assert!(!switch.has_error());
    }

    #[test]
    fn test_switch_dimensions() {
        let small_dims = SwitchDimensions::for_size(ComponentSize::Small);
        let medium_dims = SwitchDimensions::for_size(ComponentSize::Medium);
        let large_dims = SwitchDimensions::for_size(ComponentSize::Large);

        // Verify dimensions increase with size
        assert!(small_dims.track_width < medium_dims.track_width);
        assert!(medium_dims.track_width < large_dims.track_width);

        assert!(small_dims.thumb_diameter < medium_dims.thumb_diameter);
        assert!(medium_dims.thumb_diameter < large_dims.thumb_diameter);

        // Verify track radius is always height/2 (fully rounded)
        assert_eq!(small_dims.track_radius, small_dims.track_height / 2.0);
        assert_eq!(medium_dims.track_radius, medium_dims.track_height / 2.0);
        assert_eq!(large_dims.track_radius, large_dims.track_height / 2.0);
    }

    #[test]
    fn test_switch_traits() {
        let switch = Switch::on().build().expect("Should create valid switch");

        // Test SelectionWidget trait
        assert_eq!(switch.state(), SwitchState::On);
        assert!(switch.validate().is_ok());

        // Test animation support
        assert!(switch.animation_config().enabled);
    }

    #[test]
    fn test_convenience_functions() {
        let sw1 = switch_on().build().unwrap();
        let sw2 = switch_off().build().unwrap();
        let sw3 = switch_from_bool(true).build().unwrap();
        let sw4 = switch(SwitchState::Off).build().unwrap();

        assert_eq!(sw1.state(), SwitchState::On);
        assert_eq!(sw2.state(), SwitchState::Off);
        assert_eq!(sw3.state(), SwitchState::On);
        assert_eq!(sw4.state(), SwitchState::Off);
    }

    #[test]
    fn test_switch_default() {
        let switch = Switch::default();
        assert_eq!(switch.state(), SwitchState::Off);
        assert!(!switch.props().disabled);
        assert_eq!(switch.props().size, ComponentSize::Medium);
    }

    #[test]
    fn test_custom_switch_widget_preparation() {
        // Test the helper structures for Phase 4 implementation
        let widget = CustomSwitchWidget::new(SwitchState::On, "Test".to_string())
            .size(ComponentSize::Large)
            .disabled(false)
            .error(false);

        assert_eq!(widget.state, SwitchState::On);
        assert_eq!(widget.size, ComponentSize::Large);
        assert!(!widget.disabled);
        assert!(!widget.error_state);

        // Test thumb position calculation
        let off_position = widget.thumb_position(0.0);
        let on_position = widget.thumb_position(1.0);
        assert!(on_position > off_position);
    }
}
