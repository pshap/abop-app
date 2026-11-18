//! Small, consistently styled building blocks for the application shell
//!
//! These helpers style plain Iced widgets directly from [`MaterialTokens`] so the
//! app bar, library view and status bar share one visual language: neutral
//! surfaces, a single accent color, pill-shaped buttons and subtle hover states.

use iced::widget::{button, container, row, text, tooltip};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Shadow};
use iced_font_awesome::fa_icon_solid;

use crate::styling::material::MaterialTokens;

/// Corner radius for pill-shaped buttons and fields
pub const PILL_RADIUS: f32 = 20.0;
/// Corner radius for cards and panels
pub const CARD_RADIUS: f32 = 12.0;
/// Standard height for buttons in the app bar and toolbars
pub const BUTTON_HEIGHT: f32 = 36.0;

/// Visual weight of a button
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    /// Accent-colored, for the single most important action
    Primary,
    /// Neutral filled, for secondary actions
    Tonal,
    /// No background until hovered, for low-emphasis actions
    Ghost,
    /// Like [`ButtonKind::Ghost`], but hovers in the error color (e.g. Exit)
    Danger,
}

/// Blend `overlay` on top of `base` with the given opacity
#[must_use]
pub fn blend(base: Color, overlay: Color, alpha: f32) -> Color {
    Color {
        r: base.r + (overlay.r - base.r) * alpha,
        g: base.g + (overlay.g - base.g) * alpha,
        b: base.b + (overlay.b - base.b) * alpha,
        a: base.a.max(alpha),
    }
}

/// A Font Awesome solid icon in the given color
pub fn icon<'a, M: 'a>(name: &str, size: impl Into<iced::Pixels>, color: Color) -> Element<'a, M> {
    fa_icon_solid(name).size(size.into().0).color(color).into()
}

/// Button style for the given kind and interaction status
#[must_use]
pub fn button_style(
    tokens: &MaterialTokens,
    kind: ButtonKind,
    status: button::Status,
) -> button::Style {
    let colors = &tokens.colors;
    let (base, fg) = match kind {
        ButtonKind::Primary => (colors.primary.base, colors.primary.on_base),
        ButtonKind::Tonal => (colors.surface_container_highest, colors.on_surface),
        ButtonKind::Ghost | ButtonKind::Danger => (Color::TRANSPARENT, colors.on_surface_variant),
    };
    // Hover tint: the foreground color for filled buttons, error red for Danger
    let tint = if kind == ButtonKind::Danger {
        colors.error.base
    } else {
        fg
    };
    let hover_base = if base == Color::TRANSPARENT {
        Color { a: 0.0, ..tint }
    } else {
        base
    };

    let (background, text_color) = match status {
        button::Status::Active => (base, fg),
        button::Status::Hovered => (
            blend(hover_base, tint, 0.10),
            if kind == ButtonKind::Danger {
                colors.error.base
            } else {
                fg
            },
        ),
        button::Status::Pressed => (
            blend(hover_base, tint, 0.16),
            if kind == ButtonKind::Danger {
                colors.error.base
            } else {
                fg
            },
        ),
        button::Status::Disabled => (
            if base == Color::TRANSPARENT {
                base
            } else {
                Color {
                    a: 0.12,
                    ..colors.on_surface
                }
            },
            Color {
                a: 0.38,
                ..colors.on_surface
            },
        ),
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: PILL_RADIUS.into(),
        },
        shadow: Shadow::default(),
    }
}

/// A button with an optional leading icon and a text label
///
/// Passing `None` for `on_press` renders the button disabled.
pub fn action_button<'a, M: Clone + 'a>(
    tokens: &'a MaterialTokens,
    kind: ButtonKind,
    icon_name: Option<&str>,
    label: &'a str,
    on_press: Option<M>,
) -> Element<'a, M> {
    let enabled = on_press.is_some();
    let fg = button_style(
        tokens,
        kind,
        if enabled {
            button::Status::Active
        } else {
            button::Status::Disabled
        },
    )
    .text_color;

    let mut content = row![].spacing(8).align_y(Alignment::Center);
    if let Some(name) = icon_name {
        content = content.push(icon(name, 14.0, fg));
    }
    content = content.push(text(label).size(tokens.typography().label_large.size));

    button(
        container(content)
            .height(Length::Fill)
            .align_y(Alignment::Center),
    )
    .height(Length::Fixed(BUTTON_HEIGHT))
    .padding(Padding::from([0.0, 16.0]))
    .style(move |_theme, status| button_style(tokens, kind, status))
    .on_press_maybe(on_press)
    .into()
}

