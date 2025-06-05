//! Typography utilities for common operations

use super::roles::TypographyRole;

/// Calculate optimal line height for a given font size
#[must_use]
pub fn calculate_line_height(font_size: f32) -> f32 {
    (font_size * 1.4).round()
}

/// Convert rem to pixels (assuming 16px base)
#[must_use]
pub fn rem_to_px(rem: f32) -> f32 {
    rem * 16.0
}

/// Convert pixels to rem (assuming 16px base)
#[must_use]
pub fn px_to_rem(px: f32) -> f32 {
    px / 16.0
}

/// Get appropriate typography role for content type
#[must_use]
pub const fn get_recommended_role(content_type: ContentType) -> TypographyRole {
    match content_type {
        ContentType::MainHeading => TypographyRole::HeadlineLarge,
        ContentType::SectionHeading => TypographyRole::HeadlineMedium,
        ContentType::Subheading => TypographyRole::HeadlineSmall,
        ContentType::CardTitle => TypographyRole::TitleLarge,
        ContentType::ListItemTitle => TypographyRole::TitleMedium,
        ContentType::ButtonText => TypographyRole::LabelLarge,
        ContentType::TabLabel => TypographyRole::LabelMedium,
        ContentType::Caption => TypographyRole::LabelSmall,
        ContentType::BodyText => TypographyRole::BodyLarge,
        ContentType::SecondaryText => TypographyRole::BodyMedium,
        ContentType::SmallText => TypographyRole::BodySmall,
    }
}

/// Content types for typography role recommendations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// Main page or section heading
    MainHeading,
    /// Secondary section heading
    SectionHeading,
    /// Tertiary subheading
    Subheading,
    /// Title text on cards
    CardTitle,
    /// Title text for list items
    ListItemTitle,
    /// Text on buttons
    ButtonText,
    /// Text on navigation tabs
    TabLabel,
    /// Caption text and small descriptive text
    Caption,
    /// Primary body text for reading
    BodyText,
    /// Secondary descriptive text
    SecondaryText,
    /// Small informational text
    SmallText,
}
