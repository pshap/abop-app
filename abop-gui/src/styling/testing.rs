//! Style testing framework for component styling validation
//!
//! This module provides testing utilities for validating component styles,
//! ensuring consistency, and preventing style regressions.

use crate::styling::material::MaterialTokens;
use crate::styling::{color_utils::ColorUtils, traits::StyleVariant, validation::ThemeValidator};
use crate::theme::ThemeMode;
use iced::{
    Background, Border, Color, Shadow,
    widget::{button, container, text_input},
};

/// Test result for style validation
#[derive(Debug, Clone)]
pub struct StyleTestResult {
    /// Name of the test
    pub test_name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Error message if test failed
    pub error_message: Option<String>,
    /// Additional context or details
    pub details: Vec<String>,
}

impl StyleTestResult {
    /// Create a passing test result
    #[must_use]
    pub fn pass(name: &str) -> Self {
        Self {
            test_name: name.to_string(),
            passed: true,
            error_message: None,
            details: Vec::with_capacity(4), // Pre-allocate for typical test detail count
        }
    }

    /// Create a failing test result
    #[must_use]
    pub fn fail(name: &str, error: &str) -> Self {
        Self {
            test_name: name.to_string(),
            passed: false,
            error_message: Some(error.to_string()),
            details: Vec::with_capacity(4), // Pre-allocate for typical test detail count
        }
    }

    /// Add detail to the test result
    #[must_use]
    pub fn with_detail(mut self, detail: &str) -> Self {
        self.details.push(detail.to_string());
        self
    }
}

/// Test suite for style validation
#[derive(Debug)]
pub struct StyleTestSuite {
    /// Test results
    pub results: Vec<StyleTestResult>,
    /// Theme validator
    validator: ThemeValidator,
}

impl StyleTestSuite {
    /// Create a new style test suite
    #[must_use]
    pub fn new() -> Self {
        Self {
            results: Vec::with_capacity(16), // Pre-allocate for typical test suite size
            validator: ThemeValidator::new(),
        }
    }

    /// Create a strict test suite with enhanced validation
    #[must_use]
    pub fn strict() -> Self {
        Self {
            results: Vec::with_capacity(16), // Pre-allocate for typical test suite size
            validator: ThemeValidator::strict(),
        }
    }

    /// Run all style tests for a theme
    pub fn run_all_tests(&mut self, theme_mode: ThemeMode) {
        self.test_theme_validation(theme_mode);
        self.test_button_styles(theme_mode);
        self.test_input_styles(theme_mode);
        self.test_container_styles(theme_mode);
        self.test_color_consistency(theme_mode);
        self.test_accessibility_compliance(theme_mode);
        self.test_design_token_consistency();
    }

    /// Test theme validation
    pub fn test_theme_validation(&mut self, theme_mode: ThemeMode) {
        let validation_result = self.validator.validate_theme(theme_mode);

        if validation_result.is_valid() {
            self.results.push(
                StyleTestResult::pass("Theme Validation")
                    .with_detail(&format!("Score: {:.2}", validation_result.score)),
            );
        } else {
            let error_count = validation_result.errors.len();
            let warning_count = validation_result.warnings.len();
            self.results.push(
                StyleTestResult::fail(
                    "Theme Validation",
                    &format!("{error_count} errors, {warning_count} warnings"),
                )
                .with_detail(&format!("Score: {:.2}", validation_result.score)),
            );
        }
    }

    /// Test button style consistency
    pub fn test_button_styles(&mut self, theme_mode: ThemeMode) {
        let variants = [
            StyleVariant::Primary,
            StyleVariant::Secondary,
            StyleVariant::Success,
            StyleVariant::Warning,
            StyleVariant::Error,
            StyleVariant::Info,
        ];

        for variant in &variants {
            let style = self.create_test_button_style(*variant, theme_mode);
            let test_name = format!("Button Style - {variant:?}");

            if self.validate_button_style(&style) {
                self.results.push(StyleTestResult::pass(&test_name));
            } else {
                self.results
                    .push(StyleTestResult::fail(&test_name, "Style validation failed"));
            }
        }
    }

    /// Test input style consistency
    pub fn test_input_styles(&mut self, theme_mode: ThemeMode) {
        let variants = [
            StyleVariant::Primary,
            StyleVariant::Success,
            StyleVariant::Warning,
            StyleVariant::Error,
        ];

        for variant in &variants {
            let style = self.create_test_input_style(*variant, theme_mode);
            let test_name = format!("Input Style - {variant:?}");

            if self.validate_input_style(&style) {
                self.results.push(StyleTestResult::pass(&test_name));
            } else {
                self.results
                    .push(StyleTestResult::fail(&test_name, "Style validation failed"));
            }
        }
    }

