//! Serializable types and conversion logic for dynamic themes

use super::errors::ThemeLoadError;
use crate::styling::material::{
    tokens::semantic::SemanticColors,
    typography::{
        MaterialTypography, TypeStyle,
        font_mapping::{MaterialFont, MaterialWeight},
    },
};
use iced::{Color, Pixels};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Material Design 3 typography specifications
/// Line heights and letter spacing values according to MD3 specification
/// Values based on official Google Material Design 3 typescale v0.192
const MD3_TYPOGRAPHY_SPECS: [(MaterialFont, MaterialWeight, f32, f32); 15] = [
    // Display styles - Brand font, Regular weight (sizes: 57px, 45px, 36px)
    (MaterialFont::Brand, MaterialWeight::Regular, 64.0, -0.25), // display_large (4rem line, -0.015625rem tracking)
    (MaterialFont::Brand, MaterialWeight::Regular, 52.0, 0.0), // display_medium (3.25rem line, 0rem tracking)
    (MaterialFont::Brand, MaterialWeight::Regular, 44.0, 0.0), // display_small (2.75rem line, 0rem tracking)
    // Headline styles - Brand font, Regular weight (sizes: 32px, 28px, 24px)
    (MaterialFont::Brand, MaterialWeight::Regular, 40.0, 0.0), // headline_large (2.5rem line, 0rem tracking)
    (MaterialFont::Brand, MaterialWeight::Regular, 36.0, 0.0), // headline_medium (2.25rem line, 0rem tracking)
    (MaterialFont::Brand, MaterialWeight::Regular, 32.0, 0.0), // headline_small (2rem line, 0rem tracking)
    // Title styles - Mixed fonts (sizes: 22px, 16px, 14px)
    (MaterialFont::Brand, MaterialWeight::Regular, 28.0, 0.0), // title_large (1.75rem line, 0rem tracking)
    (MaterialFont::Plain, MaterialWeight::Medium, 24.0, 0.15), // title_medium (1.5rem line, 0.009375rem tracking)
    (MaterialFont::Plain, MaterialWeight::Medium, 20.0, 0.1), // title_small (1.25rem line, 0.00625rem tracking)
    // Label styles - Plain font, Medium weight (sizes: 14px, 12px, 11px)
    (MaterialFont::Plain, MaterialWeight::Medium, 20.0, 0.1), // label_large (1.25rem line, 0.00625rem tracking)
    (MaterialFont::Plain, MaterialWeight::Medium, 16.0, 0.5), // label_medium (1rem line, 0.03125rem tracking)
    (MaterialFont::Plain, MaterialWeight::Medium, 16.0, 0.5), // label_small (1rem line, 0.03125rem tracking)
    // Body styles - Plain font, Regular weight (sizes: 16px, 14px, 12px)
    (MaterialFont::Plain, MaterialWeight::Regular, 24.0, 0.5), // body_large (1.5rem line, 0.03125rem tracking)
    (MaterialFont::Plain, MaterialWeight::Regular, 20.0, 0.25), // body_medium (1.25rem line, 0.015625rem tracking)
    (MaterialFont::Plain, MaterialWeight::Regular, 16.0, 0.4), // body_small (1rem line, 0.025rem tracking)
];

/// Theme metadata
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
    /// Base theme this extends
    pub extends: Option<String>,
}

/// Serializable semantic colors for theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSemanticColors {
    /// Primary color
    pub primary: String,
    /// Secondary color
    pub secondary: String,
    /// Success color
    pub success: String,
    /// Warning color
    pub warning: String,
    /// Error color
    pub error: String,
    /// Info color
    pub info: String,
    /// Surface color
    pub surface: String,
    /// On surface color
    pub on_surface: String,
}

impl SerializableSemanticColors {
    /// Parse color string to iced::Color
    pub fn parse_color(color_str: &str) -> Result<Color, ThemeLoadError> {
        let color_str = color_str.trim_start_matches('#');

        // Handle 6-character hex (RGB)
        if color_str.len() == 6 {
            let r = u8::from_str_radix(&color_str[0..2], 16).map_err(|_| {
                ThemeLoadError::InvalidColor(format!("Invalid red component: {}", &color_str[0..2]))
            })?;
            let g = u8::from_str_radix(&color_str[2..4], 16).map_err(|_| {
                ThemeLoadError::InvalidColor(format!(
                    "Invalid green component: {}",
                    &color_str[2..4]
                ))
            })?;
            let b = u8::from_str_radix(&color_str[4..6], 16).map_err(|_| {
                ThemeLoadError::InvalidColor(format!(
                    "Invalid blue component: {}",
                    &color_str[4..6]
                ))
            })?;

            Ok(Color::from_rgb8(r, g, b))
        }
        // Handle 8-character hex (RGBA)
        else if color_str.len() == 8 {
            let r = u8::from_str_radix(&color_str[0..2], 16).map_err(|_| {
                ThemeLoadError::InvalidColor(format!("Invalid red component: {}", &color_str[0..2]))
            })?;
            let g = u8::from_str_radix(&color_str[2..4], 16).map_err(|_| {
                ThemeLoadError::InvalidColor(format!(
                    "Invalid green component: {}",
                    &color_str[2..4]
                ))
            })?;
            let b = u8::from_str_radix(&color_str[4..6], 16).map_err(|_| {
                ThemeLoadError::InvalidColor(format!(
                    "Invalid blue component: {}",
                    &color_str[4..6]
                ))
            })?;
            let a = u8::from_str_radix(&color_str[6..8], 16).map_err(|_| {
                ThemeLoadError::InvalidColor(format!(
                    "Invalid alpha component: {}",
                    &color_str[6..8]
                ))
            })?;

            Ok(Color::from_rgba8(r, g, b, a as f32 / 255.0))
        } else {
            Err(ThemeLoadError::InvalidColor(format!(
                "Color string must be 6 or 8 characters (RGB or RGBA): {}",
                color_str
            )))
        }
    }

