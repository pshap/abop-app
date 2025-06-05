//! Typography roles for Material Design 3

/// Typography roles in Material Design
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypographyRole {
    /// Display large - hero text, marketing headlines (57px)
    DisplayLarge,
    /// Display medium - prominent display text (45px)
    DisplayMedium,
    /// Display small - medium display text (36px)
    DisplaySmall,
    /// Headline large - article titles, major headings (32px)
    HeadlineLarge,
    /// Headline medium - section headers (28px)
    HeadlineMedium,
    /// Headline small - subsection headers (24px)
    HeadlineSmall,
    /// Title large - card headers, dialog titles (22px)
    TitleLarge,
    /// Title medium - smaller titles, list headers (16px)
    TitleMedium,
    /// Title small - component titles, tab labels (14px)
    TitleSmall,
    /// Label large - prominent buttons, form labels (14px)
    LabelLarge,
    /// Label medium - standard buttons, UI labels (12px)
    LabelMedium,
    /// Label small - small buttons, captions (11px)
    LabelSmall,
    /// Body large - primary reading content (16px)
    BodyLarge,
    /// Body medium - standard body text (14px)
    BodyMedium,
    /// Body small - captions, helper text (12px)
    BodySmall,
}

impl TypographyRole {
    /// Get all typography roles
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::DisplayLarge,
            Self::DisplayMedium,
            Self::DisplaySmall,
            Self::HeadlineLarge,
            Self::HeadlineMedium,
            Self::HeadlineSmall,
            Self::TitleLarge,
            Self::TitleMedium,
            Self::TitleSmall,
            Self::LabelLarge,
            Self::LabelMedium,
            Self::LabelSmall,
            Self::BodyLarge,
            Self::BodyMedium,
            Self::BodySmall,
        ]
    }

    /// Get the role name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DisplayLarge => "display-large",
            Self::DisplayMedium => "display-medium",
            Self::DisplaySmall => "display-small",
            Self::HeadlineLarge => "headline-large",
            Self::HeadlineMedium => "headline-medium",
            Self::HeadlineSmall => "headline-small",
            Self::TitleLarge => "title-large",
            Self::TitleMedium => "title-medium",
            Self::TitleSmall => "title-small",
            Self::LabelLarge => "label-large",
            Self::LabelMedium => "label-medium",
            Self::LabelSmall => "label-small",
            Self::BodyLarge => "body-large",
            Self::BodyMedium => "body-medium",
            Self::BodySmall => "body-small",
        }
    }
}
