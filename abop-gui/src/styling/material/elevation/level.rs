//! Material Design 3 elevation levels
//!
//! Defines the six standard elevation levels used in Material Design 3.
//! Each level corresponds to a specific distance in dp and has associated
//! shadow and surface tint properties.

use serde::{Deserialize, Serialize};

/// Material Design elevation levels
///
/// Defines the six standard elevation levels used in Material Design 3.
/// Each level corresponds to a specific distance in dp and has associated
/// shadow and surface tint properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElevationLevel {
    /// Surface level with no elevation (0dp)
    Level0 = 0,
    /// Raised surfaces with subtle elevation (1dp)
    Level1 = 1,
    /// Slightly raised surfaces (3dp)
    Level2 = 2,
    /// Floating action buttons and similar components (6dp)
    Level3 = 3,
    /// App bars and prominent surfaces (8dp)
    Level4 = 4,
    /// Navigation drawers and modal dialogs (12dp)
    Level5 = 5,
}

impl ElevationLevel {
    /// Get all elevation levels
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Level0,
            Self::Level1,
            Self::Level2,
            Self::Level3,
            Self::Level4,
            Self::Level5,
        ]
    }

    /// Get the DP (density-independent pixel) value for this level
    #[must_use]
    pub const fn dp(&self) -> f32 {
        match self {
            Self::Level0 => 0.0,
            Self::Level1 => 1.0,
            Self::Level2 => 3.0,
            Self::Level3 => 6.0,
            Self::Level4 => 8.0,
            Self::Level5 => 12.0,
        }
    }

    /// Get the level as a u8
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Create from u8 value
    #[must_use]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Level0),
            1 => Some(Self::Level1),
            2 => Some(Self::Level2),
            3 => Some(Self::Level3),
            4 => Some(Self::Level4),
            5 => Some(Self::Level5),
            _ => None,
        }
    }

    /// Get the level name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Level0 => "level0",
            Self::Level1 => "level1",
            Self::Level2 => "level2",
            Self::Level3 => "level3",
            Self::Level4 => "level4",
            Self::Level5 => "level5",
        }
    }
}

impl std::fmt::Display for ElevationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Level {} ({}dp)", self.as_u8(), self.dp())
    }
}
