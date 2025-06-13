// Audio controls component

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length};
use std::collections::HashSet;

use abop_core::PlayerState;
use abop_core::models::Audiobook;

use crate::components::buttons::{ButtonBuilder, ButtonVariant};
use crate::messages::Message;
use crate::styling::material::{MaterialSurface, MaterialTokens, SurfaceVariant};

/// Audio playback controls and manipulation component
///
/// This component provides playback, pause, and seek controls for audio playback.
/// It is used in the audio mixdown and library views.
///
/// # Examples
/// ```
/// use abop_gui::components::audio_controls::AudioControls;
/// use abop_gui::styling::material::MaterialTokens;
/// use abop_core::PlayerState;
/// use std::collections::HashSet;
/// use abop_core::models::audiobook::Audiobook;
///
/// let selected = HashSet::new();
/// let all = Vec::<Audiobook>::new();
/// let player_state = PlayerState::Stopped;
/// let material_tokens = &MaterialTokens::default();
/// let controls = AudioControls::view(&selected, &all, player_state, material_tokens);
/// ```
pub struct AudioControls;

impl AudioControls {
    /// Renders the audio controls view
    ///
    /// # Arguments
    /// * `selected_audiobooks` - List of selected audiobooks
    /// * `all_audiobooks` - List of all audiobooks
    /// * `player_state` - Current player state
    /// * `material_tokens` - Material Design 3 tokens for styling
    ///
    /// # Returns
    /// An Iced `Element` representing the audio controls UI
    #[must_use]
    pub fn view<'a>(
        selected_ids: &'a HashSet<String>,
        _audiobooks: &'a [Audiobook],
        player_state: PlayerState,
        material_tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let mut content = column![text("Audio Processing Controls").size(20),]
            .spacing(material_tokens.spacing().md);

        // Show processing options if audiobooks are selected
        if !selected_ids.is_empty() {
            let selected_count = selected_ids.len();
            content =
                content.push(text(format!("{selected_count} audiobook(s) selected")).size(16));            // Add process button
            content = content.push(
                ButtonBuilder::new(material_tokens)
                    .variant(ButtonVariant::Filled)
                    .label("Process Selected")
                    .on_press(Message::ProcessSelected)
                    .build()
                    .unwrap()
            );

            // Add preview button if playing
            match player_state {                PlayerState::Playing => {
                    content = content.push(
                        ButtonBuilder::new(material_tokens)
                            .variant(ButtonVariant::Outlined)
                            .label("Stop Preview")
                            .on_press(Message::StopPlayback)
                            .build()
                            .unwrap()
                    );
                }                PlayerState::Stopped => {
                    if selected_count == 1 {
                        content = content.push(
                            ButtonBuilder::new(material_tokens)
                                .variant(ButtonVariant::Outlined)
                                .label("Preview Audio")
                                .on_press(Message::StartPlayback)
                                .build()
                                .unwrap()
                        );
                    }
                }                PlayerState::Paused => {
                    content = content.push(
                        ButtonBuilder::new(material_tokens)
                            .variant(ButtonVariant::Outlined)
                            .label("Resume Preview")
                            .on_press(Message::StartPlayback)
                            .build()
                            .unwrap()
                    );
                }
            }
        }

        // Add processing options panel
        content = content.push(
            MaterialSurface::new()
                .variant(SurfaceVariant::SurfaceContainer)
                .padding(material_tokens.spacing().md)
                .container(
                    column![
                        text("Processing Options").size(18),
                        row![
                            text("Stereo to Mono:"),
                            text("Converts stereo audio to mono, reducing file size.")
                        ]
                        .spacing(material_tokens.spacing().md)
                        .align_y(Alignment::Start),
                        row![
                            text("Audio Playback:"),
                            text("Preview audio files before processing.")
                        ]
                        .spacing(material_tokens.spacing().md)
                        .align_y(Alignment::Start),
                    ]
                    .spacing(material_tokens.spacing().md)
                    .width(Length::Fill)
                    .into(),
                    material_tokens,
                ),
        );

        container(content)
            .padding(material_tokens.spacing().md)
            .width(Length::Fill)
            .into()
    }
}
