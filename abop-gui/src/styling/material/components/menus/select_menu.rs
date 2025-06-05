//! Material Design 3 Select Menu Component
//!
//! This module provides the select menu component for dropdown selection.

use iced::{
    Alignment, Background, Border, Element, Length, Padding, Theme,
    widget::{Column, Row, button, container, scrollable, text},
};

use crate::styling::material::MaterialTokens;
use crate::styling::material::components::menu_constants::{
    MenuStyleValues, dimensions, padding, spacing, typography,
};
use crate::styling::material::components::menu_container_style::{
    MenuContainerVariant, create_menu_container_style,
};
use crate::styling::material::components::menu_item_style::{
    MenuItemVariant, create_icon_text_menu_item, create_text_menu_item,
};
use crate::styling::material::components::menus::base_menu::{
    HasMaxHeight, HasMinWidth, MenuComponent, Openable,
};
use crate::styling::material::components::menus::menu_item::MenuItem;

/// Material Design 3 Select Menu component
///
/// Select menus display a list of options from which users can make a single selection.
/// They provide a compact way to present multiple choices while only showing the selected
/// option when closed. This implementation follows Material Design 3 specifications
/// for select menu behavior, styling, and interaction patterns.
///
/// # Examples
/// ```rust
/// use abop_gui::styling::material::components::menus::{MaterialSelectMenu, SelectOption};
///
/// // Create a basic select menu
/// let select = MaterialSelectMenu::new()
///     .option(SelectOption::new("Option 1", 1))
///     .option(SelectOption::new("Option 2", 2))
///     .option(SelectOption::new("Option 3", 3))
///     .placeholder("Choose an option")
///     .label("Select Menu");
///
/// // Create a select menu with icons
/// let select = MaterialSelectMenu::new()
///     .option(SelectOption::new("Home", "home").icon("home"))
///     .option(SelectOption::new("Settings", "settings").icon("settings"))
///     .option(SelectOption::new("Profile", "profile").icon("person"))
///     .selected(0);
/// ```
#[derive(Debug, Clone)]
pub struct MaterialSelectMenu<T> {
    /// The available options that can be selected from
    ///
    /// Each option contains a display text, value, optional icon, and enabled state.
    pub options: Vec<SelectOption<T>>,

    /// The index of the currently selected option in the options vector
    ///
    /// `None` indicates no option is currently selected.
    pub selected_index: Option<usize>,

    /// The placeholder text displayed when no option is selected
    ///
    /// This text appears in the select button when `selected_index` is `None`.
    pub placeholder: String,

    /// Whether the dropdown menu is currently open and visible
    ///
    /// When `true`, the options list is displayed below the select button.
    pub is_open: bool,

    /// Whether the select menu is enabled for user interaction
    ///
    /// When `false`, the select menu appears disabled and cannot be interacted with.
    pub enabled: bool,

    /// Optional label text displayed above the select menu
    ///
    /// Provides context or instructions for the selection.
    pub label: Option<String>,

    /// Minimum width of the select menu
    pub min_width: Option<f32>,

    /// Maximum height of the dropdown before scrolling
    pub max_height: Option<f32>,
}

/// A select option with value and display text
///
/// Represents a single selectable option within a select menu or autocomplete component.
/// Each option contains the data needed for display and interaction, including text,
/// value, optional icon, and enabled state.
#[derive(Debug, Clone)]
pub struct SelectOption<T> {
    /// The display text shown to the user
    ///
    /// This is the human-readable text that appears in menus and lists.
    pub text: String,

    /// The underlying value associated with this option
    ///
    /// This is the data that gets returned when the option is selected.
    pub value: T,

    /// Optional icon identifier displayed alongside the text
    ///
    /// Typically a Material Design icon name or Unicode symbol.
    pub icon: Option<String>,

    /// Whether this option can be selected by the user
    ///
    /// When `false`, the option appears disabled and cannot be selected.
    pub enabled: bool,
}

impl<T> SelectOption<T> {
    /// Create a new select option
    #[must_use]
    pub fn new<S: Into<String>>(text: S, value: T) -> Self {
        Self {
            text: text.into(),
            value,
            icon: None,
            enabled: true,
        }
    }

    /// Set the option icon
    #[must_use]
    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl<T> MenuComponent for SelectOption<T> {
    fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<T> Default for MaterialSelectMenu<T> {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            selected_index: None,
            placeholder: "Select an option".to_string(),
            is_open: false,
            enabled: true,
            label: None,
            min_width: Some(dimensions::DEFAULT_SELECT_WIDTH),
            max_height: Some(dimensions::SELECT_MENU_SCROLL_HEIGHT),
        }
    }
}

