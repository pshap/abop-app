//! Material Design 3 Container Components
//!
//! This module provides Material Design 3 container components including cards, surfaces,
//! dividers, and layout containers that integrate with the Iced framework.

use iced::{
    Background, Border, Element, Length, Padding, Shadow, Theme, Vector,
    widget::{Column, Row, container, rule},
};

use crate::styling::material::{MaterialTokens, elevation::ElevationLevel};
use crate::theme::ThemeMode;

/// Get the container style based on theme and theme mode
pub fn container_style(theme: &Theme, theme_mode: ThemeMode) -> container::Style {
    let colors = match theme_mode {
        ThemeMode::Light => theme.palette().background,
        ThemeMode::Dark => theme.palette().background,
        ThemeMode::System => theme.palette().background,
        ThemeMode::MaterialLight => theme.palette().background,
        ThemeMode::MaterialDark => theme.palette().background,
        ThemeMode::MaterialDynamic => theme.palette().background,
    };

    container::Style {
        background: Some(Background::Color(colors)),
        border: Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Material Design 3 Card component
///
/// Cards contain content and actions about a single subject. They provide a flexible
/// and extensible content container with multiple variants and elevation levels.
#[derive(Debug, Clone)]
pub struct MaterialCard {
    /// The elevation level of the card (0-5)
    pub elevation: u8,
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

impl MaterialCard {
    /// Create a new Material card with default styling
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the elevation level (0-5)
    #[must_use]
    pub fn elevation(mut self, elevation: u8) -> Self {
        self.elevation = elevation.min(5);
        self
    }

    /// Set the card variant
    #[must_use]
    pub const fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set custom padding
    #[must_use]
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Set the width
    #[must_use]
    pub const fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height
    #[must_use]
    pub const fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }
    /// Apply the card styling to a container
    #[must_use]
    pub fn style(&self, tokens: &MaterialTokens) -> container::Style {
        let colors = &tokens.colors;
        let elevation = &tokens.elevation;
        let shapes = &tokens.shapes;

        let elevation_level =
            ElevationLevel::from_u8(self.elevation).unwrap_or(ElevationLevel::Level1);

        match self.variant {
            CardVariant::Elevated => container::Style {
                background: Some(Background::Color(colors.surface_container_low)),
                border: Border {
                    radius: shapes.corner_medium.to_radius(),
                    ..Default::default()
                },
                shadow: elevation.get_level(elevation_level).shadow,
                ..Default::default()
            },
            CardVariant::Filled => container::Style {
                background: Some(Background::Color(colors.surface_container_highest)),
                border: Border {
                    radius: shapes.corner_medium.to_radius(),
                    ..Default::default()
                },
                ..Default::default()
            },
            CardVariant::Outlined => container::Style {
                background: Some(Background::Color(colors.surface)),
                border: Border {
                    radius: shapes.corner_medium.to_radius(),
                    width: 1.0,
                    color: colors.outline_variant,
                },
                ..Default::default()
            },
        }
    }
}

/// Material Design 3 Surface component
///
/// Surfaces are the foundational layer of Material Design. They provide background
/// colors and elevation that help organize content and establish hierarchy.
#[derive(Debug, Clone)]
pub struct MaterialSurface {
    /// The elevation level of the surface (0-5)
    pub elevation: u8,
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
    pub fn elevation(mut self, elevation: u8) -> Self {
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
    pub fn style(&self, tokens: &MaterialTokens) -> container::Style {
        let colors = &tokens.colors;
        let elevation = &tokens.elevation;

        // Use the correct MaterialColors fields for each surface variant
        let background_color = match self.variant {
            SurfaceVariant::Surface => colors.surface,
            SurfaceVariant::SurfaceContainer => colors.surface_container,
            SurfaceVariant::SurfaceContainerLow => colors.surface_container_low,
            SurfaceVariant::SurfaceContainerLowest => colors.surface_container_lowest,
            SurfaceVariant::SurfaceContainerHigh => colors.surface_container_high,
            SurfaceVariant::SurfaceContainerHighest => colors.surface_container_highest,
            SurfaceVariant::InverseSurface => colors.inverse_surface,
        };

        let mut style = container::Style {
            background: Some(Background::Color(background_color)),
            ..Default::default()
        };

        // Add shadow for elevated surfaces
        if self.elevation > 0 {
            let elevation_level =
                ElevationLevel::from_u8(self.elevation).unwrap_or(ElevationLevel::Level1);
            style.shadow = Shadow {
                color: colors.shadow,
                offset: Vector::new(0.0, elevation.get_level(elevation_level).shadow.offset.y),
                blur_radius: elevation.get_level(elevation_level).shadow.blur_radius,
            };
        }

        style
    }
    /// Create a container element with the surface styling
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

/// Material Design 3 Divider component
///
/// Dividers are thin lines that group content in lists and layouts. They help
/// establish hierarchy and improve readability.
#[derive(Debug, Clone)]
pub struct MaterialDivider {
    /// The orientation of the divider
    pub orientation: DividerOrientation,
    /// The thickness of the divider
    pub thickness: f32,
    /// The length of the divider
    pub length: Length,
    /// Whether to use full width/height
    pub full_bleed: bool,
}

/// Divider orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerOrientation {
    /// Horizontal divider
    Horizontal,
    /// Vertical divider
    Vertical,
}

impl Default for MaterialDivider {
    fn default() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            thickness: 1.0,
            length: Length::Fill,
            full_bleed: false,
        }
    }
}

impl MaterialDivider {
    /// Create a new Material divider
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a horizontal divider
    #[must_use]
    pub fn horizontal() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            ..Self::default()
        }
    }

    /// Create a vertical divider
    #[must_use]
    pub fn vertical() -> Self {
        Self {
            orientation: DividerOrientation::Vertical,
            ..Self::default()
        }
    }

    /// Set the thickness of the divider
    #[must_use]
    pub const fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set the length of the divider
    #[must_use]
    pub const fn length(mut self, length: Length) -> Self {
        self.length = length;
        self
    }
    /// Set whether the divider should be full bleed
    #[must_use]
    pub const fn full_bleed(mut self, full_bleed: bool) -> Self {
        self.full_bleed = full_bleed;
        self
    }

    /// Create a rule element with the divider styling
    #[must_use]
    pub fn rule<'a, Message: 'a>(&self, tokens: &MaterialTokens) -> Element<'a, Message> {
        let colors = &tokens.colors;
        let outline_color = colors.outline_variant;
        let thickness = self.thickness;

        match self.orientation {
            DividerOrientation::Horizontal => rule::Rule::horizontal(thickness)
                .style(move |_theme: &Theme| rule::Style {
                    color: outline_color,
                    width: thickness.round().clamp(0.0, f32::from(u16::MAX)) as u16,
                    radius: 0.0.into(),
                    fill_mode: rule::FillMode::Full,
                })
                .into(),
            DividerOrientation::Vertical => rule::Rule::vertical(thickness)
                .style(move |_theme: &Theme| rule::Style {
                    color: outline_color,
                    width: thickness.round().clamp(0.0, f32::from(u16::MAX)) as u16,
                    radius: 0.0.into(),
                    fill_mode: rule::FillMode::Full,
                })
                .into(),
        }
    }
}

