//! Library view module

use iced::Length;
use iced::widget::{column, container};

use crate::components::audio_toolbar::AudioToolbar;
use crate::components::status::StatusDisplay;
use crate::components::table_core::AudiobookTable;
use crate::messages::Message;
use crate::state::UiState;
use crate::styling::container::LayoutContainerStyles;
use crate::styling::material::{MaterialSurface, SurfaceVariant};

/// Creates the library management view with browsing, scanning, and audiobook list
#[must_use]
pub fn library_view(state: &UiState) -> iced::Element<Message> {
    println!(
        "=== LIBRARY VIEW RENDER: {} audiobooks ===",
        state.audiobooks.len()
    );

    // Use the StatusDisplay component
    let status_display = StatusDisplay::view(
        state.scanning,
        state.scan_progress,
        state.processing_audio,
        state.processing_progress,
        state.processing_status.as_deref(),
        &state.player_state,
        state.current_playing_file.as_ref(),
        state.selected_audiobooks.len(),
        state.audiobooks.len(),
        state.theme_mode,
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

    println!(
        "=== TABLE CONTENT CREATED: {} audiobooks ===",
        state.audiobooks.len()
    );
    // Combine components into the library view with proper space allocation
    let footer = StatusDisplay::app_footer(state.audiobooks.len(), state.theme_mode);

    // Create content without redundant toolbar (now integrated into main toolbar)
    // Create content with proper constraints
    let content = column![
        // Status display with fixed height
        container(status_display)
            .width(Length::Fill)
            .height(Length::Shrink),
        // Audio toolbar with fixed height
        {
            let mut toolbar = AudioToolbar::new();
            toolbar.set_playing(matches!(
                state.player_state,
                abop_core::PlayerState::Playing
            ));

            container(toolbar.view(&state.material_tokens))
                .width(Length::Fill)
                .height(Length::Fixed(state.material_tokens.sizing().toolbar_height)) // Use unified toolbar height
        },
        // Table content that will scroll - wrapped in a fixed height container
        {
            println!("=== CREATING TABLE CONTAINER ===");

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
        },
        // Footer with fixed height (no need for additional container)
        footer
    ]
    .spacing(state.material_tokens.spacing.xs) // Reduced from SM (8px) to XS (4px)
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
