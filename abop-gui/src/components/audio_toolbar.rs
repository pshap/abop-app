//! Audio toolbar component for playback controls
//!
//! Provides a dedicated audio control toolbar with play/pause, stop, previous, and next buttons.

use iced::widget::{container, row};
use iced::{Alignment, Element, Length};

use crate::components::buttons::variants::ButtonSize;
use crate::components::buttons::{self, ButtonVariant};
use crate::components::icons::icon_names;
use crate::messages::Message;
use crate::styling::material::MaterialTokens;

/// Audio toolbar component for playback controls
#[derive(Debug, Clone, Default)]
pub struct AudioToolbar {
    /// Whether audio is currently playing
    pub is_playing: bool,
}

impl AudioToolbar {
    /// Create a new audio toolbar
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the playing state
    pub const fn set_playing(&mut self, is_playing: bool) {
        self.is_playing = is_playing;
    }
    /// Render the audio toolbar
    #[must_use]
    pub fn view<'a>(&self, material_tokens: &'a MaterialTokens) -> Element<'a, Message> {
        let play_icon = if self.is_playing {
            icon_names::PAUSE
        } else {
            icon_names::PLAY
        };        let play_button = buttons::create_button(
            || {
                buttons::button(material_tokens)
                    .icon_only(play_icon, ButtonSize::Small)
                    .variant(ButtonVariant::Outlined)
                    .on_press(Message::PlayPause)
                    .build()
            },
            "play/pause",
            Some("Play"),
        );

        let stop_button = buttons::create_button(
            || {
                buttons::button(material_tokens)
                    .icon_only(icon_names::STOP, ButtonSize::Small)
                    .variant(ButtonVariant::Outlined)
                    .on_press(Message::Stop)
                    .build()
            },
            "stop",
            Some("Stop"),
        );

        let previous_button = buttons::button(material_tokens)
            .icon_only(icon_names::SKIP_PREVIOUS, ButtonSize::Small)
            .variant(ButtonVariant::Outlined)
            .on_press(Message::Previous)
            .build()
            .unwrap_or_else(|e| {
                log::warn!("Failed to build previous button: {e}");
                iced::widget::Text::new("").into()
            });

        let next_button = buttons::button(material_tokens)
            .icon_only(icon_names::SKIP_NEXT, ButtonSize::Small)
            .variant(ButtonVariant::Outlined)
            .on_press(Message::Next)
            .build()
            .unwrap_or_else(|e| {
                log::warn!("Failed to build next button: {e}");
                iced::widget::Text::new("").into()
            });

        // Audio controls with proper spacing and alignment
        container(
            row![previous_button, play_button, stop_button, next_button]
                .spacing(material_tokens.spacing().sm) // Use SM spacing (8px)
                .align_y(Alignment::Center),
        )
        .width(Length::Shrink)
        .height(Length::Shrink)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .padding(material_tokens.spacing().sm) // Consistent padding all around
        .into()
    }
}
