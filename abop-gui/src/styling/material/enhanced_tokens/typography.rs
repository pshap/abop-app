//! Typography and Spacing Token Definitions
//!
//! This module contains typography and spacing token definitions
//! following Material Design 3 specifications.

/// Typography tokens for text styling
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
    pub small: f32,
    /// Medium spacing (16dp)
    pub medium: f32,
    /// Large spacing (24dp)
    pub large: f32,
    /// Extra large spacing (32dp)
    pub xl: f32,
}

impl Default for SpacingTokens {
    fn default() -> Self {
        Self {
            xs: 4.0,
            small: 8.0,
            medium: 16.0,
            large: 24.0,
            xl: 32.0,
        }
    }
}
