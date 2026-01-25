//! Card Component
//!
//! This module provides Material Design 3 Card component with
//! different variants (elevated, filled, outlined) and elevation levels.

use iced::{Background, Color, Element, Length, Padding};

/// Card elevation level (0-5)
pub type ElevationLevel = u8;

/// Material Design 3 Card component
#[derive(Debug, Clone)]
pub struct MaterialCard {
    /// The elevation level of the card (0-5)
    pub elevation: ElevationLevel,
    /// The variant of the card
    pub variant: CardVariant,
    /// Optional padding override
    pub padding: Option<Padding>,
    /// Optional width override
    pub width: Option<Length>,
    /// Optional height override
    pub height: Option<Length>,
}

/// Card variants following Material Design 3 specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardVariant {
    /// Elevated card with shadow and no outline
    Elevated,
    /// Filled card with surface color and no shadow
    Filled,
    /// Outlined card with border and no shadow
    Outlined,
}

impl Default for MaterialCard {
    fn default() -> Self {
        Self {
            elevation: 1,
            variant: CardVariant::Elevated,
            padding: None,
            width: None,
            height: None,
        }
    }
}
