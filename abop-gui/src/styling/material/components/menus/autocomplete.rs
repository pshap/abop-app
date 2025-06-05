//! Material Design 3 Autocomplete Component
//!
//! This module provides the autocomplete component for text input with suggestions.

use iced::{
    Alignment, Background, Border, Element, Length, Theme,
    widget::{Column, Row, button, container, text, text_input},
};

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::MaterialTokens;
use crate::styling::material::components::menu_constants::{
    MenuStyleValues, dimensions, limits, padding, spacing, typography,
};
use crate::styling::material::components::menu_container_style::{
    MenuContainerVariant, create_menu_container_style,
};
use crate::styling::material::components::menus::base_menu::{
    ButtonStyleFn, HasMaxHeight, HasMinWidth, MenuComponent,
};
use crate::styling::material::components::menus::select_menu::SelectOption;

/// Material Design 3 Autocomplete component
///
/// Autocomplete provides suggested completions as users type, helping them
/// quickly find and select from a large set of options. This component combines
/// a text input with a filtered dropdown list that updates in real-time based
/// on the user's input, following Material Design 3 specifications.
///
/// # Examples
/// ```rust
/// use abop_gui::styling::material::components::menus::{MaterialAutocomplete, SelectOption};
///
/// // Create a basic autocomplete
/// let autocomplete = MaterialAutocomplete::new()
///     .option(SelectOption::new("Apple", "apple"))
///     .option(SelectOption::new("Banana", "banana"))
///     .option(SelectOption::new("Cherry", "cherry"))
///     .placeholder("Search fruits...")
///     .label("Fruit Selection")
///     .max_suggestions(3);
///
/// // Create an autocomplete with icons
/// let autocomplete = MaterialAutocomplete::new()
///     .option(SelectOption::new("New York", "ny").icon("location_city"))
///     .option(SelectOption::new("Los Angeles", "la").icon("location_city"))
///     .option(SelectOption::new("Chicago", "chi").icon("location_city"))
///     .input_value("New")
///     .show_suggestions(true);
/// ```
#[derive(Debug, Clone)]
pub struct MaterialAutocomplete<T> {
    /// The complete list of available options to search through
    ///
    /// All options that can potentially be suggested to the user.
    pub options: Vec<SelectOption<T>>,

    /// The current text input value entered by the user
    ///
    /// This value is used to filter the available options and display suggestions.
    pub input_value: String,

    /// Indices of options that match the current input filter
    ///
    /// Contains indices into the `options` vector for options that match
    /// the current `input_value`, limited by `max_suggestions`.
    pub filtered_options: Vec<usize>,

    /// The index of the currently highlighted option in the filtered list
    ///
    /// `None` indicates no option is currently highlighted. This is an index
    /// into `filtered_options`, not the main `options` vector.
    pub selected_index: Option<usize>,

    /// The placeholder text displayed when the input is empty
    ///
    /// Provides guidance to the user about what they can search for.
    pub placeholder: String,

    /// Whether the suggestions dropdown is currently visible
    ///
    /// When `true`, filtered suggestions are displayed below the input field.
    pub show_suggestions: bool,

    /// Whether the autocomplete input is enabled for user interaction
    ///
    /// When `false`, the input appears disabled and cannot be typed in.
    pub enabled: bool,

    /// Optional label text displayed above the autocomplete input
    ///
    /// Provides context or instructions for the autocomplete field.
    pub label: Option<String>,

    /// Maximum number of suggestions to display in the dropdown
    ///
    /// Limits the number of filtered options shown to prevent overwhelming the user.
    pub max_suggestions: usize,

    /// Minimum width of the autocomplete component
    pub min_width: Option<f32>,

    /// Maximum height of the suggestions dropdown
    pub max_height: Option<f32>,
}

impl<T> Default for MaterialAutocomplete<T> {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            input_value: String::new(),
            filtered_options: Vec::new(),
            selected_index: None,
            placeholder: "Type to search...".to_string(),
            show_suggestions: false,
            enabled: true,
            label: None,
            max_suggestions: limits::DEFAULT_MAX_SUGGESTIONS,
            min_width: Some(dimensions::DEFAULT_SELECT_WIDTH),
            max_height: Some(dimensions::SELECT_MENU_SCROLL_HEIGHT),
        }
    }
}

