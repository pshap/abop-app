//! Image caching utilities for cover art display
//!
//! This module provides efficient caching for cover art images to improve
//! performance when displaying large audiobook libraries.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use iced::widget::image::Handle;
use image::DynamicImage;

/// Cache entry for a cover art thumbnail
#[derive(Debug, Clone)]
struct CacheEntry {
    /// Ready-to-draw handle; reusing the same handle lets iced reuse the GPU texture
    handle: Handle,
    /// When this entry was created
    created_at: Instant,
    /// Last time this entry was accessed
    last_accessed: Instant,
    /// Number of times this entry has been accessed
    access_count: u32,
}

impl CacheEntry {
    /// Create a new cache entry
    fn new(handle: Handle) -> Self {
        let now = Instant::now();
        Self {
            handle,
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

/// LRU cache for cover art thumbnails
///
/// Decoding cover art is expensive, so it should happen off the UI thread via
/// [`ImageCache::preload`]. Views read thumbnails with [`ImageCache::cached_handle`],
/// which never decodes.
#[derive(Debug)]
pub struct ImageCache {
    /// The actual cache storage, keyed by `"{id}@{size}"`
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    /// Maximum number of images to cache
    max_size: usize,
    /// Maximum age for cached images
    max_age: Duration,
}

impl ImageCache {
    /// Create a new image cache with default settings
    pub fn new() -> Self {
        Self::with_settings(500, Duration::from_secs(3600)) // 1 hour
    }

    /// Create a new image cache with custom settings
    pub fn with_settings(max_size: usize, max_age: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            max_age,
        }
    }

    fn cache_key(key: &str, size: u32) -> String {
        format!("{key}@{size}")
    }

    /// Get a cached thumbnail handle without decoding anything
    ///
    /// Safe to call from `view()`; returns `None` if the thumbnail isn't ready yet.
    pub fn cached_handle(&self, key: &str, size: u32) -> Option<Handle> {
        let mut cache = self.cache.lock().ok()?;
        let cache_key = Self::cache_key(key, size);
        let entry = cache.get_mut(&cache_key)?;

        if entry.is_stale(self.max_age) {
            cache.remove(&cache_key);
            return None;
        }

        entry.touch();
        Some(entry.handle.clone())
    }

    /// Get a cached thumbnail handle, decoding and caching it if needed
    ///
    /// Decoding can take a long time for large covers; avoid calling this from `view()`.
    pub fn get_or_create_handle(&self, key: &str, image_data: Option<&[u8]>, size: u32) -> Handle {
        if let Some(handle) = self.cached_handle(key, size) {
            return handle;
        }

        if let Some(data) = image_data
            && let Some(handle) = Self::decode_thumbnail(data, size)
        {
            self.insert(key, size, handle.clone());
            return handle;
        }

        self.create_placeholder_handle(size)
    }

    /// Decode and cache thumbnails for many images; intended for background threads
    ///
    /// Returns how many thumbnails were newly decoded.
    pub fn preload<'a>(
        &self,
        items: impl IntoIterator<Item = (&'a str, &'a [u8])>,
        size: u32,
    ) -> usize {
        let mut decoded = 0;
        for (key, data) in items {
            if self.cached_handle(key, size).is_some() {
                continue;
            }
            if let Some(handle) = Self::decode_thumbnail(data, size) {
                self.insert(key, size, handle);
                decoded += 1;
            }
        }
        decoded
    }

    /// Decode image bytes into a thumbnail handle (aspect ratio preserved)
    fn decode_thumbnail(data: &[u8], size: u32) -> Option<Handle> {
        let img = image::load_from_memory(data).ok()?;
        let thumbnail = if img.width() > size || img.height() > size {
            img.thumbnail(size, size)
        } else {
            img
        };
        let rgba = thumbnail.to_rgba8();
        let (width, height) = rgba.dimensions();
        Some(Handle::from_rgba(width, height, rgba.into_raw()))
    }

    /// Create a placeholder handle for missing cover art
    pub fn create_placeholder_handle(&self, size: u32) -> Handle {
        let placeholder = DynamicImage::new_rgba8(size, size).to_rgba8();
        Handle::from_rgba(size, size, placeholder.into_raw())
    }

    /// Cache a thumbnail handle with basic LRU eviction
    fn insert(&self, key: &str, size: u32, handle: Handle) {
        if let Ok(mut cache) = self.cache.lock() {
            if cache.len() >= self.max_size {
                Self::evict_oldest(&mut cache);
            }
            cache.insert(Self::cache_key(key, size), CacheEntry::new(handle));
        }
    }

    /// Evict the least recently used cache entry
    fn evict_oldest(cache: &mut HashMap<String, CacheEntry>) {
        let oldest_key = cache
            .iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(key, _)| key.clone());

        if let Some(key) = oldest_key {
            cache.remove(&key);
        }
    }

    /// Remove all cached sizes for one image
    pub fn invalidate(&self, key: &str) {
        if let Ok(mut cache) = self.cache.lock() {
            let prefix = format!("{key}@");
            cache.retain(|k, _| !k.starts_with(&prefix));
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
        assert_eq!(cache.stats().max_size, 500);
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

    fn png_bytes(width: u32, height: u32) -> Vec<u8> {
        let img = DynamicImage::new_rgba8(width, height);
        let mut bytes = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .unwrap();
        bytes
    }

    #[test]
    fn test_preload_then_cached_handle_reuses_handle() {
        let cache = ImageCache::new();
        let data = png_bytes(200, 100);

        assert!(cache.cached_handle("book", 48).is_none());
        assert_eq!(cache.preload([("book", data.as_slice())], 48), 1);
        // Already cached: nothing new to decode
        assert_eq!(cache.preload([("book", data.as_slice())], 48), 0);

        let first = cache.cached_handle("book", 48).unwrap();
        let second = cache.cached_handle("book", 48).unwrap();
        assert_eq!(first.id(), second.id(), "same texture should be reused");
        assert!(
            cache.cached_handle("book", 96).is_none(),
            "sizes are cached separately"
        );

        cache.invalidate("book");
        assert!(cache.cached_handle("book", 48).is_none());
    }

    #[test]
    fn test_invalid_image_data_is_skipped() {
        let cache = ImageCache::new();
        assert_eq!(cache.preload([("bad", &b"not an image"[..])], 48), 0);
        assert!(cache.cached_handle("bad", 48).is_none());
    }
}
