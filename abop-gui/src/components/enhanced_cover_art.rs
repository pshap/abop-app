//! Enhanced cover art display component for audiobooks
//!
//! This module provides an enhanced component for displaying audiobook cover art
//! with loading states, edit functionality, and improved visual feedback.

use iced::widget::{button, column, container, image, row, text};
use iced::{Element, Length, Padding, Theme};
use std::sync::Arc;

use crate::utils::image_cache::ImageCache;
use crate::styling::material::MaterialTokens;
use crate::messages::Message;

/// Enhanced cover art display component with loading states and edit functionality
#[derive(Debug, Clone)]
pub struct EnhancedCoverArt {
    /// The image handle to display
    handle: iced::widget::image::Handle,
    /// Size of the cover art in pixels
    size: u32,
    /// Whether this is a placeholder
    is_placeholder: bool,
    /// Whether the image is loading
    is_loading: bool,
    /// Whether to show the edit overlay
    show_edit_overlay: bool,
    /// Optional title for accessibility
    title: Option<String>,
    /// Optional ID of the audiobook this cover belongs to
    audiobook_id: Option<String>,
}

impl EnhancedCoverArt {
    /// Create a new cover art component from image data
    pub fn from_data(
        image_data: Option<&[u8]>,
        size: u32,
        cache: &Arc<ImageCache>,
        audiobook_id: &str,
    ) -> Self {
        let handle = cache.get_or_create_handle(audiobook_id, image_data, size);
        let is_placeholder = image_data.is_none();

        Self {
            handle,
            size,
            is_placeholder,
            is_loading: false,
            show_edit_overlay: false,
            title: None,
            audiobook_id: Some(audiobook_id.to_string()),
        }
    }

    /// Create a loading skeleton for cover art
    pub fn loading(size: u32) -> Self {
        Self {
            handle: iced::widget::image::Handle::from_pixels(1, 1, vec![0, 0, 0, 0]),
            size,
            is_placeholder: true,
            is_loading: true,
            show_edit_overlay: false,
            title: Some("Loading...".to_string()),
            audiobook_id: None,
        }
    }

    /// Create a placeholder cover art component
    pub fn placeholder(size: u32) -> Self {
        let cache = Arc::new(ImageCache::new());
        let handle = cache.create_placeholder_handle(size);

        Self {
            handle,
            size,
            is_placeholder: true,
            is_loading: false,
            show_edit_overlay: false,
            title: Some("No cover art".to_string()),
            audiobook_id: None,
        }
    }

    /// Set the title for accessibility
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the loading state
    pub fn set_loading(mut self, loading: bool) -> Self {
        self.is_loading = loading;
        self
    }

    /// Show/hide the edit overlay
    pub fn show_edit_overlay(mut self, show: bool) -> Self {
        self.show_edit_overlay = show;
        self
    }

    /// Create the view for this cover art component
    pub fn view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        // Create the base image or placeholder
        let mut content = if self.is_loading {
            // Show loading skeleton
            container(
                container(
                    text("")
                        .width(Length::Fixed(self.size as f32))
                        .height(Length::Fixed(self.size as f32)),
                )
                .style(container_skeleton_style(tokens)),
            )
            .width(Length::Fixed(self.size as f32))
            .height(Length::Fixed(self.size as f32))
        } else if self.is_placeholder {
            // Show placeholder
            container(
                container(
                    text("🎵")
                        .size(self.size as f32 * 0.6)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(container_placeholder_style(tokens)),
            )
            .width(Length::Fixed(self.size as f32))
            .height(Length::Fixed(self.size as f32))
        } else {
            // Show actual cover art
            container(
                container(
                    image(self.handle.clone())
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(container_image_style(tokens)),
            )
            .width(Length::Fixed(f32::from(self.size)))
            .height(Length::Fixed(f32::from(self.size)))
        };

        // Add hover effects and edit overlay if enabled
        if self.show_edit_overlay && !self.is_loading {
            let edit_overlay = container(
                column![
                    button("Change")
                        .on_press(Message::EditCoverArt(self.audiobook_id.clone().unwrap_or_default()))
                        .style(button_style(tokens)),
                    button("Remove")
                        .on_press(Message::RemoveCoverArt(self.audiobook_id.clone().unwrap_or_default()))
                        .style(button_style(tokens)),
                ]
                .spacing(8),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(container_overlay_style(tokens));

            content = container(
                container(
                    column![
                        content,
                        container(edit_overlay)
                            .width(Length::Fill)
                            .height(Length::Fill)
                    ]
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .style(container_hover_style(tokens)),
            )
            .width(Length::Fixed(self.size as f32))
            .height(Length::Fixed(self.size as f32));
        }

        // Add padding, rounded corners, and accessibility attributes
        container(content)
            .padding(Padding::from(4.0))
            .style(container_style(tokens))
            .into()
    }

    /// Get the size of this cover art component
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Check if this is a placeholder
    pub fn is_placeholder(&self) -> bool {
        self.is_placeholder
    }

    /// Check if this is loading
    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    /// Get the title for accessibility
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

// Style functions for the cover art component
fn container_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |theme: &Theme| {
        let palette = theme.extended_palette();
        container::Appearance {
            background: Some(iced::Background::Color(palette.background.weak.color)),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: palette.background.strong.color,
            ..Default::default()
        }
    }))
}

fn container_skeleton_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |theme: &Theme| {
        let palette = theme.extended_palette();
        container::Appearance {
            background: Some(iced::Background::Color(palette.background.weak.color)),
            border_radius: 4.0.into(),
            ..Default::default()
        }
    }))
}

fn container_placeholder_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |theme: &Theme| {
        let palette = theme.extended_palette();
        container::Appearance {
            background: Some(iced::Background::Color(palette.background.weak.color)),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: palette.background.strong.color,
            ..Default::default()
        }
    }))
}

fn container_image_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |_theme: &Theme| {
        container::Appearance {
            border_radius: 4.0.into(),
            ..Default::default()
        }
    }))
}

fn container_overlay_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |theme: &Theme| {
        let palette = theme.extended_palette();
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.7))),
            border_radius: 4.0.into(),
            ..Default::default()
        }
    }))
}

fn container_hover_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |theme: &Theme| {
        container::Appearance {
            border_radius: 4.0.into(),
            ..Default::default()
        }
    }))
}

fn button_style(tokens: &MaterialTokens) -> iced::theme::Button {
    iced::theme::Button::Custom(Box::new(move |theme: &Theme| {
        let palette = theme.extended_palette();
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(palette.primary.weak.color)),
            text_color: palette.primary.weak.text,
            border_radius: 4.0.into(),
            ..Default::default()
        }
    }))
}
