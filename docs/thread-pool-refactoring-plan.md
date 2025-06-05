# ABOP Thread Pool Refactoring Plan

## Overview
Replace the current `ScanningThreadPool` with Iced 0.13's `Task` system while preserving existing scanner functionality and integrating with current architecture.

## Core Strategy
- **Preserve Current Architecture**: Use existing `Config`, `AppError`, and `RepositoryManager` patterns
- **Enhance Scanner**: Add Task support to current `LibraryScanner` without breaking existing functionality
- **Maintain Performance**: Keep current Rayon parallel processing where appropriate

## 1. Enhanced LibraryScanner

Replace manual thread pool with Task-based scanning:

```rust
// abop-core/src/scanner/library_scanner.rs - Add Task support
impl LibraryScanner {
    pub fn scan_with_tasks(&self, paths: Vec<PathBuf>) -> Task<ScanResult> {
        let config = self.config.clone();
        let repo_manager = self.repo_manager.clone();
        
        Task::perform(
            async move {
                let concurrency = config.scanner.max_concurrent_files.unwrap_or(8);
                let semaphore = Arc::new(Semaphore::new(concurrency));
                
                let results: Vec<_> = stream::iter(paths)
                    .map(|path| {
                        let semaphore = semaphore.clone();
                        let repo_manager = repo_manager.clone();
                        async move {
                            let _permit = semaphore.acquire().await?;
                            Self::scan_path(&path, &repo_manager).await
                        }
                    })
                    .buffer_unordered(concurrency)
                    .collect()
                    .await;
                    
                Ok(ScanResult::from_results(results))
            },
            Message::ScanComplete
        )
    }
    
    async fn scan_path(path: &Path, repo_manager: &RepositoryManager) -> Result<Vec<Audiobook>, AppError> {
        let metadata = AudioMetadata::from_file(path)?;
        let audiobook = Audiobook::from_metadata(metadata, path)?;
        
        repo_manager.audiobook_repo()
            .add_audiobook(&audiobook).await?;
            
        Ok(vec![audiobook])
    }
}
```

## 2. Error Integration

Leverage existing `AppError` hierarchy:

```rust
// Use existing error patterns from abop-core/src/error.rs
pub type ScanResult = Result<Vec<Audiobook>, AppError>;

impl From<ScanError> for AppError {
    fn from(err: ScanError) -> Self {
        match err {
            ScanError::IoError(e) => AppError::Io(e),
            ScanError::MetadataError(e) => AppError::AudioProcessing(e.into()),
            ScanError::DatabaseError(e) => AppError::Database(e),
        }
    }
}
```

## 3. Configuration Integration

Use existing `Config` struct:

```rust
// abop-core/src/config.rs - Extend existing config
impl Config {
    pub fn scanner_concurrency(&self) -> usize {
        self.scanner.max_concurrent_files.unwrap_or(8)
    }
    
    pub fn scanner_timeout(&self) -> Duration {
        Duration::from_secs(self.scanner.timeout_seconds.unwrap_or(30))
    }
}
```

## 4. GUI Integration

Update message system:

```rust
// abop-gui/src/messages.rs
#[derive(Debug, Clone)]
pub enum Message {
    // ...existing messages...
    ScanProgress(ScanProgressMessage),
    ScanComplete(Result<Vec<Audiobook>, AppError>),
    ScanCancelled,
}

#[derive(Debug, Clone)]  
pub enum ScanProgressMessage {
    Started { total_files: usize },
    FileProcessed { current: usize, file_name: String },
    Complete { processed: usize, errors: usize },
}
```

Progress UI component:

```rust
// abop-gui/src/components/scanner_progress.rs
pub fn scanner_progress_view(progress: &ScanProgress) -> Element<Message> {
    column![
        progress_bar(0.0..=100.0, progress.percentage()),
        text(&progress.status_message()),
        button("Cancel").on_press(Message::CancelScan)
    ]
    .spacing(10)
    .into()
}
```

## 5. Migration Strategy

**Phase 1**: Add Task support to existing `LibraryScanner`
- Keep current sync methods for compatibility
- Add new async Task-based methods

