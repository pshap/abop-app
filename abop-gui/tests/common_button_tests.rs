use abop_gui::{
    components::common::{
        button_builder, button_size_to_pixels, fab_size_to_pixels, primary_button_semantic,
        secondary_button_semantic, sizing, tertiary_button,
    },
    styling::material::{
        MaterialTokens,
        components::widgets::{ButtonSize, MaterialButtonVariant},
    },
};

// Mock message type for testing
#[derive(Clone, Debug, PartialEq)]
enum TestMessage {
    Save,
    Cancel,
    Export,
}

#[test]
fn test_button_size_calculations() {
    assert_eq!(button_size_to_pixels(ButtonSize::Small), sizing::SMALL);
    assert_eq!(button_size_to_pixels(ButtonSize::Medium), sizing::MEDIUM);
    assert_eq!(button_size_to_pixels(ButtonSize::Large), sizing::LARGE);
}

#[test]
fn test_fab_size_calculations() {
    assert_eq!(fab_size_to_pixels(ButtonSize::Small), sizing::FAB_SMALL);
    assert_eq!(fab_size_to_pixels(ButtonSize::Medium), sizing::FAB_MEDIUM);
    assert_eq!(fab_size_to_pixels(ButtonSize::Large), sizing::FAB_LARGE);
}

#[test]
fn test_builder_pattern_creation() {
    let tokens = MaterialTokens::default();

    // Test that builder can be created and configured
    let _builder = button_builder::<TestMessage>(&tokens)
        .label("Test")
        .variant(MaterialButtonVariant::Filled)
        .size(ButtonSize::Medium)
        .on_press(TestMessage::Save);

    // Builder should be ready to build (we can't test build() without Element comparison)
    // This test mainly ensures the fluent interface compiles correctly
}

#[test]
fn test_semantic_button_functions() {
    let tokens = MaterialTokens::default();

    // Test primary button creation
    let _primary = primary_button_semantic("Save", TestMessage::Save, &tokens);

    // Test secondary button creation
    let _secondary = secondary_button_semantic("Cancel", TestMessage::Cancel, &tokens);

    // Test tertiary button creation
    let _tertiary = tertiary_button("Help", TestMessage::Export, &tokens);

    // If we reach here, all functions compiled and created elements successfully
}