    /// Convert to runtime semantic colors
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

/// Serializable typography configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTypography {
    /// Label small (12px)
    pub label_small: u16,
    /// Label medium (14px)
    pub label_medium: u16,
    /// Label large (16px)
    pub label_large: u16,
    /// Body small (14px)
    pub body_small: u16,
    /// Body medium (16px)
    pub body_medium: u16,
    /// Body large (18px)
    pub body_large: u16,
    /// Title small (18px)
    pub title_small: u16,
    /// Title medium (20px)
    pub title_medium: u16,
    /// Title large (24px)
    pub title_large: u16,
    /// Headline small (20px)
    pub headline_small: u16,
    /// Headline medium (22px)
    pub headline_medium: u16,
    /// Headline large (26px)
    pub headline_large: u16,
    /// Display small (24px)
    pub display_small: u16,
    /// Display medium (26px)
    pub display_medium: u16,
    /// Display large (32px)
    pub display_large: u16,
}

impl SerializableTypography {
    /// Convert to Material Design typography with proper specifications
    #[must_use]
    pub fn to_material_typography(&self) -> MaterialTypography {
        let sizes = [
            self.display_large,   // 0: display_large
            self.display_medium,  // 1: display_medium
            self.display_small,   // 2: display_small
            self.headline_large,  // 3: headline_large
            self.headline_medium, // 4: headline_medium
            self.headline_small,  // 5: headline_small
            self.title_large,     // 6: title_large
            self.title_medium,    // 7: title_medium
            self.title_small,     // 8: title_small
            self.label_large,     // 9: label_large
            self.label_medium,    // 10: label_medium
            self.label_small,     // 11: label_small
            self.body_large,      // 12: body_large
            self.body_medium,     // 13: body_medium
            self.body_small,      // 14: body_small
        ];

        MaterialTypography {
            display_large: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[0].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[0].1.clone().into(),
                size: Pixels(f32::from(sizes[0])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[0].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[0].3,
            },
            display_medium: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[1].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[1].1.clone().into(),
                size: Pixels(f32::from(sizes[1])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[1].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[1].3,
            },
            display_small: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[2].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[2].1.clone().into(),
                size: Pixels(f32::from(sizes[2])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[2].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[2].3,
            },
            headline_large: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[3].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[3].1.clone().into(),
                size: Pixels(f32::from(sizes[3])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[3].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[3].3,
            },
            headline_medium: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[4].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[4].1.clone().into(),
                size: Pixels(f32::from(sizes[4])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[4].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[4].3,
            },
            headline_small: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[5].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[5].1.clone().into(),
                size: Pixels(f32::from(sizes[5])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[5].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[5].3,
            },
            title_large: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[6].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[6].1.clone().into(),
                size: Pixels(f32::from(sizes[6])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[6].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[6].3,
            },
            title_medium: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[7].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[7].1.clone().into(),
                size: Pixels(f32::from(sizes[7])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[7].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[7].3,
            },
            title_small: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[8].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[8].1.clone().into(),
                size: Pixels(f32::from(sizes[8])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[8].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[8].3,
            },
            label_large: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[9].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[9].1.clone().into(),
                size: Pixels(f32::from(sizes[9])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[9].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[9].3,
            },
            label_medium: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[10].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[10].1.clone().into(),
                size: Pixels(f32::from(sizes[10])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[10].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[10].3,
            },
            label_small: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[11].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[11].1.clone().into(),
                size: Pixels(f32::from(sizes[11])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[11].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[11].3,
            },
            body_large: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[12].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[12].1.clone().into(),
                size: Pixels(f32::from(sizes[12])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[12].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[12].3,
            },
            body_medium: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[13].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[13].1.clone().into(),
                size: Pixels(f32::from(sizes[13])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[13].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[13].3,
            },
            body_small: TypeStyle {
                family: MD3_TYPOGRAPHY_SPECS[14].0.clone().into(),
                weight: MD3_TYPOGRAPHY_SPECS[14].1.clone().into(),
                size: Pixels(f32::from(sizes[14])),
                line_height: Pixels(MD3_TYPOGRAPHY_SPECS[14].2),
                letter_spacing: MD3_TYPOGRAPHY_SPECS[14].3,
            },
        }
    }
}

/// Serializable material tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableMaterialTokens {
    /// Spacing tokens
    pub spacing: SerializableSpacing,
    /// Typography tokens
    pub typography: SerializableTypography,
    /// Border radius tokens (placeholder for future use)
    pub radius: HashMap<String, f32>,
    /// Elevation/shadow tokens (placeholder for future use)
    pub elevation: HashMap<String, f32>,
    /// Sizing tokens (placeholder for future use)
    pub sizing: HashMap<String, f32>,
}
