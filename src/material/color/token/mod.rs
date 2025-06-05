//! Color tokens for Material Design 3
//!
//! This module contains the color token definitions for the Material Design 3
//! color system. Tokens are organized by their semantic meaning and usage context.

mod core;
mod surface;
mod container;
mod fixed;
mod state;

pub use self::core::CoreTokens;
pub use self::surface::SurfaceTokens;
pub use self::container::ContainerTokens;
pub use self::fixed::FixedTokens;
pub use self::state::StateLayer;

/// Collection of all color tokens
#[derive(Debug, Clone)]
pub struct ColorTokens {
    /// Core semantic color tokens
    pub core: CoreTokens,
    /// Surface color tokens
    pub surface: SurfaceTokens,
    /// Container color tokens
    pub container: ContainerTokens,
    /// Fixed color tokens
    pub fixed: FixedTokens,
}

impl Default for ColorTokens {
    fn default() -> Self {
        Self {
            core: CoreTokens::default(),
            surface: SurfaceTokens::default(),
            container: ContainerTokens::default(),
            fixed: FixedTokens::default(),
        }
    }
}
