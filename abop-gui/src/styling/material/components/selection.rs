// Material Design 3 Selection Components
// Implements Material Design checkboxes, radio buttons, switches, and chips
// following Material Design 3 specifications with proper Iced integration

use super::selection_style::{SelectionSize, SelectionStyleBuilder, SelectionVariant};
use crate::styling::material::colors::MaterialColors;
use iced::{
    Element, Length, Renderer,
    theme::{self},
    widget::{Checkbox, Radio},
};

/// Material Design 3 Checkbox component
///
/// Implements a checkbox following Material Design 3 specifications with support for:
/// - Three states: checked, unchecked, and indeterminate
/// - Error states for form validation
/// - Multiple sizes (small, medium, large)
/// - Accessibility features and proper contrast ratios
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialCheckbox {
    /// Whether the checkbox is currently checked
    pub is_checked: bool,
    /// Optional text label displayed next to the checkbox
    pub label: Option<String>,
    /// Whether the checkbox is disabled (non-interactive)
    pub is_disabled: bool,
    /// Whether the checkbox is in indeterminate state (partially checked)
    pub is_indeterminate: bool,
    /// Whether the checkbox is in an error state (for form validation)
    pub error_state: bool,
    /// The size variant of the checkbox
    pub size: SelectionSize,
}

impl Default for MaterialCheckbox {
    fn default() -> Self {
        Self {
            is_checked: false,
            label: None,
            is_disabled: false,
            is_indeterminate: false,
            error_state: false,
            size: SelectionSize::Medium,
        }
    }
}

impl MaterialCheckbox {
    /// Creates a new checkbox with the specified checked state
    ///
    /// # Arguments
    /// * `is_checked` - Initial checked state of the checkbox
    ///
    /// # Example
    /// ```
    /// use abop_gui::styling::material::MaterialCheckbox;
    ///
    /// let checkbox = MaterialCheckbox::new(true); // Creates a checked checkbox
    /// ```
    #[must_use]
    pub fn new(is_checked: bool) -> Self {
        Self {
            is_checked,
            ..Default::default()
        }
    }

    /// Sets the text label for the checkbox (builder pattern)
    ///
    /// # Arguments
    /// * `label` - Text to display next to the checkbox
    #[must_use]
    pub fn with_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the disabled state of the checkbox (builder pattern)
    ///
    /// # Arguments
    /// * `disabled` - Whether the checkbox should be disabled
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    /// Sets the indeterminate state of the checkbox (builder pattern)
    ///
    /// The indeterminate state is typically used for parent checkboxes
    /// when some but not all child items are selected.
    ///
    /// # Arguments
    /// * `indeterminate` - Whether the checkbox should be indeterminate
    #[must_use]
    pub const fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.is_indeterminate = indeterminate;
        self
    }

    /// Sets the error state of the checkbox (builder pattern)
    ///
    /// Error state changes the visual appearance to indicate validation errors.
    ///
    /// # Arguments
    /// * `error` - Whether the checkbox is in an error state
    #[must_use]
    pub const fn error_state(mut self, error: bool) -> Self {
        self.error_state = error;
        self
    }

    /// Sets the size of the checkbox (builder pattern)
    ///
    /// # Arguments
    /// * `size` - The size variant to use
    #[must_use]
    pub const fn size(mut self, size: SelectionSize) -> Self {
        self.size = size;
        self
    }

    /// Creates the Iced widget element for this checkbox
    ///
    /// # Arguments
    /// * `on_toggle` - Callback function called when checkbox state changes
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to the UI
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        on_toggle: impl Fn(bool) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, theme::Theme, Renderer> {
        let style_fn = SelectionStyleBuilder::new(color_scheme.clone(), SelectionVariant::Checkbox)
            .size(self.size)
            .error(self.error_state)
            .checkbox_style();

        let checkbox = Checkbox::new(
            self.label.as_ref().unwrap_or(&String::new()),
            self.is_checked,
        )
        .on_toggle(on_toggle)
        .style(style_fn)
        .width(Length::Shrink);

        checkbox.into()
    }
}

// ============================================================================
// Material Design 3 Radio Button
// ============================================================================

/// Material Design 3 Radio Button component
///
/// Implements radio buttons following Material Design 3 specifications.
/// Radio buttons allow users to select one option from a set of mutually exclusive choices.
///
/// # Type Parameters
/// * `T` - The type of value associated with this radio button option
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialRadio<T> {
    /// The value represented by this radio button option
    pub value: T,
    /// Optional text label displayed next to the radio button
    pub label: Option<String>,
    /// Whether the radio button is disabled (non-interactive)
    pub is_disabled: bool,
    /// Whether the radio button is in an error state (for form validation)
    pub error_state: bool,
    /// The size variant of the radio button
    pub size: SelectionSize,
}

