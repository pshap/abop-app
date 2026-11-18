//! Settings dialog

use iced::widget::{Space, column, container, row, text, toggler};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Shadow, Vector};

use crate::components::ui::{self, ButtonKind};
use crate::messages::Message;
use crate::state::AppState;
use crate::styling::material::MaterialTokens;

/// Creates the settings dialog
#[must_use]
pub fn settings_view(state: &AppState) -> Element<'_, Message> {
    let tokens = &state.ui.material_tokens;
    let is_dark = matches!(state.ui.theme_mode, crate::theme::ThemeMode::Dark);

    let header = row![
        text("Settings").size(tokens.typography().title_large.size),
        Space::with_width(Length::Fill),
        ui::icon_button(
            tokens,
            ButtonKind::Ghost,
            "xmark",
            "Close (Esc)",
            Some(Message::CloseSettings),
        ),
    ]
    .align_y(Alignment::Center);

    let content = column![
        header,
        section_label(tokens, "Appearance"),
        setting_row(
            tokens,
            "Dark theme",
            "Use a dark color scheme",
            is_dark,
            Message::ToggleTheme,
        ),
        section_label(tokens, "Library"),
        setting_row(
            tokens,
            "Scan subfolders",
            "Include audiobooks in subfolders when scanning",
            state.library.scan_subdirectories,
            Message::ToggleScanSubdirectories,
        ),
        setting_row(
            tokens,
            "Auto-save library",
            "Save library changes automatically",
            state.library.auto_save_library,
            Message::ToggleAutoSaveLibrary,
        ),
        Space::with_height(8),
        row![
            Space::with_width(Length::Fill),
            ui::action_button(
                tokens,
                ButtonKind::Primary,
                None,
                "Done",
                Some(Message::CloseSettings)
            ),
        ],
    ]
    .spacing(12)
    .padding(Padding::from([20.0, 24.0]));

    container(content)
        .width(Length::Fixed(480.0))
        .style(move |_theme| container::Style {
            background: Some(Background::Color(tokens.colors.surface_container_high)),
            text_color: Some(tokens.colors.on_surface),
            border: Border {
                radius: 16.0.into(),
                ..Border::default()
            },
            shadow: Shadow {
                color: Color {
                    a: 0.4,
                    ..Color::BLACK
                },
                offset: Vector::new(0.0, 8.0),
                blur_radius: 24.0,
            },
        })
        .into()
}

/// Small accent-colored heading above a group of settings
fn section_label<'a>(tokens: &'a MaterialTokens, label: &'a str) -> Element<'a, Message> {
    container(
        text(label)
            .size(tokens.typography().label_medium.size)
            .color(tokens.colors.primary.base),
    )
    .padding(Padding::ZERO.top(8))
    .into()
}

/// A setting with a title, description and toggle switch
fn setting_row<'a>(
    tokens: &'a MaterialTokens,
    title: &'a str,
    description: &'a str,
    enabled: bool,
    on_toggle: Message,
) -> Element<'a, Message> {
    let switch = toggler(enabled)
        .on_toggle(move |_| on_toggle.clone())
        .size(22)
        .style(move |_theme, status| {
            let colors = &tokens.colors;
            let hovered = matches!(status, toggler::Status::Hovered { .. });
            let (background, foreground, border) = if enabled {
                (
                    colors.primary.base,
                    colors.primary.on_base,
                    colors.primary.base,
                )
            } else {
                (
                    colors.surface_container_highest,
                    colors.outline,
                    colors.outline,
                )
            };
            toggler::Style {
                background: if hovered {
                    ui::blend(background, colors.on_surface, 0.08)
                } else {
                    background
                },
                background_border_width: 2.0,
                background_border_color: border,
                foreground,
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
            }
        });

    row![
        column![
            text(title).size(tokens.typography().body_large.size),
            text(description)
                .size(tokens.typography().body_small.size)
                .color(tokens.colors.on_surface_variant),
        ]
        .spacing(2)
        .width(Length::Fill),
        switch,
    ]
    .spacing(16)
    .align_y(Alignment::Center)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_in_both_themes() {
        let mut state = AppState::default();
        let _ = settings_view(&state);
        state.ui.theme_mode = crate::theme::ThemeMode::Light;
        let _ = settings_view(&state);
    }
}
