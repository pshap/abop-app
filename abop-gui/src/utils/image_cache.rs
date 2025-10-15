//! Image caching utilities for cover art display
//!
//! This module provides efficient caching for cover art images to improve
//! performance when displaying large audiobook libraries.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use iced::widget::image::Handle;
use image::DynamicImage;

/// Cache entry for a cover art image
#[derive(Debug, Clone)]
struct CacheEntry {
    /// The decoded image data
    image: DynamicImage,
    /// When this entry was created
    created_at: Instant,
    /// Last time this entry was accessed
    last_accessed: Instant,
    /// Number of times this entry has been accessed
    access_count: u32,
}

impl CacheEntry {
    #[allow(dead_code)]
    /// Create a new cache entry
    fn new(image: DynamicImage) -> Self {
        let now = Instant::now();
        Self {
            image,
            created_at: now,
            last_accessed: now,
            access_count: 1,
        }
    }

    /// Update access statistics
    fn touch(&mut self) {
        self.last_accessed = Instant::now();
        self.access_count += 1;
    }

    /// Check if this entry is stale (older than max_age)
    fn is_stale(&self, max_age: Duration) -> bool {
        self.created_at.elapsed() > max_age
    }
}

/// LRU cache for cover art images
#[derive(Debug)]
pub struct ImageCache {
    /// The actual cache storage
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    /// Maximum number of images to cache
    max_size: usize,
    /// Maximum age for cached images
    max_age: Duration,
}

impl ImageCache {
    /// Create a new image cache with default settings
    pub fn new() -> Self {
        Self::with_settings(100, Duration::from_secs(3600)) // 1 hour
    }

    /// Create a new image cache with custom settings
    pub fn with_settings(max_size: usize, max_age: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            max_age,
        }
    }

    /// Get a cached image handle, or create one from raw data
    pub fn get_or_create_handle(&self, key: &str, image_data: Option<&[u8]>, size: u32) -> Handle {
        // Try to get from cache first
        if let Some(handle) = self.get_cached_handle(key, size) {
            return handle;
        }

        // Create new handle from image data
        if let Some(data) = image_data {
            if let Some((handle, dyn_img)) = self.create_handle_from_data(data, size) {
                // Cache the decoded thumbnail for future use
                self.cache_image(key, dyn_img);
                return handle;
            }
        }

        // Return placeholder if no image data or creation failed
        self.create_placeholder_handle(size)
    }

    /// Get a cached image handle if available
    fn get_cached_handle(&self, key: &str, size: u32) -> Option<Handle> {
        let mut cache = self.cache.lock().ok()?;
        let entry = cache.get_mut(key)?;

        // Check if entry is stale
        if entry.is_stale(self.max_age) {
            cache.remove(key);
            return None;
        }

        // Update access statistics
        entry.touch();

        // Create handle from cached image
        Some(self.create_handle_from_image(&entry.image, size))
    }

    /// Create an image handle from raw data, returning the handle and the thumbnail image
    fn create_handle_from_data(&self, data: &[u8], size: u32) -> Option<(Handle, DynamicImage)> {
        // Try to decode the image
        let img = image::load_from_memory(data).ok()?;

        // Resize to thumbnail size
        let thumbnail = if img.width() != size || img.height() != size {
            img.thumbnail(size, size)
        } else {
            img
        };

        // Convert to RGBA8 format for Iced
        let rgba = thumbnail.to_rgba8();
        let (width, height) = rgba.dimensions();

        // Create handle from raw RGBA data
        let handle = Handle::from_rgba(width, height, rgba.into_raw());
        Some((handle, thumbnail))
    }

    /// Create an image handle from a DynamicImage
    fn create_handle_from_image(&self, img: &DynamicImage, size: u32) -> Handle {
        // Resize to thumbnail size
        let thumbnail = if img.width() != size || img.height() != size {
            img.thumbnail(size, size)
        } else {
            img.clone()
        };

        // Convert to RGBA8 format for Iced
        let rgba = thumbnail.to_rgba8();
        let (width, height) = rgba.dimensions();

        // Create handle from raw RGBA data
        Handle::from_rgba(width, height, rgba.into_raw())
    }

    /// Create a placeholder handle for missing cover art
    pub fn create_placeholder_handle(&self, size: u32) -> Handle {
        // Create a simple placeholder image
        let placeholder = DynamicImage::new_rgba8(size, size);
        self.create_handle_from_image(&placeholder, size)
    }

    /// Cache a decoded image under a key with basic LRU eviction
    fn cache_image(&self, key: &str, image: DynamicImage) {
        if let Ok(mut cache) = self.cache.lock() {
            if cache.len() >= self.max_size {
                self.evict_oldest(&mut cache);
            }
            cache.insert(key.to_string(), CacheEntry::new(image));
        }
    }

    /// Evict the oldest cache entry
    fn evict_oldest(&self, cache: &mut HashMap<String, CacheEntry>) {
        let oldest_key = cache
            .iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(key, _)| key.clone());

        if let Some(key) = oldest_key {
            cache.remove(&key);
        }
    }

    /// Clear all cached entries
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        if let Ok(cache) = self.cache.lock() {
            let total_entries = cache.len();
            let total_accesses: u32 = cache.values().map(|entry| entry.access_count).sum();
            let avg_accesses = if total_entries > 0 {
                total_accesses as f32 / total_entries as f32
            } else {
                0.0
            };

            CacheStats {
                total_entries,
                total_accesses,
                avg_accesses,
                max_size: self.max_size,
            }
        } else {
            CacheStats::default()
        }
    }
}

impl Default for ImageCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the image cache
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total number of cached entries
    pub total_entries: usize,
    /// Total number of cache accesses
    pub total_accesses: u32,
    /// Average accesses per entry
    pub avg_accesses: f32,
    /// Maximum cache size
    pub max_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_cache_creation() {
        let cache = ImageCache::new();
        assert_eq!(cache.stats().max_size, 100);
    }

    #[test]
    fn test_image_cache_with_settings() {
        let cache = ImageCache::with_settings(50, Duration::from_secs(1800));
        let stats = cache.stats();
        assert_eq!(stats.max_size, 50);
        assert_eq!(stats.total_entries, 0);
    }

    #[test]
    fn test_placeholder_handle() {
        let cache = ImageCache::new();
        let _handle = cache.create_placeholder_handle(64);
        // We can't introspect the handle easily; creation should succeed without panic
    }
}
