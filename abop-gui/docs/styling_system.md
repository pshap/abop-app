# ABOP GUI Styling System Documentation

## Overview

The ABOP GUI styling system provides a comprehensive, modular, and extensible approach to theming and component styling. It includes semantic color tokens, component-specific styling, theme validation, testing framework, dynamic theme loading, and a plugin architecture.

## Architecture

### Core Components

1. **Design Tokens** (`design_tokens.rs`)
   - Semantic color system with light/dark theme support
   - Component tokens for consistent spacing, sizing, and styling
   - Theme-aware color selection

2. **Theme Integration** (`theme.rs`)
   - Bridge between design tokens and application themes
   - Theme mode management (Light/Dark/Custom)
   - Semantic color and component token access methods

3. **Color Strategy System** (see `color_strategy_system.md`)
   - Strategy pattern for component styling decisions
   - Ensures consistent styling, accessibility, and theme support
   - All component color decisions must go through this system

4. **Component Styling**
   - **Button Styling** (`styling/button.rs`) - Semantic button styles with variants
   - **Input Styling** (`styling/input.rs`) - Form input styling with states
   - **Container Styling** (`styling/container/base.rs`) - Layout container styles

4. **Utilities**
   - **Color Utils** (`styling/color_utils.rs`) - Color manipulation and accessibility
   - **Style Builders** (`styling/builders.rs`) - Fluent API for style construction
   - **Macros** (`styling/macros.rs`) - Style generation macros

5. **Advanced Features**
   - **Validation** (`styling/validation.rs`) - Theme validation and accessibility checks
   - **Testing** (`styling/testing.rs`) - Style testing framework
   - **Dynamic Themes** (`styling/dynamic_themes.rs`) - Runtime theme loading
   - **Plugins** (`styling/plugins.rs`) - Extensible plugin architecture

## Usage Examples

### Basic Component Styling

```rust
use crate::styling::button::ButtonStyles;
use crate::theme::ThemeMode;

// Get semantic button style
let theme = ThemeMode::Dark;
let button_style = ButtonStyles::semantic_style(&theme, "primary");

// Use with Iced button
Button::new("Click me").style(button_style)
```

### Using Style Builders

```rust
use crate::styling::builders::ButtonStyleBuilder;
use crate::design_tokens::SemanticColors;

let colors = SemanticColors::for_theme(true); // Dark theme
let style = ButtonStyleBuilder::new()
    .primary_colors(&colors)
    .medium_size()
    .rounded_corners()
    .elevated()
    .build();
```

### Dynamic Theme Loading

```rust
use crate::styling::dynamic_themes::{ThemeLoader, ThemeFormat};

// Load theme from file
let loader = ThemeLoader::new();
let theme_config = loader.load_theme_from_file("themes/custom.toml", ThemeFormat::Toml)?;
let custom_theme = loader.create_theme_mode(theme_config)?;

// Apply theme
app.update_theme(custom_theme);
```

### Plugin Development

```rust
use crate::styling::plugins::{StylePlugin, PluginInfo, CustomComponentStyle};

struct MyCustomPlugin;

impl StylePlugin for MyCustomPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "Custom Component Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Developer".to_string(),
            description: "Custom styling for special components".to_string(),
            api_version: "1.0".to_string(),
        }
    }
    
    fn initialize(&mut self, theme: &ThemeMode) -> Result<(), StylePluginError> {
        // Initialize plugin with current theme
        Ok(())
    }
    
    fn get_component_style(&self, component: &str, variant: &str) -> Option<CustomComponentStyle> {
        match (component, variant) {
            ("special_button", "glow") => Some(create_glow_button_style()),
            _ => None,
        }
    }
    
    // ... other methods
}

// Register plugin
let plugin_system = &STYLE_SYSTEM;
plugin_system.registry.register_plugin(Box::new(MyCustomPlugin))?;
```

## Theme Configuration

### JSON Theme Format

```json
{
  "theme_info": {
    "name": "Custom Theme",
    "description": "A custom theme",
    "author": "Author Name",
    "version": "1.0.0"
  },
  "colors": {
    "primary": "#FF6B35",
    "on_primary": "#FFFFFF",
    "secondary": "#7B5E57",
    "background": "#FEFDF8"
  },
  "component_tokens": {
    "button_border_radius": 12.0,
    "spacing_md": 16.0
  }
}
```

### TOML Theme Format

```toml
[theme_info]
name = "Custom Theme"
description = "A custom theme"
author = "Author Name"
version = "1.0.0"

[colors]
primary = "#FF6B35"
on_primary = "#FFFFFF"
secondary = "#7B5E57"
background = "#FEFDF8"

[component_tokens]
button_border_radius = 12.0
spacing_md = 16.0
```

## Testing

### Running Style Tests

```rust
use crate::styling::testing::StyleTester;

let tester = StyleTester::new();

// Test all themes
tester.test_all_themes()?;

// Test specific components
tester.test_button_styles()?;
tester.test_input_styles()?;
tester.test_container_styles()?;

// Test accessibility
tester.test_accessibility_compliance()?;
```

### Custom Test Cases

