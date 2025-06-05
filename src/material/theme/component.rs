//! Theme component for Iced applications
//!
//! This module provides components for managing theme state in Iced applications.
//! It includes a higher-order component that wraps your application and provides
//! theme state management.

use crate::material::color::Theme;
use iced::widget::container;
use iced::{Element, Length, Renderer};
use std::sync::Arc;

/// A component that provides theme state to its children
#[derive(Debug, Clone)]
pub struct ThemeComponent<Message> {
    /// The current theme
    pub theme: Arc<Theme>,
    /// The content to render with the theme
    pub content: Element<'static, Message>,
}

impl<Message> ThemeComponent<Message> 
where
    Message: 'static + Clone,
{
    /// Create a new theme component with the default light theme
    pub fn new(content: impl Into<Element<'static, Message>>) -> Self {
        Self {
            theme: Arc::new(Theme::light()),
            content: content.into(),
        }
    }

    /// Set the theme for this component
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = Arc::new(theme);
        self
    }

    /// Get a reference to the current theme
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Get a clone of the current theme
    pub fn theme_arc(&self) -> Arc<Theme> {
        self.theme.clone()
    }
}

impl<Message> iced::widget::Component<Message, Renderer> for ThemeComponent<Message> 
where
    Message: 'static + Clone,
{
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        // Wrap the content in a container with the theme
        container(self.content.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

/// A trait for components that can be themed
pub trait ThemedComponent {
    /// Get the current theme
    fn theme(&self) -> &Theme;
    
    /// Check if the current theme is dark
    fn is_dark(&self) -> bool {
        self.theme().is_dark()
    }
    
    /// Check if the current theme is light
    fn is_light(&self) -> bool {
        self.theme().is_light()
    }
}

impl<Message> ThemedComponent for ThemeComponent<Message> {
    fn theme(&self) -> &Theme {
        &self.theme
    }
}

/// A helper function to create a new theme component
pub fn theme<Message>(
    content: impl Into<Element<'static, Message>>,
) -> ThemeComponent<Message>
where
    Message: 'static + Clone,
{
    ThemeComponent::new(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::widget::text;
    use crate::material::color::ThemeVariant;

    #[test]
    fn test_theme_component_creation() {
        let component = ThemeComponent::new(text("Test"));
        assert!(component.theme().is_light());
    }

    #[test]
    fn test_theme_component_with_theme() {
        let mut dark_theme = Theme::light();
        dark_theme.variant = ThemeVariant::Dark;
        
        let component = ThemeComponent::new(text("Test")).with_theme(dark_theme);
        assert!(component.theme().is_dark());
    }
}
