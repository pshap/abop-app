//! Theme validation system for accessibility and consistency checks
//!
//! This module provides validation utilities to ensure themes meet
//! accessibility guidelines and maintain consistency across components.

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::SemanticColors;
use crate::theme::ThemeMode;
use iced::Color;

/// Theme validation result containing errors and warnings
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Critical errors that must be fixed
    pub errors: Vec<ValidationError>,
    /// Warnings that should be addressed
    pub warnings: Vec<ValidationWarning>,
    /// Overall validation score (0.0 to 1.0)
    pub score: f32,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationResult {
    /// Create a new validation result
    #[must_use]
    pub const fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            score: 1.0,
        }
    }

    /// Check if validation passed (no errors)
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Add an error to the result
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        self.update_score();
    }

    /// Add a warning to the result
    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
        self.update_score();
    }

    /// Update the validation score based on errors and warnings
    fn update_score(&mut self) {
        let error_penalty = self.errors.len().saturating_mul(2) as f32 * 0.1;
        let warning_penalty = self.warnings.len().saturating_mul(1) as f32 * 0.05;
        self.score = (1.0 - error_penalty - warning_penalty).max(0.0);
    }
}

/// Theme validation error types
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Insufficient contrast ratio between colors
    InsufficientContrast {
        /// Foreground color
        foreground: Color,
        /// Background color
        background: Color,
        /// Actual contrast ratio
        ratio: f32,
        /// Minimum required contrast ratio
        minimum: f32,
        /// Context where this error occurs
        context: String,
    },
    /// Color is too similar to another required color
    ColorSimilarity {
        /// First color
        color1: Color,
        /// Second color
        color2: Color,
        /// Context where this error occurs
        context: String,
    },
    /// Missing required color definition
    MissingColor {
        /// Name of the missing color
        color_name: String,
        /// Context where this error occurs
        context: String,
    },
}

/// Theme validation warning types
#[derive(Debug, Clone)]
pub enum ValidationWarning {
    /// Contrast could be improved
    SuboptimalContrast {
        /// Foreground color
        foreground: Color,
        /// Background color
        background: Color,
        /// Current contrast ratio
        ratio: f32,
        /// Recommended contrast ratio
        recommended: f32,
        /// Context where this warning occurs
        context: String,
    },
    /// Color brightness may cause eye strain
    BrightnessWarning {
        /// Color that may cause eye strain
        color: Color,
        /// Context where this warning occurs
        context: String,
    },
    /// Consistent spacing recommended
    SpacingInconsistency {
        /// Current spacing values
        values: Vec<f32>,
        /// Context where this warning occurs
        context: String,
    },
}

/// Theme validator for checking accessibility and consistency
#[derive(Debug)]
pub struct ThemeValidator {
    /// Minimum contrast ratio for normal text (WCAG AA)
    min_contrast_normal: f32,
    /// Minimum contrast ratio for large text (WCAG AA)
    min_contrast_large: f32,
    /// Enhanced contrast ratio for better accessibility (WCAG AAA)
    enhanced_contrast: f32,
}

impl ThemeValidator {
    /// Create a new theme validator with WCAG guidelines
    #[must_use]
    pub const fn new() -> Self {
        Self {
            min_contrast_normal: 4.5,
            min_contrast_large: 3.0,
            enhanced_contrast: 7.0,
        }
    }

