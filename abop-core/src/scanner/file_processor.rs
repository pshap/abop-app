//! Audio file processing functionality for the scanner
//!
//! This module provides the core file processing functionality, abstracted behind
//! a trait to allow for different processing strategies.

use crate::{
    audio::AudioMetadata, db::Database, error::Result, models::Audiobook,
    scanner::progress::ProgressReporter,
};
use futures::stream::{self, StreamExt};
use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::error;
use tracing::{debug, instrument};

/// Cache for audio file metadata
#[allow(dead_code)]
#[derive(Debug)]
pub struct MetadataCache {
    /// LRU cache for file metadata
    cache: LruCache<PathBuf, AudioMetadata>,
    /// Mutex for thread-safe access
    mutex: Arc<Mutex<()>>,
}

impl MetadataCache {
    /// Creates a new metadata cache with the specified capacity
    #[allow(dead_code)]
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(
                NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(1000).unwrap()),
            ),
            mutex: Arc::new(Mutex::new(())),
        }
    }

    /// Gets metadata for a file from the cache
    pub async fn get(&mut self, path: &PathBuf) -> Option<AudioMetadata> {
        let _guard = self.mutex.lock().await;
        self.cache.get(path).cloned()
    }

    /// Stores metadata for a file in the cache
    pub async fn insert(&mut self, path: PathBuf, metadata: AudioMetadata) {
        let _guard = self.mutex.lock().await;
        self.cache.put(path, metadata);
    }

    /// Clears the cache
    #[allow(dead_code)]
    pub async fn clear(&mut self) {
        let _guard = self.mutex.lock().await;
        self.cache.clear();
    }
}

/// Configuration for batch processing
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of files to process in a single batch
    pub batch_size: usize,
    /// Maximum number of concurrent database operations
    pub max_concurrent_db_ops: usize,
    /// Whether to use bulk inserts for better performance
    pub use_bulk_insert: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            max_concurrent_db_ops: 4,
            use_bulk_insert: true,
        }
    }
}

/// Trait for processing audio files
#[async_trait::async_trait]
pub trait FileProcessor: Send + Sync {
    /// Process a single audio file
    async fn process_file(&self, path: PathBuf) -> Result<Audiobook>;

    /// Process a batch of audio files
    async fn process_batch(&self, paths: Vec<PathBuf>) -> Vec<Result<Audiobook>>;

    /// Process files with progress reporting
    async fn process_with_progress<P: ProgressReporter + Send + Sync>(
        &self,
        paths: Vec<PathBuf>,
        reporter: &P,
    ) -> Vec<Result<Audiobook>>;
}

/// Default implementation of FileProcessor
#[allow(dead_code)]
#[derive(Debug)]
pub struct DefaultFileProcessor {
    /// Database connection
    db: Database,
    /// Metadata cache
    cache: Arc<Mutex<MetadataCache>>,
    /// Batch processing configuration
    batch_config: BatchConfig,
}

impl DefaultFileProcessor {
    /// Creates a new file processor
    #[allow(dead_code)]
    #[must_use]
    pub fn new(db: Database) -> Self {
        Self {
            db,
            cache: Arc::new(Mutex::new(MetadataCache::new(1000))),
            batch_config: BatchConfig::default(),
        }
    }

    /// Creates a new file processor with custom batch configuration
    #[allow(dead_code)]
    #[must_use]
    pub fn with_config(db: Database, config: BatchConfig) -> Self {
        Self {
            db,
            cache: Arc::new(Mutex::new(MetadataCache::new(1000))),
            batch_config: config,
        }
    }

    /// Extracts metadata from an audio file, using cache if available
    #[instrument(skip(self), fields(path = %path.display()))]
    async fn extract_metadata(&self, path: &PathBuf) -> Result<AudioMetadata> {
        // Check cache first
        if let Some(metadata) = self.cache.lock().await.get(path).await {
            debug!("Cache hit for {}", path.display());
            return Ok(metadata);
        } // Extract metadata if not in cache
        let metadata = AudioMetadata::from_file(path)?;

        // Store in cache
        self.cache
            .lock()
            .await
            .insert(path.clone(), metadata.clone())
            .await;

        Ok(metadata)
    }
    /// Creates an audiobook from metadata
    #[instrument(skip(self, metadata))]
    async fn create_audiobook(&self, path: PathBuf, metadata: AudioMetadata) -> Result<Audiobook> {
        // For now, use a placeholder library_id - this should be passed from the parent directory
        let library_id = "default"; // TODO: Get actual library_id from context

        let mut audiobook = Audiobook::new(library_id, &path);

        // Populate with metadata
        audiobook.title = metadata.title;
        audiobook.author = metadata.artist;
        audiobook.narrator = metadata.narrator;
        audiobook.description = metadata.description;
        audiobook.duration_seconds = metadata.duration_seconds.map(|d| d as u64);
        audiobook.cover_art = metadata.cover_art;

        // Set size from file system
        if let Ok(file_metadata) = std::fs::metadata(&path) {
            audiobook.size_bytes = Some(file_metadata.len());
        }

        Ok(audiobook)
    }

