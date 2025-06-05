//! Shape families and app style definitions for the Material Design 3 Shape System
//!
//! This module defines different shape families that can be applied consistently
//! across an application to create different visual personalities and styles.

/// Shape families for different component categories
///
/// Defines three shape families that can be applied consistently across
/// an application to create different visual personalities and styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeFamily {
    /// Sharp corners (no rounding) for focus, hierarchy, and serious applications
    Sharp,
    /// Rounded corners (standard rounding) for friendliness and approachability
    Rounded,
    /// Fully rounded corners for playful, organic, and casual applications
    Circular,
}

/// App style categories
///
/// Defines different application personality styles that influence
/// the choice of shape family throughout the interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppStyle {
    /// Professional style with sharp, focused aesthetics
    Professional,
    /// Friendly style with approachable, rounded aesthetics
    Friendly,
    /// Playful style with organic, circular aesthetics
    Playful,
}

impl AppStyle {
    /// Get the recommended shape family for this app style
    #[must_use]
    pub const fn recommended_family(&self) -> ShapeFamily {
        match self {
            Self::Professional => ShapeFamily::Sharp,
            Self::Friendly => ShapeFamily::Rounded,
            Self::Playful => ShapeFamily::Circular,
        }
    }
}

impl ShapeFamily {
    /// Get all shape families
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[Self::Sharp, Self::Rounded, Self::Circular]
    }

    /// Get the family name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Sharp => "sharp",
            Self::Rounded => "rounded",
            Self::Circular => "circular",
        }
    }

    /// Get description of the shape family
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Sharp => "Sharp corners for focus, hierarchy, and serious applications",
            Self::Rounded => "Rounded corners for friendliness and approachability",
            Self::Circular => "Fully rounded corners for playful, organic, and casual applications",
        }
    }
}

impl AppStyle {
    /// Get all app styles
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[Self::Professional, Self::Friendly, Self::Playful]
    }

    /// Get the style name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Professional => "professional",
            Self::Friendly => "friendly",
            Self::Playful => "playful",
        }
    }

    /// Get description of the app style
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Professional => "Professional style with sharp, focused aesthetics",
            Self::Friendly => "Friendly style with approachable, rounded aesthetics",
            Self::Playful => "Playful style with organic, circular aesthetics",
        }
    }
}