    /// Test container style consistency
    pub fn test_container_styles(&mut self, theme_mode: ThemeMode) {
        let variants = [
            StyleVariant::Primary,
            StyleVariant::Secondary,
            StyleVariant::Success,
            StyleVariant::Warning,
            StyleVariant::Error,
            StyleVariant::Info,
        ];

        for variant in &variants {
            let style = self.create_test_container_style(*variant, theme_mode);
            let test_name = format!("Container Style - {variant:?}");

            if self.validate_container_style(&style) {
                self.results.push(StyleTestResult::pass(&test_name));
            } else {
                self.results
                    .push(StyleTestResult::fail(&test_name, "Style validation failed"));
            }
        }
    }

    /// Test color consistency across components
    pub fn test_color_consistency(&mut self, theme_mode: ThemeMode) {
        let semantic_colors = theme_mode.semantic_colors();

        // Test that semantic colors are consistent
        let colors = [
            ("Primary", semantic_colors.primary),
            ("Secondary", semantic_colors.secondary),
            ("Success", semantic_colors.success),
            ("Warning", semantic_colors.warning),
            ("Error", semantic_colors.error),
            ("Info", semantic_colors.info),
        ];

        let mut consistent = true;
        for (_name, color) in &colors {
            if color.a == 0.0 {
                consistent = false;
                break;
            }
        }

        if consistent {
            self.results
                .push(StyleTestResult::pass("Color Consistency"));
        } else {
            self.results.push(StyleTestResult::fail(
                "Color Consistency",
                "Some colors are transparent",
            ));
        }
    }

    /// Test accessibility compliance
    pub fn test_accessibility_compliance(&mut self, theme_mode: ThemeMode) {
        let semantic_colors = theme_mode.semantic_colors();

        // Test minimum contrast ratios
        let contrast =
            ColorUtils::contrast_ratio(semantic_colors.on_surface, semantic_colors.surface);

        if contrast >= 4.5 {
            self.results.push(
                StyleTestResult::pass("Accessibility - Contrast")
                    .with_detail(&format!("Contrast ratio: {contrast:.2}")),
            );
        } else {
            self.results.push(StyleTestResult::fail(
                "Accessibility - Contrast",
                &format!("Insufficient contrast: {contrast:.2} (minimum: 4.5)"),
            ));
        }
    }

    /// Test design token consistency
    pub fn test_design_token_consistency(&mut self) {
        let tokens = MaterialTokens::new();

        // Test spacing progression
        let spacing_values = [
            tokens.spacing.xs,
            tokens.spacing.sm,
            tokens.spacing.md,
            tokens.spacing.lg,
            tokens.spacing.xl,
            tokens.spacing.xxl,
        ];

        let mut progressive = true;
        for i in 1..spacing_values.len() {
            if spacing_values[i] <= spacing_values[i - 1] {
                progressive = false;
                break;
            }
        }

        if progressive {
            self.results
                .push(StyleTestResult::pass("Design Token Progression"));
        } else {
            self.results.push(StyleTestResult::fail(
                "Design Token Progression",
                "Spacing tokens do not increase progressively",
            ));
        }
    }

    /// Get test summary
    #[must_use]
    pub fn summary(&self) -> TestSummary {
        let total = self.results.len();
        let passed = self.results.iter().filter(|r| r.passed).count();
        let failed = total.saturating_sub(passed);
        TestSummary {
            total,
            passed,
            failed,
            success_rate: if total > 0 {
                // Safe conversion using f64 to avoid precision loss
                let passed_f64 = passed as f64;
                let total_f64 = total as f64;
                (passed_f64 / total_f64) as f32
            } else {
                0.0
            },
        }
    }

