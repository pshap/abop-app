//! Material Design 3 Button Widget
//!
//! A complete implementation of Material Design 3 buttons as proper Iced widgets.
//! Supports all button variants with full theming and animation integration.
//!
//! # Color Strategy System Integration
//!
//! This button implementation uses the Material Design color strategy system to
//! determine all styling, including background colors, text colors, and icon colors.
//! This ensures consistent styling, proper contrast ratios, and accessibility.
//!
//! IMPORTANT: All color decisions must go through the strategy system. Never access color
//! tokens directly when rendering components. See `docs/color_strategy_system.md` for details.

use iced::{
    Background, Border, Color, Element, Length, Padding, Point, Rectangle, Renderer, Shadow, Size,
    Theme,
    advanced::{
        Clipboard, Layout, Renderer as AdvancedRenderer, Shell, Widget,
        layout::{Limits, Node},
        renderer::{Quad, Style},
        widget::{Operation, Tree},
    },
    event::{self, Event},
    mouse::{self, Cursor},
    widget::Text,
};

use crate::styling::material::{
    components::button_style::{strategy::ButtonState, variants::ButtonStyleVariant},
    elevation::ElevationLevel,
    tokens::core::MaterialTokens,
};

/// Material Design 3 button size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    /// Small button size (compact appearance)
    Small,
    /// Medium button size (default)
    Medium,
    /// Large button size (prominent appearance)
    Large,
}

/// Icon position relative to button text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPosition {
    /// Icon appears before the text
    Leading,
    /// Icon appears after the text
    Trailing,
    /// Only icon is shown (no text)
    Only,
}

/// Material Design 3 button variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialButtonVariant {
    /// High emphasis filled button (primary actions)
    Filled,
    /// Medium emphasis filled tonal button (secondary actions)
    FilledTonal,
    /// Medium emphasis outlined button
    Outlined,
    /// Low emphasis text button
    Text,
    /// High emphasis elevated button
    Elevated,
}

/// Material Design 3 button states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialButtonState {
    /// Button is interactive
    Enabled,
    /// Button is disabled
    Disabled,
    /// Button is being hovered
    Hovered,
    /// Button is being pressed
    Pressed,
    /// Button has focus
    Focused,
}

/// Material Design 3 Button Widget
pub struct MaterialButton<'a, Message> {
    /// Button content (text or text with icon)
    content: Element<'a, Message>,
    /// Button variant
    variant: MaterialButtonVariant,
    /// Current state
    state: MaterialButtonState,
    /// Material tokens for styling
    tokens: &'a MaterialTokens,
    /// Width of the button
    width: Length,
    /// Height of the button
    height: Length,
    /// Whether the button is enabled
    enabled: bool,
    /// Click handler
    on_press: Option<Message>,
    /// Button padding
    padding: Padding,
    /// Border radius
    border_radius: f32,
}

impl<'a, Message> MaterialButton<'a, Message> {
    /// Create a new Material button with text
    pub fn new(text: impl Into<String>, tokens: &'a MaterialTokens) -> Self {
        // Use strategy system default styling for initial color
        let filled_strategy = ButtonStyleVariant::Filled.get_strategy();
        let default_styling = filled_strategy.get_styling(
            ButtonState::Default,
            tokens,
            &tokens.colors,
            &tokens.elevation,
            &tokens.shapes,
        );

        let text_element = Text::new(text.into())
            .size(tokens.typography().label_large.size)
            .color(default_styling.text_color); // Use strategy-provided color

        Self {
            content: text_element.into(),
            variant: MaterialButtonVariant::Filled,
            state: MaterialButtonState::Enabled,
            tokens,
            width: Length::Shrink,
            height: Length::Fixed(40.0), // Material Design standard button height
            enabled: true,
            on_press: None,
            padding: Padding::new(24.0).top(10.0).bottom(10.0), // Material Design button padding
            border_radius: tokens.shapes.corner_small.radius.top_left, // Material Design corner radius
        }
    }