**Phase 2**: Update GUI to use new Task system
- Integrate with existing message handling
- Add progress reporting components

**Phase 3**: Remove legacy `ScanningThreadPool`
- After thorough testing and validation
- Maintain backward compatibility during transition

## Key Benefits

1. **Simplified Architecture**: Remove manual thread pool management
2. **Better Integration**: Use existing error/config/database patterns
3. **Improved UI**: Native Iced Task integration for better responsiveness
4. **Maintained Performance**: Keep current parallel processing efficiency

## Dependencies

```toml
[dependencies]
iced = { version = "0.13", features = ["tokio", "advanced"] }
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"
```

This concise plan focuses on practical implementation while preserving existing architectural patterns.
            .map_err(AppError::Database)?;
            
        Ok(audiobook)
    }
}
```

### 2. Error System Integration

**Use existing `AppError` hierarchy** instead of creating new error types:

```rust
// Leverage existing error system from abop-core/src/error.rs
impl From<ScanError> for AppError {
    fn from(err: ScanError) -> Self {
        match err {
            ScanError::IoError(e) => AppError::Io(e),
            ScanError::MetadataError(e) => AppError::AudioProcessing(e.into()),
            ScanError::DatabaseError(e) => AppError::Database(e),
            ScanError::ValidationError(e) => AppError::Validation(e),
        }
    }
}

// Use existing error patterns for scanner operations
pub type ScanResult = Result<Vec<Audiobook>, AppError>;
```

### 3. Configuration-Driven Scanning

**Leverage existing `AppConfig` and `UserPreferences`** for comprehensive configuration:

```rust
// abop-core/src/scanner/config.rs - Extend existing config patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub concurrency: Option<usize>,
    pub batch_size: usize,
    pub timeout_seconds: u64,
    pub supported_formats: Vec<String>,
    pub cache_enabled: bool,
    pub validation_level: ValidationLevel,
}

impl AppConfig {
    pub fn scanner_config(&self) -> ScanConfig {
        ScanConfig {
            concurrency: self.performance.thread_count,
            batch_size: self.scanner.batch_size.unwrap_or(100),
            timeout_seconds: self.scanner.timeout.unwrap_or(30),
            supported_formats: SUPPORTED_AUDIO_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
            cache_enabled: self.cache.enable_scan_cache.unwrap_or(true),
            validation_level: self.validation.level,
        }
    }
}
```

### 4. Database Integration with Existing Patterns

**Use established `LibraryRepository` and persistence patterns**:

```rust
// abop-core/src/scanner/persistence.rs - Build on existing DB patterns
impl ScanPersistence {
    pub async fn persist_scan_batch(
        &self, 
        books: Vec<Audiobook>,
        progress: ScanProgress
    ) -> Result<(), AppError> {
        // Use existing transaction patterns from LibraryRepository
        let mut tx = self.db.begin_transaction().await?;
        
        for book in books {
            // Leverage existing add_audiobook with retry logic
            self.db.add_audiobook_with_retry(&book, &mut tx).await
                .map_err(AppError::Database)?;
        }
        
        // Use existing progress persistence patterns
        self.save_scan_progress(&progress, &mut tx).await?;
        tx.commit().await.map_err(AppError::Database)?;
        
        Ok(())
    }
    
    // Integrate with existing health monitoring
    pub async fn validate_scan_integrity(&self) -> Result<ValidationReport, AppError> {
        // Use existing StateValidator patterns
        let validator = StateValidator::new(&self.db);
        validator.validate_library_consistency().await
            .map_err(AppError::Validation)
    }
}
```

### 5. Caching Strategy Using Existing Infrastructure

**Leverage existing validation and state persistence patterns**:

```rust
// abop-core/src/scanner/cache.rs - Use existing caching patterns
pub struct ScanCache {
    validator: StateValidator,
    config: AppConfig,
}

impl ScanCache {
    pub async fn get_cached_scan(&self, path: &Path) -> Option<CachedScanResult> {
        // Use existing validation caching patterns
        if !self.config.cache.enable_scan_cache.unwrap_or(true) {
            return None;
        }
        
        let cache_key = self.generate_cache_key(path);
        
        // Leverage existing state persistence patterns
        self.validator.get_cached_state(&cache_key).await
            .and_then(|state| self.deserialize_scan_result(&state))
    }
    
