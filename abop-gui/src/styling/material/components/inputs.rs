//! Material Design 3 Input Components
//!
//! Implements Material Design 3 input components including:
//! - Text Field (single-line text input with Material styling)
//! - Text Area (multi-line text input)
//! - Search Field (specialized search input with icons)

use iced::{
    Background, Border, Color, Length, Padding,
    widget::{TextInput, text_input},
};

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::{
    MaterialTokens, colors::MaterialColors, shapes::MaterialShapes, typography::TypographyRole,
};

/// Material Design 3 text field variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFieldVariant {
    /// Filled text field with background fill
    Filled,
    /// Outlined text field with border outline
    Outlined,
}

/// Text field size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFieldSize {
    /// Small text field (32px height)
    Small,
    /// Medium text field (40px height) - default
    Medium,
    /// Large text field (48px height)
    Large,
}

/// Text field state for interactive feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFieldState {
    /// Normal enabled state
    Enabled,
    /// Disabled state
    Disabled,
    /// Error state with validation feedback
    Error,
    /// Focused state with enhanced styling
    Focused,
}

/// Material Design 3 Text Field
///
/// A comprehensive text input implementation following Material Design 3 specifications
/// with support for filled and outlined variants, floating labels, and validation states.
#[derive(Debug, Clone)]
pub struct MaterialTextField {
    variant: TextFieldVariant,
    size: TextFieldSize,
    state: TextFieldState,
    full_width: bool,
    with_label: bool,
    with_helper_text: bool,
    with_prefix_icon: bool,
    with_suffix_icon: bool,
}

impl Default for MaterialTextField {
    fn default() -> Self {
        Self {
            variant: TextFieldVariant::Filled,
            size: TextFieldSize::Medium,
            state: TextFieldState::Enabled,
            full_width: false,
            with_label: false,
            with_helper_text: false,
            with_prefix_icon: false,
            with_suffix_icon: false,
        }
    }
}

impl MaterialTextField {
    /// Create a new Material text field with filled variant
    #[must_use]
    pub fn filled() -> Self {
        Self {
            variant: TextFieldVariant::Filled,
            ..Default::default()
        }
    }

    /// Create a new Material text field with outlined variant
    #[must_use]
    pub fn outlined() -> Self {
        Self {
            variant: TextFieldVariant::Outlined,
            ..Default::default()
        }
    }

    /// Set the text field size
    #[must_use]
    pub const fn size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    /// Set the text field state
    #[must_use]
    pub const fn state(mut self, state: TextFieldState) -> Self {
        self.state = state;
        self
    }

    /// Make the text field full width
    #[must_use]
    pub const fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Add a floating label to the text field
    #[must_use]
    pub const fn with_label(mut self) -> Self {
        self.with_label = true;
        self
    }

    /// Add helper text below the text field
    #[must_use]
    pub const fn with_helper_text(mut self) -> Self {
        self.with_helper_text = true;
        self
    }

    /// Add a prefix icon to the text field
    #[must_use]
    pub const fn with_prefix_icon(mut self) -> Self {
        self.with_prefix_icon = true;
        self
    }

    /// Add a suffix icon to the text field
    #[must_use]
    pub const fn with_suffix_icon(mut self) -> Self {
        self.with_suffix_icon = true;
        self
    }