    /// Create a new Material button with custom content
    pub fn new_with_content(
        content: impl Into<Element<'a, Message>>,
        tokens: &'a MaterialTokens,
    ) -> Self {
        Self {
            content: content.into(),
            variant: MaterialButtonVariant::Filled,
            state: MaterialButtonState::Enabled,
            tokens,
            width: Length::Shrink,
            height: Length::Fixed(40.0), // Material Design standard button height
            enabled: true,
            on_press: None,
            padding: Padding::new(24.0).top(10.0).bottom(10.0), // Material Design button padding
            border_radius: tokens.shapes.corner_small.radius.top_left, // Material Design corner radius
        }
    }

    /// Set the button variant
    pub const fn variant(mut self, variant: MaterialButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button width
    pub const fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Set the button height
    pub const fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Set the click handler
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self.enabled = true;
        self
    }

    /// Disable the button
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self.on_press = None;
        self.state = MaterialButtonState::Disabled;
        self
    }

    /// Set custom padding
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
    /// Convert MaterialButtonState to ButtonState used by the strategy system
    const fn get_button_state(&self) -> ButtonState {
        match self.state {
            MaterialButtonState::Enabled => ButtonState::Default,
            MaterialButtonState::Disabled => ButtonState::Disabled,
            MaterialButtonState::Hovered => ButtonState::Hovered,
            MaterialButtonState::Pressed => ButtonState::Pressed,
            MaterialButtonState::Focused => ButtonState::Focused,
        }
    }

    /// Convert MaterialButtonVariant to ButtonStyleVariant used by the strategy system
    const fn get_button_variant(&self) -> ButtonStyleVariant {
        match self.variant {
            MaterialButtonVariant::Filled => ButtonStyleVariant::Filled,
            MaterialButtonVariant::FilledTonal => ButtonStyleVariant::FilledTonal,
            MaterialButtonVariant::Outlined => ButtonStyleVariant::Outlined,
            MaterialButtonVariant::Text => ButtonStyleVariant::Text,
            MaterialButtonVariant::Elevated => ButtonStyleVariant::Elevated,
        }
    }
    /// Get the button colors for the current variant and state using the strategy system
    fn get_colors(&self) -> ButtonColors {
        let variant = self.get_button_variant();
        let state = self.get_button_state();
        let strategy = variant.get_strategy();

        // Get styling directly from the strategy
        let styling = strategy.get_styling(
            state,
            self.tokens,
            &self.tokens.colors,
            &self.tokens.elevation,
            &self.tokens.shapes,
        );

        // Extract background color from Background enum
        let background_color = match styling.background {
            iced::Background::Color(color) => color,
            _ => Color::TRANSPARENT,
        };

        ButtonColors {
            background: background_color,
            text: styling.text_color,
            border: styling.border.color,
            shadow: styling.shadow.map_or(Color::TRANSPARENT, |s| s.color),
            icon_color: styling.icon_color,
        }
    }

    /// Get the elevation for the current variant and state
    const fn get_elevation(&self) -> ElevationLevel {
        match self.variant {
            MaterialButtonVariant::Filled => match self.state {
                MaterialButtonState::Enabled => ElevationLevel::Level0,
                MaterialButtonState::Disabled => ElevationLevel::Level0,
                MaterialButtonState::Hovered => ElevationLevel::Level1,
                MaterialButtonState::Pressed => ElevationLevel::Level0,
                MaterialButtonState::Focused => ElevationLevel::Level0,
            },
            MaterialButtonVariant::FilledTonal => ElevationLevel::Level0,
            MaterialButtonVariant::Outlined => ElevationLevel::Level0,
            MaterialButtonVariant::Text => ElevationLevel::Level0,
            MaterialButtonVariant::Elevated => match self.state {
                MaterialButtonState::Enabled => ElevationLevel::Level1,
                MaterialButtonState::Disabled => ElevationLevel::Level0,
                MaterialButtonState::Hovered => ElevationLevel::Level2,
                MaterialButtonState::Pressed => ElevationLevel::Level1,
                MaterialButtonState::Focused => ElevationLevel::Level1,
            },
        }
    }
}

