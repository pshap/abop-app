//! Unified state behavior traits for selection components
//!
//! This module provides common behavior patterns that can be applied
//! across different component state types, promoting consistency and
//! reducing code duplication.

use super::constants;

/// Common behavior for all component states
///
/// This trait unifies the behavior of CheckboxState, SwitchState, and ChipState
/// by providing a consistent interface for state transitions and queries.
pub trait ComponentState: Copy + PartialEq + Default + std::fmt::Debug {
    /// Toggle the state to its opposite/next logical state
    fn toggle(self) -> Self;
    
    /// Check if the state represents an "active" or "selected" condition
    /// 
    /// For checkboxes: checked or indeterminate
    /// For switches: on
    /// For chips: selected
    fn is_active(self) -> bool;
    
    /// Convert to boolean representation
    /// 
    /// For checkboxes: true only if checked (not indeterminate)
    /// For switches: true if on
    /// For chips: true if selected
    fn to_bool(self) -> bool;
    
    /// Create from boolean value
    fn from_bool(value: bool) -> Self;
}

/// Extended behavior for states that support multiple selection levels
///
/// This trait is implemented by states that have more than just on/off,
/// such as CheckboxState with its indeterminate state.
pub trait MultiLevelState: ComponentState {
    /// Check if the state is in an intermediate/partial state
    fn is_intermediate(self) -> bool;
    
    /// Get all possible states for this type
    fn all_states() -> &'static [Self];
    
    /// Get the next state in sequence (useful for cycling through states)
    fn next_state(self) -> Self;
}

/// Behavior for states that support press/interaction feedback
///
/// This trait is implemented by states that can show pressed/interaction
/// states for animation and user feedback purposes.
pub trait InteractiveState: ComponentState {
    /// Check if the state represents a pressed/active interaction
    fn is_pressed(self) -> bool;
    
    /// Get the pressed version of this state (if applicable)
    fn to_pressed(self) -> Self;
    
    /// Get the non-pressed version of this state
    fn to_unpressed(self) -> Self;
}

/// Extended behavior for states that support animation transitions
///
/// This trait provides animation-specific functionality for components
/// that need smooth transitions between states.
pub trait AnimatableState: ComponentState {
    /// Check if animation should be enabled for this state transition
    fn should_animate_to(self, target: Self) -> bool {
        self != target // Default: animate if states are different
    }
      /// Get the animation duration for transitioning to the target state
    fn animation_duration_to(self, target: Self) -> std::time::Duration {
        std::time::Duration::from_millis(constants::animation::DEFAULT_DURATION_MS)
    }
    
    /// Check if this state can be interrupted by another transition
    fn is_interruptible(self) -> bool {
        true // Default: all states are interruptible
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock state for testing the traits
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum MockState {
        Inactive,
        Active,
        Pressed,
    }
    
    impl Default for MockState {
        fn default() -> Self {
            Self::Inactive
        }
    }
    
    impl ComponentState for MockState {
        fn toggle(self) -> Self {
            match self {
                Self::Inactive => Self::Active,
                Self::Active | Self::Pressed => Self::Inactive,
            }
        }
        
        fn is_active(self) -> bool {
            matches!(self, Self::Active | Self::Pressed)
        }
        
        fn to_bool(self) -> bool {
            matches!(self, Self::Active)
        }
        
        fn from_bool(value: bool) -> Self {
            if value { Self::Active } else { Self::Inactive }
        }
    }
    
    impl InteractiveState for MockState {
        fn is_pressed(self) -> bool {
            matches!(self, Self::Pressed)
        }
        
        fn to_pressed(self) -> Self {
            match self {
                Self::Active => Self::Pressed,
                other => other,
            }
        }
        
        fn to_unpressed(self) -> Self {
            match self {
                Self::Pressed => Self::Active,
                other => other,
            }
        }
    }
    
    #[test]
    fn test_component_state_trait() {
        let state = MockState::default();
        assert_eq!(state, MockState::Inactive);
        assert!(!state.is_active());
        assert!(!state.to_bool());
        
        let toggled = state.toggle();
        assert_eq!(toggled, MockState::Active);
        assert!(toggled.is_active());
        assert!(toggled.to_bool());
        
        let from_bool = MockState::from_bool(true);
        assert_eq!(from_bool, MockState::Active);
    }
    
    #[test]
    fn test_interactive_state_trait() {
        let active = MockState::Active;
        assert!(!active.is_pressed());
        
        let pressed = active.to_pressed();
        assert!(pressed.is_pressed());
        assert_eq!(pressed, MockState::Pressed);
        
        let unpressed = pressed.to_unpressed();
        assert!(!unpressed.is_pressed());
        assert_eq!(unpressed, MockState::Active);
    }
}
