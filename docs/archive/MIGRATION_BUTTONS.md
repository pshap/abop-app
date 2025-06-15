# Button API Migration Guide

This guide helps you migrate from the old button API to the new, more flexible button builder API.

## Overview

The button system has been completely refactored to:

1. Eliminate code duplication
2. Provide a more flexible and type-safe API
3. Improve consistency across the codebase
4. Make it easier to maintain and extend

## Key Changes

### Old API (Deprecated)

```rust
use abop_gui::components::common::{
    primary_button_semantic,
    secondary_button_semantic,
    primary_button_with_icon_semantic,
    filled_icon_button_semantic,
};

// Primary button
let btn = primary_button_semantic("Save", Message::Save, &tokens);

// Secondary button with icon
let btn = secondary_button_with_icon_semantic(
    "Export", 
    "download", 
    IconPosition::Leading, 
    Message::Export, 
    &tokens
);

// Icon button
let btn = filled_icon_button_semantic("settings", ButtonSize::Medium, Message::Settings, &tokens);
```

### New API (Recommended)

```rust
use abop_gui::components::buttons::{self, ButtonVariant, IconPosition};

// Primary button
let btn = buttons::button(&tokens)
    .label("Save")
    .variant(ButtonVariant::Filled)
    .on_press(Message::Save)
    .build()?;

// Secondary button with icon
let btn = buttons::button(&tokens)
    .label("Export")
    .icon("download", IconPosition::Leading)
    .variant(ButtonVariant::Outlined)
    .on_press(Message::Export)
    .build()?;

// Icon button
let btn = buttons::button(&tokens)
    .icon_only("settings", ButtonSize::Medium)
    .variant(ButtonVariant::FilledTonal)
    .on_press(Message::Settings)
    .build()?;
```

## Migration Steps

1. **Update Imports**
   - Replace imports from `components::common::*` with `components::buttons::*`
   - Import the new `ButtonVariant` and `IconPosition` enums

2. **Replace Button Creation**
   - Replace all direct button function calls with the new builder pattern
   - Use method chaining to configure the button
   - End with `.build()?` to create the button

3. **Handle Errors**
   - The new API returns `Result<Element, ButtonError>`
   - Use `?` to propagate errors or `unwrap()`/`expect()` if you're certain the button is valid

## Common Patterns

### Creating a Simple Button

```rust
// Old
let btn = primary_button_semantic("Click me", Message::Click, &tokens);

// New
let btn = buttons::button(&tokens)
    .label("Click me")
    .variant(ButtonVariant::Filled)
    .on_press(Message::Click)
    .build()?;
```

### Creating a Button with Icon

```rust
// Old
let btn = primary_button_with_icon_semantic(
    "Save", 
    "save", 
    IconPosition::Leading, 
    Message::Save, 
    &tokens
);

// New
let btn = buttons::button(&tokens)
    .label("Save")
    .icon("save", IconPosition::Leading)
    .variant(ButtonVariant::Filled)
    .on_press(Message::Save)
    .build()?;
```

### Creating an Icon-Only Button

```rust
// Old
let btn = filled_icon_button_semantic("settings", ButtonSize::Medium, Message::Settings, &tokens);

// New
let btn = buttons::button(&tokens)
    .icon_only("settings", ButtonSize::Medium)
    .variant(ButtonVariant::Filled)
    .on_press(Message::Settings)
    .build()?;
```

### Creating a Disabled Button

```rust
// Old (not directly supported)


// New
let btn = buttons::button(&tokens)
    .label("Disabled")
    .variant(ButtonVariant::Outlined)
    .disabled()
    .build()?;
```

## Benefits of the New API

1. **Consistency**: Single way to create all button types
2. **Flexibility**: Easy to add new button styles and properties
3. **Type Safety**: Compile-time checking of button configurations
4. **Better Error Handling**: Clear error messages for invalid configurations
5. **IDE Support**: Better code completion and documentation

## Backward Compatibility

The old button functions are marked as deprecated but will continue to work. A deprecation notice will be shown when using them, guiding you to the new API.

## Troubleshooting

### Common Issues

1. **Missing `on_press` handler**
   - Error: "Enabled buttons must have an on_press handler"
   - Solution: Add `.on_press(message)` or `.disabled()`

2. **Missing label and icon**
   - Error: "Button must have either a label, an icon, or both"
   - Solution: Add a label (`.label("text")`), an icon (`.icon("name")`), or both

3. **Unresolved import**
   - Error: "unresolved import"
   - Solution: Update imports to use `components::buttons` instead of `components::common`

## Complete Example

```rust
use iced::Element;
use abop_gui::components::buttons::{self, ButtonVariant, IconPosition};
use abop_gui::styling::material::MaterialTokens;

#[derive(Debug, Clone)]
enum Message {
    Save,
    Export,
    Settings,
}

fn view(tokens: &MaterialTokens) -> Element<Message> {
    // Create a row of buttons
    iced::widget::column![
        // Primary button
        buttons::button(tokens)
            .label("Save")
            .variant(ButtonVariant::Filled)
            .on_press(Message::Save)
            .build()
            .expect("Failed to create save button"),
            
        // Button with icon
        buttons::button(tokens)
            .label("Export")
            .icon("download", IconPosition::Leading)
            .variant(ButtonVariant::Outlined)
            .on_press(Message::Export)
            .build()
            .expect("Failed to create export button"),
            
        // Icon button
        buttons::button(tokens)
            .icon_only("settings", ButtonSize::Medium)
            .variant(ButtonVariant::FilledTonal)
            .on_press(Message::Settings)
            .build()
            .expect("Failed to create settings button")
    ]
    .spacing(10)
    .into()
}
```
