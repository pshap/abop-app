//! Audio toolbar component for playback controls
//!
//! Provides a dedicated audio control toolbar with play/pause, stop, previous, and next buttons.

use iced::widget::{container, row};
use iced::{Alignment, Element, Length};

use crate::components::icons::icon_names;
use crate::messages::Message;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::widgets::ButtonSize;
use crate::styling::material::components::widgets::MaterialButtonVariant;

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
        };

        use crate::components::common::material_icon_button_widget;

        let play_button = material_icon_button_widget(
            play_icon,
            MaterialButtonVariant::Outlined,
            ButtonSize::Small,
            Message::PlayPause,
            material_tokens,
        );

        let stop_button = material_icon_button_widget(
            icon_names::STOP,
            MaterialButtonVariant::Outlined,
            ButtonSize::Small,
            Message::Stop,
            material_tokens,
        );

        let previous_button = material_icon_button_widget(
            icon_names::SKIP_PREVIOUS,
            MaterialButtonVariant::Outlined,
            ButtonSize::Small,
            Message::Previous,
            material_tokens,
        );

        let next_button = material_icon_button_widget(
            icon_names::SKIP_NEXT,
            MaterialButtonVariant::Outlined,
            ButtonSize::Small,
            Message::Next,
            material_tokens,
        );

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
