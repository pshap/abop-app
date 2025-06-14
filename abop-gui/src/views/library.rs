//! Library view module

use iced::Length;
use iced::widget::{column, container};

use crate::components::audio_toolbar::AudioToolbar;
use crate::components::status::{EnhancedStatusDisplayParams, StatusDisplay};
use crate::components::table_core::AudiobookTable;
use crate::messages::Message;
use crate::state::UiState;
use crate::styling::container::LayoutContainerStyles;
use crate::styling::material::{MaterialSurface, SurfaceVariant};

/// Creates the library management view with browsing, scanning, and audiobook list
#[must_use]
pub fn library_view(state: &UiState) -> iced::Element<'_, Message> {
    log::debug!("LIBRARY VIEW RENDER: {} audiobooks", state.audiobooks.len());    // Use the enhanced StatusDisplay component with detailed progress information
    let status_display = StatusDisplay::enhanced_view(
        EnhancedStatusDisplayParams {
            scanning: state.scanning,
            scan_progress: state.enhanced_scan_progress.clone(),
            cached_scan_progress_text: state.cached_scan_progress_text.as_deref(),
            processing_audio: state.processing_audio,
            processing_progress: state.processing_progress,
            cached_processing_progress_text: state.cached_processing_progress_text.as_deref(),
            processing_status: state.processing_status.as_deref(),
            player_state: state.player_state.clone(),
            current_playing_file: state.current_playing_file.as_ref(),
            selected_count: state.selected_audiobooks.len(),
            total_count: state.audiobooks.len(),
            theme: state.theme_mode,
        },
        &state.material_tokens,
    );

    log::debug!(
        "Creating table with {} audiobooks, {} selected",
        state.audiobooks.len(),
        state.selected_audiobooks.len()
    );

    // Use the AudiobookTable component with Material Design tokens
    let table_content = AudiobookTable::view(
        &state.audiobooks,
        &state.selected_audiobooks,
        &state.table_state,
        &state.material_tokens,
    );

    log::debug!(
        "TABLE CONTENT CREATED: {} audiobooks",
        state.audiobooks.len()
    );
    // Combine components into the library view with proper space allocation
    let footer = StatusDisplay::app_footer(state.audiobooks.len(), state.theme_mode);

    // Create content without redundant toolbar (now integrated into main toolbar)
    // Create content with proper constraints
    let mut content_items = vec![
        // Status display with fixed height
        container(status_display)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into(),
    ];

    // Only show audio toolbar when audiobooks are selected
    if !state.selected_audiobooks.is_empty() {
        let mut toolbar = AudioToolbar::new();
        toolbar.set_playing(matches!(
            state.player_state,
            abop_core::PlayerState::Playing
        ));

        content_items.push(
            container(toolbar.view(&state.material_tokens))
                .width(Length::Fill)
                .height(Length::Fixed(state.material_tokens.sizing().toolbar_height)) // Use unified toolbar height
                .into(),
        );
    }

    // Add table content
    content_items.push({
        log::debug!("CREATING TABLE CONTAINER");

        // Debug container with border and background
        let debug_container = container(table_content)
            .width(Length::Fill)
            .height(Length::Fill);

        container(debug_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &iced::Theme| {
                MaterialSurface::new()
                    .variant(SurfaceVariant::SurfaceContainerLow)
                    .style(&state.material_tokens)
            })
            .padding(iced::Padding {
                top: 8.0,
                right: 8.0,
                bottom: 8.0,
                left: 8.0,
            })
            .into()
    });

    // Add footer
    content_items.push(footer);

    let content = column(content_items)
        .spacing(4) // MD3: minimal vertical spacing between toolbars
        .width(Length::Fill)
        .height(Length::Fill) // Fill available height
        .padding(state.material_tokens.spacing.md); // Add some padding around the content

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(LayoutContainerStyles::content(state.theme_mode))
        .padding(state.material_tokens.spacing.sm) // Reduced from MD (16px) to SM (8px)
        .into()
}
