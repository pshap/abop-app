//! Tests for the Material Design 3 Typography System

#[cfg(test)]
mod typography_tests {
    use crate::styling::material::typography::{
        ContentType, MaterialFont, MaterialTypography, MaterialWeight, TypeStyle, TypographyRole,
        get_recommended_role,
    };

    #[test]
    fn test_typography_creation() {
        let typography = MaterialTypography::default();
        assert!(typography.body_large.size() > typography.body_medium.size());
        assert!(typography.body_medium.size() > typography.body_small.size());
    }

    #[test]
    fn test_type_style_modifications() {
        let style = TypeStyle::new(
            MaterialFont::Plain,
            MaterialWeight::Regular,
            16.0,
            24.0,
            0.0,
        );

        let bold_style = style.with_weight(MaterialWeight::Bold);
        assert_eq!(bold_style.weight, iced::font::Weight::Bold);

        let large_style = style.with_size(20.0);
        assert_eq!(large_style.size(), 20.0);
    }

    #[test]
    fn test_typography_roles() {
        let roles = TypographyRole::all();
        assert_eq!(roles.len(), 15);
        assert_eq!(TypographyRole::BodyLarge.as_str(), "body-large");
    }

    #[test]
    fn test_typography_scaling() {
        let typography = MaterialTypography::default();
        let scaled = typography.with_scale(1.5);

        assert_eq!(scaled.body_large.size(), typography.body_large.size() * 1.5);
    }

    #[test]
    fn test_content_type_recommendations() {
        assert_eq!(
            get_recommended_role(ContentType::MainHeading),
            TypographyRole::HeadlineLarge
        );
        assert_eq!(
            get_recommended_role(ContentType::ButtonText),
            TypographyRole::LabelLarge
        );
    }
}