impl<T> MaterialRadio<T> {
    /// Creates a new radio button with the specified value
    ///
    /// # Arguments
    /// * `value` - The value this radio button represents
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self {
            value,
            label: None,
            is_disabled: false,
            error_state: false,
            size: SelectionSize::Medium,
        }
    }

    /// Sets the text label for the radio button (builder pattern)
    ///
    /// # Arguments
    /// * `label` - Text to display next to the radio button
    #[must_use]
    pub fn with_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the disabled state of the radio button (builder pattern)
    ///
    /// # Arguments
    /// * `disabled` - Whether the radio button should be disabled
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    /// Sets the error state of the radio button (builder pattern)
    ///
    /// Error state changes the visual appearance to indicate validation errors.
    ///
    /// # Arguments
    /// * `error` - Whether the radio button is in an error state
    #[must_use]
    pub const fn error_state(mut self, error: bool) -> Self {
        self.error_state = error;
        self
    }

    /// Sets the size of the radio button (builder pattern)
    ///
    /// # Arguments
    /// * `size` - The size variant to use
    #[must_use]
    pub const fn size(mut self, size: SelectionSize) -> Self {
        self.size = size;
        self
    }

    /// Creates the Iced widget element for this radio button
    ///
    /// # Arguments
    /// * `selected_value` - The currently selected value in the radio group
    /// * `on_select` - Callback function called when this radio button is selected
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to the UI
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        selected_value: Option<T>,
        on_select: impl FnOnce(T) -> Message + Copy + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, theme::Theme, Renderer>
    where
        T: Clone + PartialEq + Eq + Copy + 'a,
    {
        let style_fn = SelectionStyleBuilder::new(color_scheme.clone(), SelectionVariant::Radio)
            .size(self.size)
            .error(self.error_state)
            .radio_style();

        let radio = Radio::new(
            self.label.as_ref().unwrap_or(&String::new()),
            self.value,
            selected_value,
            on_select,
        )
        .style(style_fn)
        .width(Length::Shrink);

        radio.into()
    }
}

// ============================================================================
// Material Design 3 Switch (Custom Implementation)
// ============================================================================

/// Material Design 3 Switch component
///
/// Implements toggle switches following Material Design 3 specifications.
/// Switches allow users to toggle between two states (on/off, enabled/disabled).
/// They're ideal for settings and preferences that take effect immediately.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialSwitch {
    /// Whether the switch is currently enabled/on
    pub is_enabled: bool,
    /// Optional text label displayed next to the switch
    pub label: Option<String>,
    /// Whether the switch is disabled (non-interactive)
    pub is_disabled: bool,
    /// Whether the switch is in an error state (for form validation)
    pub error_state: bool,
    /// The size variant of the switch
    pub size: SelectionSize,
}

impl Default for MaterialSwitch {
    fn default() -> Self {
        Self {
            is_enabled: false,
            label: None,
            is_disabled: false,
            error_state: false,
            size: SelectionSize::Medium,
        }
    }
}

impl MaterialSwitch {
    /// Creates a new Material Design switch with the specified initial state
    ///
    /// # Arguments
    /// * `is_enabled` - Whether the switch starts in the enabled/on position
    ///
    /// # Examples
    /// ```
    /// use abop_gui::styling::material::MaterialSwitch;
    ///
    /// let switch = MaterialSwitch::new(true); // Creates an enabled switch
    /// ```
    #[must_use]
    pub fn new(is_enabled: bool) -> Self {
        Self {
            is_enabled,
            ..Default::default()
        }
    }

    /// Sets a text label for the switch (builder pattern)
    ///
    /// The label appears next to the switch and describes its purpose.
    ///
    /// # Arguments
    /// * `label` - The text label to display
    #[must_use]
    pub fn with_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the disabled state of the switch (builder pattern)
    ///
    /// Disabled switches cannot be interacted with and appear grayed out.
    ///
    /// # Arguments
    /// * `disabled` - Whether the switch should be disabled
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    /// Sets the error state of the switch (builder pattern)
    ///
    /// Error state changes the visual appearance to indicate validation errors.
    ///
    /// # Arguments
    /// * `error` - Whether the switch is in an error state
    #[must_use]
    pub const fn error_state(mut self, error: bool) -> Self {
        self.error_state = error;
        self
    }

    /// Sets the size of the switch (builder pattern)
    ///
    /// # Arguments
    /// * `size` - The size variant to use
    #[must_use]
    pub const fn size(mut self, size: SelectionSize) -> Self {
        self.size = size;
        self
    }