    pub async fn cache_scan_result(&self, path: &Path, result: &ScanResult) -> Result<(), AppError> {
        let cache_key = self.generate_cache_key(path);
        let serialized = self.serialize_scan_result(result)?;
        
        // Use existing async save patterns with progress reporting
        self.validator.save_state_with_progress(&cache_key, &serialized).await
            .map_err(AppError::Cache)
    }
      // Integrate with existing directory change detection
    fn should_invalidate_cache(&self, path: &Path) -> bool {
        // Use existing file system monitoring patterns
        self.validator.has_directory_changed(path)
    }
}
```

## GUI Integration with Existing Architecture

### 1. Message System Integration

**Enhance existing message handling** with modern Task patterns:

```rust
// abop-gui/src/messages.rs - Extend existing message system
#[derive(Debug, Clone)]
pub enum Message {
    // ...existing messages...
    
    // Enhanced scanner messages using existing patterns
    ScanProgress(ScanProgressMessage),
    ScanComplete(Result<Vec<Audiobook>, AppError>), // Use existing AppError
    ScanCancelled,
}

#[derive(Debug, Clone)]
pub enum ScanProgressMessage {
    Started { total_files: usize },
    FileProcessed { current: usize, total: usize, file_name: String },
    BatchComplete { processed: usize, cached: usize, errors: usize },
    ValidationProgress { current: usize, total: usize },
}
```

### 2. State Management with Existing Patterns

**Integrate with current application state** and use existing validation patterns:

```rust
// abop-gui/src/state.rs - Enhance existing state management
impl Application {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartScan(path) => {
                // Use existing config patterns
                let scanner = LibraryScanner::new(self.config.clone());
                
                // Leverage existing validation before scanning
                if let Err(e) = self.state_validator.validate_scan_prerequisites(&path) {
                    return Task::none(); // Handle validation error
                }
                
                self.scan_state = ScanState::InProgress {
                    start_time: Instant::now(),
                    progress: ScanProgress::default(),
                    cancellation_token: CancellationToken::new(),
                };
                
                scanner.scan_with_progress(path)
                    .map(Message::ScanProgress)
            }
            
            Message::ScanProgress(progress_msg) => {
                // Use existing progress persistence patterns
                if let ScanState::InProgress { progress, .. } = &mut self.scan_state {
                    progress.update(progress_msg.clone());
                    
                    // Persist progress using existing patterns
                    return self.persist_scan_progress(progress.clone())
                        .map(|_| Message::None);
                }
                Task::none()
            }
            
            // ...existing message handling...
        }
    }
}
```

### 3. UI Components with Material Theme Integration

**Use existing `MaterialTheme` and styling patterns**:

```rust
// abop-gui/src/components/scanner_progress.rs - New component using existing theme
use crate::theme::MaterialTheme;

pub fn scanner_progress_view(
    progress: &ScanProgress,
    theme: &MaterialTheme
) -> Element<Message> {
    let progress_bar = progress_bar(0.0..=100.0, progress.percentage())
        .style(theme.progress_bar_style());
        
    let status_text = text(&progress.status_message())
        .style(theme.body_text_style());
        
    let cancel_button = button("Cancel")
        .on_press(Message::CancelScan)
        .style(theme.secondary_button_style());
    
    column![progress_bar, status_text, cancel_button]
        .spacing(theme.spacing.medium)
        .into()
}
```

## Performance and Reliability Integration

### 1. Validation and Recovery Integration

**Leverage existing `StateValidator` and recovery systems**:

```rust
// abop-core/src/scanner/validation.rs - Use existing validation patterns
impl ScanValidator {
    pub async fn validate_and_repair_scan(&self, scan_result: &ScanResult) -> Result<RepairReport, AppError> {
        // Use existing StateValidator patterns
        let validator = StateValidator::new(&self.db);
        
        // Leverage existing repair context system
        let repair_context = RepairContext::from_scan_result(scan_result);
        
        // Use existing validation and repair patterns
        validator.validate_and_repair_with_context(repair_context).await
            .map_err(AppError::Validation)
    }
    
