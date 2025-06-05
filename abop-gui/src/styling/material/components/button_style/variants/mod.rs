//! Button variant strategy implementations
//!
//! This module contains the concrete implementations of `ButtonStyleStrategy`
//! for each Material Design 3 button variant.

use iced::{self, Border, Color, Shadow};

pub mod elevated;
pub mod fab;
pub mod filled;
pub mod filled_tonal;
pub mod icon;
pub mod outlined;
pub mod text;

pub use elevated::ElevatedButtonStrategy;
pub use fab::FabButtonStrategy;
pub use filled::FilledButtonStrategy;
pub use filled_tonal::FilledTonalButtonStrategy;
pub use icon::IconButtonStrategy;
pub use outlined::OutlinedButtonStrategy;
pub use text::TextButtonStrategy;

/// Button styling variants for different Material Design button types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyleVariant {
    /// High emphasis filled button for primary actions
    Filled,
    /// Medium emphasis filled tonal button for secondary actions
    FilledTonal,
    /// Medium emphasis outlined button with border
    Outlined,
    /// Low emphasis text-only button
    Text,
    /// High emphasis elevated button with shadow
    Elevated,
    /// Compact icon-only button
    Icon,
    /// Floating action button for primary screen actions
    Fab,
}

/// Button size variants affecting dimensions and padding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSizeVariant {
    /// Small button size (compact appearance)
    Small,
    /// Medium button size (default)
    Medium,
    /// Large button size (prominent appearance)
    Large,
}

impl ButtonStyleVariant {
    /// Get the appropriate strategy for this variant
    #[must_use]
    pub fn get_strategy(self) -> Box<dyn super::strategy::ButtonStyleStrategy> {
        create_strategy(self)
    }
}

/// Factory function to create strategy instances
#[must_use]
pub fn create_strategy(
    variant: ButtonStyleVariant,
) -> Box<dyn super::strategy::ButtonStyleStrategy> {
    use ButtonStyleVariant::{Elevated, Fab, Filled, FilledTonal, Icon, Outlined, Text};

    match variant {
        Filled => Box::new(FilledButtonStrategy),
        FilledTonal => Box::new(FilledTonalButtonStrategy),
        Outlined => Box::new(OutlinedButtonStrategy),
        Text => Box::new(TextButtonStrategy),
        Elevated => Box::new(ElevatedButtonStrategy),
        Icon => Box::new(IconButtonStrategy),
        Fab => Box::new(FabButtonStrategy),
    }
}

/// Helper function to create standard button borders
pub(crate) fn create_button_border(color: Color, width: f32, radius: f32) -> Border {
    Border {
        color,
        width,
        radius: radius.into(),
    }
}

/// Helper function to create button shadows
#[allow(dead_code)]
const fn create_button_shadow(_elevation_level: f32) -> Option<Shadow> {
    None
}
