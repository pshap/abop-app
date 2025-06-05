# Color Strategy System Guide

## Overview

This document outlines best practices for using the Color Strategy System in our Material Design 3 implementation. The strategy system ensures UI components maintain consistent styling, accessibility, and theming support across the application.

## Core Principles

1. **All color decisions must go through the strategy system** - Never directly access color tokens (`tokens.colors`) in component rendering.
2. **Use variant-specific strategies** - Each component variant (e.g., `FilledButton`, `OutlinedButton`) should have its own strategy implementation.
3. **Consolidated styling logic** - Keep styling decisions in strategy implementations, not scattered throughout UI components.
4. **Accessibility first** - The strategy system helps ensure proper contrast ratios for all UI elements.

## Strategy Pattern Implementation

Our implementation follows the strategy pattern, allowing component styling to vary based on:
- Component variant (Filled, Outlined, etc.)
- Component state (Default, Hovered, Pressed, etc.)
- Theme (Light, Dark, custom theme)

### Key Components

1. **Strategy Traits** - Define the interface for variant-specific styling
2. **Strategy Implementations** - Implement styling for each variant
3. **Component APIs** - Use the strategy system to determine styling

## Common Pitfalls to Avoid

### ❌ Direct Color Access

```rust
// BAD: Direct color access
let button_text = Text::new("Click me")
    .color(tokens.colors.on_primary);
```

### ✅ Strategy-Based Color Access

```rust
// GOOD: Using strategy system
let strategy = ButtonStyleVariant::Filled.get_strategy();
let styling = strategy.get_styling(
    ButtonState::Default,
    tokens,
    &tokens.colors,
    &tokens.elevation,
    &tokens.shapes,
);

let button_text = Text::new("Click me")
    .color(styling.text_color);
```

### ❌ Hardcoded Color Logic in Components

```rust
// BAD: Hardcoded component color logic
fn get_colors(&self) -> Colors {
    let background = if self.pressed {
        self.tokens.colors.primary.light
    } else {
        self.tokens.colors.primary.base
    };
    
    Colors {
        background,
        text: self.tokens.colors.on_primary,
    }
}
```

### ✅ Strategy-Based Component Logic

```rust
// GOOD: Using strategy system in components
fn get_colors(&self) -> Colors {
    let variant = self.get_button_variant();
    let state = self.get_button_state();
    let strategy = variant.get_strategy();
    
    let styling = strategy.get_styling(
        state,
        self.tokens,
        &self.tokens.colors,
        &self.tokens.elevation,
        &self.tokens.shapes,
    );
    
    Colors {
        background: styling.background_color,
        text: styling.text_color,
    }
}
```

## Icons and Contrast Considerations

Icon buttons require special handling for contrast. Each button strategy should provide `icon_color` separately from `text_color`:

```rust
// Strategy implementation
ButtonStyling {
    background: Background::Color(base_background),
    text_color: text_color,
    border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
    shadow: None,
    // Separate color for icons to ensure contrast
    icon_color: Some(colors.on_surface),
}
```

The component should then respect this distinction:

```rust
// When rendering
let content_style = Style {
    text_color: if colors.icon_color.is_some() {
        colors.icon_color.unwrap_or(colors.text)
    } else {
        colors.text
    },
    ..style.clone()
};
```

## Adding New Components

When adding new components:

1. Define a strategy trait if needed
2. Implement strategy for each variant
3. Use the strategy system in the component logic
4. Add tests to verify correct styling and contrast

## Testing and Validation

Always validate:
- Color contrast meets WCAG 2.1 AA standards (4.5:1 for text, 3:1 for UI elements)
- Colors maintain proper contrast in both light and dark themes
- Theme changes are properly reflected

## Enforcing Strategy System Usage

Code reviews should check for direct color token usage. Additionally, our CI pipeline includes linting rules to detect bypassing of the strategy system.
