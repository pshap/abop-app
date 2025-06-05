//! Material Design integration for design tokens

use super::DesignTokens;
use crate::styling::material::MaterialTokens;

/// Create design tokens from Material Design tokens
#[must_use]
pub fn create_from_material_tokens(material_tokens: &MaterialTokens) -> DesignTokens {
    let mut tokens = DesignTokens::new();

    // Map Material Design spacing to design tokens
    // Material uses 4dp base unit, which maps well to our system
    tokens.spacing.xs = 4.0; // Material: 4dp
    tokens.spacing.sm = 8.0; // Material: 8dp
    tokens.spacing.md = 16.0; // Material: 16dp
    tokens.spacing.lg = 24.0; // Material: 24dp
    tokens.spacing.xl = 32.0; // Material: 32dp
    tokens.spacing.xxl = 48.0; // Material: 48dp

    // Map Material Design typography sizes
    let _typography = &material_tokens.typography;
    tokens.typography.caption = 12; // Material: Label small
    tokens.typography.body = 14; // Material: Body medium
    tokens.typography.body_large = 16; // Material: Body large
    tokens.typography.heading_3 = 18; // Material: Title small
    tokens.typography.heading_2 = 20; // Material: Title medium
    tokens.typography.heading_1 = 24; // Material: Title large
    tokens.typography.display = 32; // Material: Display small

    // Map Material Design border radius
    let shapes = &material_tokens.shapes;
    tokens.radius.none = 0.0;
    tokens.radius.sm = shapes.corner_extra_small.radius.top_left;
    tokens.radius.md = shapes.corner_small.radius.top_left;
    tokens.radius.lg = shapes.corner_medium.radius.top_left;
    tokens.radius.xl = shapes.corner_large.radius.top_left;
    tokens.radius.full = shapes.corner_full.radius.top_left;

    tokens
}

/// Merge Material Design colors with existing design tokens
#[must_use]
pub const fn merge_material_colors(
    mut tokens: DesignTokens,
    material_tokens: &MaterialTokens,
) -> DesignTokens {
    let colors = &material_tokens.colors;

    // Update semantic colors with Material Design equivalents
    tokens.semantic_colors.primary = colors.primary.base;
    tokens.semantic_colors.secondary = colors.secondary.base;
    tokens.semantic_colors.surface = colors.surface;
    tokens.semantic_colors.on_surface = colors.on_surface;
    tokens.semantic_colors.success = colors.tertiary.base; // Use tertiary as Material doesn't have success
    tokens.semantic_colors.warning = colors.secondary.base; // Use secondary for warnings
    tokens.semantic_colors.error = colors.error.base;

    tokens
}
