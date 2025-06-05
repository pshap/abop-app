# Material Design 3 Color System

## Table of Contents
1. [Introduction](#introduction)
2. [Color Tokens](#color-tokens)
   - [Core Tokens](#core-tokens)
   - [Surface Tokens](#surface-tokens)
   - [Container Tokens](#container-tokens)
   - [Fixed Tokens](#fixed-tokens)
3. [Color Relationships](#color-relationships)
4. [Theming](#theming)
5. [Usage Examples](#usage-examples)
6. [Accessibility](#accessibility)

## Introduction

The Material Design 3 (MD3) color system is built on a semantic token system that ensures consistency and accessibility across your application. This guide documents the color tokens, their relationships, and usage patterns in the `abop-iced` implementation.

## Color Tokens

### Core Tokens

Core tokens represent the fundamental color roles in your application. They are the building blocks for your UI's color scheme.

#### Primary Colors
- `primary`: The primary brand color
- `on_primary`: Text/iconography on primary color
- `primary_container`: Container for primary elements
- `on_primary_container`: Text/iconography on primary container

#### Secondary Colors
- `secondary`: The secondary brand color
- `on_secondary`: Text/iconography on secondary color
- `secondary_container`: Container for secondary elements
- `on_secondary_container`: Text/iconography on secondary container

#### Tertiary Colors
- `tertiary`: The tertiary/accent color
- `on_tertiary`: Text/iconography on tertiary color
- `tertiary_container`: Container for tertiary elements
- `on_tertiary_container`: Text/iconography on tertiary container

#### Error Colors
- `error`: Color for error states
- `on_error`: Text/iconography on error color
- `error_container`: Container for error messages
- `on_error_container`: Text/iconography on error container

### Surface Tokens

Surface tokens define the colors of surfaces and their content.

- `background`: Background color of the app
- `on_background`: Text/iconography on background
- `surface`: Color of surfaces (cards, sheets, menus)
- `on_surface`: Text/iconography on surface
- `surface_variant`: Variant surface color
- `on_surface_variant`: Text/iconography on surface variant

### Container Tokens

Container tokens are used for containing related UI elements.

- `outline`: Color of outlines
- `outline_variant`: Variant outline color
- `shadow`: Color of shadows
- `scrim`: Color of scrims/overlays

### Fixed Tokens

Fixed tokens maintain consistent colors regardless of theme.

- `inverse_surface`: Inverse surface color
- `inverse_on_surface`: Text/iconography on inverse surface
- `inverse_primary`: Inverse primary color

## Color Relationships

### Naming Convention
- `color`: The base color
- `on_color`: Content that appears on top of the base color
- `color_container`: Container for related UI elements
- `on_color_container`: Content that appears on top of the container

### Semantic Meaning
Each color role has a specific semantic meaning:
- **Primary**: Primary brand color, used for key UI elements
- **Secondary**: Secondary brand color, used for less prominent UI elements
- **Tertiary**: Accent color, used for highlighting UI elements
- **Error**: For indicating errors and destructive actions
- **Surface**: For surfaces and backgrounds
- **Outline**: For dividers and borders

## Theming

The color system supports light and dark themes. The theme can be toggled programmatically:

```rust
use abop_iced::material::color::{Theme, ThemeVariant};

// Create a light theme
let mut theme = Theme::light();

// Toggle to dark theme
theme.toggle();

// Or set theme variant directly
theme.set_variant(ThemeVariant::Dark);
```

### Custom Theming

You can create custom themes using the `DynamicTheme` builder:

```rust
use abop_iced::material::color::{DynamicTheme, Srgb};

let theme = DynamicTheme::new()
    .with_seed_color(Srgb::new(0.3, 0.5, 0.8))
    .with_custom_color("accent", Srgb::new(1.0, 0.5, 0.0))
    .generate_theme();
```

## Usage Examples

### Basic Usage

```rust
use abop_iced::material::color::{
    Theme, ThemeVariant, Srgb,
    token::{CoreTokens, SurfaceTokens}
};

// Create a light theme
let theme = Theme::light();

// Access color tokens
let primary = theme.colors.primary;
let on_primary = theme.colors.on_primary;

// Use in UI components
// container()
//     .background(theme.colors.primary)
//     .text_color(theme.colors.on_primary)
```

### Themed Component

```rust
use iced::widget::{container, text};
use iced::Element;
use abop_iced::material::color::Theme;

fn themed_button(theme: &Theme) -> Element<'static, Message> {
    container(
        text("Click me")
            .size(16)
    )
    .padding(10)
    .background(theme.colors.primary)
    .text_color(theme.colors.on_primary)
    .into()
}
```

## Accessibility

### Contrast Ratios

- **Text on background**: At least 4.5:1 for normal text, 3:1 for large text
- **UI components**: At least 3:1 against adjacent colors
- **Interactive elements**: Clear visual indication of state (hover, focus, pressed)

### Color Blindness

- Use patterns or textures in addition to color
- Ensure sufficient contrast between adjacent colors
- Test with color blindness simulators

### Dark Mode

- Use semantic color tokens that automatically adapt to light/dark themes
- Test colors in both themes for readability and contrast
- Consider using slightly desaturated colors in dark mode to reduce eye strain

## Best Practices

1. **Use semantic tokens** instead of hardcoded colors
2. **Maintain contrast** for readability and accessibility
3. **Limit color variations** to maintain consistency
4. **Test in different lighting conditions** and themes
5. **Document color usage** in your design system

## Troubleshooting

### Colors Don't Update on Theme Change
Ensure you're using the theme's color tokens and not hardcoded colors. The theme system only works when using the provided tokens.

### Poor Contrast
If text is hard to read, check the contrast ratio between the text color and its background. Use the `on_` variants for text on colored backgrounds.

### Inconsistent Colors
Always use the theme's color tokens to ensure consistency across your application. Avoid defining custom colors unless absolutely necessary.
