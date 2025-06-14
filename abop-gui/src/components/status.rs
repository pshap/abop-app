use iced::widget::{column, container, row, text};
use iced::{Element, Length};
use std::path::PathBuf;

use abop_core::PlayerState;
use abop_core::scanner::ScanProgress;

use crate::messages::Message;
use crate::styling::material::MaterialTokens;
use crate::theme::ThemeMode;

/// Parameters for the status display view
#[derive(Debug, Clone)]
pub struct StatusDisplayParams<'a> {
    /// Whether the library is being scanned
    pub scanning: bool,
    /// Progress of the scan operation
    pub scan_progress: Option<f32>,
    /// Whether audio is being processed
    pub processing_audio: bool,
    /// Progress of audio processing
    pub processing_progress: Option<f32>,
    /// Status message for processing
    pub processing_status: Option<&'a str>,
    /// Current audio player state
    pub player_state: PlayerState,
    /// Path to the currently playing file
    pub current_playing_file: Option<&'a PathBuf>,
    /// Number of selected audiobooks
    pub selected_count: usize,
    /// Total number of audiobooks
    pub total_count: usize,
    /// The current theme mode for styling
    pub theme: ThemeMode,
}

/// Parameters for the enhanced status display view
#[derive(Debug, Clone)]
pub struct EnhancedStatusDisplayParams<'a> {
    /// Whether the library is being scanned
    pub scanning: bool,
    /// Enhanced scan progress information
    pub scan_progress: Option<ScanProgress>,
    /// Cached scan progress text to avoid frequent formatting
    pub cached_scan_progress_text: Option<&'a str>,
    /// Whether audio is being processed
    pub processing_audio: bool,
    /// Progress of audio processing
    pub processing_progress: Option<f32>,
    /// Cached processing progress text to avoid frequent formatting
    pub cached_processing_progress_text: Option<&'a str>,
    /// Status message for processing
    pub processing_status: Option<&'a str>,
    /// Current audio player state
    pub player_state: PlayerState,
    /// Path to the currently playing file
    pub current_playing_file: Option<&'a PathBuf>,
    /// Number of selected audiobooks
    pub selected_count: usize,
    /// Total number of audiobooks
    pub total_count: usize,
    /// The current theme mode for styling
    pub theme: ThemeMode,
}

/// Status display component for showing application and playback status.
///
/// This struct is used to render the status bar in the application, providing feedback on
/// scanning, processing, and playback operations, as well as selection counts. It is typically
/// used in the library and audio mixdown views to keep users informed about ongoing actions.
pub struct StatusDisplay;

impl StatusDisplay {
    /// Renders the status display view
    ///
    /// # Arguments
    /// * `scanning` - Whether the library is being scanned
    /// * `scan_progress` - Progress of the scan operation
    /// * `processing_audio` - Whether audio is being processed
    /// * `processing_progress` - Progress of audio processing
    /// * `processing_status` - Status message for processing
    /// * `player_state` - Current audio player state
    /// * `current_playing_file` - Path to the currently playing file
    /// * `selected_count` - Number of selected audiobooks
    /// * `total_count` - Total number of audiobooks
    /// * `theme` - The current theme mode for styling
    /// * `tokens` - Material Design tokens for styling
    ///
    /// # Returns    /// An Iced `Element` representing the status display UI
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn view<'a>(
        scanning: bool,
        scan_progress: Option<f32>,
        processing_audio: bool,
        processing_progress: Option<f32>,
        processing_status: Option<&'a str>,
        _player_state: &PlayerState,
        _current_playing_file: Option<&PathBuf>,
        _selected_count: usize, // Mark as intentionally unused
        _total_count: usize,    // Mark as intentionally unused
        _theme: ThemeMode,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let mut status_column = column![];

        // Show scanning progress if active
        if scanning {
            let scan_label = String::from("Scanning library...");
            let progress_text = if let Some(progress) = scan_progress {
                format!("{:.1}% - {}", progress * 100.0, scan_label)
            } else {
                scan_label
            };
            status_column = status_column.push(text(progress_text));
        }

        // Show audio processing progress if active
        if processing_audio {
            let processing_label = processing_status
                .unwrap_or("Processing audio...")
                .to_string();
            let progress_text = if let Some(progress) = processing_progress {
                format!("{:.1}% - {}", progress * 100.0, processing_label)
            } else {
                processing_label
            };
            status_column = status_column.push(text(progress_text));
        }

        status_column.spacing(tokens.spacing().md as u16).into()
    }

    /// Creates a footer bar for displaying the total audiobooks available
    #[must_use]
    pub fn app_footer<'a>(total_count: usize, _theme: ThemeMode) -> Element<'a, Message> {
        let footer_text = format!("{total_count} audiobooks available");

        // Create a simple container with the footer text
        container(text(footer_text).size(14))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fixed(36.0)) // Fixed height for the footer
            .into()
    }

    /// Enhanced status display with detailed progress information and ETA
    #[must_use]
    pub fn enhanced_view<'a>(
        params: EnhancedStatusDisplayParams<'a>,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let mut status_column = column![];

        // Show enhanced scanning progress if active
        if params.scanning {
            if let Some(progress) = &params.scan_progress {
                // Extract information from ScanProgress enum
                let (progress_percentage, processed, total, current_file) = match progress {
                    abop_core::scanner::ScanProgress::Started { total_files } => {
                        (0.0, 0, *total_files, "Starting scan...")
                    }
                    abop_core::scanner::ScanProgress::FileProcessed {
                        current,
                        total,
                        file_name,
                        progress_percentage,
                    } => (*progress_percentage, *current, *total, file_name.as_str()),
                    abop_core::scanner::ScanProgress::BatchCommitted {
                        total_processed, ..
                    } => (
                        0.5,
                        *total_processed,
                        *total_processed,
                        "Processing batch...",
                    ),
                    abop_core::scanner::ScanProgress::Complete { processed, .. } => {
                        (1.0, *processed, *processed, "Scan completed!")
                    }
                    abop_core::scanner::ScanProgress::Cancelled { processed, .. } => {
                        (0.0, *processed, *processed, "Scan cancelled")
                    }
                };

                // Use cached progress text if available, otherwise fall back to formatting
                let progress_text = if let Some(cached_text) = params.cached_scan_progress_text {
                    text(cached_text).size(12)
                } else {
                    text(format!("Progress: {:.1}%", progress_percentage * 100.0)).size(12)
                };
                let scan_label =
                    format!("Scanning: {processed} of {total} files\nCurrent: {current_file}");
                let progress_display = text(scan_label);
                let progress_row = column![
                    progress_display,
                    row![progress_text].spacing(tokens.spacing().md as u16)
                ]
                .spacing(tokens.spacing().sm as u16);

                status_column = status_column.push(progress_row);
            } else {
                let scan_label = String::from("Scanning library...");
                status_column = status_column.push(text(scan_label));
            }
        }

        // Show audio processing progress if active
        if params.processing_audio {
            let processing_status = params
                .processing_status
                .unwrap_or("Processing audio...")
                .to_string();
            let progress_text = if let Some(cached_text) = params.cached_processing_progress_text {
                // Use cached percentage text with status
                format!("{cached_text} - {processing_status}")
            } else if let Some(progress) = params.processing_progress {
                // Fall back to direct formatting if no cache available
                format!("{:.1}% - {}", progress * 100.0, processing_status)
            } else {
                processing_status
            };
            status_column = status_column.push(text(progress_text));
        }

        status_column.spacing(tokens.spacing().md as u16).into()
    }
}
