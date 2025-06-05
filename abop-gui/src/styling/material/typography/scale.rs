//! Typography scale and type styles for Material Design 3

use super::{
    constants::{letter_spacing, line_heights, sizes},
    font_mapping::{MaterialFont, MaterialWeight},
    roles::TypographyRole,
};
use iced::font::Family;
use iced::{Font, Pixels};

/// Individual typography style with all properties
#[derive(Debug, Clone, PartialEq)]
pub struct TypeStyle {
    /// The font family to use
    pub family: Family,
    /// The font weight
    pub weight: iced::font::Weight,
    /// The font size in pixels
    pub size: Pixels,
    /// The line height in pixels
    pub line_height: Pixels,
    /// The letter spacing adjustment in em units
    pub letter_spacing: f32,
}

impl TypeStyle {
    /// Create a new `TypeStyle`
    #[must_use]
    pub fn new(
        family: MaterialFont,
        weight: MaterialWeight,
        size: f32,
        line_height: f32,
        letter_spacing: f32,
    ) -> Self {
        Self {
            family: family.into(),
            weight: weight.into(),
            size: Pixels(size),
            line_height: Pixels(line_height),
            letter_spacing,
        }
    }

    /// Convert to Iced Font
    #[must_use]
    pub fn to_font(&self) -> Font {
        // Use system default font to avoid wingdings issue
        Font {
            family: Family::default(),
            weight: self.weight,
            ..Font::default()
        }
    }

    /// Get the font size in pixels
    #[must_use]
    pub const fn size(&self) -> f32 {
        self.size.0
    }

    /// Get the line height in pixels
    #[must_use]
    pub const fn line_height(&self) -> f32 {
        self.line_height.0
    }

    /// Create a variant with different weight
    #[must_use]
    pub fn with_weight(&self, weight: MaterialWeight) -> Self {
        Self {
            weight: weight.into(),
            ..self.clone()
        }
    }

    /// Create a variant with different size
    #[must_use]
    pub fn with_size(&self, size: f32) -> Self {
        Self {
            size: Pixels(size),
            ..self.clone()
        }
    }

    /// Create a variant with different letter spacing
    #[must_use]
    pub fn with_letter_spacing(&self, letter_spacing: f32) -> Self {
        Self {
            letter_spacing,
            ..self.clone()
        }
    }
}

/// Material Design 3 typography scale
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialTypography {
    // Display styles
    /// Large display text style
    pub display_large: TypeStyle,
    /// Medium display text style
    pub display_medium: TypeStyle,
    /// Small display text style
    pub display_small: TypeStyle,

    // Headline styles
    /// Large headline text style
    pub headline_large: TypeStyle,
    /// Medium headline text style
    pub headline_medium: TypeStyle,
    /// Small headline text style
    pub headline_small: TypeStyle,

    // Title styles
    /// Large title text style
    pub title_large: TypeStyle,
    /// Medium title text style
    pub title_medium: TypeStyle,
    /// Small title text style
    pub title_small: TypeStyle,

    // Label styles
    /// Large label text style
    pub label_large: TypeStyle,
    /// Medium label text style
    pub label_medium: TypeStyle,
    /// Small label text style
    pub label_small: TypeStyle,

    // Body styles
    /// Large body text style
    pub body_large: TypeStyle,
    /// Medium body text style
    pub body_medium: TypeStyle,
    /// Small body text style
    pub body_small: TypeStyle,
}

/// Generate a `TypeStyle` with the given parameters
macro_rules! create_style {
    ($font:expr, $weight:expr, $size:ident, $line_height:ident, $letter_spacing:ident) => {
        TypeStyle::new(
            $font,
            $weight,
            sizes::$size,
            line_heights::$line_height,
            letter_spacing::$letter_spacing,
        )
    };
}

