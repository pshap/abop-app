# Material Design 3 Styling System

## Overview

The ABOP GUI uses a comprehensive Material Design 3 token system for consistent styling across the application. The system is built on Material Design 3 specifications and provides a robust foundation for theming and customization.

## Token System Architecture

1. **Material Design Tokens** (`styling/material/tokens/`)
   - Core token structures
   - Semantic color mappings
   - State management
   - Theme support

2. **Color System** (`styling/material/colors/`)
   - Material Design 3 color palettes
   - Semantic color roles
   - Dynamic color generation
   - Theme-aware color management

3. **Typography** (`styling/material/typography/`)
   - Material Design 3 type scale
   - Font roles and styles
   - Responsive typography

4. **Spacing & Sizing** (`styling/material/spacing/`, `styling/material/sizing/`)
   - Consistent spacing scale
   - Component sizing tokens
   - Layout guidelines

5. **Elevation & Shapes** (`styling/material/elevation/`, `styling/material/shapes/`)
   - Material Design elevation system
   - Shape tokens and variants
   - Surface treatments

## Usage Examples

```rust
use crate::styling::material::{
    MaterialTokens,
    spacing::SpacingTokens,
    typography::MaterialTypography,
    tokens::semantic::SemanticColors,
};

// Get Material Design tokens
let tokens = MaterialTokens::default();

// Use spacing tokens
let padding = tokens.spacing().md;

// Use typography
let font_size = tokens.typography().body;

// Use semantic colors
let primary = tokens.semantic_colors().primary;
```

## Theme System

The theme system provides:

1. **Dynamic Theming**
   - Light/dark theme support
   - Custom color schemes
   - Runtime theme switching

2. **Component Customization**
   - Style overrides
   - Variant support
   - State management

3. **Serialization**
   - Theme persistence
   - Custom theme loading
   - Theme validation

## Best Practices

1. **Use Token System**
   - Always use Material Design tokens instead of hardcoded values
   - Follow Material Design 3 specifications
   - Use semantic color roles

2. **Component Development**
   - Use Material Design components as base
   - Follow component guidelines
   - Support theme customization

3. **Performance**
   - Minimize token conversions
   - Use const where possible
   - Cache token values when appropriate

## API Reference

- [Material Design Tokens API](./material/tokens.md)
- [Color System API](./material/colors.md)
- [Typography API](./material/typography.md)
- [Theme System API](./material/themes.md)
