use iced::Element;
use iced::Length;
use iced::widget::{column, container, text};
use std::path::PathBuf;

use abop_core::PlayerState;

use crate::components::common::create_progress_indicator;
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
        theme: ThemeMode,
        tokens: &MaterialTokens,
    ) -> Element<'a, Message> {
        let mut status_column = column![];

        // Show scanning progress if active
        if scanning {
            status_column = status_column.push(create_progress_indicator(
                scan_progress,
                "Scanning library...",
                theme,
                tokens,
            ));
        }

        // Show audio processing progress if active
        if processing_audio {
            status_column = status_column.push(create_progress_indicator(
                processing_progress,
                processing_status.unwrap_or("Processing audio..."),
                theme,
                tokens,
            ));
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
}