```rust
use crate::styling::testing::{StyleTestCase, TestResult};

let test_case = StyleTestCase {
    name: "Custom Button Test".to_string(),
    component: "button".to_string(),
    variant: "primary".to_string(),
    theme: ThemeMode::Dark,
    expected_properties: vec![
        ("background_color".to_string(), TestValue::Color(expected_bg)),
        ("border_radius".to_string(), TestValue::Float(8.0)),
    ],
};

let result = tester.run_test_case(&test_case)?;
assert!(result.passed);
```

## Validation

### Theme Validation

```rust
use crate::styling::validation::ThemeValidator;

let validator = ThemeValidator::new();

// Validate theme accessibility
let result = validator.validate_theme_accessibility(&theme)?;
if !result.is_valid {
    eprintln!("Accessibility issues: {:?}", result.issues);
}

// Validate contrast ratios
let contrast_result = validator.validate_contrast_ratios(&colors)?;
if !contrast_result.all_passed {
    eprintln!("Contrast issues: {:?}", contrast_result.failed_pairs);
}
```

## Best Practices

### 1. Use Semantic Colors

```rust
// Good: Use semantic colors
let colors = SemanticColors::for_theme(is_dark);
let bg_color = colors.primary;

// Avoid: Hard-coded colors
let bg_color = Color::from_rgb(0.2, 0.4, 0.6);
```

### 2. Leverage Component Tokens

```rust
// Good: Use component tokens
let tokens = ComponentTokens::default();
let border_radius = tokens.button_border_radius;

// Avoid: Magic numbers
let border_radius = 8.0;
```

### 3. Test Style Changes

```rust
// Always test style changes
#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::testing::StyleTester;
    
    #[test]
    fn test_new_button_style() {
        let tester = StyleTester::new();
        let result = tester.test_button_styles().unwrap();
        assert!(result.all_passed());
    }
}
```

### 4. Validate Accessibility

```rust
// Ensure accessibility compliance
let validator = ThemeValidator::new();
let result = validator.validate_theme_accessibility(&theme).unwrap();
assert!(result.is_valid, "Theme must be accessible");
```

## Advanced Customization

### Creating Custom Style Variants

```rust
use crate::styling::traits::{StyleVariant, ComponentStyle};

#[derive(Debug, Clone)]
pub enum CustomButtonVariant {
    Neon,
    Glass,
    Gradient,
}

impl StyleVariant for CustomButtonVariant {
    fn name(&self) -> &str {
        match self {
            Self::Neon => "neon",
            Self::Glass => "glass", 
            Self::Gradient => "gradient",
        }
    }
}

impl ComponentStyle<CustomButtonVariant> for ButtonStyle {
    fn apply_variant(&mut self, variant: &CustomButtonVariant, theme: &ThemeMode) {
        match variant {
            CustomButtonVariant::Neon => apply_neon_effect(self, theme),
            CustomButtonVariant::Glass => apply_glass_effect(self, theme),
            CustomButtonVariant::Gradient => apply_gradient_effect(self, theme),
        }
    }
}
```

### Plugin Architecture

The plugin system allows extending the styling system with custom components and behaviors:

```rust
// 1. Implement StylePlugin trait
// 2. Register plugin with system
// 3. Use custom styles in components
// 4. Handle theme changes automatically
```

## Migration Guide

### From Basic to Semantic Styling

1. Replace direct color usage with semantic colors
2. Use component tokens instead of magic numbers
3. Add accessibility validation to tests
4. Implement theme-aware styling

### From Static to Dynamic Themes

1. Create theme configuration files
2. Use ThemeLoader for runtime loading
3. Update components to handle theme changes
4. Test with multiple themes

## Performance Considerations

- Style calculations are cached where possible
- Theme changes trigger efficient updates
- Plugin system uses lazy loading
- Validation runs in development only

## Troubleshooting

### Common Issues

1. **Contrast Ratio Failures**
   - Check color combinations in validation
   - Use contrast ratio calculator
   - Adjust colors to meet WCAG guidelines

2. **Theme Loading Errors**
   - Validate JSON/TOML syntax
   - Check required fields in configuration
   - Ensure color format is correct (#RRGGBB)

3. **Plugin Registration Failures**
   - Verify API version compatibility
   - Check plugin initialization code
   - Ensure all required methods are implemented

### Debug Tools

```rust
// Enable style debugging
env::set_var("ABOP_STYLE_DEBUG", "1");

// Validate theme on startup
let validator = ThemeValidator::new();
validator.validate_all_themes().unwrap();

// Test style consistency
let tester = StyleTester::new();
tester.run_regression_tests().unwrap();
```

## Contributing

When contributing to the styling system:

1. Add tests for new components
2. Validate accessibility compliance
3. Update documentation with examples
4. Follow semantic color conventions
5. Maintain backward compatibility

## API Reference

See individual module documentation for detailed API reference:

- [Design Tokens API](./design_tokens.rs)
- [Theme API](./theme.rs)
- [Button Styling API](./styling/button.rs)
- [Input Styling API](./styling/input.rs)
- [Container Styling API](./styling/container/base.rs)
- [Validation API](./styling/validation.rs)
- [Testing API](./styling/testing.rs)
- [Plugin API](./styling/plugins.rs)
