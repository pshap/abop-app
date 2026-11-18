//! Library view: search, selection actions and the audiobook table

use iced::widget::{Space, column, container, progress_bar, row, text, text_input};
use iced::{Alignment, Background, Border, Element, Length, Padding};

use crate::components::table_core::AudiobookTable;
use crate::components::ui::{self, ButtonKind};
use crate::messages::{Command, Message};
use crate::state::AppState;
use crate::styling::material::MaterialTokens;

/// Widget id of the search field, used to focus it with Ctrl+F
#[must_use]
pub fn search_input_id() -> text_input::Id {
    text_input::Id::new("library-search")
}

/// Creates the library view
#[must_use]
pub fn library_view(state: &AppState) -> Element<'_, Message> {
    let tokens = &state.ui.material_tokens;
    let library = &state.library;

    let mut content = column![search_row(state)].spacing(12);
    if !library.selected_audiobooks.is_empty() {
        content = content.push(selection_bar(state));
    }

    let body: Element<'_, Message> = if library.is_scanning() && library.audiobooks.is_empty() {
        scanning_state(state)
    } else if library.audiobooks.is_empty() {
        empty_library_state(state)
    } else if library.search_active && library.filtered_audiobooks.is_empty() {
        no_results_state(state)
    } else {
        AudiobookTable::view(
            library.get_display_audiobooks(),
            &library.selected_audiobooks,
            &library.table_state,
            tokens,
            &library.image_cache,
            state.player.current_playing_file.as_deref(),
            &library.progress,
        )
    };

    let card = container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(1) // keep content inside the card's border
        .style(ui::card_style(tokens, tokens.colors.surface_container_low));

    container(content.push(card))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding::from([16.0, 20.0]))
        .into()
}

/// Search field plus a short library summary
fn search_row(state: &AppState) -> Element<'_, Message> {
    let tokens = &state.ui.material_tokens;
    let library = &state.library;
    let muted = tokens.colors.on_surface_variant;

    let input = text_input("Search by title, author or narrator", &library.search_query)
        .id(search_input_id())
        .on_input(Message::SearchQuery)
        .size(tokens.typography().body_large.size)
        .padding(Padding::from([8.0, 4.0]))
        .style(move |_theme, status| {
            let focused = matches!(status, text_input::Status::Focused);
            text_input::Style {
                background: Background::Color(iced::Color::TRANSPARENT),
                border: Border::default(),
                icon: muted,
                placeholder: muted,
                value: tokens.colors.on_surface,
                selection: iced::Color {
                    a: if focused { 0.35 } else { 0.25 },
                    ..tokens.colors.primary.base
                },
            }
        });

    let mut field = row![ui::icon("magnifying-glass", 14.0, muted), input]
        .spacing(8)
        .align_y(Alignment::Center);
    if !library.search_query.is_empty() {
        field = field.push(ui::icon_button(
            tokens,
            ButtonKind::Ghost,
            "xmark",
            "Clear search",
            Some(Message::SearchQuery(String::new())),
        ));
    }

    let search_box = container(field)
        .width(Length::Fill)
        .height(Length::Fixed(44.0))
        .padding(Padding::from([0.0, 16.0]))
        .align_y(Alignment::Center)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(tokens.colors.surface_container_high)),
            border: Border {
                radius: 22.0.into(),
                ..Border::default()
            },
            ..container::Style::default()
        });

    let summary = library_summary(state);
    let mut search_row = row![search_box].spacing(20).align_y(Alignment::Center);
    if !summary.is_empty() {
        search_row = search_row.push(
            text(summary)
                .size(tokens.typography().body_medium.size)
                .color(muted)
                .wrapping(text::Wrapping::None),
        );
    }
    search_row.into()
}

/// "7 audiobooks · 3.1 GB · 52h 10m" (or match counts while searching)
fn library_summary(state: &AppState) -> String {
    let library = &state.library;
    let books = &library.audiobooks;
    if books.is_empty() {
        return String::new();
    }
    if library.search_active {
        return format!(
            "{} of {} audiobooks",
            library.filtered_audiobooks.len(),
            books.len()
        );
    }

    let total_bytes: u64 = books.iter().filter_map(|b| b.size_bytes).sum();
    let total_seconds: u64 = books.iter().filter_map(|b| b.duration_seconds).sum();
    format!(
        "{} · {} · {}",
        plural(books.len(), "audiobook"),
        abop_core::utils::casting::format_file_size_standard(total_bytes),
        format_total_duration(total_seconds)
    )
}

fn plural(count: usize, noun: &str) -> String {
    if count == 1 {
        format!("1 {noun}")
    } else {
        format!("{count} {noun}s")
    }
}

/// Compact duration for summaries, e.g. "52h 10m" or "45m"
fn format_total_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

