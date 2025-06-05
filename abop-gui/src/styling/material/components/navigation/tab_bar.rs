//! Material Design 3 Tab Bar Component
//!
//! Tab bars organize content across different screens, data sets, and other interactions.
//! They allow users to quickly switch between different views - perfect for audiobook
//! organizing apps to switch between Library, Currently Playing, Settings, etc.

use iced::{
    Alignment, Background, Border, Element, Length, Padding, Theme,
    widget::{Row, Text, button, container, text},
};

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::MaterialTokens;

/// Type alias for complex button style function
type ButtonStyleFn<'a> = Box<dyn Fn(&Theme, button::Status) -> button::Style + 'a>;

/// Material Design 3 Tab Bar component
///
/// Tab bars organize content across different screens, data sets, and other interactions.
/// They allow users to quickly switch between different views.
#[derive(Debug, Clone, Default)]
pub struct MaterialTabBar {
    /// The tabs in the tab bar
    pub tabs: Vec<Tab>,
    /// The currently selected tab index
    pub selected_index: Option<usize>,
    /// Whether the tabs are scrollable
    pub scrollable: bool,
}

/// A tab with label and optional icon
#[derive(Debug, Clone, Default)]
pub struct Tab {
    /// The tab label
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Whether this tab is enabled
    pub enabled: bool,
    /// Optional badge count
    pub badge: Option<u32>,
}

impl Tab {
    /// Create a new tab
    #[must_use]
    pub fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            icon: None,
            enabled: true,
            badge: None,
        }
    }

    /// Set the tab icon
    #[must_use]
    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set whether this tab is enabled
    #[must_use]
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Add a badge with count
    #[must_use]
    pub const fn badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }
}

impl MaterialTabBar {
    /// Create a new tab bar
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a tab to the tab bar
    #[must_use]
    pub fn tab(mut self, tab: Tab) -> Self {
        self.tabs.push(tab);
        self
    }

    /// Set the selected tab index
    #[must_use]
    pub const fn selected(mut self, index: usize) -> Self {
        self.selected_index = Some(index);
        self
    }

    /// Set whether the tabs are scrollable
    #[must_use]
    pub const fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    /// Create the tab bar element
    pub fn view<'a, Message>(
        &'a self,
        on_select: impl Fn(usize) -> Message + 'a,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message>
    where
        Message: 'a + Clone,
    {
        let colors = &tokens.colors;

        let tabs: Vec<Element<'a, Message>> =
            self.tabs
                .iter()
                .enumerate()
                .map(|(index, tab)| {
                    let is_selected = self.selected_index == Some(index);
                    let mut content = Row::<Message>::new().spacing(8).align_y(Alignment::Center);

                    // Add icon if present
                    if let Some(icon) = &tab.icon {
                        let icon_color = if is_selected {
                            colors.primary.base
                        } else if tab.enabled {
                            colors.on_surface_variant
                        } else {
                            ColorUtils::with_alpha(colors.on_surface, 0.38)
                        };
                        content =
                            content.push(Text::new(icon).size(18).style(move |_theme: &Theme| {
                                text::Style {
                                    color: Some(icon_color),
                                }
                            }));
                    } // Add label
                    let label_color = if is_selected {
                        colors.primary.base
                    } else if tab.enabled {
                        colors.on_surface_variant
                    } else {
                        ColorUtils::with_alpha(colors.on_surface, 0.38)
                    };
                    content = content.push(Text::new(&tab.label).size(14).style(
                        move |_theme: &Theme| text::Style {
                            color: Some(label_color),
                        },
                    ));

                    let primary = colors.primary.base;
                    let on_surface_variant = colors.on_surface_variant;
                    let button_style: ButtonStyleFn<'a> = Box::new(
                        move |_theme: &Theme, _status: button::Status| button::Style {
                            background: Some(Background::Color(iced::Color::TRANSPARENT)),
                            text_color: if is_selected {
                                primary
                            } else {
                                on_surface_variant
                            },
                            border: Border {
                                radius: 0.0.into(),
                                width: if is_selected { 3.0 } else { 0.0 },
                                color: if is_selected {
                                    primary
                                } else {
                                    iced::Color::TRANSPARENT
                                },
                            },
                            ..Default::default()
                        },
                    );
                    (button(content)
                        .style(button_style)
                        .padding(Padding::from([12, 16]))
                        .on_press_maybe(if tab.enabled {
                            Some(on_select(index))
                        } else {
                            None
                        }))
                    .into()
                })
                .collect();

        let tab_row = Row::with_children(tabs)
            .spacing(0)
            .align_y(Alignment::Center);

        container(tab_row)
            .style({
                let surface = colors.surface;
                let outline_variant = colors.outline_variant;

                move |_theme: &Theme| container::Style {
                    background: Some(Background::Color(surface)),
                    border: Border {
                        width: 1.0,
                        color: outline_variant,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .padding(Padding::from([0, 8]))
            .width(Length::Fill)
            .into()
    }
}
