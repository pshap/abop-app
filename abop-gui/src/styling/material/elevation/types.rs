//! Core type definitions for the Material Design 3 elevation system
//!
//! This module contains the fundamental types used throughout the elevation system,
//! including strong newtypes for type safety and utility types for internal operations.

use iced::Color;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Strong newtype for density-independent pixels (dp)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Dp(pub f32);

impl fmt::Display for Dp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}dp", self.0)
    }
}

impl Dp {
    /// Get the underlying f32 value
    #[must_use]
    pub const fn as_f32(&self) -> f32 {
        self.0
    }

    /// Clamp the dp value between min and max
    #[must_use]
    pub const fn clamp(self, min: f32, max: f32) -> Self {
        Self(self.0.clamp(min, max))
    }
}

/// Strong newtype for opacity (0.0-1.0)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Opacity(pub f32);

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl Opacity {
    /// Get the underlying f32 value
    #[must_use]
    pub const fn as_f32(&self) -> f32 {
        self.0
    }

    /// Clamp the opacity value between min and max
    #[must_use]
    pub const fn clamp(self, min: f32, max: f32) -> Self {
        Self(self.0.clamp(min, max))
    }
}

/// Wrapper for `iced::Color` to make it hashable
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorKey(pub Color);

impl std::hash::Hash for ColorKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the RGBA components as u32 for consistency
        (self.0.r.to_bits()).hash(state);
        (self.0.g.to_bits()).hash(state);
        (self.0.b.to_bits()).hash(state);
        (self.0.a.to_bits()).hash(state);
    }
}

impl Eq for ColorKey {}

impl From<Color> for ColorKey {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl From<ColorKey> for Color {
    fn from(key: ColorKey) -> Self {
        key.0
    }
}

/// Shadow configuration parameters for Material Design elevation levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowParams {
    /// The vertical offset of the shadow in pixels
    pub offset_y: f32,
    /// The blur radius of the shadow in pixels
    pub blur_radius: f32,
    /// The opacity of the shadow (0.0-1.0)
    pub opacity: f32,
}
