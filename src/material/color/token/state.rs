//! State layer colors for Material Design 3
//!
//! This module defines the state layer colors used to indicate different
//! interaction states (hover, pressed, focused, etc.) in Material Design 3.

use super::super::Srgb;

/// State layer colors for interactive elements
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateLayer {
    /// Hover state color
    pub hover: Srgb,
    /// Focused state color
    pub focus: Srgb,
    /// Pressed state color
    pub pressed: Srgb,
    /// Dragged state color
    pub dragged: Srgb,
    /// Selected state color
    pub selected: Srgb,
    /// Activated state color
    pub activated: Srgb,
    /// Pressed and selected state color
    pub pressed_selected: Srgb,
    /// Selected and hovered state color
    pub selected_hover: Srgb,
    /// Selected and focused state color
    pub selected_focus: Srgb,
    /// Disabled state color
    pub disabled: Srgb,
    /// Disabled and selected state color
    pub disabled_selected: Srgb,
    /// Disabled and unselected state color
    pub disabled_unselected: Srgb,
}

impl Default for StateLayer {
    fn default() -> Self {
        // Default state layer colors (light theme)
        Self {
            hover: Srgb::new(0.0, 0.0, 0.0),      // Black with 8% opacity
            focus: Srgb::new(0.0, 0.0, 0.0),      // Black with 12% opacity
            pressed: Srgb::new(0.0, 0.0, 0.0),    // Black with 12% opacity
            dragged: Srgb::new(0.0, 0.0, 0.0),    // Black with 16% opacity
            selected: Srgb::new(0.0, 0.0, 0.0),   // Black with 8% opacity
            activated: Srgb::new(0.0, 0.0, 0.0),  // Black with 12% opacity
            pressed_selected: Srgb::new(0.0, 0.0, 0.0),  // Black with 20% opacity
            selected_hover: Srgb::new(0.0, 0.0, 0.0),    // Black with 12% opacity
            selected_focus: Srgb::new(0.0, 0.0, 0.0),    // Black with 16% opacity
            disabled: Srgb::new(0.0, 0.0, 0.0),   // Black with 0% opacity (fully transparent)
            disabled_selected: Srgb::new(0.0, 0.0, 0.0), // Black with 0% opacity
            disabled_unselected: Srgb::new(0.0, 0.0, 0.0), // Black with 0% opacity
        }
    }
}

impl StateLayer {
    /// Get the opacity value for a state layer color
    pub fn get_opacity(&self, state: State) -> f32 {
        match state {
            State::Hover => 0.08,
            State::Focus => 0.12,
            State::Pressed => 0.12,
            State::Dragged => 0.16,
            State::Selected => 0.08,
            State::Activated => 0.12,
            State::PressedSelected => 0.20,
            State::SelectedHover => 0.12,
            State::SelectedFocus => 0.16,
            State::Disabled => 0.0,
            State::DisabledSelected => 0.0,
            State::DisabledUnselected => 0.0,
        }
    }
    
    /// Get the state layer color for a specific state
    pub fn get_color(&self, state: State) -> u32 {
        let color = match state {
            State::Hover => self.hover,
            State::Focus => self.focus,
            State::Pressed => self.pressed,
            State::Dragged => self.dragged,
            State::Selected => self.selected,
            State::Activated => self.activated,
            State::PressedSelected => self.pressed_selected,
            State::SelectedHover => self.selected_hover,
            State::SelectedFocus => self.selected_focus,
            State::Disabled => self.disabled,
            State::DisabledSelected => self.disabled_selected,
            State::DisabledUnselected => self.disabled_unselected,
        };
        
        let opacity = self.get_opacity(state);
        color.to_rgba(opacity)
    }
}

/// Interaction states for state layers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    /// Hover state
    Hover,
    /// Focus state
    Focus,
    /// Pressed state
    Pressed,
    /// Dragged state
    Dragged,
    /// Selected state
    Selected,
    /// Activated state
    Activated,
    /// Pressed and selected state
    PressedSelected,
    /// Selected and hovered state
    SelectedHover,
    /// Selected and focused state
    SelectedFocus,
    /// Disabled state
    Disabled,
    /// Disabled and selected state
    DisabledSelected,
    /// Disabled and unselected state
    DisabledUnselected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state_layer() {
        let state_layer = StateLayer::default();
        
        // Test get_opacity for various states
        assert_eq!(state_layer.get_opacity(State::Hover), 0.08);
        assert_eq!(state_layer.get_opacity(State::Pressed), 0.12);
        assert_eq!(state_layer.get_opacity(State::Disabled), 0.0);
        
        // Test get_color returns a valid RGBA value
        let hover_color = state_layer.get_color(State::Hover);
        assert!(hover_color > 0);
        
        // Disabled state should be fully transparent
        let disabled_color = state_layer.get_color(State::Disabled);
        assert_eq!(disabled_color & 0xFF000000, 0x00000000);
    }
}
