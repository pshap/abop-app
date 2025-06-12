//! Enhanced Material Design 3 Token System
//!
//! This module provides a comprehensive token system that integrates with the unified
//! MaterialColors and provides additional semantic and component-specific tokens.

use crate::styling::material::unified_colors::MaterialColors;
use iced::Color;

/// Complete Material Design 3 token system
#[derive(Debug, Clone)]
pub struct EnhancedMaterialTokens {
    /// Core color tokens
    pub colors: MaterialColors,
    /// Typography tokens
    pub typography: TypographyTokens,
    /// Spacing tokens
    pub spacing: SpacingTokens,
    /// Elevation tokens
    pub elevation: ElevationTokens,
    /// Shape tokens
    pub shapes: ShapeTokens,
    /// Motion tokens
    pub motion: MotionTokens,
    /// State layer tokens
    pub state_layers: StateLayerTokens,
}

impl EnhancedMaterialTokens {
    /// Create tokens for light theme
    pub fn light() -> Self {
        Self {
            colors: MaterialColors::light_default(),
            typography: TypographyTokens::default(),
            spacing: SpacingTokens::default(),
            elevation: ElevationTokens::default(),
            shapes: ShapeTokens::default(),
            motion: MotionTokens::default(),
            state_layers: StateLayerTokens::light(),
        }
    }

    /// Create tokens for dark theme
    pub fn dark() -> Self {
        Self {
            colors: MaterialColors::dark_default(),
            typography: TypographyTokens::default(),
            spacing: SpacingTokens::default(),
            elevation: ElevationTokens::default(),
            shapes: ShapeTokens::default(),
            motion: MotionTokens::default(),
            state_layers: StateLayerTokens::dark(),
        }
    }

    /// Create tokens from a seed color
    pub fn from_seed(seed: Color, is_dark: bool) -> Self {
        Self {
            colors: MaterialColors::from_seed(seed, is_dark),
            typography: TypographyTokens::default(),
            spacing: SpacingTokens::default(),
            elevation: ElevationTokens::default(),
            shapes: ShapeTokens::default(),
            motion: MotionTokens::default(),
            state_layers: if is_dark {
                StateLayerTokens::dark()
            } else {
                StateLayerTokens::light()
            },
        }
    }

    /// Get component-specific tokens for buttons
    pub fn button_tokens(&self) -> ButtonTokens {
        ButtonTokens::new(&self.colors, &self.state_layers, &self.shapes)
    }

    /// Get component-specific tokens for text fields
    pub fn text_field_tokens(&self) -> TextFieldTokens {
        TextFieldTokens::new(&self.colors, &self.state_layers, &self.shapes)
    }

    /// Get component-specific tokens for selection components
    pub fn selection_tokens(&self) -> SelectionTokens {
        SelectionTokens::new(&self.colors, &self.state_layers)
    }
}

/// Typography tokens following Material Design 3 type scale
#[derive(Debug, Clone)]
pub struct TypographyTokens {
    /// Display large (57sp)
    pub display_large: f32,
    /// Display medium (45sp)
    pub display_medium: f32,
    /// Display small (36sp)
    pub display_small: f32,
    /// Headline large (32sp)
    pub headline_large: f32,
    /// Headline medium (28sp)
    pub headline_medium: f32,
    /// Headline small (24sp)
    pub headline_small: f32,
    /// Title large (22sp)
    pub title_large: f32,
    /// Title medium (16sp)
    pub title_medium: f32,
    /// Title small (14sp)
    pub title_small: f32,
    /// Body large (16sp)
    pub body_large: f32,
    /// Body medium (14sp)
    pub body_medium: f32,
    /// Body small (12sp)
    pub body_small: f32,
    /// Label large (14sp)
    pub label_large: f32,
    /// Label medium (12sp)
    pub label_medium: f32,
    /// Label small (11sp)
    pub label_small: f32,
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            display_large: 57.0,
            display_medium: 45.0,
            display_small: 36.0,
            headline_large: 32.0,
            headline_medium: 28.0,
            headline_small: 24.0,
            title_large: 22.0,
            title_medium: 16.0,
            title_small: 14.0,
            body_large: 16.0,
            body_medium: 14.0,
            body_small: 12.0,
            label_large: 14.0,
            label_medium: 12.0,
            label_small: 11.0,
        }
    }
}