impl<T> MaterialAutocomplete<T>
where
    T: Clone,
{
    /// Create a new autocomplete
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an option to the autocomplete
    #[must_use]
    pub fn option(mut self, option: SelectOption<T>) -> Self {
        self.options.push(option);
        self.update_filtered_options();
        self
    }

    /// Set the input value
    #[must_use]
    pub fn input_value<S: Into<String>>(mut self, value: S) -> Self {
        self.input_value = value.into();
        self.update_filtered_options();
        self
    }

    /// Set the placeholder text
    #[must_use]
    pub fn placeholder<S: Into<String>>(mut self, placeholder: S) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set whether suggestions are shown
    #[must_use]
    pub const fn show_suggestions(mut self, show: bool) -> Self {
        self.show_suggestions = show;
        self
    }

    /// Set the label
    #[must_use]
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the maximum number of suggestions
    #[must_use]
    pub fn max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max;
        self.update_filtered_options();
        self
    }

    /// Update the filtered options based on current input
    fn update_filtered_options(&mut self) {
        let query = self.input_value.to_lowercase();
        self.filtered_options = self
            .options
            .iter()
            .enumerate()
            .filter(|(_, option)| option.enabled && option.text.to_lowercase().contains(&query))
            .map(|(index, _)| index)
            .take(self.max_suggestions)
            .collect();

        // Reset selection if it's out of bounds
        if let Some(selected) = self.selected_index
            && selected >= self.filtered_options.len()
        {
            self.selected_index = None;
        }
    }

    /// Get the currently selected value
    #[must_use]
    pub fn selected_value(&self) -> Option<&T> {
        self.selected_index
            .and_then(|filtered_index| self.filtered_options.get(filtered_index))
            .and_then(|&option_index| self.options.get(option_index))
            .map(|option| &option.value)
    }

    /// Create the autocomplete element
    pub fn view<'a, Message>(
        &'a self,
        on_input: impl Fn(String) -> Message + 'a,
        on_select: impl Fn(usize) -> Message + 'a,
        _on_focus: Message,
        _on_blur: Message,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message>
    where
        Message: 'a + Clone,
    {
        let style_values = MenuStyleValues::from_tokens(tokens);
        let mut column = Column::new().spacing(spacing::LABEL_INPUT);

        // Add label if present
        if let Some(label) = &self.label {
            let on_surface_variant = style_values.on_surface_variant;
            column = column.push(text(label).size(typography::LABEL_TEXT_SIZE).style(
                move |_theme: &Theme| text::Style {
                    color: Some(on_surface_variant),
                },
            ));
        }

        // Create the text input
        let surface_container_highest = style_values.surface_container_highest;
        let corner_radius = style_values.corner_radius;
        let primary = style_values.primary;
        let on_surface = style_values.on_surface;
        let on_surface_variant = style_values.on_surface_variant;

        let input = text_input(&self.placeholder, &self.input_value)
            .on_input(on_input)
            .padding(padding::SELECT_BUTTON)
            .size(typography::MENU_ITEM_TEXT_SIZE)
            .width(Length::Fill)
            .style(move |_theme: &Theme, status| text_input::Style {
                background: Background::Color(surface_container_highest),
                border: Border {
                    radius: corner_radius,
                    width: 1.0,
                    color: match status {
                        text_input::Status::Active | text_input::Status::Focused => primary,
                        text_input::Status::Hovered => on_surface,
                        text_input::Status::Disabled => ColorUtils::with_alpha(on_surface, 0.12),
                    },
                },
                icon: iced::Color::TRANSPARENT,
                placeholder: on_surface_variant,
                value: on_surface,
                selection: primary,
            });

        column = column.push(input);

        // Add suggestions dropdown if enabled and there are filtered options
        if self.show_suggestions && !self.filtered_options.is_empty() {
            column = column.push(self.create_suggestions_dropdown(on_select, tokens));
        }

        column.into()
    }
    /// Create the suggestions dropdown
    fn create_suggestions_dropdown<'a, Message: Clone + 'a>(
        &'a self,
        on_select: impl Fn(usize) -> Message + 'a,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let _style_values = MenuStyleValues::from_tokens(tokens);
        let mut suggestions_column = Column::new();

        for (filtered_index, &option_index) in self.filtered_options.iter().enumerate() {
            if let Some(option) = self.options.get(option_index) {
                let is_selected = self.selected_index == Some(filtered_index);
                suggestions_column = suggestions_column.push(self.create_suggestion_item(
                    option,
                    is_selected,
                    on_select(option_index),
                    tokens,
                ));
            }
        }

        let suggestions_container = container(suggestions_column)
            .style(create_menu_container_style(
                MenuContainerVariant::AutocompleteSuggestions,
                tokens,
            ))
            .padding(padding::AUTOCOMPLETE_SUGGESTIONS)
            .width(Length::Fill);

        suggestions_container.into()
    }

    /// Create a single suggestion item
    fn create_suggestion_item<'a, Message: Clone + 'a>(
        &self,
        option: &'a SelectOption<T>,
        is_selected: bool,
        on_press: Message,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message> {
        let style_values = MenuStyleValues::from_tokens(tokens);
        let mut row = Row::new().spacing(12).align_y(Alignment::Center);

        // Add icon if present
        if let Some(icon) = &option.icon {
            let on_surface = style_values.on_surface;
            row = row.push(text(icon).size(typography::MENU_ICON_SIZE).style(
                move |_theme: &Theme| text::Style {
                    color: Some(on_surface),
                },
            ));
        }

        // Add text
        let on_surface = style_values.on_surface;
        row = row.push(
            text(&option.text)
                .size(typography::MENU_ITEM_TEXT_SIZE)
                .style(move |_theme: &Theme| text::Style {
                    color: Some(on_surface),
                }),
        );

        let button_style: ButtonStyleFn<'a> = if is_selected {
            let secondary_container = style_values.secondary_container;
            let on_secondary_container = style_values.on_secondary_container;
            let corner_radius = style_values.corner_radius;
            Box::new(
                move |_theme: &Theme, _status: button::Status| button::Style {
                    background: Some(Background::Color(secondary_container)),
                    text_color: on_secondary_container,
                    border: Border {
                        radius: corner_radius,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
        } else {
            let on_surface = style_values.on_surface;
            let corner_radius = style_values.corner_radius;
            Box::new(
                move |_theme: &Theme, _status: button::Status| button::Style {
                    background: Some(Background::Color(iced::Color::TRANSPARENT)),
                    text_color: on_surface,
                    border: Border {
                        radius: corner_radius,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
        };

        button(row)
            .style(button_style)
            .padding(padding::MENU_ITEM)
            .width(Length::Fill)
            .on_press(on_press)
            .into()
    }
}

impl<T> HasMinWidth for MaterialAutocomplete<T> {
    fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    fn get_min_width(&self) -> Option<f32> {
        self.min_width
    }
}

impl<T> HasMaxHeight for MaterialAutocomplete<T> {
    fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    fn get_max_height(&self) -> Option<f32> {
        self.max_height
    }
}

impl<T> MenuComponent for MaterialAutocomplete<T> {
    fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}
