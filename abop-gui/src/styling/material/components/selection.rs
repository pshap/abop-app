// Material Design 3 Selection Components
// Implements Material Design checkboxes, radio buttons, switches, and chips
// following Material Design 3 specifications with proper Iced integration

use super::selection_style::{SelectionSize, SelectionStyleBuilder, SelectionVariant};
use crate::styling::material::colors::MaterialColors;
use iced::{
    Element, Renderer,
    theme::{self},
    widget::{Checkbox, Radio},
};

// ============================================================================
// Typography Constants (Phase 1 Improvement)
// ============================================================================

/// Typography constants for consistent text sizing across selection components
mod typography {
    /// Small text size for compact selection components
    pub const SMALL_TEXT: f32 = 12.0;
    /// Medium text size for default selection components
    pub const MEDIUM_TEXT: f32 = 14.0;
    /// Large text size for accessible selection components
    pub const LARGE_TEXT: f32 = 16.0;
}

// ============================================================================
// Common Trait Interface (Phase 1 Improvement)
// ============================================================================

/// Common interface for selection components to reduce code duplication
/// 
/// This trait provides builder pattern methods that are shared across
/// all selection components (checkbox, radio, switch, chip).
pub trait SelectionComponent: Sized {
    /// Sets the disabled state (builder pattern)
    fn disabled(self, disabled: bool) -> Self;
    
    /// Sets the error state for validation (builder pattern)
    fn error_state(self, error: bool) -> Self;
    
    /// Sets the size variant (builder pattern)
    fn size(self, size: SelectionSize) -> Self;
}

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
    }    /// Sets the disabled state of the checkbox (builder pattern)
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
    }    /// Creates the Iced widget element for this checkbox
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
        .style(style_fn);

        checkbox.into()
    }
}

// Implement common trait for MaterialCheckbox
impl SelectionComponent for MaterialCheckbox {
    fn disabled(self, disabled: bool) -> Self {
        self.disabled(disabled)
    }
    
    fn error_state(self, error: bool) -> Self {
        self.error_state(error)
    }
    
    fn size(self, size: SelectionSize) -> Self {
        self.size(size)
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
    }    /// Creates the Iced widget element for this radio button
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
        .style(style_fn);

        radio.into()
    }
}

// Implement common trait for MaterialRadio
impl<T> SelectionComponent for MaterialRadio<T> {
    fn disabled(self, disabled: bool) -> Self {
        self.disabled(disabled)
    }
    
    fn error_state(self, error: bool) -> Self {
        self.error_state(error)
    }
    
    fn size(self, size: SelectionSize) -> Self {
        self.size(size)
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
    }    /// Creates the Iced widget element for this switch
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
        // For now, implement switch as a styled checkbox
        // In a full implementation, this would be a custom switch widget
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

// Implement common trait for MaterialSwitch
impl SelectionComponent for MaterialSwitch {
    fn disabled(self, disabled: bool) -> Self {
        self.disabled(disabled)
    }
    
    fn error_state(self, error: bool) -> Self {
        self.error_state(error)
    }
    
    fn size(self, size: SelectionSize) -> Self {
        self.size(size)
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
    }    /// Creates the Iced widget element for this chip
    ///
    /// Note: This is currently implemented as a styled button.
    /// In a full implementation, this would be a custom chip widget.
    ///
    /// # Arguments
    /// * `on_press` - Optional callback when the chip is pressed
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///    /// # Returns
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
            SelectionSize::Small => typography::SMALL_TEXT,
            SelectionSize::Medium => typography::MEDIUM_TEXT,
            SelectionSize::Large => typography::LARGE_TEXT,
        });

        let chip_button = button(content).style(style_fn);

        if let Some(message) = on_press {
            chip_button.on_press(message).into()
        } else {
            chip_button.into()
        }
    }
}

// Implement common trait for MaterialChip
impl SelectionComponent for MaterialChip {
    fn disabled(self, disabled: bool) -> Self {
        self.disabled(disabled)
    }
    
    fn error_state(self, _error: bool) -> Self {
        // Note: Chips don't typically have error states in Material Design
        // This could be implemented in the future if needed
        self
    }
    
    fn size(self, size: SelectionSize) -> Self {
        self.size(size)
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

// ============================================================================
// Tests (Phase 1 Improvement)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_builder_pattern() {
        let checkbox = MaterialCheckbox::new(true)
            .with_label("Test Checkbox")
            .disabled(false)
            .indeterminate(true)
            .error_state(false)
            .size(SelectionSize::Large);

        assert_eq!(checkbox.is_checked, true);
        assert_eq!(checkbox.label, Some("Test Checkbox".to_string()));
        assert_eq!(checkbox.is_disabled, false);
        assert_eq!(checkbox.is_indeterminate, true);
        assert_eq!(checkbox.error_state, false);
        assert_eq!(checkbox.size, SelectionSize::Large);
    }

