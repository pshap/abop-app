//! Dynamic theme loading system for extensible theming
//!
//! This module provides functionality to load themes from external files,
//! enabling runtime theme customization and plugin-based extensions.

use crate::styling::material::{
    spacing::SpacingTokens,
    tokens::{core::MaterialTokens, semantic::SemanticColors},
    typography::{
        MaterialTypography, TypeStyle,
        font_mapping::{MaterialFont, MaterialWeight},
    },
};
use iced::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Material Design 3 typography specifications
/// Line heights and letter spacing values according to MD3 specification
/// Values based on official Google Material Design 3 typescale v0.192
const MD3_TYPOGRAPHY_SPECS: [(MaterialFont, MaterialWeight, f32, f32); 15] = [
    // Display styles - Brand font, Regular weight (sizes: 57px, 45px, 36px)
    (MaterialFont::Brand, MaterialWeight::Regular, 64.0, -0.25),  // display_large (4rem line, -0.015625rem tracking)
    (MaterialFont::Brand, MaterialWeight::Regular, 52.0, 0.0),    // display_medium (3.25rem line, 0rem tracking)
    (MaterialFont::Brand, MaterialWeight::Regular, 44.0, 0.0),    // display_small (2.75rem line, 0rem tracking)
    // Headline styles - Brand font, Regular weight (sizes: 32px, 28px, 24px)
    (MaterialFont::Brand, MaterialWeight::Regular, 40.0, 0.0),    // headline_large (2.5rem line, 0rem tracking)
    (MaterialFont::Brand, MaterialWeight::Regular, 36.0, 0.0),    // headline_medium (2.25rem line, 0rem tracking)
    (MaterialFont::Brand, MaterialWeight::Regular, 32.0, 0.0),    // headline_small (2rem line, 0rem tracking)
    // Title styles - Mixed fonts (sizes: 22px, 16px, 14px)
    (MaterialFont::Brand, MaterialWeight::Regular, 28.0, 0.0),    // title_large (1.75rem line, 0rem tracking)
    (MaterialFont::Plain, MaterialWeight::Medium, 24.0, 0.15),    // title_medium (1.5rem line, 0.009375rem tracking)
    (MaterialFont::Plain, MaterialWeight::Medium, 20.0, 0.1),     // title_small (1.25rem line, 0.00625rem tracking)
    // Label styles - Plain font, Medium weight (sizes: 14px, 12px, 11px)
    (MaterialFont::Plain, MaterialWeight::Medium, 20.0, 0.1),     // label_large (1.25rem line, 0.00625rem tracking)
    (MaterialFont::Plain, MaterialWeight::Medium, 16.0, 0.5),     // label_medium (1rem line, 0.03125rem tracking)
    (MaterialFont::Plain, MaterialWeight::Medium, 16.0, 0.5),     // label_small (1rem line, 0.03125rem tracking)
    // Body styles - Plain font, Regular weight (sizes: 16px, 14px, 12px)
    (MaterialFont::Plain, MaterialWeight::Regular, 24.0, 0.5),    // body_large (1.5rem line, 0.03125rem tracking)
    (MaterialFont::Plain, MaterialWeight::Regular, 20.0, 0.25),   // body_medium (1.25rem line, 0.015625rem tracking)
    (MaterialFont::Plain, MaterialWeight::Regular, 16.0, 0.4),    // body_small (1rem line, 0.025rem tracking)
];

/// Unified theme configuration for loading and runtime use
/// 
/// This unified structure eliminates redundancy between ThemeConfig and SerializableTheme
/// while maintaining compatibility with both file loading and runtime theme management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Theme metadata
    pub metadata: ThemeMetadata,
    /// Semantic color definitions
    pub semantic_colors: SerializableSemanticColors,
    /// Material Design tokens - unified structure
    pub material_tokens: SerializableMaterialTokens,
    /// Type-safe component overrides
    pub component_overrides: Vec<ComponentOverride>,
}

impl ThemeConfig {
    /// Convert theme configuration to runtime theme
    /// 
    /// This method provides direct conversion from the unified ThemeConfig
    /// to the runtime Theme structure, eliminating intermediate conversions.
    pub fn to_runtime_theme(&self) -> Result<Theme, ThemeLoadError> {
        // Validate component overrides
        for override_config in &self.component_overrides {
            override_config.validate().map_err(|e| {
                ThemeLoadError::ValidationError(format!("Component override validation failed: {}", e))
            })?;
        }

        Ok(Theme {
            name: self.metadata.name.clone(),
            description: self.metadata.description.clone().unwrap_or_default(),
            material_tokens: Self::convert_to_material_tokens(&self.material_tokens)?,
            component_overrides: self.component_overrides.clone(),
        })
    }

