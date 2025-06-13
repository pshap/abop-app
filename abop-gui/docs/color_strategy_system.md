# Color Strategy System Guide

## Overview

This document outlines best practices for using the Color Strategy System in our Material Design 3 implementation. The strategy system ensures UI components maintain consistent styling, accessibility, and theming support across the application.

**Latest Update**: The system now uses our unified color system with guaranteed semantic colors and proper luminance-based theme detection.

## Core Principles

1. **All color decisions must go through the strategy system** - Never directly access color tokens (`MaterialColors`, `SemanticColors`) in component rendering.
2. **Use variant-specific strategies** - Each component variant (e.g., `FilledButton`, `OutlinedButton`) should have its own strategy implementation.
3. **Consolidated styling logic** - Keep styling decisions in strategy implementations, not scattered throughout UI components.
4. **Accessibility first** - The strategy system ensures proper contrast ratios for all UI elements.
5. **Use semantic colors properly** - Success=green, Warning=amber, Error=red, Info=blue (guaranteed).

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
let colors = MaterialColors::dark_default();
let button_text = Text::new("Click me")
    .color(colors.primary.base);
```

### ✅ Strategy-Based Color Access

```rust
// GOOD: Using strategy system
let strategy = ButtonStyleVariant::Filled.get_strategy();
let styling = strategy.get_styling(
    ButtonState::Default,
    &tokens,
    &tokens.colors,
    &tokens.elevation,
    &tokens.shapes,
);

let button_text = Text::new("Click me")
    .color(styling.text_color);
```

### ❌ Hardcoded Semantic Color Logic in Components

```rust
// BAD: Hardcoded semantic color assumptions
fn get_success_color(&self) -> Color {
    // WRONG: Assumes tertiary is green (it might not be!)
    self.colors.tertiary.base
}
```

### ✅ Proper Semantic Color Usage

```rust
// GOOD: Using guaranteed semantic colors
fn get_success_color(&self) -> Color {
    // CORRECT: Always green regardless of seed color
    if self.is_dark() {
        SemanticColors::dark().success
    } else {
        SemanticColors::light().success
    }
}
```

### ❌ Hardcoded Color Logic in Components

```rust
// BAD: Hardcoded component color logic with wrong assumptions
fn get_colors(&self) -> Colors {
    let background = if self.pressed {
        self.colors.primary.container  // Hardcoded state logic
    } else {
        self.colors.primary.base
    };
    
    // BAD: Assuming tertiary = success (wrong!)
    let success_color = self.colors.tertiary.base;
    
    Colors { background, text: self.colors.on_primary }
}
```

### ✅ Strategy-Based Component Logic

```rust
// GOOD: Using strategy system with proper semantic colors
fn get_colors(&self) -> Colors {
    let variant = self.get_button_variant();
    let state = self.get_button_state();
    let strategy = variant.get_strategy();
    
    let styling = strategy.get_styling(
        state,
        &self.tokens,
        &self.tokens.colors,
        &self.tokens.elevation,
        &self.tokens.shapes,
    );
    
    // Use guaranteed semantic colors when needed
    let semantic = if self.is_dark() {
        SemanticColors::dark()
    } else {
        SemanticColors::light()
    };
    
    Colors {
        background: styling.background_color,
        text: styling.text_color,
        success: semantic.success,  // Guaranteed green
        warning: semantic.warning,  // Guaranteed amber
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

## Recent System Improvements

### ✅ **Fixed Semantic Color Issues**
Our system previously had a critical flaw where semantic colors were incorrectly mapped:
- **Problem**: Assumed `tertiary.base` would always be green for success
- **Reality**: Material Design tertiary colors are derived from seed color and could be any hue
- **Solution**: Dedicated semantic colors with guaranteed hues:
  ```rust
  // Now guaranteed to always be the right color
  SemanticColors::dark().success   // Always green
  SemanticColors::light().warning  // Always amber  
  SemanticColors::dark().error     // Always red
  SemanticColors::light().info     // Always blue
  ```

### ✅ **Improved Theme Detection**
- **Old**: Unreliable `colors.primary.base.r < 0.5` detection
- **New**: Proper luminance-based detection using ITU-R BT.709 standards
- **Result**: Accurate theme detection regardless of color schemes

### ✅ **Eliminated Legacy Code** 
- Removed 1,078+ lines of duplicate/dead color systems
- Single source of truth for all color decisions
- Zero hard-coded color constants

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
- **Semantic colors have correct hues** (success=green, warning=amber, error=red, info=blue)
- Luminance-based theme detection works correctly
- No hardcoded `Color::from_rgb()` values in component logic

### Test Cases to Include
```rust
#[test]
fn test_semantic_colors_are_correct_hues() {
    let light_semantic = SemanticColors::light();
    let dark_semantic = SemanticColors::dark();
    
    // Success should always be green-ish
    assert!(is_green_hue(light_semantic.success));
    assert!(is_green_hue(dark_semantic.success));
    
    // Warning should always be amber/orange-ish  
    assert!(is_amber_hue(light_semantic.warning));
    assert!(is_amber_hue(dark_semantic.warning));
}

#[test]
fn test_strategy_system_accessibility() {
    let strategy = ButtonStyleVariant::Filled.get_strategy();
    let styling = strategy.get_styling(/* ... */);
    
    // Verify contrast ratios
    let contrast = calculate_contrast(styling.background_color, styling.text_color);
    assert!(contrast >= 4.5); // WCAG AA for text
}
```

## Enforcing Strategy System Usage

Code reviews should check for direct color token usage. Additionally, our CI pipeline includes linting rules to detect bypassing of the strategy system.
