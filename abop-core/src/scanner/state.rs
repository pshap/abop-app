//! Scanner state management

/// Represents the current state of a scanner operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScannerState {
    /// Scanner is idle and ready to start
    Idle,
    /// Scanner is currently scanning
    Scanning,
    /// Scanner is paused
    Paused,
    /// Scanner has completed successfully
    Complete,
    /// Scanner encountered an error
    Error,
    /// Scanner was cancelled
    Cancelled,
}

impl Default for ScannerState {
    fn default() -> Self {
        Self::Idle
    }
}

impl ScannerState {
    /// Check if the scanner is in an active state
    pub const fn is_active(&self) -> bool {
        matches!(self, Self::Scanning | Self::Paused)
    }

    /// Check if the scanner is finished (completed, error, or cancelled)
    pub const fn is_finished(&self) -> bool {
        matches!(self, Self::Complete | Self::Error | Self::Cancelled)
    }

    /// Check if the scanner can be started
    pub const fn can_start(&self) -> bool {
        matches!(
            self,
            Self::Idle | Self::Complete | Self::Error | Self::Cancelled
        )
    }

    /// Check if the scanner can be paused
    pub const fn can_pause(&self) -> bool {
        matches!(self, Self::Scanning)
    }

    /// Check if the scanner can be resumed
    pub const fn can_resume(&self) -> bool {
        matches!(self, Self::Paused)
    }

    /// Check if the scanner can be cancelled
    pub const fn can_cancel(&self) -> bool {
        matches!(self, Self::Scanning | Self::Paused)
    }
}
