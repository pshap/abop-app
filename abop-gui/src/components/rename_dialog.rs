//! Rename audiobook dialog component
//!
//! Provides a modal dialog for renaming an audiobook file and its database record.

use iced::widget::{column, container, row, text, text_input};
use iced::{Alignment, Border, Element, Length};

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
        .size(material_tokens.typography().headline_small.size)
        .width(Length::Fill);

    // Input field for the new name
    let input = text_input("New filename", current_input)
        .on_input(Message::RenameInputChanged)
        .on_submit(Message::CommitRename(
            audiobook_id.clone(),
            current_input.to_string(),
        ))
        .padding(material_tokens.spacing().sm)
        .size(material_tokens.typography().body_large.size);

    // Action buttons
    let cancel_button = iced::widget::button(
        text("Cancel")
            .size(material_tokens.typography().label_large.size)
            .align_x(Alignment::Center),
    )
    .on_press(Message::CancelRename)
    .padding(material_tokens.spacing().sm);

    let rename_button = iced::widget::button(
        text("Rename")
            .size(material_tokens.typography().label_large.size)
            .align_x(Alignment::Center),
    )
    .on_press(Message::CommitRename(
        audiobook_id,
        current_input.to_string(),
    ))
    .padding(material_tokens.spacing().sm);

    let actions = row![cancel_button, rename_button]
        .spacing(material_tokens.spacing().md)
        .align_y(Alignment::Center);

    // Assemble the dialog content
    let content = column![
        header,
        text("Enter a new name for the audiobook file:")
            .size(material_tokens.typography().body_medium.size),
        input,
        actions
    ]
    .spacing(material_tokens.spacing().md)
    .padding(material_tokens.spacing().lg);

    // Wrap in a container for the modal look
    container(content)
        .width(Length::Fixed(400.0))
        .style(move |_theme| container::Style {
            background: Some(material_tokens.colors().surface_container_high.into()),
            border: Border::radius(12.0),
            ..container::Style::default()
        })
        .into()
}