    /// Create a strict validator with enhanced accessibility requirements
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            min_contrast_normal: 7.0,
            min_contrast_large: 4.5,
            enhanced_contrast: 9.0,
        }
    }

    /// Validate a complete theme
    #[must_use]
    pub fn validate_theme(&self, theme_mode: ThemeMode) -> ValidationResult {
        let mut result = ValidationResult::new();

        let semantic_colors = theme_mode.semantic_colors();

        // Validate semantic colors
        self.validate_semantic_colors(&semantic_colors, &mut result);

        // Validate contrast ratios
        self.validate_contrast_ratios(&semantic_colors, &mut result);

        // Validate color distinctiveness
        self.validate_color_distinctiveness(&semantic_colors, &mut result);

        result
    }

    /// Validate semantic color definitions
    fn validate_semantic_colors(&self, colors: &SemanticColors, result: &mut ValidationResult) {
        // Check for required colors
        let required_colors = [
            ("primary", colors.primary),
            ("secondary", colors.secondary),
            ("success", colors.success),
            ("warning", colors.warning),
            ("error", colors.error),
            ("info", colors.info),
            ("surface", colors.surface),
            ("on_surface", colors.on_surface),
        ];

        for (name, color) in &required_colors {
            if color.r == 0.0 && color.g == 0.0 && color.b == 0.0 && color.a == 0.0 {
                result.add_error(ValidationError::MissingColor {
                    color_name: (*name).to_string(),
                    context: "Semantic colors".to_string(),
                });
            }
        }
    }

    /// Validate contrast ratios for accessibility
    fn validate_contrast_ratios(&self, colors: &SemanticColors, result: &mut ValidationResult) {
        // Text on surface contrast
        let surface_contrast = ColorUtils::contrast_ratio(colors.on_surface, colors.surface);
        if surface_contrast < self.min_contrast_normal {
            result.add_error(ValidationError::InsufficientContrast {
                foreground: colors.on_surface,
                background: colors.surface,
                ratio: surface_contrast,
                minimum: self.min_contrast_normal,
                context: "Text on surface".to_string(),
            });
        } else if surface_contrast < self.enhanced_contrast {
            result.add_warning(ValidationWarning::SuboptimalContrast {
                foreground: colors.on_surface,
                background: colors.surface,
                ratio: surface_contrast,
                recommended: self.enhanced_contrast,
                context: "Text on surface".to_string(),
            });
        }

        // Check semantic color contrasts against surface
        let semantic_checks = [
            ("Primary", colors.primary),
            ("Secondary", colors.secondary),
            ("Success", colors.success),
            ("Warning", colors.warning),
            ("Error", colors.error),
            ("Info", colors.info),
        ];

        for (name, color) in &semantic_checks {
            let contrast = ColorUtils::contrast_ratio(*color, colors.surface);
            if contrast < self.min_contrast_large {
                result.add_warning(ValidationWarning::SuboptimalContrast {
                    foreground: *color,
                    background: colors.surface,
                    ratio: contrast,
                    recommended: self.min_contrast_large,
                    context: format!("{name} on surface"),
                });
            }
        }
    }

    /// Validate color distinctiveness to avoid confusion
    fn validate_color_distinctiveness(
        &self,
        colors: &SemanticColors,
        result: &mut ValidationResult,
    ) {
        let color_pairs = [
            ("Success", colors.success, "Error", colors.error),
            ("Warning", colors.warning, "Error", colors.error),
            ("Primary", colors.primary, "Secondary", colors.secondary),
            ("Info", colors.info, "Primary", colors.primary),
        ];

        for (name1, color1, name2, color2) in &color_pairs {
            let similarity = self.color_similarity(*color1, *color2);
            if similarity > 0.9 {
                result.add_error(ValidationError::ColorSimilarity {
                    color1: *color1,
                    color2: *color2,
                    context: format!("{name1} and {name2} are too similar"),
                });
            } else if similarity > 0.7 {
                result.add_warning(ValidationWarning::BrightnessWarning {
                    color: *color1,
                    context: format!("{name1} and {name2} may be confused"),
                });
            }
        }
    }

    /// Calculate color similarity (0.0 = completely different, 1.0 = identical)
    fn color_similarity(&self, color1: Color, color2: Color) -> f32 {
        let dr = (color1.r - color2.r).abs();
        let dg = (color1.g - color2.g).abs();
        let db = (color1.b - color2.b).abs();
        let da = (color1.a - color2.a).abs();

        1.0 - ((dr + dg + db + da) / 4.0)
    }

    /// Check if a color is effectively zero (all components near zero)
    #[must_use]
    pub fn is_zero_color(color: &Color) -> bool {
        (color.r.abs() < f32::EPSILON)
            && (color.g.abs() < f32::EPSILON)
            && (color.b.abs() < f32::EPSILON)
            && (color.a.abs() < f32::EPSILON)
    }
}

impl Default for ThemeValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_creation() {
        let result = ValidationResult::new();
        assert!(result.is_valid());
        assert_eq!(result.score, 1.0);
    }
    #[test]
    fn test_contrast_validation() {
        let validator = ThemeValidator::new();
        let theme_mode = ThemeMode::Dark;
        let result = validator.validate_theme(theme_mode);

        // Validation should complete successfully (this test verifies the function runs without panic)
        // The result may or may not have errors/warnings depending on the theme
        assert!(result.score >= 0.0 && result.score <= 1.0);
    }

    #[test]
    fn test_color_similarity() {
        let validator = ThemeValidator::new();
        let color1 = Color::from_rgb(1.0, 0.0, 0.0);
        let color2 = Color::from_rgb(1.0, 0.0, 0.0);
        let color3 = Color::from_rgb(0.0, 1.0, 0.0);
        let color4 = Color::from_rgb(0.0, 0.0, 1.0);

        // Identical colors should have perfect similarity
        assert_eq!(validator.color_similarity(color1, color2), 1.0);

        // Different colors should have lower similarity
        assert!(validator.color_similarity(color1, color3) <= 0.5);

        // Very different colors should have low similarity
        assert!(validator.color_similarity(color1, color4) <= 0.5);

        // Similar colors should have higher similarity
        let color_red_similar = Color::from_rgb(0.9, 0.1, 0.1);
        assert!(validator.color_similarity(color1, color_red_similar) > 0.8);
    }
}
