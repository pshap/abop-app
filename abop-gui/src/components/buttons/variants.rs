//! Button variants and related types

/// The visual variant of a button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ButtonVariant {
    /// A filled button with a solid background
    Filled,
    
    /// A filled button with a secondary, tonal background
    FilledTonal,
    
    /// An outlined button with a border
    Outlined,
    
    /// A text-only button with minimal visual treatment
    Text,
}

/// The size of a button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ButtonSize {
    /// Small size, typically 24-32px height
    Small,
    
    /// Medium size, typically 36-40px height (default)
    Medium,
    
    /// Large size, typically 48-52px height
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// The position of an icon relative to the label
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IconPosition {
    /// Icon appears before the label
    Leading,
    
    /// Icon appears after the label
    Trailing,
    
    /// Only the icon is shown (icon-only button)
    Only,
}

impl Default for IconPosition {
    fn default() -> Self {
        Self::Leading
    }
}

/// Icon size for different button sizes
pub(crate) fn icon_size(size: ButtonSize) -> f32 {
    match size {
        ButtonSize::Small => 18.0,
        ButtonSize::Medium => 20.0,
        ButtonSize::Large => 24.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_size_default() {
        assert_eq!(ButtonSize::default(), ButtonSize::Medium);
    }

    #[test]
    fn test_icon_position_default() {
        assert_eq!(IconPosition::default(), IconPosition::Leading);
    }

    #[test]
    fn test_icon_size() {
        assert_eq!(icon_size(ButtonSize::Small), 18.0);
        assert_eq!(icon_size(ButtonSize::Medium), 20.0);
        assert_eq!(icon_size(ButtonSize::Large), 24.0);
    }
}
