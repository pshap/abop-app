//! Theme management for Material Design 3
//!
//! This module provides components and utilities for managing the application's theme
//! and making it accessible throughout the component hierarchy.

mod component;
pub use component::{ThemeComponent, ThemedComponent, theme};

use crate::material::color::Theme;
use iced::widget::container;
use iced::{Element, Length, Renderer};
use std::sync::Arc;

/// A theme provider that makes the current theme available to all child components
#[derive(Debug, Clone)]
pub struct ThemeProvider<Message> {
    /// The current theme
    pub theme: Arc<Theme>,
    /// The content to render with the theme
    pub content: Element<'static, Message>,
}

impl<Message> ThemeProvider<Message> 
where
    Message: 'static + Clone,
{
    /// Create a new theme provider with the default light theme
    pub fn new(content: impl Into<Element<'static, Message>>) -> Self {
        Self {
            theme: Arc::new(Theme::light()),
            content: content.into(),
        }
    }

    /// Create a new theme provider with the dark theme
    pub fn dark(content: impl Into<Element<'static, Message>>) -> Self {
        Self {
            theme: Arc::new(Theme::dark()),
            content: content.into(),
        }
    }

    /// Set the theme variant (light or dark)
    pub fn with_variant(mut self, variant: crate::material::color::ThemeVariant) -> Self {
        self.theme = match variant {
            crate::material::color::ThemeVariant::Light => Arc::new(Theme::light()),
            crate::material::color::ThemeVariant::Dark => Arc::new(Theme::dark()),
        };
        self
    }

    /// Set a custom theme
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = Arc::new(theme);
        self
    }

    /// Get the current theme
    pub fn theme(&self) -> Arc<Theme> {
        self.theme.clone()
    }
}

impl<Message> iced::widget::Component<Message, Renderer> for ThemeProvider<Message> 
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
pub trait Themed {
    /// Get the current theme
    fn theme(&self) -> Arc<Theme>;
    
    /// Check if the current theme is dark
    fn is_dark(&self) -> bool {
        self.theme().is_dark()
    }
    
    /// Check if the current theme is light
    fn is_light(&self) -> bool {
        self.theme().is_light()
    }
}

impl<Message> Themed for ThemeProvider<Message> {
    fn theme(&self) -> Arc<Theme> {
        self.theme()
    }
}

/// A hook for accessing the current theme from within a component
pub fn use_theme<Message>() -> Arc<Theme> 
where
    Message: 'static,
{
    // In a real implementation, this would use iced's context system
    // to access the theme from the component hierarchy
    // For now, we'll return a default theme
    Arc::new(Theme::light())
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::widget::text;
    use crate::material::color::ThemeVariant;

    #[test]
    fn test_theme_provider_creation() {
        let provider = ThemeProvider::new(text("Test"));
        assert!(provider.theme().is_light());

        let provider = ThemeProvider::dark(text("Test"));
        assert!(provider.theme().is_dark());
    }

    #[test]
    fn test_theme_provider_variant() {
        let provider = ThemeProvider::new(text("Test")).with_variant(ThemeVariant::Dark);
        assert!(provider.theme().is_dark());

        let provider = ThemeProvider::dark(text("Test")).with_variant(ThemeVariant::Light);
        assert!(provider.theme().is_light());
    }
}