impl<T> MaterialSelectMenu<T>
where
    T: Clone,
{
    /// Create a new select menu
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an option to the select menu
    #[must_use]
    pub fn option(mut self, option: SelectOption<T>) -> Self {
        self.options.push(option);
        self
    }

    /// Set the selected option index
    #[must_use]
    pub const fn selected(mut self, index: usize) -> Self {
        self.selected_index = Some(index);
        self
    }

    /// Set the placeholder text
    #[must_use]
    pub fn placeholder<S: Into<String>>(mut self, placeholder: S) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the label
    #[must_use]
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get the currently selected value
    #[must_use]
    pub fn selected_value(&self) -> Option<&T> {
        self.selected_index
            .and_then(|index| self.options.get(index))
            .map(|option| &option.value)
    }

    /// Create the select menu element
    pub fn view<'a, Message>(
        &'a self,
        on_toggle: Message,
        on_select: impl Fn(usize) -> Message + 'a,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message>
    where
        Message: 'a + Clone,
    {
        let style_values = MenuStyleValues::from_tokens(tokens);
        let mut column = Column::new().spacing(spacing::LABEL_INPUT);

        // Add label if present
        if let Some(label) = &self.label {
            column = column.push(text(label).size(typography::LABEL_TEXT_SIZE).style(
                move |_theme: &Theme| text::Style {
                    color: Some(style_values.on_surface_variant),
                },
            ));
        }

        // Create the select button
        let display_text = self.get_display_text();
        let select_button = Self::create_select_button(display_text, on_toggle, &style_values);

        column = column.push(select_button);

        // Add dropdown menu if open
        if self.is_open {
            let menu_items: Vec<MenuItem> = self
                .options
                .iter()
                .map(|option| {
                    let mut item = MenuItem::new(&option.text).enabled(option.enabled);
                    if let Some(icon) = &option.icon {
                        item = item.leading_icon(icon);
                    }
                    item
                })
                .collect();

            let mut menu_column = Column::new();

            for (index, item) in menu_items.iter().enumerate() {
                // Determine the appropriate style variant
                let style_variant = if !item.enabled {
                    MenuItemVariant::Disabled
                } else if item.selected || self.selected_index == Some(index) {
                    MenuItemVariant::Selected
                } else {
                    MenuItemVariant::Normal
                };

                // Create the menu item using the centralized helper function
                let menu_item = if item.leading_icon.is_some() {
                    create_icon_text_menu_item(
                        item.leading_icon.as_deref(),
                        &item.label,
                        None, // No trailing icon for select menu items
                        style_variant,
                        if item.enabled {
                            Some(on_select(index))
                        } else {
                            None
                        },
                        tokens,
                    )
                } else {
                    create_text_menu_item(
                        &item.label,
                        style_variant,
                        if item.enabled {
                            Some(on_select(index))
                        } else {
                            None
                        },
                        tokens,
                    )
                };

                menu_column = menu_column.push(menu_item);
            }

            let menu_container = container(
                scrollable(menu_column)
                    .width(Length::Fill)
                    .height(Length::Fixed(
                        self.max_height
                            .unwrap_or(dimensions::SELECT_MENU_SCROLL_HEIGHT),
                    )),
            )
            .style(create_menu_container_style(
                MenuContainerVariant::SelectMenu,
                tokens,
            ))
            .padding(Padding::from([8, 0]))
            .width(Length::Fixed(
                self.min_width.unwrap_or(dimensions::DEFAULT_SELECT_WIDTH),
            ));

            column = column.push(menu_container);
        }

        column.into()
    }

    /// Get the display text for the select button
    fn get_display_text(&self) -> String {
        self.selected_index
            .and_then(|index| self.options.get(index))
            .map_or_else(|| self.placeholder.clone(), |option| option.text.clone())
    }

    /// Create the select button with consistent styling
    fn create_select_button<'a, Message: Clone + 'a>(
        display_text: String,
        on_toggle: Message,
        style_values: &MenuStyleValues,
    ) -> Element<'a, Message> {
        let mut row = Row::new()
            .spacing(spacing::SELECT_BUTTON_ELEMENTS)
            .align_y(Alignment::Center)
            .push(text(display_text).size(typography::MENU_ITEM_TEXT_SIZE));

        // Add dropdown arrow
        row = row.push(text("▼").size(typography::SMALL_ICON_SIZE));

        // Clone the style values we need for the closure
        let surface_container_highest = style_values.surface_container_highest;
        let on_surface = style_values.on_surface;
        let outline = style_values.outline;
        let corner_radius = style_values.corner_radius;

        button(row)
            .style(move |_theme: &Theme, status: button::Status| {
                let background_color = match status {
                    button::Status::Hovered => surface_container_highest,
                    _ => iced::Color::TRANSPARENT,
                };

                button::Style {
                    background: Some(Background::Color(background_color)),
                    text_color: on_surface,
                    border: Border {
                        color: outline,
                        width: 1.0,
                        radius: corner_radius,
                    },
                    shadow: iced::Shadow::default(),
                }
            })
            .padding(padding::SELECT_BUTTON)
            .width(Length::Fixed(dimensions::DEFAULT_SELECT_WIDTH))
            .on_press(on_toggle)
            .into()
    }
}

impl<T> Openable for MaterialSelectMenu<T> {
    fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    fn is_open(&self) -> bool {
        self.is_open
    }
}

impl<T> HasMinWidth for MaterialSelectMenu<T> {
    fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    fn get_min_width(&self) -> Option<f32> {
        self.min_width
    }
}

impl<T> HasMaxHeight for MaterialSelectMenu<T> {
    fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    fn get_max_height(&self) -> Option<f32> {
        self.max_height
    }
}

impl<T> MenuComponent for MaterialSelectMenu<T> {
    fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}