/// Spacing tokens following Material Design 3 spacing system
#[derive(Debug, Clone)]
pub struct SpacingTokens {
    /// Extra small spacing (4dp)
    pub xs: f32,
    /// Small spacing (8dp)
    pub sm: f32,
    /// Medium spacing (12dp)
    pub md: f32,
    /// Large spacing (16dp)
    pub lg: f32,
    /// Extra large spacing (24dp)
    pub xl: f32,
    /// Extra extra large spacing (32dp)
    pub xxl: f32,
}

impl Default for SpacingTokens {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            xxl: 32.0,
        }
    }
}

/// Elevation tokens for Material Design 3 elevation system
#[derive(Debug, Clone)]
pub struct ElevationTokens {
    /// Level 0 (0dp) - No elevation
    pub level0: f32,
    /// Level 1 (1dp) - Low elevation
    pub level1: f32,
    /// Level 2 (3dp) - Medium-low elevation
    pub level2: f32,
    /// Level 3 (6dp) - Medium elevation
    pub level3: f32,
    /// Level 4 (8dp) - Medium-high elevation
    pub level4: f32,
    /// Level 5 (12dp) - High elevation
    pub level5: f32,
}

impl Default for ElevationTokens {
    fn default() -> Self {
        Self {
            level0: 0.0,
            level1: 1.0,
            level2: 3.0,
            level3: 6.0,
            level4: 8.0,
            level5: 12.0,
        }
    }
}

/// Shape tokens for Material Design 3 shape system
#[derive(Debug, Clone)]
pub struct ShapeTokens {
    /// Extra small corner radius (4dp)
    pub corner_xs: f32,
    /// Small corner radius (8dp)
    pub corner_sm: f32,
    /// Medium corner radius (12dp)
    pub corner_md: f32,
    /// Large corner radius (16dp)
    pub corner_lg: f32,
    /// Extra large corner radius (28dp)
    pub corner_xl: f32,
    /// Full corner radius (50% of height)
    pub corner_full: f32,
}

impl Default for ShapeTokens {
    fn default() -> Self {
        Self {
            corner_xs: 4.0,
            corner_sm: 8.0,
            corner_md: 12.0,
            corner_lg: 16.0,
            corner_xl: 28.0,
            corner_full: 1000.0, // Large value to ensure full rounding
        }
    }
}

/// Motion tokens for Material Design 3 motion system
#[derive(Debug, Clone)]
pub struct MotionTokens {
    /// Short duration (150ms)
    pub duration_short: u64,
    /// Medium duration (250ms)
    pub duration_medium: u64,
    /// Long duration (400ms)
    pub duration_long: u64,
    /// Extra long duration (700ms)
    pub duration_extra_long: u64,
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            duration_short: 150,
            duration_medium: 250,
            duration_long: 400,
            duration_extra_long: 700,
        }
    }
}

/// State layer tokens for interaction states
#[derive(Debug, Clone)]
pub struct StateLayerTokens {
    /// Hover state opacity
    pub hover_opacity: f32,
    /// Focus state opacity
    pub focus_opacity: f32,
    /// Pressed state opacity
    pub pressed_opacity: f32,
    /// Dragged state opacity
    pub dragged_opacity: f32,
    /// Disabled state opacity
    pub disabled_opacity: f32,
}

impl StateLayerTokens {
    /// Create state layer tokens for light theme
    pub fn light() -> Self {
        Self {
            hover_opacity: 0.08,
            focus_opacity: 0.12,
            pressed_opacity: 0.12,
            dragged_opacity: 0.16,
            disabled_opacity: 0.12,
        }
    }

    /// Create state layer tokens for dark theme
    pub fn dark() -> Self {
        Self {
            hover_opacity: 0.08,
            focus_opacity: 0.12,
            pressed_opacity: 0.12,
            dragged_opacity: 0.16,
            disabled_opacity: 0.12,
        }
    }
}