    /// Create the text input widget with Material Design styling
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        value: &str,
        placeholder: &str,
        on_change: impl Fn(String) -> Message + 'a,
        tokens: &MaterialTokens,
    ) -> TextInput<'a, Message> {
        let mut text_input = TextInput::new(placeholder, value);

        // Apply on_change handler
        text_input = text_input.on_input(on_change);

        // Apply Material Design styling
        text_input = self.apply_material_styling(text_input, tokens);

        text_input
    }

    /// Apply Material Design 3 styling to the text input
    fn apply_material_styling<'a, Message: Clone + 'a>(
        &self,
        mut text_input: TextInput<'a, Message>,
        tokens: &MaterialTokens,
    ) -> TextInput<'a, Message> {
        // Get styling based on variant and state
        let styling = self.get_text_field_styling(tokens);

        // Apply Material Design styling function
        let style_fn =
            move |_theme: &iced::Theme, status: text_input::Status| -> text_input::Style {
                let mut style = text_input::Style {
                    background: styling.background,
                    border: styling.border,
                    icon: styling.icon_color,
                    placeholder: styling.placeholder_color,
                    value: styling.text_color,
                    selection: styling.text_color, // Use text color for selection
                };

                // Apply state-specific styling
                match status {
                    text_input::Status::Active => {
                        // Use default styling for active state
                    }
                    text_input::Status::Hovered => {
                        if let Some(hover_bg) = styling.hover_background {
                            style.background = hover_bg;
                        }
                        if let Some(hover_border) = styling.hover_border {
                            style.border = hover_border;
                        }
                    }
                    text_input::Status::Focused => {
                        if let Some(focused_bg) = styling.focused_background {
                            style.background = focused_bg;
                        }
                        if let Some(focused_border) = styling.focused_border {
                            style.border = focused_border;
                        }
                        if let Some(focused_text) = styling.focused_text_color {
                            style.value = focused_text;
                        }
                    }
                    text_input::Status::Disabled => {
                        if let Some(disabled_bg) = styling.disabled_background {
                            style.background = disabled_bg;
                        }
                        if let Some(disabled_border) = styling.disabled_border {
                            style.border = disabled_border;
                        }
                        if let Some(disabled_text) = styling.disabled_text_color {
                            style.value = disabled_text;
                        }
                    }
                }

                style
            };

        text_input = text_input.style(style_fn);

        // Apply size-specific properties
        let (width, _height, padding) = self.get_size_properties();

        if self.full_width {
            text_input = text_input.width(Length::Fill);
        } else {
            text_input = text_input.width(width);
        }

        text_input = text_input.size(self.get_font_size(tokens));
        text_input = text_input.padding(padding);

        text_input
    }

    /// Get Material Design styling for the current text field variant and state
    fn get_text_field_styling(&self, tokens: &MaterialTokens) -> TextFieldStyling {
        let colors = &tokens.colors;
        let shapes = &tokens.shapes;

        match self.variant {
            TextFieldVariant::Filled => self.get_filled_styling(colors, shapes),
            TextFieldVariant::Outlined => self.get_outlined_styling(colors, shapes),
        }
    }

    /// Get styling for filled text field variant
    fn get_filled_styling(
        &self,
        colors: &MaterialColors,
        shapes: &MaterialShapes,
    ) -> TextFieldStyling {
        let base_background = Background::Color(colors.surface_container_highest);
        let border_color = match self.state {
            TextFieldState::Error => colors.error.base,
            TextFieldState::Focused => colors.primary.base,
            _ => colors.outline,
        };

        TextFieldStyling {
            background: base_background,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: shapes.corner_extra_small.radius,
            },
            text_color: colors.on_surface,
            placeholder_color: colors.on_surface_variant,
            icon_color: colors.on_surface_variant,
            hover_background: Some(Background::Color(ColorUtils::blend_colors(
                colors.surface_container_highest,
                colors.on_surface,
                0.08,
            ))),
            hover_border: None,

            focused_background: Some(Background::Color(colors.surface_container_highest)),
            focused_border: Some(Border {
                color: border_color,
                width: 2.0,
                radius: shapes.corner_extra_small.radius,
            }),
            focused_text_color: Some(colors.on_surface),
            disabled_background: Some(Background::Color(ColorUtils::with_alpha(
                colors.on_surface,
                0.04,
            ))),
            disabled_border: None,
            disabled_text_color: Some(ColorUtils::with_alpha(colors.on_surface, 0.38)),
        }
    }

    /// Get styling for outlined text field variant
    const fn get_outlined_styling(
        &self,
        colors: &MaterialColors,
        shapes: &MaterialShapes,
    ) -> TextFieldStyling {
        let border_color = match self.state {
            TextFieldState::Error => colors.error.base,
            TextFieldState::Focused => colors.primary.base,
            _ => colors.outline,
        };

        let border_width = match self.state {
            TextFieldState::Focused => 2.0,
            _ => 1.0,
        };

        TextFieldStyling {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                color: border_color,
                width: border_width,
                radius: shapes.corner_extra_small.radius,
            },
            text_color: colors.on_surface,
            placeholder_color: colors.on_surface_variant,
            icon_color: colors.on_surface_variant,

            hover_background: None,
            hover_border: Some(Border {
                color: colors.on_surface,
                width: 1.0,
                radius: shapes.corner_extra_small.radius,
            }),

            focused_background: None,
            focused_border: Some(Border {
                color: colors.primary.base,
                width: 2.0,
                radius: shapes.corner_extra_small.radius,
            }),
            focused_text_color: Some(colors.on_surface),

            disabled_background: None,
            disabled_border: Some(Border {
                color: ColorUtils::with_alpha(colors.on_surface, 0.12),
                width: 1.0,
                radius: shapes.corner_extra_small.radius,
            }),
            disabled_text_color: Some(ColorUtils::with_alpha(colors.on_surface, 0.38)),
        }
    }

    /// Get size-specific properties for the text field
    fn get_size_properties(&self) -> (Length, Length, Padding) {
        match self.size {
            TextFieldSize::Small => (
                Length::Shrink,
                Length::Fixed(32.0),
                Padding::new(12.0).top(8.0).bottom(8.0),
            ),
            TextFieldSize::Medium => (
                Length::Shrink,
                Length::Fixed(40.0),
                Padding::new(16.0).top(10.0).bottom(10.0),
            ),
            TextFieldSize::Large => (
                Length::Shrink,
                Length::Fixed(48.0),
                Padding::new(20.0).top(14.0).bottom(14.0),
            ),
        }
    }

    /// Get font size based on text field size and Material tokens
    fn get_font_size(&self, tokens: &MaterialTokens) -> f32 {
        let body_style = tokens.typography.get_style(TypographyRole::BodyLarge);
        match self.size {
            TextFieldSize::Small => body_style.size() * 0.875, // 14px if body is 16px
            TextFieldSize::Medium => body_style.size(),        // 16px
            TextFieldSize::Large => body_style.size() * 1.125, // 18px if body is 16px
        }
    }
}