/// A round icon-only button with a tooltip
pub fn icon_button<'a, M: Clone + 'a>(
    tokens: &'a MaterialTokens,
    kind: ButtonKind,
    icon_name: &str,
    tooltip_text: &'a str,
    on_press: Option<M>,
) -> Element<'a, M> {
    let enabled = on_press.is_some();
    // Icons inherit no text color, so pick the resting foreground explicitly
    let fg = button_style(
        tokens,
        kind,
        if enabled {
            button::Status::Active
        } else {
            button::Status::Disabled
        },
    )
    .text_color;

    let btn = button(
        container(icon(icon_name, 16.0, fg))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .width(Length::Fixed(BUTTON_HEIGHT))
    .height(Length::Fixed(BUTTON_HEIGHT))
    .padding(0)
    .style(move |_theme, status| button_style(tokens, kind, status))
    .on_press_maybe(on_press);

    with_tooltip(tokens, btn, tooltip_text)
}

/// Wrap any element in a small tooltip shown below it
pub fn with_tooltip<'a, M: 'a>(
    tokens: &'a MaterialTokens,
    content: impl Into<Element<'a, M>>,
    tip: &'a str,
) -> Element<'a, M> {
    tooltip(
        content,
        container(text(tip).size(tokens.typography().body_small.size))
            .padding(Padding::from([4.0, 8.0]))
            .style(move |_theme| container::Style {
                background: Some(Background::Color(tokens.colors.inverse_surface)),
                text_color: Some(tokens.colors.inverse_on_surface),
                border: Border {
                    radius: 4.0.into(),
                    ..Border::default()
                },
                ..container::Style::default()
            }),
        tooltip::Position::Bottom,
    )
    .gap(6)
    .into()
}

/// Background style for a rounded card on the given surface color
pub fn card_style(
    tokens: &MaterialTokens,
    background: Color,
) -> impl Fn(&iced::Theme) -> container::Style + '_ {
    move |_theme| container::Style {
        background: Some(Background::Color(background)),
        text_color: Some(tokens.colors.on_surface),
        border: Border {
            color: tokens.colors.outline_variant,
            width: 1.0,
            radius: CARD_RADIUS.into(),
        },
        shadow: Shadow::default(),
    }
}

/// A thin vertical divider for separating groups in a row
pub fn vertical_divider<'a, M: 'a>(tokens: &'a MaterialTokens, height: f32) -> Element<'a, M> {
    container(iced::widget::Space::new(
        Length::Fixed(1.0),
        Length::Fixed(height),
    ))
    .style(move |_theme| container::Style {
        background: Some(Background::Color(tokens.colors.outline_variant)),
        ..container::Style::default()
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_interpolates_channels() {
        let mixed = blend(Color::BLACK, Color::WHITE, 0.25);
        assert!((mixed.r - 0.25).abs() < f32::EPSILON);
        assert!((mixed.g - 0.25).abs() < f32::EPSILON);
        assert!((mixed.b - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn disabled_buttons_are_dimmed() {
        let tokens = MaterialTokens::default();
        for kind in [ButtonKind::Primary, ButtonKind::Tonal, ButtonKind::Ghost] {
            let style = button_style(&tokens, kind, button::Status::Disabled);
            assert!(style.text_color.a < 0.5, "{kind:?} should look disabled");
        }
    }

    #[test]
    fn danger_buttons_hover_in_error_color() {
        let tokens = MaterialTokens::default();
        let style = button_style(&tokens, ButtonKind::Danger, button::Status::Hovered);
        assert_eq!(style.text_color, tokens.colors.error.base);
    }
}
