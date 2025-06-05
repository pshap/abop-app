//! Core data types and shared components

use serde::{Deserialize, Serialize};

/// Represents a chapter in an audiobook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// Start time in seconds
    pub start_time: u64,
    /// End time in seconds
    pub end_time: u64,
    /// Chapter title
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapter_creation() {
        let chapter = Chapter {
            start_time: 0,
            end_time: 300,
            title: "Introduction".to_string(),
        };

        assert_eq!(chapter.start_time, 0);
        assert_eq!(chapter.end_time, 300);
        assert_eq!(chapter.title, "Introduction");
    }
}
