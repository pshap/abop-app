//! Enhanced cover art display component for audiobooks
//!
//! This module provides an enhanced component for displaying audiobook cover art
//! with loading states, edit functionality, and improved visual feedback.

use iced::widget::{button, column, container, image, row, text};
use iced::{Element, Length, Padding, Theme};
use std::sync::Arc;

use crate::utils::image_cache::{CacheStats, ImageCache};
use crate::styling::material::MaterialTokens;
use crate::messages::Message;

/// State of the cover art
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverArtState {
    /// Cover art is currently loading
    Loading,
    /// Cover art is loaded and displayed
    Loaded,
    /// Error occurred while loading cover art
    Error,
    /// No cover art available, showing placeholder
    Placeholder,
}

/// Enhanced cover art display component with additional features
#[derive(Debug, Clone)]
pub struct EnhancedCoverArt {
    /// The image handle to display
    handle: iced::widget::image::Handle,
    /// Size of the cover art in pixels
    size: u32,
    /// Current state of the cover art
    state: CoverArtState,
    /// Whether to show edit controls on hover
    show_edit_controls: bool,
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
        let state = if image_data.is_some() {
            CoverArtState::Loaded
        } else {
            CoverArtState::Placeholder
        };

        Self {
            handle,
            size,
            state,
            show_edit_controls: false,
            title: None,
            audiobook_id: Some(audiobook_id.to_string()),
        }
    }

    /// Create a loading state cover art
    pub fn loading(size: u32) -> Self {
        let cache = Arc::new(ImageCache::new());
        let handle = cache.create_placeholder_handle(size);

        Self {
            handle,
            size,
            state: CoverArtState::Loading,
            show_edit_controls: false,
            title: Some("Loading...".to_string()),
            audiobook_id: None,
        }
    }

    /// Create a placeholder cover art
    pub fn placeholder(size: u32) -> Self {
        let cache = Arc::new(ImageCache::new());
        let handle = cache.create_placeholder_handle(size);

        Self {
            handle,
            size,
            state: CoverArtState::Placeholder,
            show_edit_controls: false,
            title: Some("No cover art".to_string()),
            audiobook_id: None,
        }
    }

    /// Set the title for accessibility
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Enable edit controls for this cover art
    pub fn with_edit_controls(mut self, audiobook_id: impl Into<String>) -> Self {
        self.show_edit_controls = true;
        self.audiobook_id = Some(audiobook_id.into());
        self
    }

    /// Set the current state of the cover art
    pub fn with_state(mut self, state: CoverArtState) -> Self {
        self.state = state;
        self
    }

    /// Create the view for this cover art component
    pub fn view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        // Base content based on state
        let content = match self.state {
            CoverArtState::Loading => self.loading_view(tokens),
            CoverArtState::Error => self.error_view(tokens),
            CoverArtState::Placeholder => self.placeholder_view(tokens),
            CoverArtState::Loaded => self.image_view(tokens),
        };

        // Wrap with edit controls if enabled
        if self.show_edit_controls && self.state == CoverArtState::Loaded {
            self.with_edit_overlay(content, tokens)
        } else {
            container(content)
                .width(Length::Fixed(self.size as f32))
                .height(Length::Fixed(self.size as f32))
                .style(container_style(tokens))
                .into()
        }
    }

    fn loading_view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        container(
            text("⏳")
                .size(self.size as f32 * 0.6)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container_loading_style(tokens))
        .into()
    }

    fn error_view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        container(
            text("❌")
                .size(self.size as f32 * 0.6)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container_error_style(tokens))
        .into()
    }

    fn placeholder_view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        container(
            text("🎵")
                .size(self.size as f32 * 0.6)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container_placeholder_style(tokens))
        .into()
    }

    fn image_view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        container(
            image(self.handle.clone())
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container_image_style(tokens))
        .into()
    }

    fn with_edit_overlay<'a>(
        &self,
        content: impl Into<Element<'a, Message>>,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let overlay = container(
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

        container(
            container(
                column![
                    content,
                    container(overlay)
                        .width(Length::Fill)
                        .height(Length::Fill)
                ]
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .style(container_hover_style(tokens)),
        )
        .width(Length::Fixed(self.size as f32))
        .height(Length::Fixed(self.size as f32))
        .into()
    }

    /// Get the size of this cover art
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Get the current state
    pub fn state(&self) -> CoverArtState {
        self.state
    }
}

// Style functions
fn container_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |_theme: &Theme| {
        container::Appearance {
            border_radius: 8.0.into(),
            border_width: 1.0,
            ..Default::default()
        }
    }))
}

fn container_loading_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |theme: &Theme| {
        let palette = theme.extended_palette();
        container::Appearance {
            background: Some(iced::Background::Color(palette.background.weak.color)),
            border_radius: 4.0.into(),
            ..Default::default()
        }
    }))
}

fn container_error_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |theme: &Theme| {
        let palette = theme.extended_palette();
        container::Appearance {
            background: Some(iced::Background::Color(palette.danger.weak.color)),
            border_radius: 4.0.into(),
            ..Default::default()
        }
    }))
}

fn container_placeholder_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |theme: &Theme| {
        let palette = theme.extended_palette();
        container::Appearance {
            background: Some(iced::Background::Color(palette.background.base.color)),
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
    iced::theme::Container::Custom(Box::new(move |_theme: &Theme| {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.7))),
            border_radius: 4.0.into(),
            ..Default::default()
        }
    }))
}

fn container_hover_style(tokens: &MaterialTokens) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(move |_theme: &Theme| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use iced::widget::renderer::Style;

    #[test]
    fn test_cover_art_creation() {
        let cache = Arc::new(ImageCache::new());
        let cover = EnhancedCoverArt::from_data(Some(&[]), 100, &cache, "test");
        assert_eq!(cover.size(), 100);
        assert_eq!(cover.state(), CoverArtState::Loaded);
    }

    #[test]
    fn test_cover_art_loading() {
        let cover = EnhancedCoverArt::loading(100);
        assert_eq!(cover.size(), 100);
        assert_eq!(cover.state(), CoverArtState::Loading);
    }

    #[test]
    fn test_cover_art_placeholder() {
        let cover = EnhancedCoverArt::placeholder(100);
        assert_eq!(cover.size(), 100);
        assert_eq!(cover.state(), CoverArtState::Placeholder);
    }

    #[test]
    fn test_cover_art_with_title() {
        let cover = EnhancedCoverArt::placeholder(100).with_title("Test Title");
        assert_eq!(cover.title, Some("Test Title".to_string()));
    }
}
