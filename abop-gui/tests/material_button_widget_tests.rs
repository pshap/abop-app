//! Tests for Material Design 3 Button Widget Implementation
//!
//! This test file verifies that the MaterialButton widget compiles correctly,
//! can be created with proper Material Design tokens, and integrates with Iced.

use abop_gui::components::common::material_icon_button_widget;
use abop_gui::styling::material::components::widgets::MaterialButtonVariant;
use abop_gui::styling::material::{
    components::{ButtonSize, MaterialButton},
    tokens::core::MaterialTokens,
};
use iced::advanced::Widget;
use iced::{Element, Length};

#[derive(Debug, Clone)]
enum TestMessage {
    ButtonPressed,
}

#[test]
fn test_material_button_creation() {
    // Create material tokens for testing
    let tokens = MaterialTokens::default();
    // Test creating a basic MaterialButton with explicit type annotation
    let button: MaterialButton<'_, TestMessage> = MaterialButton::new("Test Button", &tokens);

    // Verify basic properties are set correctly
    assert_eq!(button.size().width, Length::Shrink);
    assert_eq!(button.size().height, Length::Fixed(40.0));
}

#[test]
fn test_material_button_variants() {
    let tokens = MaterialTokens::default();
    // Test different button variants can be created
    let _filled_button: MaterialButton<'_, TestMessage> =
        MaterialButton::new("Filled", &tokens).variant(MaterialButtonVariant::Filled);

    let _outlined_button: MaterialButton<'_, TestMessage> =
        MaterialButton::new("Outlined", &tokens).variant(MaterialButtonVariant::Outlined);

    let _text_button: MaterialButton<'_, TestMessage> =
        MaterialButton::new("Text", &tokens).variant(MaterialButtonVariant::Text);

    // Test passes if no compilation errors
}

#[test]
fn test_material_button_methods() {
    let tokens = MaterialTokens::default();
    // Test builder pattern methods
    let button: MaterialButton<'_, TestMessage> = MaterialButton::new("Test", &tokens)
        .width(Length::Fixed(200.0))
        .height(Length::Fixed(50.0))
        .on_press(TestMessage::ButtonPressed)
        .disabled();

    // Verify size properties
    assert_eq!(button.size().width, Length::Fixed(200.0));
    assert_eq!(button.size().height, Length::Fixed(50.0));

    // Test passes if no compilation errors and correct sizes
}

#[test]
fn test_material_button_element_conversion() {
    let tokens = MaterialTokens::default();
    // Test that MaterialButton can be converted to Element
    let button: MaterialButton<'_, TestMessage> = MaterialButton::new("Test", &tokens);
    let _element: Element<TestMessage> = button.into();

    // Test passes if no compilation errors
}

#[test]
fn test_material_icon_button_function() {
    let tokens = MaterialTokens::default();

    // Test that the material_icon_button_widget function works
    let _icon_button = material_icon_button_widget(
        "play_arrow",
        MaterialButtonVariant::Filled,
        ButtonSize::Medium,
        TestMessage::ButtonPressed,
        &tokens,
    );

    // Test passes if no compilation errors
}

#[test]
fn test_button_variant_enum() {
    // Test that MaterialButtonVariant enum values are accessible
    let _filled = MaterialButtonVariant::Filled;
    let _tonal = MaterialButtonVariant::FilledTonal;
    let _outlined = MaterialButtonVariant::Outlined;
    let _text = MaterialButtonVariant::Text;
    let _elevated = MaterialButtonVariant::Elevated;

    // Test passes if no compilation errors
}

#[test]
fn test_button_size_enum() {
    // Test that ButtonSize enum values are accessible
    let _small = ButtonSize::Small;
    let _medium = ButtonSize::Medium;
    let _large = ButtonSize::Large;

    // Test passes if no compilation errors
}
