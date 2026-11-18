//! Cover art display component for audiobooks
//!
//! This module provides a reusable component for displaying audiobook cover art
//! with proper fallbacks and caching.

use iced::widget::{container, image, text};
use iced::{Element, Length};
use std::sync::Arc;

use crate::styling::material::MaterialTokens;
use crate::utils::image_cache::{CacheStats, ImageCache};

/// Pixel size of the cover thumbnails shown in the library table
pub const TABLE_THUMBNAIL_SIZE: u32 = 40;

/// Cover art display component
#[derive(Debug, Clone)]
pub struct CoverArt {
    /// The image handle to display
    handle: iced::widget::image::Handle,
    /// Size of the cover art in pixels
    size: u32,
    /// Whether this is a placeholder
    is_placeholder: bool,
    /// Optional title for accessibility
    title: Option<String>,
}

impl CoverArt {
    /// Create a new cover art component from image data
    pub fn from_data(
        image_data: Option<&[u8]>,
        size: u32,
        cache: &Arc<ImageCache>,
        audiobook_id: &str,
    ) -> Self {
        // Never decode in the view: thumbnails are preloaded in the background
        // (see `ImageCache::preload`) and a placeholder is shown until then.
        let cached = image_data.and_then(|_| cache.cached_handle(audiobook_id, size));
        let is_placeholder = cached.is_none();
        let handle = cached.unwrap_or_else(|| cache.create_placeholder_handle(size));

        Self {
            handle,
            size,
            is_placeholder,
            title: None,
        }
    }

    /// Create a placeholder cover art component
    pub fn placeholder(size: u32) -> Self {
        // Use a static placeholder handle to avoid repeated cache creation
        let handle = ImageCache::default().create_placeholder_handle(size);
        Self {
            handle,
            size,
            is_placeholder: true,
            title: Some("No cover art".to_string()),
        }
    }

    /// Set the title for accessibility
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Create the view for this cover art component
    pub fn view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, crate::messages::Message> {
        let size = self.size as f32;
        let content: Element<'a, crate::messages::Message> = if self.is_placeholder {
            crate::components::ui::icon("book", size * 0.45, tokens.colors.on_surface_variant)
        } else {
            image(self.handle.clone())
                .width(Length::Fixed(size))
                .height(Length::Fixed(size))
                .content_fit(iced::ContentFit::Cover)
                .into()
        };

        container(content)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .clip(true)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(tokens.colors.surface_container_highest.into()),
                border: iced::Border {
                    radius: 6.0.into(),
                    ..iced::Border::default()
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}

impl CoverArt {
    /// Get the size of this cover art component
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Check if this is a placeholder
    pub fn is_placeholder(&self) -> bool {
        self.is_placeholder
    }

    /// Get the title for accessibility
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

/// Cover art display with loading state
#[derive(Debug, Clone)]
pub struct CoverArtWithLoading {
    /// The cover art component
    cover_art: CoverArt,
    /// Whether the image is currently loading
    loading: bool,
    /// Loading indicator size
    loading_size: u32,
}

impl CoverArtWithLoading {
    /// Create a new cover art with loading state
    pub fn new(
        image_data: Option<&[u8]>,
        size: u32,
        cache: &Arc<ImageCache>,
        audiobook_id: &str,
    ) -> Self {
        let cover_art = CoverArt::from_data(image_data, size, cache, audiobook_id);

        Self {
            cover_art,
            loading: false,
            loading_size: size,
        }
    }

    /// Set the loading state
    pub fn set_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Create the view for this cover art component with loading state
    pub fn view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, crate::messages::Message> {
        if self.loading {
            // Show loading indicator
            container(text("⏳").size(self.loading_size as f32 * 0.6))
                .width(Length::Fixed(self.loading_size as f32))
                .height(Length::Fixed(self.loading_size as f32))
                .style(tokens.container_style())
                .into()
        } else {
            self.cover_art.view(tokens)
        }
    }
}

/// Cover art display with statistics
#[derive(Debug, Clone)]
pub struct CoverArtWithStats {
    /// The cover art component
    cover_art: CoverArt,
    /// Cache statistics
    cache_stats: CacheStats,
}

impl CoverArtWithStats {
    /// Create a new cover art with statistics
    pub fn new(
        image_data: Option<&[u8]>,
        size: u32,
        cache: &Arc<ImageCache>,
        audiobook_id: &str,
    ) -> Self {
        let cover_art = CoverArt::from_data(image_data, size, cache, audiobook_id);
        let cache_stats = cache.stats();

        Self {
            cover_art,
            cache_stats,
        }
    }

    /// Create the view for this cover art component with statistics
    pub fn view<'a>(&self, tokens: &'a MaterialTokens) -> Element<'a, crate::messages::Message> {
        self.cover_art.view(tokens)
    }

    /// Get the cache statistics
    pub fn cache_stats(&self) -> &CacheStats {
        &self.cache_stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cover_art_creation() {
        let cache = Arc::new(ImageCache::new());
        let cover_art = CoverArt::from_data(None, 64, &cache, "test-id");

        assert_eq!(cover_art.size(), 64);
        assert!(cover_art.is_placeholder());
    }

    #[test]
    fn test_cover_art_placeholder() {
        let cover_art = CoverArt::placeholder(128);

        assert_eq!(cover_art.size(), 128);
        assert!(cover_art.is_placeholder());
        assert_eq!(cover_art.title(), Some("No cover art"));
    }

    #[test]
    fn test_cover_art_with_title() {
        let cache = Arc::new(ImageCache::new());
        let cover_art =
            CoverArt::from_data(None, 64, &cache, "test-id").with_title("Test Audiobook");

        assert_eq!(cover_art.title(), Some("Test Audiobook"));
    }
}
