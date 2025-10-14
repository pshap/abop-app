//! Tests for `about` component

use crate::components::about::AboutView;
use crate::theme::ThemeMode;

#[test]
fn test_about_view_component_creation() {
    let element = AboutView::view(ThemeMode::Light);
    let _ = element; // Just verify it compiles and runs
}

#[test]
fn test_about_view_with_different_themes() {
    // Test light theme
    let light_element = AboutView::view(ThemeMode::Light);
    let _ = light_element; // Just verify it compiles and runs

    // Test dark theme
    let dark_element = AboutView::view(ThemeMode::Dark);
    let _ = dark_element; // Just verify it compiles and runs
}