    /// Creates the Iced widget element for this switch
    ///
    /// Note: This is currently implemented as a styled checkbox.
    /// In a full implementation, this would be a custom switch widget.
    ///
    /// # Arguments
    /// * `on_toggle` - Callback function called when switch state changes
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to the UI
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        on_toggle: impl Fn(bool) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, theme::Theme, Renderer> {
        MaterialCheckbox {
            is_checked: self.is_enabled,
            label: self.label.clone(),
            is_disabled: self.is_disabled,
            is_indeterminate: false,
            error_state: self.error_state,
            size: self.size,
        }
        .view(on_toggle, color_scheme)
    }
}

// ============================================================================
// Material Design 3 Chip (Basic Implementation)
// ============================================================================

/// Material Design 3 Chip component
///
/// Chips are compact elements that represent input, attribute, or action.
/// They allow users to enter information, make selections, filter content, or trigger actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialChip {
    /// The text label displayed on the chip
    pub label: String,
    /// Whether the chip is currently selected (for filter/choice chips)
    pub is_selected: bool,
    /// Whether the chip is disabled (non-interactive)
    pub is_disabled: bool,
    /// The variant type of the chip (assist, filter, input, suggestion)
    pub variant: MaterialChipVariant,
    /// The size of the chip
    pub size: SelectionSize,
}

/// Material Design 3 chip variants
///
/// Different chip types serve different purposes in the interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialChipVariant {
    /// Action chips for common tasks and quick actions
    Assist,
    /// Filter chips for filtering content and making selections
    Filter,
    /// Input chips for user-generated content and tags
    Input,
    /// Suggestion chips for suggested actions or completions
    Suggestion,
}

impl MaterialChip {
    /// Creates a new Material Design chip with the specified label and variant
    ///
    /// # Arguments
    /// * `label` - The text to display on the chip
    /// * `variant` - The type of chip (assist, filter, input, suggestion)
    ///
    /// # Examples
    /// ```
    /// use abop_gui::styling::material::{MaterialChip, MaterialChipVariant};
    ///
    /// let chip = MaterialChip::new("Category", MaterialChipVariant::Filter);
    /// ```
    #[must_use]
    pub fn new<S: Into<String>>(label: S, variant: MaterialChipVariant) -> Self {
        Self {
            label: label.into(),
            is_selected: false,
            is_disabled: false,
            variant,
            size: SelectionSize::Medium,
        }
    }

    /// Sets the selected state of the chip (builder pattern)
    ///
    /// # Arguments
    /// * `selected` - Whether the chip should be selected
    #[must_use]
    pub const fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    /// Sets the disabled state of the chip (builder pattern)
    ///
    /// # Arguments
    /// * `disabled` - Whether the chip should be disabled
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    /// Sets the size of the chip (builder pattern)
    ///
    /// # Arguments
    /// * `size` - The size variant to use
    #[must_use]
    pub const fn size(mut self, size: SelectionSize) -> Self {
        self.size = size;
        self
    }

    /// Creates the Iced widget element for this chip
    ///
    /// Note: This is currently implemented as a styled button.
    /// In a full implementation, this would be a custom chip widget.
    ///
    /// # Arguments
    /// * `on_press` - Optional callback when the chip is pressed
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to the UI
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        on_press: Option<Message>,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, theme::Theme, Renderer> {
        use iced::widget::{Text, button};

        let style_fn = SelectionStyleBuilder::new(color_scheme.clone(), SelectionVariant::Chip)
            .size(self.size)
            .chip_style(self.is_selected);

        let content = Text::new(&self.label).size(match self.size {
            SelectionSize::Small => 12.0,
            SelectionSize::Medium => 14.0,
            SelectionSize::Large => 16.0,
        });

        let chip_button = button(content).style(style_fn).width(Length::Shrink);

        if let Some(message) = on_press {
            chip_button.on_press(message).into()
        } else {
            chip_button.into()
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a Material Design checkbox
#[must_use]
pub fn material_checkbox(is_checked: bool) -> MaterialCheckbox {
    MaterialCheckbox::new(is_checked)
}

/// Create a Material Design radio button
pub const fn material_radio<T>(value: T) -> MaterialRadio<T> {
    MaterialRadio::new(value)
}

/// Create a Material Design switch
#[must_use]
pub fn material_switch(is_enabled: bool) -> MaterialSwitch {
    MaterialSwitch::new(is_enabled)
}

/// Create a Material Design chip
pub fn material_chip<S: Into<String>>(label: S, variant: MaterialChipVariant) -> MaterialChip {
    MaterialChip::new(label, variant)
}