    /// Adds an audiobook to the database
    #[instrument(skip(self, audiobook))]
    async fn add_to_database(&self, audiobook: Audiobook) -> Result<Audiobook> {
        self.db.add_audiobook(&audiobook).await?;
        Ok(audiobook)
    }

    /// Adds multiple audiobooks to the database in bulk
    #[instrument(skip(self, audiobooks))]
    async fn add_batch_to_database(&self, audiobooks: Vec<Audiobook>) -> Result<Vec<Audiobook>> {
        if self.batch_config.use_bulk_insert {
            self.db.add_audiobooks_bulk(&audiobooks).await?;
        } else {
            // Process in smaller chunks if bulk insert is disabled
            for chunk in audiobooks.chunks(self.batch_config.batch_size) {
                self.db.add_audiobooks_bulk(chunk).await?;
            }
        }
        Ok(audiobooks)
    }
}

#[async_trait::async_trait]
impl FileProcessor for DefaultFileProcessor {
    #[instrument(skip(self), fields(path = %path.display()))]
    async fn process_file(&self, path: PathBuf) -> Result<Audiobook> {
        let metadata = self.extract_metadata(&path).await?;
        let audiobook = self.create_audiobook(path, metadata).await?;
        self.add_to_database(audiobook).await
    }

    #[instrument(skip(self, paths), fields(count = paths.len()))]
    async fn process_batch(&self, paths: Vec<PathBuf>) -> Vec<Result<Audiobook>> {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(
            self.batch_config.max_concurrent_db_ops,
        ));
        let mut results = Vec::with_capacity(paths.len());
        let mut audiobooks = Vec::with_capacity(paths.len());

        // Process files in parallel with concurrency limit
        let futures = paths.into_iter().map(|path| {
            let sem = semaphore.clone();
            async move {
                let _permit = sem.acquire().await.unwrap();
                self.process_file(path).await
            }
        });

        // Collect results
        let mut stream =
            stream::iter(futures).buffer_unordered(self.batch_config.max_concurrent_db_ops);
        while let Some(result) = stream.next().await {
            match result {
                Ok(audiobook) => audiobooks.push(audiobook),
                Err(e) => {
                    error!("Error processing file: {}", e);
                    results.push(Err(e));
                }
            }
        }

        // Bulk insert successful audiobooks
        if !audiobooks.is_empty() {
            match self.add_batch_to_database(audiobooks).await {
                Ok(inserted) => results.extend(inserted.into_iter().map(Ok)),
                Err(e) => {
                    error!("Error bulk inserting audiobooks: {}", e);
                    results.push(Err(e));
                }
            }
        }

        results
    }

    #[instrument(skip(self, reporter, paths), fields(count = paths.len()))]
    async fn process_with_progress<P: ProgressReporter + Send + Sync>(
        &self,
        paths: Vec<PathBuf>,
        reporter: &P,
    ) -> Vec<Result<Audiobook>> {
        let total = paths.len();
        let mut processed = 0;
        let mut results = Vec::with_capacity(total);

        // Process in batches
        for chunk in paths.chunks(self.batch_config.batch_size) {
            let batch_results = self.process_batch(chunk.to_vec()).await;

            // Update progress
            processed += batch_results.len();
            let progress = processed as f32 / total as f32;
            reporter.report_progress(progress).await;

            results.extend(batch_results);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_metadata_cache() {
        let mut cache = MetadataCache::new(2);
        let path1 = PathBuf::from("test1.mp3");
        let path2 = PathBuf::from("test2.mp3");
        let path3 = PathBuf::from("test3.mp3");

        let metadata1 = AudioMetadata::default();
        let metadata2 = AudioMetadata::default();
        let metadata3 = AudioMetadata::default();

        // Insert and retrieve
        cache.insert(path1.clone(), metadata1.clone()).await;
        assert_eq!(cache.get(&path1).await, Some(metadata1));

        // Test LRU eviction
        cache.insert(path2.clone(), metadata2.clone()).await;
        cache.insert(path3.clone(), metadata3.clone()).await;
        assert_eq!(cache.get(&path1).await, None); // Should be evicted
        assert_eq!(cache.get(&path2).await, Some(metadata2));
        assert_eq!(cache.get(&path3).await, Some(metadata3));
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let dir = tempdir().unwrap();
        let db = Database::in_memory().unwrap();
        let processor = DefaultFileProcessor::new(db);

        // Create test files
        let paths: Vec<PathBuf> = (0..5)
            .map(|i| {
                let path = dir.path().join(format!("test{i}.mp3"));
                let mut file = File::create(&path).unwrap();
                file.write_all(b"dummy mp3 content").unwrap();
                path
            })
            .collect();

        // Test batch processing
        let results = processor.process_batch(paths).await;
        assert_eq!(results.len(), 5);

        // All should fail since they're not real MP3s
        for result in results {
            assert!(result.is_err());
        }
    }
}
