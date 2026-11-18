//! Rename audiobook dialog component
//!
//! Provides a modal dialog for renaming an audiobook file and its database record.

use iced::widget::{Space, column, container, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding};

use crate::components::ui::{self, ButtonKind};

use crate::messages::Message;
use crate::styling::material::MaterialTokens;

/// Renders the rename audiobook dialog
///
/// # Arguments
/// * `audiobook_id` - The ID of the audiobook being renamed
/// * `current_input` - The current value of the name input field
/// * `material_tokens` - Material Design 3 tokens for consistent styling
#[must_use]
pub fn view<'a>(
    audiobook_id: String,
    current_input: &str,
    material_tokens: &'a MaterialTokens,
) -> Element<'a, Message> {
    // Dialog header
    let header = text("Rename Audiobook")
        .size(material_tokens.typography().title_large.size)
        .width(Length::Fill);

    let trimmed = current_input.trim();
    let commit = (!trimmed.is_empty())
        .then(|| Message::CommitRename(audiobook_id.clone(), trimmed.to_string()));

    // Input field for the new name
    let muted = material_tokens.colors.on_surface_variant;
    let input = text_input("New filename", current_input)
        .on_input(Message::RenameInputChanged)
        .on_submit_maybe(commit.clone())
        .padding(Padding::from([10.0, 12.0]))
        .size(material_tokens.typography().body_large.size)
        .style(move |_theme, status| {
            let focused = matches!(status, text_input::Status::Focused);
            text_input::Style {
                background: Background::Color(material_tokens.colors.surface_container_highest),
                border: Border {
                    color: if focused {
                        material_tokens.colors.primary.base
                    } else {
                        Color::TRANSPARENT
                    },
                    width: 2.0,
                    radius: 8.0.into(),
                },
                icon: muted,
                placeholder: muted,
                value: material_tokens.colors.on_surface,
                selection: Color {
                    a: 0.35,
                    ..material_tokens.colors.primary.base
                },
            }
        });

    let actions = row![
        Space::with_width(Length::Fill),
        ui::action_button(
            material_tokens,
            ButtonKind::Ghost,
            None,
            "Cancel",
            Some(Message::CancelRename),
        ),
        ui::action_button(material_tokens, ButtonKind::Primary, None, "Rename", commit),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    // Assemble the dialog content
    let content = column![
        header,
        text("Enter a new name for the audiobook file.")
            .size(material_tokens.typography().body_medium.size)
            .color(muted),
        input,
        actions
    ]
    .spacing(16)
    .padding(24);

    // Wrap in a container for the modal look
    container(content)
        .width(Length::Fixed(440.0))
        .style(move |_theme| container::Style {
            background: Some(material_tokens.colors().surface_container_high.into()),
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 12.0.into(),
            },
            ..container::Style::default()
        })
        .into()
}