    /// Convert serializable material tokens to runtime tokens
    fn convert_to_material_tokens(
        tokens: &SerializableMaterialTokens,
    ) -> Result<MaterialTokens, ThemeLoadError> {
        let material_tokens = MaterialTokens {
            spacing: SpacingTokens {
                xs: tokens.spacing.xs,
                sm: tokens.spacing.sm,
                md: tokens.spacing.md,
                lg: tokens.spacing.lg,
                xl: tokens.spacing.xl,
                xxl: tokens.spacing.xxl,
            },
            typography: tokens.typography.to_material_typography(),
            ..Default::default()
        };

        // TODO: Add conversion for radius, elevation, sizing in Phase 3
        Ok(material_tokens)
    }
}

/// Theme metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    /// Theme name
    pub name: String,
    /// Theme version
    pub version: String,
    /// Theme author
    pub author: Option<String>,
    /// Theme description
    pub description: Option<String>,
    /// Whether this is a dark theme
    pub is_dark: bool,
    /// Parent theme to inherit from
    pub extends: Option<String>,
}

/// Serializable semantic colors for file loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSemanticColors {
    /// Primary color as hex string
    pub primary: String,
    /// Secondary color as hex string  
    pub secondary: String,
    /// Success color as hex string
    pub success: String,
    /// Warning color as hex string
    pub warning: String,
    /// Error color as hex string
    pub error: String,
    /// Info color as hex string
    pub info: String,
    /// Surface color as hex string
    pub surface: String,
    /// Text on surface color as hex string
    pub on_surface: String,
}

impl SerializableSemanticColors {
    /// Convert to runtime `SemanticColors`
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any color hex string is invalid or malformed
    /// - Color parsing fails for any semantic color
    pub fn to_semantic_colors(&self) -> Result<SemanticColors, ThemeLoadError> {
        Ok(SemanticColors {
            primary: Self::parse_color(&self.primary)?,
            secondary: Self::parse_color(&self.secondary)?,
            success: Self::parse_color(&self.success)?,
            warning: Self::parse_color(&self.warning)?,
            error: Self::parse_color(&self.error)?,
            info: Self::parse_color(&self.info)?,
            surface: Self::parse_color(&self.surface)?,
            on_surface: Self::parse_color(&self.on_surface)?,
        })
    }

    /// Parse hex color string to Color
    fn parse_color(hex: &str) -> Result<Color, ThemeLoadError> {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 && hex.len() != 8 {
            return Err(ThemeLoadError::InvalidColor(hex.to_string()));
        }
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| ThemeLoadError::InvalidColor(hex.to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| ThemeLoadError::InvalidColor(hex.to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| ThemeLoadError::InvalidColor(hex.to_string()))?;

        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16)
                .map_err(|_| ThemeLoadError::InvalidColor(hex.to_string()))?
        } else {
            255
        };

        Ok(Color::from_rgba(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
            f32::from(a) / 255.0,
        ))
    }
}

/// Unified Material Design tokens structure
/// 
/// This structure now includes all token types and eliminates the need for
/// separate SerializableDesignTokens. TODO tokens will be implemented in Phase 3.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableMaterialTokens {
    /// Spacing values
    pub spacing: SerializableSpacing,
    /// Typography values
    pub typography: SerializableTypography,
    /// Border radius values (TODO: implement in Phase 3)
    #[serde(default)]
    pub radius: HashMap<String, f32>,
    /// Elevation/shadow values (TODO: implement in Phase 3)
    #[serde(default)]
    pub elevation: HashMap<String, f32>,
    /// Component sizing values (TODO: implement in Phase 3)
    #[serde(default)]
    pub sizing: HashMap<String, f32>,
}

/// Serializable spacing tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSpacing {
    /// Extra small spacing
    pub xs: f32,
    /// Small spacing
    pub sm: f32,
    /// Medium spacing
    pub md: f32,
    /// Large spacing
    pub lg: f32,
    /// Extra large spacing
    pub xl: f32,
    /// Extra extra large spacing
    pub xxl: f32,
}