    #[test]
    fn test_checkbox_default() {
        let checkbox = MaterialCheckbox::default();
        
        assert_eq!(checkbox.is_checked, false);
        assert_eq!(checkbox.label, None);
        assert_eq!(checkbox.is_disabled, false);
        assert_eq!(checkbox.is_indeterminate, false);
        assert_eq!(checkbox.error_state, false);
        assert_eq!(checkbox.size, SelectionSize::Medium);
    }

    #[test]
    fn test_radio_builder_pattern() {
        let radio = MaterialRadio::new("test_value")
            .with_label("Test Radio")
            .disabled(true)
            .error_state(true)
            .size(SelectionSize::Small);

        assert_eq!(radio.value, "test_value");
        assert_eq!(radio.label, Some("Test Radio".to_string()));
        assert_eq!(radio.is_disabled, true);
        assert_eq!(radio.error_state, true);
        assert_eq!(radio.size, SelectionSize::Small);
    }

    #[test]
    fn test_switch_builder_pattern() {
        let switch = MaterialSwitch::new(true)
            .with_label("Test Switch")
            .disabled(false)
            .error_state(true)
            .size(SelectionSize::Medium);

        assert_eq!(switch.is_enabled, true);
        assert_eq!(switch.label, Some("Test Switch".to_string()));
        assert_eq!(switch.is_disabled, false);
        assert_eq!(switch.error_state, true);
        assert_eq!(switch.size, SelectionSize::Medium);
    }

    #[test]
    fn test_switch_default() {
        let switch = MaterialSwitch::default();
        
        assert_eq!(switch.is_enabled, false);
        assert_eq!(switch.label, None);
        assert_eq!(switch.is_disabled, false);
        assert_eq!(switch.error_state, false);
        assert_eq!(switch.size, SelectionSize::Medium);
    }

    #[test]
    fn test_chip_builder_pattern() {
        let chip = MaterialChip::new("Test Chip", MaterialChipVariant::Filter)
            .selected(true)
            .disabled(false)
            .size(SelectionSize::Large);

        assert_eq!(chip.label, "Test Chip");
        assert_eq!(chip.variant, MaterialChipVariant::Filter);
        assert_eq!(chip.is_selected, true);
        assert_eq!(chip.is_disabled, false);
        assert_eq!(chip.size, SelectionSize::Large);
    }

    #[test]
    fn test_chip_variants() {
        let assist = MaterialChip::new("Assist", MaterialChipVariant::Assist);
        let filter = MaterialChip::new("Filter", MaterialChipVariant::Filter);
        let input = MaterialChip::new("Input", MaterialChipVariant::Input);
        let suggestion = MaterialChip::new("Suggestion", MaterialChipVariant::Suggestion);

        assert_eq!(assist.variant, MaterialChipVariant::Assist);
        assert_eq!(filter.variant, MaterialChipVariant::Filter);
        assert_eq!(input.variant, MaterialChipVariant::Input);
        assert_eq!(suggestion.variant, MaterialChipVariant::Suggestion);
    }

    #[test]
    fn test_selection_component_trait() {
        // Test that all components implement the common trait
        let checkbox = MaterialCheckbox::new(false)
            .disabled(true)
            .error_state(true)
            .size(SelectionSize::Large);

        let radio = MaterialRadio::new(42)
            .disabled(true)
            .error_state(true)
            .size(SelectionSize::Small);

        let switch = MaterialSwitch::new(true)
            .disabled(true)
            .error_state(true)
            .size(SelectionSize::Medium);

        let chip = MaterialChip::new("Test", MaterialChipVariant::Filter)
            .disabled(true)
            .error_state(true)  // Should be no-op for chips
            .size(SelectionSize::Large);

        assert_eq!(checkbox.is_disabled, true);
        assert_eq!(checkbox.error_state, true);
        assert_eq!(checkbox.size, SelectionSize::Large);

        assert_eq!(radio.is_disabled, true);
        assert_eq!(radio.error_state, true);
        assert_eq!(radio.size, SelectionSize::Small);

        assert_eq!(switch.is_disabled, true);
        assert_eq!(switch.error_state, true);
        assert_eq!(switch.size, SelectionSize::Medium);

        assert_eq!(chip.is_disabled, true);
        assert_eq!(chip.size, SelectionSize::Large);
    }

    #[test]
    fn test_helper_functions() {
        let checkbox = material_checkbox(true);
        let radio = material_radio("value");
        let switch = material_switch(false);
        let chip = material_chip("Test", MaterialChipVariant::Assist);

        assert_eq!(checkbox.is_checked, true);
        assert_eq!(radio.value, "value");
        assert_eq!(switch.is_enabled, false);
        assert_eq!(chip.label, "Test");
    }

    #[test]
    fn test_typography_constants() {
        // Test that typography constants are reasonable values
        assert_eq!(typography::SMALL_TEXT, 12.0);
        assert_eq!(typography::MEDIUM_TEXT, 14.0);
        assert_eq!(typography::LARGE_TEXT, 16.0);
        
        // Test relative sizes
        assert!(typography::SMALL_TEXT < typography::MEDIUM_TEXT);
        assert!(typography::MEDIUM_TEXT < typography::LARGE_TEXT);
    }
}
