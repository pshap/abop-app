//! Progress text caching for performance optimization
//!
//! This module provides caching mechanisms for progress text formatting to avoid
//! expensive string operations during UI rendering.

/// Minimum progress change threshold (0.1%) for updating cached text to prevent excessive formatting.
///
/// This constant defines the minimum change in progress value (as a fraction from 0.0 to 1.0)
/// required before the cached progress text is regenerated. Values below this threshold will
/// continue to use the previously cached text, preventing expensive string formatting operations
/// on every UI render frame.
///
/// # Units
/// - Value: 0.001 (represents 0.1% change)
/// - Range: 0.0 to 1.0 (fractional progress values)
/// - Example: Progress must change by at least 0.001 (0.1%) to trigger cache update
const PROGRESS_CACHE_THRESHOLD: f32 = 0.001;

/// Cached progress information
#[derive(Debug, Clone)]
struct CachedProgress {
    /// The progress value that was cached
    last_value: f32,
    /// The formatted text for this progress value
    cached_text: String,
}

impl CachedProgress {
    fn new(value: f32, text: String) -> Self {
        Self {
            last_value: value,
            cached_text: text,
        }
    }
}

/// Progress text caching system for performance optimization
#[derive(Debug, Clone)]
pub struct ProgressCache {
    /// Cached scan progress text
    scan_progress: Option<CachedProgress>,
    /// Cached processing progress text
    processing_progress: Option<CachedProgress>,
    /// Cached save progress text
    save_progress: Option<CachedProgress>,
    /// Cached task progress text
    task_progress: Option<CachedProgress>,
}

impl ProgressCache {
    /// Create new progress cache
    #[must_use]
    pub fn new() -> Self {
        Self {
            scan_progress: None,
            processing_progress: None,
            save_progress: None,
            task_progress: None,
        }
    }

    /// Get cached scan progress text, updating cache if needed
    /// This avoids frequent float-to-string formatting in UI renders
    pub fn get_scan_progress_text(&mut self, progress_percentage: f32) -> String {
        let cache_slot = &mut self.scan_progress;
        Self::get_cached_progress_text_impl(cache_slot, progress_percentage, |p| {
            format!("Scanning: {:.1}%", p * 100.0)
        })
    }

    /// Get cached processing progress text, updating cache if needed
    pub fn get_processing_progress_text(&mut self, progress_percentage: f32) -> String {
        let cache_slot = &mut self.processing_progress;
        Self::get_cached_progress_text_impl(cache_slot, progress_percentage, |p| {
            format!("Processing: {:.1}%", p * 100.0)
        })
    }

    /// Get cached save progress text, updating cache if needed
    pub fn get_save_progress_text(&mut self, progress_percentage: f32) -> String {
        let cache_slot = &mut self.save_progress;
        Self::get_cached_progress_text_impl(cache_slot, progress_percentage, |p| {
            format!("Saving: {:.1}%", p * 100.0)
        })
    }

    /// Get cached task progress text, updating cache if needed
    pub fn get_task_progress_text(&mut self, progress_percentage: f32, task_name: &str) -> String {
        let cache_slot = &mut self.task_progress;
        let task_name = task_name.to_owned();
        Self::get_cached_progress_text_impl(cache_slot, progress_percentage, move |p| {
            format!("{}: {:.1}%", task_name, p * 100.0)
        })
    }

    /// Get simple percentage text (just the percentage)
    pub fn get_simple_percentage_text(&mut self, progress_percentage: f32) -> String {
        // Use processing cache slot for simple percentages
        let cache_slot = &mut self.processing_progress;
        Self::get_cached_progress_text_impl(cache_slot, progress_percentage, |p| {
            format!("{:.1}%", p * 100.0)
        })
    }

    /// Clear all progress caches
    pub fn clear_all_caches(&mut self) {
        self.scan_progress = None;
        self.processing_progress = None;
        self.save_progress = None;
        self.task_progress = None;
    }

    /// Clear scan progress cache
    pub fn clear_scan_cache(&mut self) {
        self.scan_progress = None;
    }

    /// Clear processing progress cache
    pub fn clear_processing_cache(&mut self) {
        self.processing_progress = None;
    }

    /// Clear save progress cache
    pub fn clear_save_cache(&mut self) {
        self.save_progress = None;
    }

    /// Clear task progress cache
    pub fn clear_task_cache(&mut self) {
        self.task_progress = None;
    }

    /// Generic method to get cached progress text with custom formatter
    fn get_cached_progress_text_impl<F>(
        cache_slot: &mut Option<CachedProgress>,
        progress_percentage: f32,
        formatter: F,
    ) -> String
    where
        F: FnOnce(f32) -> String,
    {
        // Check if we need to update the cache
        let needs_update = cache_slot.as_ref().is_none_or(|cached| {
            (cached.last_value - progress_percentage).abs() >= PROGRESS_CACHE_THRESHOLD
        });

        if needs_update {
            let new_text = formatter(progress_percentage);
            *cache_slot = Some(CachedProgress::new(progress_percentage, new_text));
        }

        // Return the cached text (guaranteed to exist after the update above)
        cache_slot.as_ref().unwrap().cached_text.clone()
    }
}

impl Default for ProgressCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_cache_threshold() {
        let mut cache = ProgressCache::new();

        // First call should generate text
        let text1 = cache.get_scan_progress_text(0.5);
        assert_eq!(text1, "Scanning: 50.0%");

        // Small change below threshold should return cached text
        let text2 = cache.get_scan_progress_text(0.5005); // 0.05% change
        assert_eq!(text2, "Scanning: 50.0%"); // Should still be cached

        // Larger change above threshold should generate new text
        let text3 = cache.get_scan_progress_text(0.51); // 1% change
        assert_eq!(text3, "Scanning: 51.0%"); // Should be updated
    }

    #[test]
    fn test_different_progress_types() {
        let mut cache = ProgressCache::new();

        let scan_text = cache.get_scan_progress_text(0.3);
        let processing_text = cache.get_processing_progress_text(0.7);
        let save_text = cache.get_save_progress_text(0.9);
        let task_text = cache.get_task_progress_text(0.1, "Test Task");

        assert_eq!(scan_text, "Scanning: 30.0%");
        assert_eq!(processing_text, "Processing: 70.0%");
        assert_eq!(save_text, "Saving: 90.0%");
        assert_eq!(task_text, "Test Task: 10.0%");
    }

    #[test]
    fn test_clear_caches() {
        let mut cache = ProgressCache::new();

        // Generate some cached values
        cache.get_scan_progress_text(0.5);
        cache.get_processing_progress_text(0.3);

        // Clear all caches
        cache.clear_all_caches();

        // Next calls should regenerate text (we can't directly test this,
        // but we can ensure the method doesn't panic and returns correct values)
        let text1 = cache.get_scan_progress_text(0.5);
        let text2 = cache.get_processing_progress_text(0.3);

        assert_eq!(text1, "Scanning: 50.0%");
        assert_eq!(text2, "Processing: 30.0%");
    }
}
