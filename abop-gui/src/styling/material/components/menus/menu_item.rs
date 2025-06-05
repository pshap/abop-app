//! Menu item components and types
//!
//! This module provides the `MenuItem` struct and related types for all menu components.

use crate::styling::material::MaterialTokens;
use crate::styling::material::components::menu_item_style::{
    MenuItemVariant, create_icon_text_menu_item, create_text_menu_item,
};
use crate::styling::material::components::menus::base_menu::MenuComponent;

use crate::styling::material::components::menu_constants::{padding, typography};
use crate::styling::material::components::menu_container_style::{
    MenuContainerVariant, create_menu_container_style,
};
use iced::{
    Element, Length, Theme,
    widget::{Space, container, text},
};

/// A menu item with label and optional icon
///
/// Menu items represent individual selectable options within a menu. They can include
/// leading and trailing icons, be enabled or disabled, and support different types
/// including normal items, dividers, and section headers.
///
/// # Examples
/// ```rust
/// use abop_gui::styling::material::components::menus::{MenuItem, MenuItemType};
///
/// // Create a simple menu item
/// let item = MenuItem::new("Save File");
///
/// // Create a menu item with icons
/// let item = MenuItem::new("Copy")
///     .leading_icon("content_copy")
///     .trailing_icon("keyboard_shortcut");
///
/// // Create a disabled menu item
/// let mut item = MenuItem::new("Unavailable Option");
/// item.enabled = false;
///
/// // Create a selected menu item
/// let item = MenuItem::new("Current Selection")
///     .selected(true);
///
/// // Create menu structure elements
/// let header = MenuItem::header("File Operations");
/// let divider = MenuItem::divider();
/// ```
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// The item label
    pub label: String,
    /// Optional leading icon
    pub leading_icon: Option<String>,
    /// Optional trailing icon
    pub trailing_icon: Option<String>,
    /// Whether this item is enabled
    pub enabled: bool,
    /// Whether this item is selected
    pub selected: bool,
    /// Item type (normal, divider, header)
    pub item_type: MenuItemType,
}

/// Types of menu items
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemType {
    /// Regular menu item
    Normal,
    /// Divider between menu sections
    Divider,
    /// Section header (non-clickable)
    Header,
}

impl MenuItem {
    /// Create a new menu item
    #[must_use]
    pub fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            leading_icon: None,
            trailing_icon: None,
            enabled: true,
            selected: false,
            item_type: MenuItemType::Normal,
        }
    }

    /// Create a divider item
    #[must_use]
    pub const fn divider() -> Self {
        Self {
            label: String::new(),
            leading_icon: None,
            trailing_icon: None,
            enabled: false,
            selected: false,
            item_type: MenuItemType::Divider,
        }
    }

    /// Create a header item
    #[must_use]
    pub fn header<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            leading_icon: None,
            trailing_icon: None,
            enabled: false,
            selected: false,
            item_type: MenuItemType::Header,
        }
    }

    /// Set the leading icon
    #[must_use]
    pub fn leading_icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set the trailing icon
    #[must_use]
    pub fn trailing_icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set whether this item is selected
    #[must_use]
    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Render the menu item as an Element
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        index: usize,
        on_select: impl Fn(usize) -> Message + 'a,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        match self.item_type {
            MenuItemType::Divider => container(Space::with_height(Length::Fixed(1.0)))
                .style(create_menu_container_style(
                    MenuContainerVariant::Divider,
                    tokens,
                ))
                .width(Length::Fill)
                .height(Length::Fixed(1.0))
                .into(),
            MenuItemType::Header => container(
                text(&self.label)
                    .size(typography::MENU_HEADER_TEXT_SIZE)
                    .style({
                        let on_surface_variant = tokens.colors.on_surface_variant;
                        move |_theme: &Theme| text::Style {
                            color: Some(on_surface_variant),
                        }
                    }),
            )
            .padding(padding::MENU_HEADER)
            .width(Length::Fill)
            .into(),
            MenuItemType::Normal => {
                // Determine the appropriate style variant
                let style_variant = if !self.enabled {
                    MenuItemVariant::Disabled
                } else if self.selected {
                    MenuItemVariant::Selected
                } else {
                    MenuItemVariant::Normal
                };

                // Create the menu item using the centralized helper function
                if self.leading_icon.is_some() || self.trailing_icon.is_some() {
                    create_icon_text_menu_item(
                        self.leading_icon.as_deref(),
                        &self.label,
                        self.trailing_icon.as_deref(),
                        style_variant,
                        if self.enabled {
                            Some(on_select(index))
                        } else {
                            None
                        },
                        tokens,
                    )
                } else {
                    create_text_menu_item(
                        &self.label,
                        style_variant,
                        if self.enabled {
                            Some(on_select(index))
                        } else {
                            None
                        },
                        tokens,
                    )
                }
            }
        }
    }
}

impl MenuComponent for MenuItem {
    fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}