/// Button color scheme derived from the strategy system
///
/// This structure contains color values obtained from the strategy system.
/// It should NEVER be populated with direct color values from tokens.
/// All colors must come from the appropriate strategy's `get_styling()` method.
#[derive(Debug, Clone)]
struct ButtonColors {
    background: Color,
    text: Color,
    border: Color,
    shadow: Color,
    /// Specific color for icons, may differ from text for better contrast
    /// This is essential for proper icon visibility on colored backgrounds
    icon_color: Option<Color>,
}

impl<Message> Widget<Message, Theme, Renderer> for MaterialButton<'_, Message>
where
    Message: Clone,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);

        // Layout the content with padding constraints
        let content_limits = limits.shrink(self.padding);
        let content_node =
            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, &content_limits);

        // Calculate the final button size
        let content_size = content_node.size();
        let padded_size = Size::new(
            content_size.width + self.padding.horizontal(),
            content_size.height + self.padding.vertical(),
        );

        let size = limits.resolve(self.width, self.height, padded_size);

        // Position the content within the button with padding
        let content_position = Point::new(self.padding.left, self.padding.top);

        Node::with_children(size, vec![content_node.move_to(content_position)])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let colors = self.get_colors();
        let elevation = self.get_elevation();

        // Draw elevation shadow if needed
        if elevation != ElevationLevel::Level0 {
            let elevation_style = self.tokens.elevation().get_level(elevation);
            let shadow = Shadow {
                color: colors.shadow,
                offset: elevation_style.shadow.offset,
                blur_radius: elevation_style.shadow.blur_radius,
            };

            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: bounds.x + shadow.offset.x,
                        y: bounds.y + shadow.offset.y,
                        ..bounds
                    },
                    border: Border {
                        radius: self.border_radius.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow,
                },
                Background::Color(colors.shadow),
            );
        }

        // Draw button background
        renderer.fill_quad(
            Quad {
                bounds,
                border: Border {
                    radius: self.border_radius.into(),
                    width: if colors.border == Color::TRANSPARENT {
                        0.0
                    } else {
                        1.0
                    },
                    color: colors.border,
                },
                shadow: Shadow::default(),
            },
            Background::Color(colors.background),
        ); // Draw the button content (text, icons, etc.)
        // Apply icon_color to content when rendering
        if let Some(content_layout) = layout.children().next() {
            // Create a new style with icon color applied
            let content_style = &Style {
                text_color: if colors.icon_color.is_some() {
                    colors.icon_color.unwrap_or(colors.text)
                } else {
                    colors.text
                },
                ..*style
            };

            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                content_style,
                content_layout,
                cursor,
                viewport,
            );
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if !self.enabled {
            return event::Status::Ignored;
        }

        let bounds = layout.bounds();
        let cursor_over = cursor.is_over(bounds);

        // Handle button-specific events
        let button_status = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if cursor_over => {
                self.state = MaterialButtonState::Pressed;
                event::Status::Captured
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) if cursor_over => {
                self.state = MaterialButtonState::Hovered;
                if let Some(message) = self.on_press.clone() {
                    shell.publish(message);
                }
                event::Status::Captured
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let new_state = if cursor_over {
                    MaterialButtonState::Hovered
                } else {
                    MaterialButtonState::Enabled
                };

                if self.state != new_state {
                    self.state = new_state;
                }
                event::Status::Captured
            }
            _ => event::Status::Ignored,
        };

        // If button didn't handle the event, pass it to content
        if button_status == event::Status::Ignored
            && let Some(content_layout) = layout.children().next()
        {
            return self.content.as_widget_mut().on_event(
                &mut tree.children[0],
                event,
                content_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        button_status
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.enabled && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        // Pass operations to content
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget().operate(
                &mut tree.children[0],
                content_layout,
                renderer,
                operation,
            );
        }
    }
}

impl<'a, Message> From<MaterialButton<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(button: MaterialButton<'a, Message>) -> Self {
        Self::new(button)
    }
}

/// Get the button style based on theme
pub fn button_style(theme: &Theme, _theme_mode: &Theme) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: Some(Background::Color(theme.palette().primary)),
        border: Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