/// Material Design 3 Container component
///
/// A flexible container that applies Material Design spacing, padding, and styling
/// while providing layout capabilities.
#[derive(Debug, Clone)]
pub struct MaterialContainer {
    /// Optional background surface variant
    pub surface_variant: Option<SurfaceVariant>,
    /// Container padding
    pub padding: Padding,
    /// Container spacing for child elements
    pub spacing: f32,
    /// Optional width
    pub width: Option<Length>,
    /// Optional height
    pub height: Option<Length>,
    /// Container alignment
    pub alignment: ContainerAlignment,
}

/// Container alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerAlignment {
    /// Align items to the start
    Start,
    /// Center items
    Center,
    /// Align items to the end
    End,
    /// Stretch items to fill
    Fill,
}

impl Default for MaterialContainer {
    fn default() -> Self {
        Self {
            surface_variant: None,
            padding: Padding::from(16),
            spacing: 16.0,
            width: None,
            height: None,
            alignment: ContainerAlignment::Fill,
        }
    }
}

impl MaterialContainer {
    /// Create a new Material container
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the surface variant
    #[must_use]
    pub const fn surface_variant(mut self, variant: SurfaceVariant) -> Self {
        self.surface_variant = Some(variant);
        self
    }

    /// Set the container padding
    #[must_use]
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Set the spacing between child elements
    #[must_use]
    pub const fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the container width
    #[must_use]
    pub const fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the container height
    #[must_use]
    pub const fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the container alignment
    #[must_use]
    pub const fn alignment(mut self, alignment: ContainerAlignment) -> Self {
        self.alignment = alignment;
        self
    }
    /// Create a column container with Material styling
    #[must_use]
    pub fn column<'a, Message: Clone + 'a>(
        &self,
        children: Vec<Element<'a, Message>>,
        tokens: &MaterialTokens,
    ) -> Element<'a, Message> {
        let mut column = Column::with_children(children).spacing(self.spacing); // Apply alignment
        match self.alignment {
            ContainerAlignment::Start => column = column.align_x(iced::Alignment::Start),
            ContainerAlignment::Center => column = column.align_x(iced::Alignment::Center),
            ContainerAlignment::End => column = column.align_x(iced::Alignment::End),
            ContainerAlignment::Fill => {} // Fill is default for columns
        }