/// Component-specific tokens for buttons
#[derive(Debug, Clone)]
pub struct ButtonTokens {
    /// Button primary colors
    pub primary: ButtonColorSet,
    /// Button secondary colors
    pub secondary: ButtonColorSet,
    /// Button tertiary colors
    pub tertiary: ButtonColorSet,
    /// Button shapes
    pub shapes: ButtonShapes,
}

impl ButtonTokens {
    fn new(colors: &MaterialColors, state_layers: &StateLayerTokens, shapes: &ShapeTokens) -> Self {
        Self {
            primary: ButtonColorSet {
                container: colors.primary.base,
                on_container: colors.primary.on_base,
                hover_overlay: Color {
                    a: state_layers.hover_opacity,
                    ..colors.primary.on_base
                },
                pressed_overlay: Color {
                    a: state_layers.pressed_opacity,
                    ..colors.primary.on_base
                },
            },
            secondary: ButtonColorSet {
                container: colors.secondary.base,
                on_container: colors.secondary.on_base,
                hover_overlay: Color {
                    a: state_layers.hover_opacity,
                    ..colors.secondary.on_base
                },
                pressed_overlay: Color {
                    a: state_layers.pressed_opacity,
                    ..colors.secondary.on_base
                },
            },
            tertiary: ButtonColorSet {
                container: colors.tertiary.base,
                on_container: colors.tertiary.on_base,
                hover_overlay: Color {
                    a: state_layers.hover_opacity,
                    ..colors.tertiary.on_base
                },
                pressed_overlay: Color {
                    a: state_layers.pressed_opacity,
                    ..colors.tertiary.on_base
                },
            },
            shapes: ButtonShapes {
                corner_radius: shapes.corner_lg,
            },
        }
    }
}

/// Color set for button variants
#[derive(Debug, Clone)]
pub struct ButtonColorSet {
    /// Container background color for the button
    pub container: Color,
    /// Text/icon color that appears on the container
    pub on_container: Color,
    /// Overlay color applied on hover interactions
    pub hover_overlay: Color,
    /// Overlay color applied when button is pressed
    pub pressed_overlay: Color,
}

/// Shape tokens for buttons
#[derive(Debug, Clone)]
pub struct ButtonShapes {
    /// Corner radius for button borders in logical pixels
    pub corner_radius: f32,
}

/// Component-specific tokens for text fields
#[derive(Debug, Clone)]
pub struct TextFieldTokens {
    /// Text field colors
    pub colors: TextFieldColors,
    /// Text field shapes
    pub shapes: TextFieldShapes,
}

impl TextFieldTokens {
    fn new(colors: &MaterialColors, state_layers: &StateLayerTokens, shapes: &ShapeTokens) -> Self {
        Self {
            colors: TextFieldColors {
                container: colors.surface_variant,
                on_container: colors.on_surface_variant,
                outline: colors.outline,
                outline_focused: colors.primary.base,
                error: colors.error.base,
                hover_overlay: Color {
                    a: state_layers.hover_opacity,
                    ..colors.on_surface
                },
            },
            shapes: TextFieldShapes {
                corner_radius: shapes.corner_sm,
            },
        }
    }
}

/// Color tokens for text fields
#[derive(Debug, Clone)]
pub struct TextFieldColors {
    /// Background color for the text field container
    pub container: Color,
    /// Text color that appears within the container
    pub on_container: Color,
    /// Default outline color for text field borders
    pub outline: Color,
    /// Outline color when text field is focused
    pub outline_focused: Color,
    /// Error state color for validation failures
    pub error: Color,
    /// Overlay color applied on hover interactions
    pub hover_overlay: Color,
}

/// Shape tokens for text fields
#[derive(Debug, Clone)]
pub struct TextFieldShapes {
    /// Corner radius for text field borders in logical pixels
    pub corner_radius: f32,
}

/// Component-specific tokens for selection components
#[derive(Debug, Clone)]
pub struct SelectionTokens {
    /// Selection colors
    pub colors: SelectionColors,
    /// State layer tokens
    pub state_layers: StateLayerTokens,
}

