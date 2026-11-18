//! Status bar shown along the bottom of the window
//!
//! Left: what the app is doing (scanning, processing, playing, ready).
//! Right: selection count and keyboard hint.

use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Background, Border, Element, Length, Padding};

use crate::components::ui;
use crate::messages::Message;
use crate::state::{AppState, NoticeKind};

/// Height of the status bar
pub const STATUS_BAR_HEIGHT: f32 = 30.0;

/// Status bar component
pub struct StatusBar;

impl StatusBar {
    /// Renders the status bar for the current state
    #[must_use]
    pub fn view(state: &AppState) -> Element<'_, Message> {
        let tokens = &state.ui.material_tokens;
        let muted = tokens.colors.on_surface_variant;
        let size = tokens.typography().body_small.size;
        let (icon_name, message, accent) = status_message(state);
        let icon_color = if accent {
            tokens.colors.primary.base
        } else {
            muted
        };

        let mut bar = row![
            ui::icon(icon_name, size, icon_color),
            text(message)
                .size(size)
                .color(muted)
                .wrapping(text::Wrapping::None),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        // Compact transport controls while something is playing or paused
        let transport = match state.player.player_state {
            abop_core::PlayerState::Playing => Some(("pause", "Pause (Space)")),
            abop_core::PlayerState::Paused => Some(("play", "Resume (Space)")),
            abop_core::PlayerState::Stopped => None,
        };
        if let Some((toggle_icon, toggle_tip)) = transport {
            bar = bar
                .push(compact_button(
                    state,
                    toggle_icon,
                    toggle_tip,
                    Message::PlayPause,
                ))
                .push(compact_button(state, "stop", "Stop", Message::Stop));
        }
        bar = bar.push(Space::with_width(Length::Fill));

        if let Some(notice) = &state.ui.notice {
            let (icon_name, color) = match notice.kind {
                NoticeKind::Error => ("circle-exclamation", tokens.colors.error.base),
                NoticeKind::Info => ("circle-info", tokens.colors.on_surface),
            };
            let dismiss = compact_button(state, "xmark", "Dismiss", Message::DismissNotice);
            bar = bar.push(
                row![
                    ui::icon(icon_name, size, color),
                    text(notice.message.as_str())
                        .size(size)
                        .color(color)
                        .wrapping(text::Wrapping::None),
                    dismiss,
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            );
        }

        let selected = state.library.selected_audiobooks.len();
        if selected > 0 {
            bar = bar.push(text(format!("{selected} selected")).size(size).color(muted));
        }

        container(bar)
            .width(Length::Fill)
            .height(Length::Fixed(STATUS_BAR_HEIGHT))
            .padding(Padding::from([0.0, 20.0]))
            .align_y(Alignment::Center)
            .style(move |_theme| container::Style {
                background: Some(Background::Color(tokens.colors.surface_container)),
                border: Border::default(),
                ..container::Style::default()
            })
            .into()
    }
}

/// A small ghost icon button sized for the status bar, with a tooltip
fn compact_button<'a>(
    state: &'a AppState,
    icon_name: &str,
    tip: &'a str,
    message: Message,
) -> Element<'a, Message> {
    let tokens = &state.ui.material_tokens;
    let size = tokens.typography().body_small.size;
    let btn = button(ui::icon(icon_name, size, tokens.colors.on_surface_variant))
        .padding(Padding::from([2.0, 6.0]))
        .style(move |_theme, status| ui::button_style(tokens, ui::ButtonKind::Ghost, status))
        .on_press(message);
    ui::with_tooltip(tokens, btn, tip)
}

/// Icon, text and whether to highlight the icon for the current activity
fn status_message(state: &AppState) -> (&'static str, String, bool) {
    let library = &state.library;
    let player = &state.player;

    if library.is_scanning() {
        let progress = library
            .get_scan_progress_legacy()
            .map(|p| format!(" {:.0}%", p * 100.0))
            .unwrap_or_default();
        return (
            "arrows-rotate",
            format!("Scanning library…{progress}"),
            true,
        );
    }
    if player.processing_audio {
        let status = player
            .processing_status
            .as_deref()
            .unwrap_or("Processing audio…");
        let progress = player
            .processing_progress
            .map(|p| format!(" ({:.0}%)", p * 100.0))
            .unwrap_or_default();
        return ("wand-magic-sparkles", format!("{status}{progress}"), true);
    }

    let now_playing = || {
        player
            .current_playing_file
            .as_ref()
            .and_then(|path| {
                // Prefer the audiobook's title over its file name
                library
                    .audiobooks
                    .iter()
                    .find(|book| &book.path == path)
                    .and_then(|book| book.title.clone())
                    .or_else(|| path.file_stem().map(|s| s.to_string_lossy().into_owned()))
            })
            .unwrap_or_else(|| "audio".to_string())
    };
    match player.player_state {
        abop_core::PlayerState::Playing => {
            return ("play", format!("Playing “{}”", now_playing()), true);
        }
        abop_core::PlayerState::Paused => {
            return ("pause", format!("Paused “{}”", now_playing()), false);
        }
        abop_core::PlayerState::Stopped => {}
    }

    if library.scan_failed() {
        return (
            "triangle-exclamation",
            "Last scan failed".to_string(),
            false,
        );
    }
    if let Some(status) = player.processing_status.as_deref() {
        return ("circle-check", status.to_string(), false);
    }
    if library.audiobooks.is_empty() {
        ("circle-check", "Ready".to_string(), false)
    } else {
        (
            "circle-check",
            "Ready — double-click an audiobook to play it".to_string(),
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_state_reports_ready() {
        let state = AppState::default();
        assert_eq!(status_message(&state).1, "Ready");
    }

    #[test]
    fn scanning_takes_priority() {
        let mut state = AppState::default();
        state.library.start_scanning();
        state.player.player_state = abop_core::PlayerState::Playing;
        assert!(status_message(&state).1.starts_with("Scanning library"));
    }

    #[test]
    fn playing_shows_audiobook_title() {
        let mut state = AppState::default();
        let mut book = abop_core::models::Audiobook::new("lib", "/books/a.mp3");
        book.title = Some("The Martian".to_string());
        state.library.audiobooks.push(book);
        state.player.player_state = abop_core::PlayerState::Playing;
        state.player.current_playing_file = Some("/books/a.mp3".into());
        assert_eq!(status_message(&state).1, "Playing “The Martian”");
    }
}
