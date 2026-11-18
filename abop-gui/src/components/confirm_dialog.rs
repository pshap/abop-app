//! Generic confirmation dialog

use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Vector};

use crate::components::ui::{self, ButtonKind};
use crate::messages::Message;
use crate::styling::material::MaterialTokens;

/// Content of a confirmation dialog
pub struct ConfirmDialog<'a> {
    /// Dialog title (a question, e.g. "Convert 2 audiobooks to mono?")
    pub title: String,
    /// Explanatory paragraphs shown under the title
    pub body: Vec<String>,
    /// Label of the confirming button
    pub confirm_label: &'a str,
    /// Message sent when confirming
    pub on_confirm: Message,
    /// Message sent when cancelling
    pub on_cancel: Message,
}

impl<'a> ConfirmDialog<'a> {
    /// Render the dialog card (place it in a modal overlay)
    pub fn view(self, tokens: &'a MaterialTokens) -> Element<'a, Message> {
        let mut content = column![text(self.title).size(tokens.typography().title_large.size)]
            .spacing(12)
            .padding(24);
        for paragraph in self.body {
            content = content.push(
                text(paragraph)
                    .size(tokens.typography().body_medium.size)
                    .color(tokens.colors.on_surface_variant),
            );
        }
        content = content.push(Space::with_height(4)).push(
            row![
                Space::with_width(Length::Fill),
                ui::action_button(
                    tokens,
                    ButtonKind::Ghost,
                    None,
                    "Cancel",
                    Some(self.on_cancel)
                ),
                ui::action_button(
                    tokens,
                    ButtonKind::Primary,
                    None,
                    self.confirm_label,
                    Some(self.on_confirm),
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        );

        container(content)
            .width(Length::Fixed(460.0))
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
}
