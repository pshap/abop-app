//! Elevation helper trait for Material Design token system
//!
//! This trait provides helper methods for working with Material Design elevation system,
//! including common elevation levels and transition utilities.

use crate::styling::material::elevation::{self, ElevationLevel};
use iced::Shadow;

/// Helper trait for elevation-related functionality
///
/// This trait provides convenient methods for accessing common elevation levels
/// and utilities for creating elevation transitions. All methods follow Material Design 3
/// elevation specifications.
pub trait ElevationHelpers {
    /// Access to the elevation system
    fn elevation(&self) -> &crate::styling::material::elevation::MaterialElevation;

    /// Get elevation style for cards (Level 1)
    ///
    /// Material Design cards typically use Level 1 elevation for subtle depth
    /// without overwhelming the content hierarchy.
    fn card_elevation(&self) -> &elevation::ElevationStyle {
        self.elevation().get_level(ElevationLevel::Level1)
    }

    /// Get elevation style for menus and dialogs (Level 2)
    ///
    /// Material Design menus and most dialogs use Level 2 elevation to appear
    /// above the main content but below high-priority surfaces.
    fn menu_elevation(&self) -> &elevation::ElevationStyle {
        self.elevation().get_level(ElevationLevel::Level2)
    }

    /// Get elevation style for floating action buttons (Level 3)
    ///
    /// Material Design FABs use Level 3 elevation by default to maintain
    /// visual hierarchy as primary action elements.
    fn fab_elevation(&self) -> &elevation::ElevationStyle {
        self.elevation().get_level(ElevationLevel::Level3)
    }

    /// Get elevation style for navigation bars (Level 2)
    ///
    /// Material Design navigation components typically use Level 2 elevation
    /// to establish clear spatial relationships with content.
    fn navigation_elevation(&self) -> &elevation::ElevationStyle {
        self.elevation().get_level(ElevationLevel::Level2)
    }

    /// Get elevation style for modals and overlays (Level 3)
    ///
    /// Material Design modals use higher elevation levels to appear above
    /// all other interface elements as they require immediate attention.
    fn modal_elevation(&self) -> &elevation::ElevationStyle {
        self.elevation().get_level(ElevationLevel::Level3)
    }

    /// Get elevation style for tooltips and temporary surfaces (Level 2)
    ///
    /// Material Design tooltips and temporary UI elements use Level 2
    /// to provide information without disrupting the main interface flow.
    fn tooltip_elevation(&self) -> &elevation::ElevationStyle {
        self.elevation().get_level(ElevationLevel::Level2)
    }

    /// Get no elevation (Level 0)
    ///
    /// For surfaces that should appear flat against the background,
    /// maintaining visual continuity with the base layer.
    fn no_elevation(&self) -> &elevation::ElevationStyle {
        self.elevation().get_level(ElevationLevel::Level0)
    }

    /// Get elevation style for a specific level (0-5)
    ///
    /// This is the most commonly used elevation utility. It provides the complete
    /// elevation style including shadow and tint for any elevation level.
    ///
    /// # Arguments
    /// * `level` - Elevation level from 0 to 5
    ///
    /// # Returns
    /// Complete elevation style with shadow and surface tint
    fn elevation_style(&self, level: u8) -> &elevation::ElevationStyle {
        let elevation_level = ElevationLevel::from_u8(level).unwrap_or(ElevationLevel::Level0);
        self.elevation().get_level(elevation_level)
    }

    /// Get just the shadow for a specific elevation level
    ///
    /// Use this when you only need the shadow part of elevation styling.
    ///
    /// # Arguments
    /// * `level` - Elevation level from 0 to 5
    ///
    /// # Returns
    /// Shadow configuration for the specified elevation level
    fn elevation_shadow(&self, level: u8) -> Shadow {
        let elevation_level = ElevationLevel::from_u8(level).unwrap_or(ElevationLevel::Level0);
        self.elevation().get_level(elevation_level).shadow
    }

    /// Get the surface tint opacity for a specific elevation level
    ///
    /// Use this when you need to apply surface tinting at specific elevation levels.
    ///
    /// # Arguments
    /// * `level` - Elevation level from 0 to 5
    ///
    /// # Returns
    /// Opacity value for surface tinting (0.0 to 1.0)
    fn elevation_tint_opacity(&self, level: u8) -> f32 {
        let elevation_level = ElevationLevel::from_u8(level).unwrap_or(ElevationLevel::Level0);
        self.elevation().get_level(elevation_level).tint_opacity
    }

    /// Get elevation transition shadows for state changes
    ///
    /// Provides smooth elevation changes for interactive components following
    /// Material Design motion principles.
    ///
    /// # Arguments
    /// * `from_level` - Starting elevation level
    /// * `to_level` - Target elevation level
    ///
    /// # Returns
    /// Tuple of (`from_shadow`, `to_shadow`) for animation transitions
    fn elevation_transition(&self, from_level: u8, to_level: u8) -> (Shadow, Shadow) {
        (
            self.elevation_shadow(from_level),
            self.elevation_shadow(to_level),
        )
    }

    /// Get elevation transition for button hover state (Level 1 -> Level 2)
    ///
    /// Material Design buttons increase elevation on hover to provide
    /// immediate visual feedback for interactive elements.
    ///
    /// # Returns
    /// Tuple of (`normal_shadow`, `hover_shadow`) for button hover animation
    fn button_hover_elevation(&self) -> (Shadow, Shadow) {
        self.elevation_transition(1, 2)
    }

    /// Get elevation transition for FAB hover state (Level 3 -> Level 4)
    ///
    /// Material Design FABs increase elevation on hover while maintaining
    /// their prominence in the interface hierarchy.
    ///
    /// # Returns
    /// Tuple of (`normal_shadow`, `hover_shadow`) for FAB hover animation
    fn fab_hover_elevation(&self) -> (Shadow, Shadow) {
        self.elevation_transition(3, 4)
    }
}