/// Serializable typography tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTypography {
    /// Label small font size
    pub label_small: u16,
    /// Label medium font size
    pub label_medium: u16,
    /// Label large font size
    pub label_large: u16,
    /// Body small font size
    pub body_small: u16,
    /// Body medium font size
    pub body_medium: u16,
    /// Body large font size
    pub body_large: u16,
    /// Title small font size
    pub title_small: u16,
    /// Title medium font size
    pub title_medium: u16,
    /// Title large font size
    pub title_large: u16,
    /// Headline small font size
    pub headline_small: u16,
    /// Headline medium font size
    pub headline_medium: u16,
    /// Headline large font size
    pub headline_large: u16,
    /// Display small font size
    pub display_small: u16,
    /// Display medium font size
    pub display_medium: u16,
    /// Display large font size
    pub display_large: u16,
}

impl SerializableTypography {
    /// Convert to Material Design typography with proper MD3 specifications
    ///
    /// This method centralizes typography conversion logic and ensures consistency
    /// with Material Design 3 specifications for line heights and letter spacing.
    #[must_use]
    pub fn to_material_typography(&self) -> MaterialTypography {
        let font_sizes = [
            self.display_large,
            self.display_medium,
            self.display_small,
            self.headline_large,
            self.headline_medium,
            self.headline_small,
            self.title_large,
            self.title_medium,
            self.title_small,
            self.label_large,
            self.label_medium,
            self.label_small,
            self.body_large,
            self.body_medium,
            self.body_small,
        ];

        MaterialTypography {
            display_large: Self::create_type_style(font_sizes[0], 0),
            display_medium: Self::create_type_style(font_sizes[1], 1),
            display_small: Self::create_type_style(font_sizes[2], 2),
            headline_large: Self::create_type_style(font_sizes[3], 3),
            headline_medium: Self::create_type_style(font_sizes[4], 4),
            headline_small: Self::create_type_style(font_sizes[5], 5),
            title_large: Self::create_type_style(font_sizes[6], 6),
            title_medium: Self::create_type_style(font_sizes[7], 7),
            title_small: Self::create_type_style(font_sizes[8], 8),
            label_large: Self::create_type_style(font_sizes[9], 9),
            label_medium: Self::create_type_style(font_sizes[10], 10),
            label_small: Self::create_type_style(font_sizes[11], 11),
            body_large: Self::create_type_style(font_sizes[12], 12),
            body_medium: Self::create_type_style(font_sizes[13], 13),
            body_small: Self::create_type_style(font_sizes[14], 14),
        }
    }

    /// Create a TypeStyle using Material Design 3 specifications
    #[must_use]
    fn create_type_style(font_size: u16, spec_index: usize) -> TypeStyle {
        let spec = &MD3_TYPOGRAPHY_SPECS[spec_index];
        let (font, weight, line_height, letter_spacing) = (spec.0.clone(), spec.1.clone(), spec.2, spec.3);
        TypeStyle::new(font, weight, font_size as f32, line_height, letter_spacing)
    }
}

/// Type-safe component style override system
/// 
/// This replaces the previous HashMap<String, serde_json::Value> approach with 
/// strongly-typed overrides that correspond to actual component properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentOverride {
    /// Component type identifier
    pub component_type: ComponentType,
    /// Style variant (optional)
    pub variant: Option<String>,
    /// Type-safe component overrides
    pub overrides: ComponentOverrides,
}

/// Component type enumeration for type-safe overrides
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComponentType {
    /// Button components (all variants)
    Button,
    /// Text input components  
    Input,
    /// Container/card components
    Container,
    /// Modal/dialog components
    Modal,
    /// Menu components
    Menu,
    /// Navigation components
    Navigation,
    /// Progress/feedback components
    Progress,
    /// Selection components (chips, switches, etc.)
    Selection,
}

/// Type-safe component override definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentOverrides {
    /// Button component overrides
    Button(ButtonOverride),
    /// Input component overrides
    Input(InputOverride),
    /// Container component overrides
    Container(ContainerOverride),
    /// Modal component overrides
    Modal(ModalOverride),
    /// Menu component overrides
    Menu(MenuOverride),
    /// Navigation component overrides
    Navigation(NavigationOverride),
    /// Progress component overrides
    Progress(ProgressOverride),
    /// Selection component overrides
    Selection(SelectionOverride),
}

