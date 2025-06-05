//! Progress tracking models and utilities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents playback progress for an audiobook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    /// Unique identifier for the progress record
    pub id: String,
    /// ID of the audiobook
    pub audiobook_id: String,
    /// Current playback position in seconds
    pub position_seconds: u64,
    /// Whether the audiobook has been completed
    pub completed: bool,
    /// When the audiobook was last played
    pub last_played: Option<DateTime<Utc>>,
    /// When the progress was first recorded
    pub created_at: DateTime<Utc>,
    /// When the progress was last updated
    pub updated_at: DateTime<Utc>,
}

impl Progress {
    /// Creates a new progress record for an audiobook
    #[must_use]
    pub fn new(audiobook_id: &str, position_seconds: u64) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            audiobook_id: audiobook_id.to_string(),
            position_seconds,
            completed: false,
            last_played: Some(now),
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the current position and marks as played
    pub fn update_position(&mut self, position_seconds: u64) {
        self.position_seconds = position_seconds;
        self.last_played = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Marks the audiobook as completed
    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.last_played = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Gets the progress percentage (0.0 to 1.0) if duration is known
    #[must_use]
    pub fn percentage(&self, total_duration_seconds: u64) -> Option<f32> {
        if total_duration_seconds == 0 {
            return None;
        }

        // Convert to f32 with explicit handling of precision loss
        // Note: u64 to f32 conversion may lose precision for large values (> 2^24)
        // but is acceptable for percentage calculations
        #[allow(clippy::cast_precision_loss)]
        let position = self.position_seconds as f32;

        #[allow(clippy::cast_precision_loss)]
        let total = total_duration_seconds as f32;

        // Prevent division by zero in case of unexpected float conversion
        if total <= 0.0 {
            return None;
        }

        Some(position / total)
    }
    /// Formats the position as a human-readable time string (HH:MM:SS)
    #[must_use]
    pub fn formatted_position(&self) -> String {
        crate::utils::time::format_seconds(
            self.position_seconds,
            crate::utils::time::TimeFormat::HoursWhenNonZero,
        )
    }

    /// Returns whether this progress record has been played recently (within 7 days)
    #[must_use]
    pub fn is_recently_played(&self) -> bool {
        self.last_played.is_some_and(|played| {
            let seven_days_ago = Utc::now() - chrono::Duration::days(7);
            played > seven_days_ago
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::*;

    #[test]
    fn test_progress_creation() {
        let progress = Progress::new(audiobook::TEST_ID, 300);
        assert_eq!(progress.audiobook_id, audiobook::TEST_ID);
        assert_eq!(progress.position_seconds, 300);
        assert!(!progress.completed);
        assert!(progress.last_played.is_some());
        assert!(!progress.id.is_empty());
    }

    #[test]
    fn test_progress_update() {
        let mut progress = Progress::new(audiobook::TEST_ID, 300);
        let original_updated = progress.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(1));
        progress.update_position(600);

        assert_eq!(progress.position_seconds, 600);
        assert!(progress.updated_at > original_updated);
        assert!(progress.last_played.is_some());
    }

    #[test]
    fn test_progress_percentage() {
        let progress = Progress::new(audiobook::TEST_ID, 1800); // 30 minutes

        // 30 minutes out of 60 minutes = 50%
        assert_eq!(progress.percentage(3600), Some(0.5));

        // No duration = no percentage
        assert_eq!(progress.percentage(0), None);
    }

    #[test]
    fn test_formatted_position() {
        let progress1 = Progress::new(audiobook::TEST_ID, 3665); // 1:01:05
        assert_eq!(progress1.formatted_position(), "01:01:05");

        let progress2 = Progress::new(audiobook::TEST_ID, 125); // 2:05
        assert_eq!(progress2.formatted_position(), "02:05");
    }

    #[test]
    fn test_mark_completed() {
        let mut progress = Progress::new(audiobook::TEST_ID, 300);
        assert!(!progress.completed);

        progress.mark_completed();
        assert!(progress.completed);
        assert!(progress.last_played.is_some());
    }
}
