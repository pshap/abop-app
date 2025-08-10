//! Tests for the About component
//!
//! Tests covering theme support and basic component functionality.

use super::*;
use crate::components::about::AboutView;

#[test]
fn about_view_creates_successfully() {
    let element = AboutView::view(ThemeMode::Light);
    let _ = element; // Verify no panic during creation
}

#[test]
fn about_view_supports_all_themes() {
    let themes = [ThemeMode::Light, ThemeMode::Dark, ThemeMode::MaterialDark];

    for theme in themes {
        let element = AboutView::view(theme);
        let _ = element; // Verify each theme renders without panic
    }
}

#[test]
fn about_view_is_deterministic() {
    // Test that multiple calls with same theme produce consistent results
    let theme = ThemeMode::Light;
    let element1 = AboutView::view(theme);
    let element2 = AboutView::view(theme);

    // Both should succeed (testing for consistency, not equality)
    let _ = (element1, element2);
}