    /// Create a test button style
    fn create_test_button_style(
        &self,
        variant: StyleVariant,
        theme_mode: ThemeMode,
    ) -> button::Style {
        let semantic_colors = theme_mode.semantic_colors();
        let material_tokens = MaterialTokens::new();

        let (bg_color, text_color) = match variant {
            StyleVariant::Primary => (semantic_colors.primary, semantic_colors.on_surface),
            StyleVariant::Secondary => (semantic_colors.secondary, semantic_colors.on_surface),
            StyleVariant::Success => (semantic_colors.success, semantic_colors.on_surface),
            StyleVariant::Warning => (semantic_colors.warning, semantic_colors.on_surface),
            StyleVariant::Error => (semantic_colors.error, semantic_colors.on_surface),
            StyleVariant::Info => (semantic_colors.info, semantic_colors.on_surface),
            _ => (semantic_colors.primary, semantic_colors.on_surface),
        };

        button::Style {
            background: Some(Background::Color(bg_color)),
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: material_tokens.shapes().button().to_radius(),
            },
            shadow: Shadow::default(),
        }
    }

    /// Create a test input style
    fn create_test_input_style(
        &self,
        variant: StyleVariant,
        theme_mode: ThemeMode,
    ) -> text_input::Style {
        let semantic_colors = theme_mode.semantic_colors();
        let material_tokens = MaterialTokens::new();

        let border_color = match variant {
            StyleVariant::Primary => semantic_colors.primary,
            StyleVariant::Success => semantic_colors.success,
            StyleVariant::Warning => semantic_colors.warning,
            StyleVariant::Error => semantic_colors.error,
            _ => semantic_colors.primary,
        };

        text_input::Style {
            background: Background::Color(semantic_colors.surface),
            border: Border {
                color: border_color,
                width: material_tokens.ui().border_width_standard,
                radius: material_tokens.shapes().text_field().to_radius(),
            },
            icon: border_color,
            placeholder: ColorUtils::with_alpha(semantic_colors.on_surface, 0.6),
            value: semantic_colors.on_surface,
            selection: ColorUtils::with_alpha(border_color, 0.2),
        }
    }

    /// Create a test container style
    fn create_test_container_style(
        &self,
        variant: StyleVariant,
        theme_mode: ThemeMode,
    ) -> container::Style {
        let semantic_colors = theme_mode.semantic_colors();
        let material_tokens = MaterialTokens::new();

        let bg_color = match variant {
            StyleVariant::Primary => ColorUtils::with_alpha(semantic_colors.primary, 0.1),
            StyleVariant::Secondary => ColorUtils::with_alpha(semantic_colors.secondary, 0.1),
            StyleVariant::Success => ColorUtils::with_alpha(semantic_colors.success, 0.1),
            StyleVariant::Warning => ColorUtils::with_alpha(semantic_colors.warning, 0.1),
            StyleVariant::Error => ColorUtils::with_alpha(semantic_colors.error, 0.1),
            StyleVariant::Info => ColorUtils::with_alpha(semantic_colors.info, 0.1),
            _ => semantic_colors.surface,
        };

        container::Style {
            text_color: Some(semantic_colors.on_surface),
            background: Some(Background::Color(bg_color)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: material_tokens.shapes().card().to_radius(),
            },
            shadow: Shadow::default(),
        }
    }
    /// Validate button style
    fn validate_button_style(&self, style: &button::Style) -> bool {
        // Check that background and text colors are defined
        style.background.is_some()
            && style.text_color.a > 0.0
            && style.border.radius.top_left >= 0.0
    }

    /// Validate input style
    fn validate_input_style(&self, style: &text_input::Style) -> bool {
        // Check that required properties are defined
        style.value.a > 0.0 && style.placeholder.a > 0.0 && style.border.radius.top_left >= 0.0
    }

    /// Validate container style
    fn validate_container_style(&self, style: &container::Style) -> bool {
        // Check that basic properties are defined
        style.text_color.is_some()
            && style.background.is_some()
            && style.border.radius.top_left >= 0.0
    }
}

impl Default for StyleTestSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Test summary statistics
#[derive(Debug, Clone)]
pub struct TestSummary {
    /// Total number of tests
    pub total: usize,
    /// Number of tests that passed
    pub passed: usize,
    /// Number of tests that failed
    pub failed: usize,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f32,
}

impl TestSummary {
    /// Check if all tests passed
    #[must_use]
    pub const fn all_passed(&self) -> bool {
        self.failed == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_test_suite_creation() {
        let suite = StyleTestSuite::new();
        assert_eq!(suite.results.len(), 0);
    }

    #[test]
    fn test_run_all_tests() {
        let mut suite = StyleTestSuite::new();
        suite.run_all_tests(ThemeMode::Dark);

        assert!(!suite.results.is_empty());
        let summary = suite.summary();
        assert_eq!(summary.total, suite.results.len());
    }

    #[test]
    fn test_test_result_creation() {
        let pass_result = StyleTestResult::pass("Test");
        assert!(pass_result.passed);
        assert!(pass_result.error_message.is_none());

        let fail_result = StyleTestResult::fail("Test", "Error");
        assert!(!fail_result.passed);
        assert!(fail_result.error_message.is_some());
    }
}
