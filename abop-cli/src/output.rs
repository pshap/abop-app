//! JSON output structures for ABOP CLI
//!
//! This module provides structured output formats for machine consumption.
//! All output structures are designed to be stable and backwards-compatible.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Top-level JSON output structure for all CLI operations
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum CliOutput {
    /// Successful operation result
    #[serde(rename = "success")]
    Success { data: OutputData },
    /// Error result
    #[serde(rename = "error")]
    Error { error: ErrorOutput },
}

/// Data payload for successful operations
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "operation")]
pub enum OutputData {
    /// Scan operation results
    #[serde(rename = "scan")]
    Scan(ScanOutput),
    /// Database operation results
    #[serde(rename = "database")]
    Database(DatabaseOutput),
}

/// Scan operation output
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanOutput {
    /// Total number of audiobooks found
    pub total_audiobooks: usize,
    /// Libraries that were scanned
    pub libraries: Vec<LibraryInfo>,
    /// Sample audiobooks (first few for preview)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sample_audiobooks: Vec<AudiobookInfo>,
    /// Scan performance metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<ScanMetrics>,
}

/// Database operation output
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum DatabaseOutput {
    /// Database initialization result
    #[serde(rename = "init")]
    Init { database_path: PathBuf },
    /// Database statistics
    #[serde(rename = "stats")]
    Stats(DatabaseStats),
    /// Audiobook listing
    #[serde(rename = "list")]
    List {
        total_count: usize,
        audiobooks: Vec<AudiobookInfo>,
    },
    /// Database cleanup result
    #[serde(rename = "clean")]
    Clean { libraries_validated: usize },
}

/// Library information
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryInfo {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub audiobook_count: usize,
}

/// Audiobook information for JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiobookInfo {
    pub id: String,
    pub title: String,
    pub author: String,
    pub narrator: Option<String>,
    pub path: PathBuf,
    pub duration_seconds: Option<u64>,
    pub size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Database statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub total_audiobooks: usize,
    pub total_libraries: usize,
}

/// Scan performance metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanMetrics {
    pub files_scanned: usize,
    pub duration_ms: u64,
    pub files_per_second: f64,
}

/// Error output structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub message: String,
    pub error_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<String>>,
}

impl CliOutput {
    /// Create a successful scan result
    pub fn scan_success(
        total_audiobooks: usize,
        libraries: Vec<LibraryInfo>,
        sample_audiobooks: Vec<AudiobookInfo>,
    ) -> Self {
        Self::Success {
            data: OutputData::Scan(ScanOutput {
                total_audiobooks,
                libraries,
                sample_audiobooks,
                metrics: None,
            }),
        }
    }

    /// Create a successful database stats result
    pub fn database_stats_success(total_audiobooks: usize, total_libraries: usize) -> Self {
        Self::Success {
            data: OutputData::Database(DatabaseOutput::Stats(DatabaseStats {
                total_audiobooks,
                total_libraries,
            })),
        }
    }

    /// Create a successful database init result
    pub fn database_init_success(database_path: PathBuf) -> Self {
        Self::Success {
            data: OutputData::Database(DatabaseOutput::Init { database_path }),
        }
    }

    /// Create a successful database list result
    pub fn database_list_success(audiobooks: Vec<AudiobookInfo>) -> Self {
        let total_count = audiobooks.len();
        Self::Success {
            data: OutputData::Database(DatabaseOutput::List {
                total_count,
                audiobooks,
            }),
        }
    }

    /// Create a successful database clean result
    pub fn database_clean_success(libraries_validated: usize) -> Self {
        Self::Success {
            data: OutputData::Database(DatabaseOutput::Clean {
                libraries_validated,
            }),
        }
    }