/// Button component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonOverride {
    /// Override minimum height
    pub min_height: Option<f32>,
    /// Override horizontal padding
    pub padding_horizontal: Option<f32>,
    /// Override vertical padding
    pub padding_vertical: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override minimum width
    pub min_width: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override text color
    pub text_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override border width
    pub border_width: Option<f32>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
}

/// Input component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputOverride {
    /// Override input field height
    pub height: Option<f32>,
    /// Override internal padding
    pub padding: Option<f32>,
    /// Override border width
    pub border_width: Option<f32>,
    /// Override border width when focused
    pub focus_border_width: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override text color
    pub text_color: Option<String>,
    /// Override placeholder color
    pub placeholder_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override focus border color
    pub focus_border_color: Option<String>,
}

/// Container component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerOverride {
    /// Override padding inside containers
    pub padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override border width
    pub border_width: Option<f32>,
    /// Override margin
    pub margin: Option<f32>,
}

/// Modal component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalOverride {
    /// Override maximum width
    pub max_width: Option<f32>,
    /// Override padding inside modals
    pub padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override backdrop opacity
    pub backdrop_opacity: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
}

/// Menu component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuOverride {
    /// Override menu item height
    pub item_height: Option<f32>,
    /// Override menu padding
    pub padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override maximum height
    pub max_height: Option<f32>,
    /// Override minimum width
    pub min_width: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override item hover color
    pub item_hover_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
}

/// Navigation component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationOverride {
    /// Override navigation bar height
    pub bar_height: Option<f32>,
    /// Override item padding
    pub item_padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override active item color
    pub active_item_color: Option<String>,
    /// Override inactive item color
    pub inactive_item_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
}

/// Progress component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressOverride {
    /// Override progress bar height
    pub bar_height: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override progress color
    pub progress_color: Option<String>,
    /// Override track color
    pub track_color: Option<String>,
    /// Override animation duration
    pub animation_duration: Option<f32>,
}

/// Selection component style overrides (chips, switches, checkboxes, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionOverride {
    /// Override item height
    pub height: Option<f32>,
    /// Override padding
    pub padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override minimum width
    pub min_width: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override selected background color
    pub selected_background_color: Option<String>,
    /// Override text color
    pub text_color: Option<String>,
    /// Override selected text color
    pub selected_text_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override selected border color
    pub selected_border_color: Option<String>,
    /// Override border width
    pub border_width: Option<f32>,
}

impl ComponentOverride {
    /// Create a new button override
    pub fn button() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Button)
    }

    /// Create a new input override
    pub fn input() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Input)
    }

    /// Create a new container override
    pub fn container() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Container)
    }

    /// Create a new modal override
    pub fn modal() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Modal)
    }

    /// Create a new menu override
    pub fn menu() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Menu)
    }

    /// Create a new navigation override
    pub fn navigation() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Navigation)
    }

    /// Create a new progress override
    pub fn progress() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Progress)
    }

    /// Create a new selection override
    pub fn selection() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Selection)
    }

    /// Validate the component override configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate that the component type matches the override type
        match (&self.component_type, &self.overrides) {
            (ComponentType::Button, ComponentOverrides::Button(_)) => Ok(()),
            (ComponentType::Input, ComponentOverrides::Input(_)) => Ok(()),
            (ComponentType::Container, ComponentOverrides::Container(_)) => Ok(()),
            (ComponentType::Modal, ComponentOverrides::Modal(_)) => Ok(()),
            (ComponentType::Menu, ComponentOverrides::Menu(_)) => Ok(()),
            (ComponentType::Navigation, ComponentOverrides::Navigation(_)) => Ok(()),
            (ComponentType::Progress, ComponentOverrides::Progress(_)) => Ok(()),
            (ComponentType::Selection, ComponentOverrides::Selection(_)) => Ok(()),
            _ => Err(format!(
                "Component type {:?} does not match override type", 
                self.component_type
            )),
        }
    }
}

/// Builder for creating component overrides with fluent API
pub struct ComponentOverrideBuilder {
    component_type: ComponentType,
    variant: Option<String>,
}

impl ComponentOverrideBuilder {
    /// Create a new builder for the specified component type
    pub fn new(component_type: ComponentType) -> Self {
        Self {
            component_type,
            variant: None,
        }
    }

    /// Set the component variant
    pub fn variant<S: Into<String>>(mut self, variant: S) -> Self {
        self.variant = Some(variant.into());
        self
    }

