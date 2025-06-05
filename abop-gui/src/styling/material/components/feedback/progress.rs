//! Material Design 3 Progress Indicators
//!
//! This module provides Material Design 3 progress indicator components including
//! linear and circular variants with determinate and indeterminate states.

use iced::{
    Alignment, Background, Border, Color, Element,
    widget::{Column, Space, container, progress_bar, text},
};

use crate::styling::material::MaterialTokens;

/// Material Design 3 Progress Indicator variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressVariant {
    /// Linear progress bar (horizontal)
    Linear,
    /// Circular progress indicator
    Circular,
}

/// Progress indicator state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressState {
    /// Determinate progress with known completion percentage
    Determinate,
    /// Indeterminate progress with unknown completion time
    Indeterminate,
}

/// Progress indicator size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressSize {
    /// Small progress indicator (16px for circular, 2px height for linear)
    Small,
    /// Medium progress indicator (24px for circular, 4px height for linear)
    Medium,
    /// Large progress indicator (32px for circular, 6px height for linear)
    Large,
}

/// Material Design 3 Progress Indicator
///
/// Progress indicators inform users about the status of ongoing processes,
/// such as loading an app, submitting a form, or saving updates.
#[derive(Debug, Clone)]
pub struct MaterialProgressIndicator {
    variant: ProgressVariant,
    state: ProgressState,
    size: ProgressSize,
    progress: f32, // 0.0 to 1.0 for determinate, ignored for indeterminate
    color_override: Option<Color>,
    track_color_override: Option<Color>,
    with_label: bool,
    label_text: Option<String>,
}

impl Default for MaterialProgressIndicator {
    fn default() -> Self {
        Self {
            variant: ProgressVariant::Linear,
            state: ProgressState::Determinate,
            size: ProgressSize::Medium,
            progress: 0.0,
            color_override: None,
            track_color_override: None,
            with_label: false,
            label_text: None,
        }
    }
}

impl MaterialProgressIndicator {
    /// Create a new linear progress indicator
    #[must_use]
    pub fn linear() -> Self {
        Self {
            variant: ProgressVariant::Linear,
            ..Default::default()
        }
    }

    /// Create a new circular progress indicator
    #[must_use]
    pub fn circular() -> Self {
        Self {
            variant: ProgressVariant::Circular,
            ..Default::default()
        }
    }

    /// Set the progress state
    #[must_use]
    pub const fn state(mut self, state: ProgressState) -> Self {
        self.state = state;
        self
    }

    /// Set the progress size
    #[must_use]
    pub const fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self
    }

    /// Set the progress value (0.0 to 1.0)
    #[must_use]
    pub const fn progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Override the progress color
    #[must_use]
    pub const fn color(mut self, color: Color) -> Self {
        self.color_override = Some(color);
        self
    }

    /// Override the track color
    #[must_use]
    pub const fn track_color(mut self, color: Color) -> Self {
        self.track_color_override = Some(color);
        self
    }

    /// Add a label to the progress indicator
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.with_label = true;
        self.label_text = Some(label.into());
        self
    }

    /// Create the progress indicator element
    #[must_use]
    pub fn view<'a, Message>(&'a self, tokens: &'a MaterialTokens) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let progress_color = self.color_override.unwrap_or(tokens.colors.primary.base);
        let track_color = self
            .track_color_override
            .unwrap_or(tokens.colors.surface_variant);

        let progress_element: Element<'a, Message> = match self.variant {
            ProgressVariant::Linear => {
                let height = match self.size {
                    ProgressSize::Small => 2.0,
                    ProgressSize::Medium => 4.0,
                    ProgressSize::Large => 6.0,
                };

                progress_bar(0.0..=1.0, self.progress).height(height).into()
            }
            ProgressVariant::Circular => {
                let size = match self.size {
                    ProgressSize::Small => 16.0,
                    ProgressSize::Medium => 24.0,
                    ProgressSize::Large => 32.0,
                };

                container(Space::new(size, size))
                    .style({
                        move |_theme| container::Style {
                            background: Some(Background::Color(track_color)),
                            border: Border {
                                color: progress_color,
                                width: 2.0,
                                radius: (size / 2.0).into(),
                            },
                            ..Default::default()
                        }
                    })
                    .into()
            }
        };

        if self.with_label {
            Column::new()
                .push(progress_element)
                .push(Space::new(0, 8))
                .push(
                    text(self.label_text.as_deref().unwrap_or("Loading..."))
                        .size(tokens.typography.label_medium.size)
                        .color(tokens.colors.on_surface),
                )
                .align_x(Alignment::Center)
                .into()
        } else {
            progress_element
        }
    }
}

/// Extension methods for `MaterialTokens`
impl MaterialTokens {
    /// Create a linear progress indicator with default styling
    #[must_use]
    pub fn linear_progress(&self) -> MaterialProgressIndicator {
        MaterialProgressIndicator::linear()
    }

    /// Create a circular progress indicator with default styling
    #[must_use]
    pub fn circular_progress(&self) -> MaterialProgressIndicator {
        MaterialProgressIndicator::circular()
    }
}
