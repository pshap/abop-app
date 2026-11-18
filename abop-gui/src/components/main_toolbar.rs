//! Application bar shown at the top of every view
//!
//! Layout: `[logo ABOP | current library] ........ [Open Folder] [Rescan] | [Settings] [Exit]`

use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Background, Border, Element, Length, Padding};

use crate::components::ui::{self, ButtonKind};
use crate::messages::{Command, Message};
use crate::state::AppState;
use crate::styling::material::MaterialTokens;

/// Height of the application bar
pub const APP_BAR_HEIGHT: f32 = 64.0;

/// Application bar component
pub struct MainToolbar;

impl MainToolbar {
    /// Renders the application bar for the current state
    #[must_use]
    pub fn view(state: &AppState) -> Element<'_, Message> {
        let tokens = &state.ui.material_tokens;
        let library_path = &state.library.library_path;
        // The configured default path may not exist until the user picks a folder
        let has_library = library_path.is_dir();
        let scanning = state.library.is_scanning();

        let brand = row![
            ui::icon(
                "headphones",
                tokens.typography().title_large.size,
                tokens.colors.primary.base
            ),
            text("ABOP")
                .size(tokens.typography().title_large.size)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..iced::Font::DEFAULT
                }),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let open_folder = ui::action_button(
            tokens,
            ButtonKind::Tonal,
            Some("folder-open"),
            "Open Folder",
            Some(Message::command(Command::BrowseDirectory)),
        );
        let rescan = ui::action_button(
            tokens,
            ButtonKind::Ghost,
            Some("arrows-rotate"),
            if scanning { "Scanning…" } else { "Rescan" },
            (has_library && !scanning).then(|| {
                Message::command(Command::ScanLibrary {
                    library_path: library_path.clone(),
                })
            }),
        );
        let settings = ui::icon_button(
            tokens,
            ButtonKind::Ghost,
            "gear",
            "Settings (Ctrl+,)",
            Some(Message::ShowSettings),
        );
        let exit = ui::icon_button(
            tokens,
            ButtonKind::Danger,
            "power-off",
            "Exit (Ctrl+Q)",
            Some(Message::command(Command::Quit)),
        );

        let bar = row![
            brand,
            ui::vertical_divider(tokens, 28.0),
            Self::library_label(state, tokens),
            Space::with_width(Length::Fill),
            open_folder,
            ui::with_tooltip(tokens, rescan, "Scan the library folder again"),
            ui::vertical_divider(tokens, 28.0),
            settings,
            exit,
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        container(bar)
            .width(Length::Fill)
            .height(Length::Fixed(APP_BAR_HEIGHT))
            .padding(Padding::from([0.0, 20.0]))
            .align_y(Alignment::Center)
            .style(move |_theme| container::Style {
                background: Some(Background::Color(tokens.colors.surface_container)),
                text_color: Some(tokens.colors.on_surface),
                border: Border {
                    color: tokens.colors.outline_variant,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..container::Style::default()
            })
            .into()
    }

    /// Name and location of the open library, or a prompt when none is open
    fn library_label<'a>(state: &'a AppState, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        let path = &state.library.library_path;
        let muted = tokens.colors.on_surface_variant;

        if !path.is_dir() {
            return text("No library open")
                .size(tokens.typography().body_medium.size)
                .color(muted)
                .into();
        }

        let name = path
            .file_name()
            .map_or_else(|| path.to_string_lossy(), |n| n.to_string_lossy())
            .into_owned();
        let location = path
            .parent()
            .map(|p| abbreviate_home(&p.to_string_lossy()))
            .unwrap_or_default();

        row![
            ui::icon("folder", 16.0, muted),
            column![
                text(name)
                    .size(tokens.typography().title_small.size)
                    .wrapping(text::Wrapping::None),
                text(location)
                    .size(tokens.typography().body_small.size)
                    .color(muted)
                    .wrapping(text::Wrapping::None),
            ]
            .spacing(0),
        ]
        .spacing(10)
        .align_y(Alignment::Center)
        .into()
    }
}

/// Replace the user's home directory prefix with `~` for compact display
fn abbreviate_home(path: &str) -> String {
    match std::env::var("HOME") {
        Ok(home) if !home.is_empty() && path.starts_with(&home) => {
            format!("~{}", &path[home.len()..])
        }
        _ => path.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abbreviates_paths_under_home() {
        let Ok(home) = std::env::var("HOME") else {
            return;
        };
        assert_eq!(
            abbreviate_home(&format!("{home}/Audiobooks")),
            "~/Audiobooks"
        );
        assert_eq!(abbreviate_home("/mnt/media"), "/mnt/media");
    }

    #[test]
    fn renders_with_and_without_library() {
        let mut state = AppState::default();
        let _ = MainToolbar::view(&state);
        state.library.library_path = std::path::PathBuf::from("/tmp/books");
        let _ = MainToolbar::view(&state);
    }
}
