//! Surface Component
//!
//! This module provides Material Design 3 Surface component with
//! different variants and elevation levels for consistent visual hierarchy.

use iced::{Background, Color, Element, Length, Padding, Theme, Vector};

/// Surface elevation level (0-5)
pub type ElevationLevel = u8;

/// Material Design 3 Surface component
#[derive(Debug, Clone)]
pub struct MaterialSurface {
    /// The elevation level of the surface (0-5)
    pub elevation: ElevationLevel,
    /// The surface variant to use
    pub variant: SurfaceVariant,
    /// Optional padding
    pub padding: Option<Padding>,
}

/// Surface variants following Material Design 3 specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceVariant {
    /// Base surface color
    Surface,
    /// Slightly elevated surface
    SurfaceContainer,
    /// Low elevation surface container
    SurfaceContainerLow,
    /// Lowest elevation surface container
    SurfaceContainerLowest,
    /// High elevation surface container
    SurfaceContainerHigh,
    /// Highest elevation surface container
    SurfaceContainerHighest,
    /// Inverse surface for high contrast
    InverseSurface,
}

impl Default for MaterialSurface {
    fn default() -> Self {
        Self {
            elevation: 0,
            variant: SurfaceVariant::Surface,
            padding: None,
        }
    }
}

impl MaterialSurface {
    /// Create a new Material surface with default styling
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the elevation level (0-5)
    #[must_use]
    pub fn elevation(mut self, elevation: ElevationLevel) -> Self {
        self.elevation = elevation.min(5);
        self
    }

    /// Set the surface variant
    #[must_use]
    pub const fn variant(mut self, variant: SurfaceVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set custom padding
    #[must_use]
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Apply the surface styling to a container
    #[must_use]
    pub fn container<'a, Message: Clone + 'a>(
        &self,
        content: Element<'a, Message>,
        tokens: &MaterialTokens,
    ) -> Element<'a, Message> {
        let style = self.style(tokens);

        let mut container = container(content).style(move |_theme: &Theme| style);

        if let Some(padding) = self.padding {
            container = container.padding(padding);
        }

        container.into()
    }
}
