//! Material Design 3 Breadcrumbs Component
//!
//! Breadcrumbs show users their current location in a navigational hierarchy
//! and provide a way to navigate back to any level. Perfect for audiobook apps
//! to navigate through folder structures, collections, and hierarchical content.

use iced::{
    Alignment, Background, Element, Theme,
    widget::{Row, Text, button, text},
};

use crate::styling::material::MaterialTokens;

/// Material Design 3 Breadcrumbs component
///
/// Breadcrumbs show users their current location in a navigational hierarchy
/// and provide a way to navigate back to any level.
#[derive(Debug, Clone)]
pub struct MaterialBreadcrumbs {
    /// The breadcrumb items
    pub items: Vec<BreadcrumbItem>,
    /// The separator character
    pub separator: String,
    /// Maximum number of items to show before collapsing
    pub max_items: Option<usize>,
}

/// A breadcrumb item
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    /// The item label
    pub label: String,
    /// Whether this item is clickable
    pub clickable: bool,
    /// Optional icon
    pub icon: Option<String>,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item
    #[must_use]
    pub fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            clickable: true,
            icon: None,
        }
    }

    /// Set whether this item is clickable
    #[must_use]
    pub const fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Set the item icon
    #[must_use]
    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl Default for MaterialBreadcrumbs {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            separator: "/".to_string(),
            max_items: None,
        }
    }
}

impl MaterialBreadcrumbs {
    /// Create new breadcrumbs
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a breadcrumb item
    #[must_use]
    pub fn item(mut self, item: BreadcrumbItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set the separator
    #[must_use]
    pub fn separator<S: Into<String>>(mut self, separator: S) -> Self {
        self.separator = separator.into();
        self
    }

    /// Set the maximum number of items
    #[must_use]
    pub const fn max_items(mut self, max: usize) -> Self {
        self.max_items = Some(max);
        self
    }

    /// Create the breadcrumbs element
    pub fn view<'a, Message>(
        &self,
        on_select: impl Fn(usize) -> Message + 'a,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message>
    where
        Message: 'a + Clone,
    {
        let on_surface = tokens.colors.on_surface;
        let on_surface_variant = tokens.colors.on_surface_variant;
        let primary = tokens.colors.primary.base;        let items_to_show: Vec<&BreadcrumbItem> = self.max_items.map_or_else(
            || self.items.iter().collect(),
            |max| {
                if self.items.len() > max {
                    let mut result = Vec::new();
                    result.push(&self.items[0]);
                    if self.items.len() > max + 1 {
                        // Ellipsis logic could go here
                    }
                    result.extend(&self.items[self.items.len().saturating_sub(max - 1)..]);
                    result
                } else {
                    self.items.iter().collect()
                }
            }
        );

        let elements: Vec<Element<'a, Message>> = items_to_show
            .iter()
            .enumerate()
            .flat_map(|(index, item)| {
                let is_last = index == items_to_show.len() - 1;
                let mut v = Vec::new();

                // Add icon if present
                if let Some(icon) = &item.icon {
                    let icon_color = on_surface_variant;
                    let icon = icon.clone();
                    v.push(
                        Text::new(icon)
                            .size(16)
                            .style({
                                move |_theme: &Theme| text::Style {
                                    color: Some(icon_color),
                                }
                            })
                            .into(),
                    );
                }

                // Add the item
                if item.clickable && !is_last {
                    let button_color = primary;
                    let label = item.label.clone();
                    v.push(
                        button(Text::new(label))
                            .style({
                                move |_theme: &Theme, _status: button::Status| button::Style {
                                    background: Some(Background::Color(iced::Color::TRANSPARENT)),
                                    text_color: button_color,
                                    ..Default::default()
                                }
                            })
                            .on_press(on_select(index))
                            .into(),
                    );
                } else {
                    let text_color = if is_last {
                        on_surface
                    } else {
                        on_surface_variant
                    };
                    let label = item.label.clone();
                    v.push(
                        Text::new(label)
                            .size(14)
                            .style({
                                move |_theme: &Theme| text::Style {
                                    color: Some(text_color),
                                }
                            })
                            .into(),
                    );
                }

                // Add separator if not the last item
                if !is_last {
                    let separator_color = on_surface_variant;
                    let separator = self.separator.clone();
                    v.push(
                        Text::new(separator)
                            .size(14)
                            .style({
                                move |_theme: &Theme| text::Style {
                                    color: Some(separator_color),
                                }
                            })
                            .into(),
                    );
                }
                v
            })
            .collect();

        Row::<'a, Message>::with_children(elements)
            .spacing(8)
            .align_y(Alignment::Center)
            .into()
    }
}