impl SelectionTokens {
    fn new(colors: &MaterialColors, state_layers: &StateLayerTokens) -> Self {
        Self {
            colors: SelectionColors {
                selected: colors.primary.base,
                on_selected: colors.primary.on_base,
                unselected: Color::TRANSPARENT,
                on_unselected: colors.on_surface_variant,
                disabled: Color {
                    a: state_layers.disabled_opacity,
                    ..colors.on_surface
                },
            },
            state_layers: state_layers.clone(),
        }
    }
}

/// Color tokens for selection components
#[derive(Debug, Clone)]
pub struct SelectionColors {
    /// Background color when the selection component is selected
    pub selected: Color,
    /// Text/icon color that appears on selected background
    pub on_selected: Color,
    /// Background color when the selection component is unselected
    pub unselected: Color,
    /// Text/icon color that appears on unselected background
    pub on_unselected: Color,
    /// Color used when the selection component is disabled
    pub disabled: Color,
}

/// Theme builder for creating custom token sets
pub struct ThemeBuilder {
    colors: Option<MaterialColors>,
    typography: Option<TypographyTokens>,
    spacing: Option<SpacingTokens>,
    elevation: Option<ElevationTokens>,
    shapes: Option<ShapeTokens>,
    motion: Option<MotionTokens>,
}

impl ThemeBuilder {
    /// Create a new theme builder
    pub fn new() -> Self {
        Self {
            colors: None,
            typography: None,
            spacing: None,
            elevation: None,
            shapes: None,
            motion: None,
        }
    }

    /// Set custom colors
    pub fn with_colors(mut self, colors: MaterialColors) -> Self {
        self.colors = Some(colors);
        self
    }

    /// Set custom typography
    pub fn with_typography(mut self, typography: TypographyTokens) -> Self {
        self.typography = Some(typography);
        self
    }

    /// Set custom spacing
    pub fn with_spacing(mut self, spacing: SpacingTokens) -> Self {
        self.spacing = Some(spacing);
        self
    }

    /// Build the final token set
    pub fn build(self, is_dark: bool) -> EnhancedMaterialTokens {
        let base = if is_dark {
            EnhancedMaterialTokens::dark()
        } else {
            EnhancedMaterialTokens::light()
        };

        EnhancedMaterialTokens {
            colors: self.colors.unwrap_or(base.colors),
            typography: self.typography.unwrap_or(base.typography),
            spacing: self.spacing.unwrap_or(base.spacing),
            elevation: self.elevation.unwrap_or(base.elevation),
            shapes: self.shapes.unwrap_or(base.shapes),
            motion: self.motion.unwrap_or(base.motion),
            state_layers: base.state_layers,
        }
    }
}

impl Default for ThemeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_tokens_creation() {
        let tokens = EnhancedMaterialTokens::light();

        // Verify all token categories are present
        assert!(tokens.typography.body_medium > 0.0);
        assert!(tokens.spacing.md > 0.0);
        assert!(tokens.elevation.level3 > 0.0);
    }

    #[test]
    fn test_component_tokens() {
        let tokens = EnhancedMaterialTokens::light();
        let button_tokens = tokens.button_tokens();

        // Button tokens should be valid
        assert_ne!(button_tokens.primary.container, Color::TRANSPARENT);
        assert!(button_tokens.shapes.corner_radius > 0.0);
    }

    #[test]
    fn test_theme_builder() {
        let custom_colors = MaterialColors::from_seed(Color::from_rgb(0.2, 0.6, 0.9), false);
        let tokens = ThemeBuilder::new()
            .with_colors(custom_colors.clone())
            .build(false);

        assert_eq!(tokens.colors.primary.base, custom_colors.primary.base);
    }

    #[test]
    fn test_state_layer_opacity() {
        let state_layers = StateLayerTokens::light();

        // All opacities should be reasonable values
        assert!(state_layers.hover_opacity > 0.0 && state_layers.hover_opacity < 1.0);
        assert!(state_layers.focus_opacity > 0.0 && state_layers.focus_opacity < 1.0);
    }
}