    /// Build a button override
    pub fn button_override(self, override_def: ButtonOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Button(override_def),
        }
    }

    /// Build an input override
    pub fn input_override(self, override_def: InputOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Input(override_def),
        }
    }

    /// Build a container override
    pub fn container_override(self, override_def: ContainerOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Container(override_def),
        }
    }

    /// Build a modal override
    pub fn modal_override(self, override_def: ModalOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Modal(override_def),
        }
    }

    /// Build a menu override
    pub fn menu_override(self, override_def: MenuOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Menu(override_def),
        }
    }

    /// Build a navigation override
    pub fn navigation_override(self, override_def: NavigationOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Navigation(override_def),
        }
    }

    /// Build a progress override
    pub fn progress_override(self, override_def: ProgressOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Progress(override_def),
        }
    }

    /// Build a selection override
    pub fn selection_override(self, override_def: SelectionOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Selection(override_def),
        }
    }
}
/// Theme loading errors
#[derive(Debug, Clone)]
pub enum ThemeLoadError {
    /// File not found or cannot be read
    FileError(String),
    /// Invalid JSON or TOML format
    ParseError(String),
    /// Invalid color format
    InvalidColor(String),
    /// Required field missing
    MissingField(String),
    /// Theme validation failed
    ValidationError(String),
}

impl std::fmt::Display for ThemeLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileError(msg) => write!(f, "File error: {msg}"),
            Self::ParseError(msg) => write!(f, "Parse error: {msg}"),
            Self::InvalidColor(color) => write!(f, "Invalid color: {color}"),
            Self::MissingField(field) => write!(f, "Missing field: {field}"),
            Self::ValidationError(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}

impl std::error::Error for ThemeLoadError {}

/// Dynamic theme loader for loading themes from files
pub struct ThemeLoader {
    /// Cache of loaded themes
    theme_cache: HashMap<String, ThemeConfig>,
}

impl ThemeLoader {
    /// Create a new theme loader
    #[must_use]
    pub fn new() -> Self {
        Self {
            theme_cache: HashMap::new(),
        }
    }

    /// Create a theme loader with a specific directory
    #[must_use]
    pub fn with_directory<P: AsRef<Path>>(_directory: P) -> Self {
        Self {
            theme_cache: HashMap::new(),
        }
    }

    /// Load theme from JSON file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - JSON parsing fails
    /// - Theme validation fails
    pub fn load_from_json<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<ThemeConfig, ThemeLoadError> {
        let content = fs::read_to_string(&path).map_err(|e| {
            ThemeLoadError::FileError(format!("{}: {}", path.as_ref().display(), e))
        })?;
        let config: ThemeConfig = serde_json::from_str(&content)
            .map_err(|e| ThemeLoadError::ParseError(e.to_string()))?;

        Self::validate_theme(&config)?;

        // Cache the theme
        self.theme_cache
            .insert(config.metadata.name.clone(), config.clone());

        Ok(config)
    }

    /// Load theme from TOML file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - TOML parsing fails
    /// - Theme validation fails
    pub fn load_from_toml<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<ThemeConfig, ThemeLoadError> {
        let content = fs::read_to_string(&path).map_err(|e| {
            ThemeLoadError::FileError(format!("{}: {}", path.as_ref().display(), e))
        })?;
        let config: ThemeConfig =
            toml::from_str(&content).map_err(|e| ThemeLoadError::ParseError(e.to_string()))?;

        Self::validate_theme(&config)?;

        // Cache the theme
        self.theme_cache
            .insert(config.metadata.name.clone(), config.clone());

        Ok(config)
    }

    /// Get cached theme by name
    #[must_use]
    pub fn get_theme(&self, name: &str) -> Option<&ThemeConfig> {
        self.theme_cache.get(name)
    }

    /// List all loaded themes
    #[must_use]
    pub fn list_themes(&self) -> Vec<&ThemeConfig> {
        self.theme_cache.values().collect()
    }