        if let Some(width) = self.width {
            column = column.width(width);
        }

        if let Some(height) = self.height {
            column = column.height(height);
        }

        // Wrap in surface container if specified
        if let Some(surface_variant) = self.surface_variant {
            MaterialSurface::new()
                .variant(surface_variant)
                .padding(self.padding)
                .container(column.into(), tokens)
        } else {
            container(column).padding(self.padding).into()
        }
    }
    /// Create a row container with Material styling
    #[must_use]
    pub fn row<'a, Message: Clone + 'a>(
        &self,
        children: Vec<Element<'a, Message>>,
        tokens: &MaterialTokens,
    ) -> Element<'a, Message> {
        let mut row = Row::with_children(children).spacing(self.spacing); // Apply alignment
        match self.alignment {
            ContainerAlignment::Start => row = row.align_y(iced::Alignment::Start),
            ContainerAlignment::Center => row = row.align_y(iced::Alignment::Center),
            ContainerAlignment::End => row = row.align_y(iced::Alignment::End),
            ContainerAlignment::Fill => {} // Fill is default for rows
        }

        if let Some(width) = self.width {
            row = row.width(width);
        }

        if let Some(height) = self.height {
            row = row.height(height);
        }

        // Wrap in surface container if specified
        if let Some(surface_variant) = self.surface_variant {
            MaterialSurface::new()
                .variant(surface_variant)
                .padding(self.padding)
                .container(row.into(), tokens)
        } else {
            container(row).padding(self.padding).into()
        }
    }
}

/// Helper functions for creating Material container components
impl MaterialTokens {
    /// Create a Material surface with the current tokens
    #[must_use]
    pub fn surface(&self) -> MaterialSurface {
        MaterialSurface::new()
    }

    /// Create a Material divider with the current tokens
    #[must_use]
    pub fn divider(&self) -> MaterialDivider {
        MaterialDivider::new()
    }

    /// Create a Material container with the current tokens
    #[must_use]
    pub fn container(&self) -> MaterialContainer {
        MaterialContainer::new()
    }
}