/// Internal styling structure for text fields
#[derive(Debug, Clone)]
struct TextFieldStyling {
    background: Background,
    border: Border,
    text_color: Color,
    placeholder_color: Color,
    icon_color: Color,
    hover_background: Option<Background>,
    hover_border: Option<Border>,
    focused_background: Option<Background>,
    focused_border: Option<Border>,
    focused_text_color: Option<Color>,
    disabled_background: Option<Background>,
    disabled_border: Option<Border>,
    disabled_text_color: Option<Color>,
}

/// Helper function to create a Material text field
pub fn material_text_field<'a, Message: Clone + 'a>(
    value: &str,
    placeholder: &str,
    variant: TextFieldVariant,
    on_change: impl Fn(String) -> Message + 'a,
    tokens: &MaterialTokens,
) -> TextInput<'a, Message> {
    let text_field = MaterialTextField::default().variant(variant);

    text_field.view(value, placeholder, on_change, tokens)
}

impl MaterialTextField {
    /// Set the text field variant (builder pattern continuation)
    const fn variant(mut self, variant: TextFieldVariant) -> Self {
        self.variant = variant;
        self
    }
}

/// Material Design 3 Search Field
///
/// Specialized text input for search functionality with built-in search icon
/// and clear functionality.
#[derive(Debug, Clone)]
pub struct MaterialSearchField {
    base: MaterialTextField,
}

impl Default for MaterialSearchField {
    fn default() -> Self {
        Self {
            base: MaterialTextField::filled()
                .with_prefix_icon()
                .with_suffix_icon(),
        }
    }
}

impl MaterialSearchField {
    /// Create a new Material search field
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the search field size
    #[must_use]
    pub const fn size(mut self, size: TextFieldSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    /// Make the search field full width
    #[must_use]
    pub const fn full_width(mut self) -> Self {
        self.base = self.base.full_width();
        self
    }

    /// Create the search field widget
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        value: &str,
        on_change: impl Fn(String) -> Message + 'a,
        tokens: &MaterialTokens,
    ) -> TextInput<'a, Message> {
        self.base.view(value, "Search...", on_change, tokens)
    }
}