impl MaterialTypography {
    /// Create the default Material Design typography scale
    #[must_use]
    pub fn new() -> Self {
        Self {
            // Display styles
            display_large: create_style!(
                MaterialFont::Brand,
                MaterialWeight::Regular,
                DISPLAY_LARGE,
                DISPLAY_LARGE,
                DISPLAY_LARGE
            ),
            display_medium: create_style!(
                MaterialFont::Brand,
                MaterialWeight::Regular,
                DISPLAY_MEDIUM,
                DISPLAY_MEDIUM,
                DISPLAY_MEDIUM
            ),
            display_small: create_style!(
                MaterialFont::Brand,
                MaterialWeight::Regular,
                DISPLAY_SMALL,
                DISPLAY_SMALL,
                DISPLAY_SMALL
            ),

            // Headline styles
            headline_large: create_style!(
                MaterialFont::Brand,
                MaterialWeight::Regular,
                HEADLINE_LARGE,
                HEADLINE_LARGE,
                HEADLINE_LARGE
            ),
            headline_medium: create_style!(
                MaterialFont::Brand,
                MaterialWeight::Regular,
                HEADLINE_MEDIUM,
                HEADLINE_MEDIUM,
                HEADLINE_MEDIUM
            ),
            headline_small: create_style!(
                MaterialFont::Brand,
                MaterialWeight::Regular,
                HEADLINE_SMALL,
                HEADLINE_SMALL,
                HEADLINE_SMALL
            ),

            // Title styles
            title_large: create_style!(
                MaterialFont::Brand,
                MaterialWeight::Regular,
                TITLE_LARGE,
                TITLE_LARGE,
                TITLE_LARGE
            ),
            title_medium: create_style!(
                MaterialFont::Plain,
                MaterialWeight::Medium,
                TITLE_MEDIUM,
                TITLE_MEDIUM,
                TITLE_MEDIUM
            ),
            title_small: create_style!(
                MaterialFont::Plain,
                MaterialWeight::Medium,
                TITLE_SMALL,
                TITLE_SMALL,
                TITLE_SMALL
            ),

            // Label styles
            label_large: create_style!(
                MaterialFont::Plain,
                MaterialWeight::Medium,
                LABEL_LARGE,
                LABEL_LARGE,
                LABEL_LARGE
            ),
            label_medium: create_style!(
                MaterialFont::Plain,
                MaterialWeight::Medium,
                LABEL_MEDIUM,
                LABEL_MEDIUM,
                LABEL_MEDIUM
            ),
            label_small: create_style!(
                MaterialFont::Plain,
                MaterialWeight::Medium,
                LABEL_SMALL,
                LABEL_SMALL,
                LABEL_SMALL
            ),

            // Body styles
            body_large: create_style!(
                MaterialFont::Plain,
                MaterialWeight::Regular,
                BODY_LARGE,
                BODY_LARGE,
                BODY_LARGE
            ),
            body_medium: create_style!(
                MaterialFont::Plain,
                MaterialWeight::Regular,
                BODY_MEDIUM,
                BODY_MEDIUM,
                BODY_MEDIUM
            ),
            body_small: create_style!(
                MaterialFont::Plain,
                MaterialWeight::Regular,
                BODY_SMALL,
                BODY_SMALL,
                BODY_SMALL
            ),
        }
    }

    /// Get a `TypeStyle` by role name
    #[must_use]
    pub const fn get_style(&self, role: TypographyRole) -> &TypeStyle {
        match role {
            TypographyRole::DisplayLarge => &self.display_large,
            TypographyRole::DisplayMedium => &self.display_medium,
            TypographyRole::DisplaySmall => &self.display_small,
            TypographyRole::HeadlineLarge => &self.headline_large,
            TypographyRole::HeadlineMedium => &self.headline_medium,
            TypographyRole::HeadlineSmall => &self.headline_small,
            TypographyRole::TitleLarge => &self.title_large,
            TypographyRole::TitleMedium => &self.title_medium,
            TypographyRole::TitleSmall => &self.title_small,
            TypographyRole::LabelLarge => &self.label_large,
            TypographyRole::LabelMedium => &self.label_medium,
            TypographyRole::LabelSmall => &self.label_small,
            TypographyRole::BodyLarge => &self.body_large,
            TypographyRole::BodyMedium => &self.body_medium,
            TypographyRole::BodySmall => &self.body_small,
        }
    }

    /// Scale all typography sizes by a factor
    #[must_use]
    pub fn with_scale(&self, scale: f32) -> Self {
        Self {
            display_large: self
                .display_large
                .with_size(self.display_large.size() * scale),
            display_medium: self
                .display_medium
                .with_size(self.display_medium.size() * scale),
            display_small: self
                .display_small
                .with_size(self.display_small.size() * scale),
            headline_large: self
                .headline_large
                .with_size(self.headline_large.size() * scale),
            headline_medium: self
                .headline_medium
                .with_size(self.headline_medium.size() * scale),
            headline_small: self
                .headline_small
                .with_size(self.headline_small.size() * scale),
            title_large: self.title_large.with_size(self.title_large.size() * scale),
            title_medium: self
                .title_medium
                .with_size(self.title_medium.size() * scale),
            title_small: self.title_small.with_size(self.title_small.size() * scale),
            label_large: self.label_large.with_size(self.label_large.size() * scale),
            label_medium: self
                .label_medium
                .with_size(self.label_medium.size() * scale),
            label_small: self.label_small.with_size(self.label_small.size() * scale),
            body_large: self.body_large.with_size(self.body_large.size() * scale),
            body_medium: self.body_medium.with_size(self.body_medium.size() * scale),
            body_small: self.body_small.with_size(self.body_small.size() * scale),
        }
    }
}

impl Default for MaterialTypography {
    fn default() -> Self {
        Self::new()
    }
}