    /// Create a runtime theme mode from a theme config
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Semantic color conversion fails
    /// - Material token conversion fails
    /// - Theme configuration is invalid
    /// - Component override validation fails
    pub fn create_theme_mode(
        &self,
        config: &ThemeConfig,
    ) -> Result<CustomThemeMode, ThemeLoadError> {
        let semantic_colors = config.semantic_colors.to_semantic_colors()?;

        // Validate component overrides
        for override_config in &config.component_overrides {
            override_config.validate().map_err(|e| {
                ThemeLoadError::ValidationError(format!("Component override validation failed: {}", e))
            })?;
        }

        Ok(CustomThemeMode {
            metadata: config.metadata.clone(),
            semantic_colors,
            material_tokens: Self::convert_material_tokens(&config.material_tokens)?,
            component_overrides: config.component_overrides.clone(),
        })
    }
    /// Validate a loaded theme configuration
    fn validate_theme(config: &ThemeConfig) -> Result<(), ThemeLoadError> {
        if config.metadata.name.is_empty() {
            return Err(ThemeLoadError::MissingField("metadata.name".to_string()));
        }

        // Try to parse colors to validate them
        config.semantic_colors.to_semantic_colors()?;

        // Validate component overrides
        for override_config in &config.component_overrides {
            override_config.validate().map_err(|e| {
                ThemeLoadError::ValidationError(format!("Component override validation failed: {}", e))
            })?;
        }

        Ok(())
    }

    /// Convert serializable material tokens to runtime tokens
    /// 
    /// This unified method replaces the old convert_design_tokens method
    /// and handles the complete material token conversion process.
    fn convert_material_tokens(
        tokens: &SerializableMaterialTokens,
    ) -> Result<MaterialTokens, ThemeLoadError> {
        // Create Material tokens with proper initialization
        let material_tokens = MaterialTokens {
            spacing: SpacingTokens {
                xs: tokens.spacing.xs,
                sm: tokens.spacing.sm,
                md: tokens.spacing.md,
                lg: tokens.spacing.lg,
                xl: tokens.spacing.xl,
                xxl: tokens.spacing.xxl,
            },
            typography: tokens.typography.to_material_typography(),
            ..Default::default()
        };

        // TODO: Add conversion for radius, elevation, sizing in Phase 3
        // For now, these remain as default values

        Ok(material_tokens)
    }
}

impl Default for ThemeLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Custom theme mode created from loaded configuration
/// 
/// This structure represents a fully processed theme with runtime-ready tokens
/// and eliminates redundancy with the base ThemeConfig structure.
#[derive(Debug, Clone)]
pub struct CustomThemeMode {
    /// Theme metadata
    pub metadata: ThemeMetadata,
    /// Semantic colors
    pub semantic_colors: SemanticColors,
    /// Material Design tokens (runtime-ready)
    pub material_tokens: MaterialTokens,
    /// Type-safe component overrides
    pub component_overrides: Vec<ComponentOverride>,
}

impl CustomThemeMode {
    /// Get the theme name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// Check if this is a dark theme
    #[must_use]
    pub const fn is_dark(&self) -> bool {
        self.metadata.is_dark
    }

    /// Get semantic colors
    #[must_use]
    pub const fn semantic_colors(&self) -> &SemanticColors {
        &self.semantic_colors
    }

    /// Get material tokens
    #[must_use]
    pub const fn material_tokens(&self) -> &MaterialTokens {
        &self.material_tokens
    }

    /// Get component overrides
    #[must_use]
    pub fn component_overrides(&self) -> &[ComponentOverride] {
        &self.component_overrides
    }

    /// Find component override by type and optional variant
    #[must_use]
    pub fn find_component_override(&self, component_type: ComponentType, variant: Option<&str>) -> Option<&ComponentOverride> {
        self.component_overrides.iter().find(|override_config| {
            override_config.component_type == component_type 
                && override_config.variant.as_deref() == variant
        })
    }
}


/// Runtime theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: String,
    /// Material Design tokens
    pub material_tokens: MaterialTokens,
    /// Component style overrides
    pub component_overrides: Vec<ComponentOverride>,
}

impl Theme {
    /// Get the Material Design tokens
    #[must_use]
    pub const fn material_tokens(&self) -> &MaterialTokens {
        &self.material_tokens
    }

    /// Get component overrides
    #[must_use]
    pub fn component_overrides(&self) -> &[ComponentOverride] {
        &self.component_overrides
    }

    /// Find component override by type and optional variant
    #[must_use]
    pub fn find_component_override(&self, component_type: ComponentType, variant: Option<&str>) -> Option<&ComponentOverride> {
        self.component_overrides.iter().find(|override_config| {
            override_config.component_type == component_type 
                && override_config.variant.as_deref() == variant
        })
    }

    /// Get theme name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get theme description
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Pixels;

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
        let found_override = runtime_theme.find_component_override(ComponentType::Button, Some("primary"));
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
