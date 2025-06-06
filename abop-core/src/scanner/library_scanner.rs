//! Audio file scanner module for ABOP
//!
//! This module provides functionality to scan directories for audio files,
//! extract metadata, and update the database with the found files.

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use walkdir::WalkDir;

use crate::error::{AppError, Result};
use crate::models::audiobook::Audiobook;
use crate::db::repositories::AudiobookRepository;
use crate::scanner::error::ScanError;
use crate::scanner::progress::ScanProgress;

/// Core scanner implementation with thread pool
pub struct LibraryScanner {
    state: Arc<Mutex<ScannerState>>,
    progress: Arc<Mutex<ScanProgress>>,
    repository: Arc<AudiobookRepository>,
    max_workers: usize,
    batch_size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScannerState {
    Idle,
    Scanning,
    Paused,
    Completed,
    Error,
}

impl LibraryScanner {
    /// Creates a new scanner with default configuration
    pub fn new(repository: Arc<AudiobookRepository>, max_workers: usize, batch_size: usize) -> Self {
        Self {
            state: Arc::new(Mutex::new(ScannerState::Idle)),
            progress: Arc::new(Mutex::new(ScanProgress::default())),
            repository,
            max_workers,
            batch_size,
        }
    }
    
    /// Initiates an asynchronous scan operation
    pub async fn scan_directory(&self, path: PathBuf) -> Result<()> {
        let mut state = self.state.lock().await;
        if *state != ScannerState::Idle {
            return Err(AppError::Scan(ScanError::Unknown(
                "Scanner is already running".into(),
            )));
        }
        *state = ScannerState::Scanning;
        drop(state);

        // Count total files first
        let total_files = WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .count();

        let (tx, mut rx) = mpsc::channel(100);
        let mut progress = self.progress.lock().await;
        progress.total_files = total_files;
        progress.files_processed = 0;
        progress.current_file = None;
        progress.errors.clear();
        drop(progress);

        // Spawn worker tasks
        let mut handles = Vec::new();
        for _ in 0..self.max_workers {
            let tx = tx.clone();
            let repository = self.repository.clone();
            let path = path.clone();
            let handle = tokio::spawn(async move {
                let mut batch = Vec::new();
                for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                    if !entry.file_type().is_file() {
                        continue;
                    }

                    // Here, you would extract metadata for the file
                    // For now, just use Audiobook::new as a placeholder
                    // let metadata = ...
                    // match Audiobook::from_metadata(metadata, entry.path()) {
                    //     Ok(audiobook) => batch.push(audiobook),
                    //     Err(e) => { ... }
                    // }
                    let audiobook = Audiobook::new("library", entry.path());
                    batch.push(audiobook);
                    if batch.len() >= 10 {
                        if let Err(e) = repository.batch_add(&batch) {
                            let mut progress = ScanProgress::default();
                            progress.errors.push(ScanError::Unknown(format!("Database error: {e}")));
                            let _ = tx.send(progress).await;
                        }
                        batch.clear();
                    }
                }

                // Process remaining items
                if !batch.is_empty() {
                    if let Err(e) = repository.batch_add(&batch) {
                        let mut progress = ScanProgress::default();
                        progress.errors.push(ScanError::Unknown(format!("Database error: {e}")));
                        let _ = tx.send(progress).await;
                    }
                }
            });
            handles.push(handle);
        }

        // Process progress updates
        while let Some(progress_update) = rx.recv().await {
            let mut progress = self.progress.lock().await;
            progress.files_processed += 1;
            if let Some(file) = progress_update.current_file {
                progress.current_file = Some(file);
            }
            progress.errors.extend(progress_update.errors);
        }

        // Wait for all workers to complete
        for handle in handles {
            if let Err(e) = handle.await {
                return Err(AppError::Other(format!("Worker task failed: {e}")));
            }
        }

        let mut state = self.state.lock().await;
        *state = ScannerState::Completed;

        Ok(())
    }

    pub async fn get_state(&self) -> ScannerState {
        *self.state.lock().await
    }

    pub async fn get_progress(&self) -> ScanProgress {
        self.progress.lock().await.clone()
    }

    pub async fn pause(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        if *state != ScannerState::Scanning {
            return Err(AppError::Scan(ScanError::Unknown(
                "Scanner is not running".into(),
            )));
        }
        *state = ScannerState::Paused;
        Ok(())
    }

    pub async fn resume(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        if *state != ScannerState::Paused {
            return Err(AppError::Scan(ScanError::Unknown(
                "Scanner is not paused".into(),
            )));
        }
        *state = ScannerState::Scanning;
        Ok(())
    }

    pub async fn cancel(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        if *state != ScannerState::Scanning && *state != ScannerState::Paused {
            return Err(AppError::Scan(ScanError::Unknown(
                "Scanner is not running".into(),
            )));
        }
        *state = ScannerState::Idle;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_find_audio_files() {
        // Test implementation
    }
}