/// Contextual actions for the current selection
fn selection_bar(state: &AppState) -> Element<'_, Message> {
    let tokens = &state.ui.material_tokens;
    let count = state.library.selected_audiobooks.len();
    let playing = matches!(state.player.player_state, abop_core::PlayerState::Playing);

    let play: Element<'_, Message> = if playing {
        ui::action_button(
            tokens,
            ButtonKind::Tonal,
            Some("stop"),
            "Stop",
            Some(Message::Stop),
        )
    } else {
        ui::action_button(
            tokens,
            ButtonKind::Tonal,
            Some("play"),
            "Play",
            Some(Message::StartPlayback),
        )
    };

    let bar = row![
        text(format!("{count} selected")).size(tokens.typography().title_small.size),
        Space::with_width(Length::Fill),
        ui::with_tooltip(
            tokens,
            play,
            if count == 1 {
                "Play the selected audiobook"
            } else {
                "Play the first selected audiobook"
            }
        ),
        ui::action_button(
            tokens,
            ButtonKind::Primary,
            Some("wand-magic-sparkles"),
            "Process…",
            Some(Message::ProcessSelected),
        ),
        ui::icon_button(
            tokens,
            ButtonKind::Ghost,
            "xmark",
            "Clear selection",
            Some(Message::DeselectAll),
        ),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    container(bar)
        .width(Length::Fill)
        .padding(Padding::from([6.0, 8.0]).left(16.0))
        .style(move |_theme| container::Style {
            background: Some(Background::Color(ui::blend(
                tokens.colors.surface_container_low,
                tokens.colors.primary.base,
                0.12,
            ))),
            text_color: Some(tokens.colors.on_surface),
            border: Border {
                color: ui::blend(
                    tokens.colors.outline_variant,
                    tokens.colors.primary.base,
                    0.3,
                ),
                width: 1.0,
                radius: ui::CARD_RADIUS.into(),
            },
            ..container::Style::default()
        })
        .into()
}

/// Centered icon, headline, explanation and optional actions
fn placeholder<'a>(
    tokens: &'a MaterialTokens,
    icon_name: &str,
    headline: String,
    body: String,
    actions: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let muted = tokens.colors.on_surface_variant;
    let mut content = column![
        ui::icon(icon_name, 44.0, muted),
        Space::with_height(4),
        text(headline).size(tokens.typography().title_large.size),
        text(body)
            .size(tokens.typography().body_medium.size)
            .color(muted)
            .width(Length::Fill)
            .align_x(Alignment::Center),
    ]
    .spacing(8)
    .max_width(460)
    .align_x(Alignment::Center);

    if !actions.is_empty() {
        content = content
            .push(Space::with_height(8))
            .push(row(actions).spacing(8));
    }

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
}

fn empty_library_state(state: &AppState) -> Element<'_, Message> {
    let tokens = &state.ui.material_tokens;
    let library = &state.library;
    let formats = abop_core::SUPPORTED_AUDIO_EXTENSIONS
        .iter()
        .map(|ext| ext.to_uppercase())
        .collect::<Vec<_>>()
        .join(", ");
    let open = ui::action_button(
        tokens,
        ButtonKind::Primary,
        Some("folder-open"),
        "Open Folder",
        Some(Message::command(Command::BrowseDirectory)),
    );

    if !library.library_path.is_dir() || !library.has_scanned() {
        return placeholder(
            tokens,
            "book-open",
            "Your library is empty".into(),
            format!("Open a folder that contains your audiobooks ({formats})."),
            vec![open],
        );
    }

    let name = library
        .library_path
        .file_name()
        .map_or_else(
            || library.library_path.to_string_lossy(),
            |n| n.to_string_lossy(),
        )
        .into_owned();
    let rescan = ui::action_button(
        tokens,
        ButtonKind::Tonal,
        Some("arrows-rotate"),
        "Rescan",
        Some(Message::command(Command::ScanLibrary {
            library_path: library.library_path.clone(),
        })),
    );

    if library.scan_failed() {
        placeholder(
            tokens,
            "triangle-exclamation",
            format!("Couldn't scan “{name}”"),
            "Check that the folder still exists and is readable, then try again.".into(),
            vec![rescan, open],
        )
    } else {
        placeholder(
            tokens,
            "folder-open",
            format!("No audiobooks in “{name}”"),
            format!(
                "Supported formats: {formats}. Subfolders are included when enabled in Settings."
            ),
            vec![open, rescan],
        )
    }
}

fn scanning_state(state: &AppState) -> Element<'_, Message> {
    let tokens = &state.ui.material_tokens;
    let progress = state.library.get_scan_progress_legacy().unwrap_or(0.0);

    let bar = container(
        progress_bar(0.0..=1.0, progress)
            .height(6)
            .style(move |_theme| progress_bar::Style {
                background: Background::Color(tokens.colors.surface_container_highest),
                bar: Background::Color(tokens.colors.primary.base),
                border: Border {
                    radius: 3.0.into(),
                    ..Border::default()
                },
            }),
    )
    .width(Length::Fixed(280.0));

    let content = column![
        ui::icon("magnifying-glass", 36.0, tokens.colors.on_surface_variant),
        text("Scanning library…").size(tokens.typography().title_medium.size),
        bar,
    ]
    .spacing(14)
    .align_x(Alignment::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
}

fn no_results_state(state: &AppState) -> Element<'_, Message> {
    let tokens = &state.ui.material_tokens;
    placeholder(
        tokens,
        "magnifying-glass",
        format!("No matches for “{}”", state.library.search_query.trim()),
        "Try a different title, author or narrator.".into(),
        vec![ui::action_button(
            tokens,
            ButtonKind::Tonal,
            None,
            "Clear Search",
            Some(Message::SearchQuery(String::new())),
        )],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_total_duration_compactly() {
        assert_eq!(format_total_duration(0), "0m");
        assert_eq!(format_total_duration(45 * 60), "45m");
        assert_eq!(format_total_duration(52 * 3600 + 10 * 60 + 59), "52h 10m");
    }

    #[test]
    fn pluralizes_counts() {
        assert_eq!(plural(1, "audiobook"), "1 audiobook");
        assert_eq!(plural(3, "audiobook"), "3 audiobooks");
    }

    #[test]
    fn summary_is_empty_without_books() {
        assert_eq!(library_summary(&AppState::default()), "");
    }
}
