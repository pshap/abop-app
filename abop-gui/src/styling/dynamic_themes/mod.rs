//! Dynamic theme loading system for extensible theming
//!
//! This module provides functionality to load themes from external files,
//! enabling runtime theme customization and plugin-based extensions.

pub mod config;
pub mod errors;
pub mod loader;
pub mod overrides;
pub mod serialization;

// Re-export the main types for easier access
pub use config::{CustomThemeMode, Theme, ThemeConfig};
pub use errors::ThemeLoadError;
pub use loader::ThemeLoader;
pub use overrides::{
    ButtonOverride, ComponentOverride, ComponentOverrideBuilder, ComponentOverrides, ComponentType,
    ContainerOverride, InputOverride, MenuOverride, ModalOverride, NavigationOverride,
    ProgressOverride, SelectionOverride,
};
pub use serialization::{
    SerializableMaterialTokens, SerializableSemanticColors, SerializableSpacing,
    SerializableTypography, ThemeMetadata,
};

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Pixels;
    use std::collections::HashMap;

    #[test]
    fn test_color_parsing() {
        assert!(SerializableSemanticColors::parse_color("#FF0000").is_ok());
        assert!(SerializableSemanticColors::parse_color("#FF0000FF").is_ok());
        assert!(SerializableSemanticColors::parse_color("FF0000").is_ok());
        assert!(SerializableSemanticColors::parse_color("#INVALID").is_err());
    }

    #[test]
    fn test_theme_loader_creation() {
        let loader = ThemeLoader::new();
        assert_eq!(loader.theme_cache.len(), 0);
    }

    #[test]
    fn test_theme_config_validation() {
        let config = ThemeConfig {
            metadata: ThemeMetadata {
                name: "Test Theme".to_string(),
                version: "1.0.0".to_string(),
                author: None,
                description: None,
                is_dark: true,
                extends: None,
            },
            semantic_colors: SerializableSemanticColors {
                primary: "#FF0000".to_string(),
                secondary: "#00FF00".to_string(),
                success: "#00AA00".to_string(),
                warning: "#FFAA00".to_string(),
                error: "#AA0000".to_string(),
                info: "#0000FF".to_string(),
                surface: "#333333".to_string(),
                on_surface: "#FFFFFF".to_string(),
            },
            material_tokens: SerializableMaterialTokens {
                spacing: SerializableSpacing {
                    xs: 4.0,
                    sm: 8.0,
                    md: 16.0,
                    lg: 24.0,
                    xl: 32.0,
                    xxl: 48.0,
                },
                typography: SerializableTypography {
                    label_small: 12,
                    label_medium: 14,
                    label_large: 16,
                    body_small: 14,
                    body_medium: 16,
                    body_large: 18,
                    title_small: 18,
                    title_medium: 20,
                    title_large: 24,
                    headline_small: 20,
                    headline_medium: 22,
                    headline_large: 26,
                    display_small: 24,
                    display_medium: 26,
                    display_large: 32,
                },
                radius: HashMap::new(),
                elevation: HashMap::new(),
                sizing: HashMap::new(),
            },
            component_overrides: Vec::new(),
        };

        assert!(ThemeLoader::validate_theme(&config).is_ok());
    }

    #[test]
    fn test_theme_serialization() {
        let theme_config = ThemeConfig {
            metadata: ThemeMetadata {
                name: "Test Theme".to_string(),
                version: "1.0.0".to_string(),
                author: None,
                description: Some("A test theme".to_string()),
                is_dark: false,
                extends: None,
            },
            semantic_colors: SerializableSemanticColors {
                primary: "#FF0000".to_string(),
                secondary: "#00FF00".to_string(),
                success: "#00AA00".to_string(),
                warning: "#FFAA00".to_string(),
                error: "#AA0000".to_string(),
                info: "#0000FF".to_string(),
                surface: "#FFFFFF".to_string(),
                on_surface: "#000000".to_string(),
            },
            material_tokens: SerializableMaterialTokens {
                spacing: SerializableSpacing {
                    xs: 4.0,
                    sm: 8.0,
                    md: 16.0,
                    lg: 24.0,
                    xl: 32.0,
                    xxl: 48.0,
                },
                typography: SerializableTypography {
                    label_small: 12,
                    label_medium: 14,
                    label_large: 16,
                    body_small: 14,
                    body_medium: 16,
                    body_large: 18,
                    title_small: 18,
                    title_medium: 20,
                    title_large: 24,
                    headline_small: 20,
                    headline_medium: 22,
                    headline_large: 26,
                    display_small: 24,
                    display_medium: 26,
                    display_large: 32,
                },
                radius: HashMap::new(),
                elevation: HashMap::new(),
                sizing: HashMap::new(),
            },
            component_overrides: Vec::new(),
        };

        let runtime_theme = theme_config.to_runtime_theme().unwrap();
        assert_eq!(runtime_theme.name, "Test Theme");
        assert_eq!(runtime_theme.description, "A test theme");
        assert_eq!(runtime_theme.material_tokens.spacing.xs, 4.0);
        assert_eq!(runtime_theme.material_tokens.spacing.xxl, 48.0);
    }

    #[test]
    fn test_typography_conversion_consistency() {
        let typography = SerializableTypography {
            label_small: 12,
            label_medium: 14,
            label_large: 16,
            body_small: 14,
            body_medium: 16,
            body_large: 18,
            title_small: 18,
            title_medium: 20,
            title_large: 24,
            headline_small: 20,
            headline_medium: 22,
            headline_large: 26,
            display_small: 24,
            display_medium: 26,
            display_large: 32,
        };

        let material_typography = typography.to_material_typography();

        // Test that font sizes are correctly converted
        assert_eq!(material_typography.display_large.size, Pixels(32.0));
        assert_eq!(material_typography.body_medium.size, Pixels(16.0));
        assert_eq!(material_typography.label_small.size, Pixels(12.0));

        // Test that Material Design 3 specifications are applied
        assert_eq!(material_typography.display_large.line_height, Pixels(64.0));
        assert_eq!(material_typography.title_medium.letter_spacing, 0.15);
        assert_eq!(material_typography.body_large.letter_spacing, 0.5);
    }

    #[test]
    fn test_component_override_system() {
        // Test type-safe component override creation
        let button_override = ComponentOverrideBuilder::new(ComponentType::Button)
            .variant("primary")
            .button_override(ButtonOverride {
                min_height: Some(40.0),
                padding_horizontal: Some(12.0),
                padding_vertical: Some(8.0),
                border_radius: Some(8.0),
                min_width: Some(80.0),
                background_color: Some("#FF0000".to_string()),
                text_color: Some("#FFFFFF".to_string()),
                border_color: Some("#FF0000".to_string()),
                border_width: Some(1.0),
                elevation: Some(1.0),
            });

        // Test validation
        assert!(button_override.validate().is_ok());

        // Test that component type matches override type
        assert_eq!(button_override.component_type, ComponentType::Button);
        assert_eq!(button_override.variant, Some("primary".to_string()));

        if let ComponentOverrides::Button(ref button_props) = button_override.overrides {
            assert_eq!(button_props.background_color, Some("#FF0000".to_string()));
            assert_eq!(button_props.text_color, Some("#FFFFFF".to_string()));
            assert_eq!(button_props.border_radius, Some(8.0));
        } else {
            panic!("Expected Button override, got different type");
        }

        // Test theme config with component overrides
        let theme_config = ThemeConfig {
            metadata: ThemeMetadata {
                name: "Test Theme".to_string(),
                version: "1.0.0".to_string(),
                author: None,
                description: None,
                is_dark: false,
                extends: None,
            },
            semantic_colors: SerializableSemanticColors {
                primary: "#FF0000".to_string(),
                secondary: "#00FF00".to_string(),
                success: "#00AA00".to_string(),
                warning: "#FFAA00".to_string(),
                error: "#AA0000".to_string(),
                info: "#0000FF".to_string(),
                surface: "#FFFFFF".to_string(),
                on_surface: "#000000".to_string(),
            },
            material_tokens: SerializableMaterialTokens {
                spacing: SerializableSpacing {
                    xs: 4.0,
                    sm: 8.0,
                    md: 16.0,
                    lg: 24.0,
                    xl: 32.0,
                    xxl: 48.0,
                },
                typography: SerializableTypography {
                    label_small: 12,
                    label_medium: 14,
                    label_large: 16,
                    body_small: 14,
                    body_medium: 16,
                    body_large: 18,
                    title_small: 18,
                    title_medium: 20,
                    title_large: 24,
                    headline_small: 20,
                    headline_medium: 22,
                    headline_large: 26,
                    display_small: 24,
                    display_medium: 26,
                    display_large: 32,
                },
                radius: HashMap::new(),
                elevation: HashMap::new(),
                sizing: HashMap::new(),
            },
            component_overrides: vec![button_override],
        };

        // Test validation with component overrides
        assert!(ThemeLoader::validate_theme(&theme_config).is_ok());

        // Test runtime theme creation
        let runtime_theme = theme_config.to_runtime_theme().unwrap();
        assert_eq!(runtime_theme.component_overrides().len(), 1);

        // Test finding component override
        let found_override =
            runtime_theme.find_component_override(ComponentType::Button, Some("primary"));
        assert!(found_override.is_some());

        let not_found = runtime_theme.find_component_override(ComponentType::Input, Some("text"));
        assert!(not_found.is_none());
    }

    #[test]
    fn test_component_override_validation() {
        // Test mismatched component type and override type
        let invalid_override = ComponentOverride {
            component_type: ComponentType::Button,
            variant: None,
            overrides: ComponentOverrides::Input(InputOverride {
                height: Some(40.0),
                padding: Some(8.0),
                border_width: Some(1.0),
                focus_border_width: Some(2.0),
                border_radius: Some(4.0),
                background_color: Some("#FFFFFF".to_string()),
                text_color: Some("#000000".to_string()),
                placeholder_color: Some("#999999".to_string()),
                border_color: Some("#CCCCCC".to_string()),
                focus_border_color: Some("#0088FF".to_string()),
            }),
        };

        // This should fail validation because component_type is Button but overrides is Input
        assert!(invalid_override.validate().is_err());
    }
}
