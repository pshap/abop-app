//! Font family and weight mapping for Material Design
//!
//! Note: Currently uses system default fonts to avoid Unicode symbol rendering issues.
//! For better Unicode support, consider adding embedded fonts with extended character sets.

use iced::font::{Family, Weight};

/// Material Design font families
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterialFont {
    /// Brand font family - used for display, headlines, and titles
    Brand,
    /// Plain font family - used for body text and labels
    Plain,
}

/// Material Design font weights
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterialWeight {
    /// Regular weight (400)
    Regular,
    /// Medium weight (500)
    Medium,
    /// Bold weight (700)
    Bold,
}

impl From<MaterialFont> for Family {
    fn from(font: MaterialFont) -> Self {
        match font {
            // Use system default font instead of custom font names to avoid wingdings
            // This ensures compatibility across different systems
            MaterialFont::Brand | MaterialFont::Plain => Self::default(),
        }
    }
}

impl From<MaterialWeight> for Weight {
    fn from(weight: MaterialWeight) -> Self {
        match weight {
            MaterialWeight::Regular => Self::Normal,
            MaterialWeight::Medium => Self::Medium,
            MaterialWeight::Bold => Self::Bold,
        }
    }
}