    /// Create an error result
    pub fn error(message: String, error_type: String, context: Option<Vec<String>>) -> Self {
        Self::Error {
            error: ErrorOutput {
                message,
                error_type,
                context,
            },
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl From<&abop_core::models::Library> for LibraryInfo {
    fn from(lib: &abop_core::models::Library) -> Self {
        Self {
            id: lib.id.clone(),
            name: lib.name.clone(),
            path: lib.path.clone(),
            audiobook_count: 0, // Will be filled separately
        }
    }
}

impl From<&abop_core::models::Audiobook> for AudiobookInfo {
    fn from(book: &abop_core::models::Audiobook) -> Self {
        Self {
            id: book.id.clone(),
            title: book.title.clone().unwrap_or_else(|| "Unknown Title".to_string()),
            author: book.author.clone().unwrap_or_else(|| "Unknown Author".to_string()),
            narrator: book.narrator.clone(),
            path: book.path.clone(),
            duration_seconds: book.duration_seconds,
            size_bytes: book.size_bytes,
            description: book.description.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_output_serialization() {
        let output = CliOutput::scan_success(
            5,
            vec![LibraryInfo {
                id: "lib1".to_string(),
                name: "Test Library".to_string(),
                path: PathBuf::from("/test/path"),
                audiobook_count: 5,
            }],
            vec![AudiobookInfo {
                id: "book1".to_string(),
                title: "Test Book".to_string(),
                author: "Test Author".to_string(),
                narrator: None,
                path: PathBuf::from("/test/book.mp3"),
                duration_seconds: Some(3600),
                size_bytes: Some(1024000),
                description: None,
            }],
        );

        let json = output.to_json().expect("Should serialize to JSON");
        assert!(json.contains("success"));
        assert!(json.contains("scan"));
        assert!(json.contains("Test Library"));
    }

    #[test]
    fn test_error_output_serialization() {
        let output = CliOutput::error(
            "Test error message".to_string(),
            "TestError".to_string(),
            Some(vec!["Context 1".to_string(), "Context 2".to_string()]),
        );

        let json = output.to_json().expect("Should serialize to JSON");
        assert!(json.contains("error"));
        assert!(json.contains("Test error message"));
        assert!(json.contains("TestError"));
    }

    #[test]
    fn test_database_stats_serialization() {
        let output = CliOutput::database_stats_success(42, 3);

        let json = output.to_json().expect("Should serialize to JSON");
        assert!(json.contains("success"));
        assert!(json.contains("database"));
        assert!(json.contains("stats"));
        assert!(json.contains("42"));
    }

    #[test]
    fn test_database_init_serialization() {
        let output = CliOutput::database_init_success(PathBuf::from("/test/db.sqlite"));

        let json = output.to_json().expect("Should serialize to JSON");
        assert!(json.contains("success"));
        assert!(json.contains("database"));
        assert!(json.contains("init"));
        assert!(json.contains("db.sqlite"));
    }

    #[test]
    fn test_database_list_serialization() {
        let audiobooks = vec![AudiobookInfo {
            id: "book1".to_string(),
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            narrator: Some("Test Narrator".to_string()),
            path: PathBuf::from("/test/book.mp3"),
            duration_seconds: Some(3600),
            size_bytes: Some(1024000),
            description: Some("Test description".to_string()),
        }];

        let output = CliOutput::database_list_success(audiobooks);

        let json = output.to_json().expect("Should serialize to JSON");
        assert!(json.contains("success"));
        assert!(json.contains("database"));
        assert!(json.contains("list"));
        assert!(json.contains("Test Book"));
        assert!(json.contains("Test Author"));
        assert!(json.contains("Test Narrator"));
    }

    #[test]
    fn test_database_clean_serialization() {
        let output = CliOutput::database_clean_success(5);

        let json = output.to_json().expect("Should serialize to JSON");
        assert!(json.contains("success"));
        assert!(json.contains("database"));
        assert!(json.contains("clean"));
        assert!(json.contains("5"));
    }

    #[test]
    fn test_scan_output_with_metrics() {
        let mut output = CliOutput::scan_success(
            10,
            vec![LibraryInfo {
                id: "lib1".to_string(),
                name: "Test Library".to_string(),
                path: PathBuf::from("/test/path"),
                audiobook_count: 10,
            }],
            vec![],
        );

        // Add metrics
        if let CliOutput::Success { data: OutputData::Scan(ref mut scan_output) } = output {
            scan_output.metrics = Some(ScanMetrics {
                files_scanned: 100,
                duration_ms: 5000,
                files_per_second: 20.0,
            });
        }

        let json = output.to_json().expect("Should serialize to JSON");
        assert!(json.contains("success"));
        assert!(json.contains("scan"));
        assert!(json.contains("metrics"));
        assert!(json.contains("100"));
        assert!(json.contains("5000"));
        assert!(json.contains("20.0"));
    }

    #[test]
    fn test_error_with_context_serialization() {
        let output = CliOutput::error(
            "Main error message".to_string(),
            "ValidationError".to_string(),
            Some(vec!["Context 1".to_string(), "Context 2".to_string()]),
        );

        let json = output.to_json().expect("Should serialize to JSON");
        assert!(json.contains("error"));
        assert!(json.contains("Main error message"));
        assert!(json.contains("ValidationError"));
        assert!(json.contains("Context 1"));
        assert!(json.contains("Context 2"));
    }

    #[test]
    fn test_audiobook_info_from_conversion() {
        use abop_core::models::Audiobook;
        use std::path::PathBuf;

        let audiobook = Audiobook {
            id: "test_id".to_string(),
            title: Some("Test Title".to_string()),
            author: Some("Test Author".to_string()),
            narrator: Some("Test Narrator".to_string()),
            path: PathBuf::from("/test/path.mp3"),
            duration_seconds: Some(3600),
            size_bytes: Some(1024000),
            description: Some("Test description".to_string()),
            library_id: "lib1".to_string(),
            cover_art: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            selected: false,
        };

        let info = AudiobookInfo::from(&audiobook);

        assert_eq!(info.id, "test_id");
        assert_eq!(info.title, "Test Title");
        assert_eq!(info.author, "Test Author");
        assert_eq!(info.narrator, Some("Test Narrator".to_string()));
        assert_eq!(info.path, PathBuf::from("/test/path.mp3"));
        assert_eq!(info.duration_seconds, Some(3600));
        assert_eq!(info.size_bytes, Some(1024000));
        assert_eq!(info.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_audiobook_info_from_conversion_with_none_fields() {
        use abop_core::models::Audiobook;
        use std::path::PathBuf;

        let audiobook = Audiobook {
            id: "test_id".to_string(),
            title: None,
            author: None,
            narrator: None,
            path: PathBuf::from("/test/path.mp3"),
            duration_seconds: None,
            size_bytes: None,
            description: None,
            library_id: "lib1".to_string(),
            cover_art: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            selected: false,
        };

        let info = AudiobookInfo::from(&audiobook);

        assert_eq!(info.id, "test_id");
        assert_eq!(info.title, "Unknown Title");
        assert_eq!(info.author, "Unknown Author");
        assert_eq!(info.narrator, None);
        assert_eq!(info.path, PathBuf::from("/test/path.mp3"));
        assert_eq!(info.duration_seconds, None);
        assert_eq!(info.size_bytes, None);
        assert_eq!(info.description, None);
    }
}