    // Integrate with existing backup/recovery strategies
    pub async fn create_scan_checkpoint(&self, progress: &ScanProgress) -> Result<(), AppError> {
        // Use existing backup patterns for scan state
        self.backup_manager.create_checkpoint(
            "scan_progress",
            &serde_json::to_vec(progress)?
        ).await.map_err(AppError::Backup)
    }
}
```

### 2. Memory Management and Resource Optimization

**Build on existing patterns** for efficient resource usage:

```rust
// abop-core/src/scanner/memory.rs - Use existing memory management patterns
pub struct MemoryAwareScanManager {
    config: AppConfig,
    metrics: SystemMetrics, // Use existing system monitoring
}

impl MemoryAwareScanManager {
    pub fn adjust_concurrency_for_memory(&self) -> usize {
        let available_memory = self.metrics.available_memory();
        let base_concurrency = self.config.scanner_concurrency().unwrap_or(num_cpus::get());
        
        // Use existing resource calculation patterns
        if available_memory < self.config.memory_limits.scan_threshold {
            base_concurrency / 2
        } else {
            base_concurrency
        }
    }
    
    // Integrate with existing cleanup patterns
    pub async fn cleanup_scan_resources(&self) -> Result<(), AppError> {
        // Use existing resource cleanup patterns
        self.metrics.cleanup_temporary_resources().await
            .map_err(AppError::Resource)
    }
}
```

## Migration Strategy

### Incremental Integration Plan

1. **Phase 1**: Enhance existing `LibraryScanner` with Task support (maintain backward compatibility)
2. **Phase 2**: Integrate error handling and database persistence using existing patterns
3. **Phase 3**: Add caching and validation using existing infrastructure
4. **Phase 4**: Update GUI components to use new Task-based scanning
5. **Phase 5**: Remove legacy thread pool code after validation

### Backward Compatibility

```rust
// Maintain existing API while adding new functionality
impl LibraryScanner {
    // Keep existing sync method for compatibility
    pub fn scan_directory_sync(&self, path: PathBuf) -> Result<Vec<Audiobook>, AppError> {
        // Use existing synchronous implementation
        self.scan_directory_blocking(path)
    }
    
    // New async Task-based method
    pub fn scan_directory_async(&self, path: PathBuf) -> Task<Result<Vec<Audiobook>, AppError>> {
        // New implementation using existing patterns
        self.scan_with_tasks(path)
    }
}
```

## Key Benefits of Architecture Integration

### 1. Consistency
- **Error Handling**: Single source of truth with `AppError` hierarchy
- **Configuration**: Unified config system with `AppConfig` and `UserPreferences`
- **Database**: Consistent patterns with existing `LibraryRepository`
- **Validation**: Integrated with existing `StateValidator` system

### 2. Performance
- **Memory Efficiency**: Leverage existing memory management patterns
- **Caching**: Build on existing validation and state persistence
- **Resource Management**: Use established resource cleanup patterns
- **Progress Reporting**: Integrate with existing async save patterns

### 3. Maintainability
- **Code Reuse**: Leverage existing, tested infrastructure
- **Pattern Consistency**: Follow established architectural patterns
- **Incremental Migration**: Gradual transition with backward compatibility
- **Future-Proofing**: Extensible design using existing plugin patterns

## Success Metrics

- **Integration Completeness**: 100% use of existing error, config, and database patterns
- **Performance**: Maintain current throughput while improving responsiveness
- **Memory**: No memory regression vs existing implementation  
- **Reliability**: Leverage existing validation and recovery systems
- **Code Reduction**: Eliminate duplicate patterns and infrastructure

## Dependencies

```toml
[dependencies]
# Core dependencies (already in use)
iced = { version = "0.13", features = ["tokio", "advanced"] }
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"

# Error handling (existing)
thiserror = "1.0"
miette = { version = "7.0", features = ["fancy"] }

# Performance (existing)
num_cpus = "1.0"
```

This integration approach ensures the thread pool refactoring enhances your existing architecture rather than creating parallel systems, reducing complexity while improving functionality.