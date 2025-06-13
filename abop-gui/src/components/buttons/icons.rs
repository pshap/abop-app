//! Icon-related functionality for buttons

use iced::Element;
use super::variants::IconPosition;

/// Configuration for an icon in a button
#[derive(Debug, Clone)]
pub(crate) struct IconConfig<'a> {
    /// The name of the icon
    pub name: &'a str,
    
    /// The size of the icon
    pub size: f32,
}

impl<'a> IconConfig<'a> {
    /// Create a new icon configuration
    pub fn new(name: &'a str, _position: IconPosition, size: f32) -> Self {
        Self { name, size }
    }
    
    /// Convert the icon configuration to an element with the given message type
    pub fn to_element<M: 'a>(&self) -> Element<'a, M> {
        iced_font_awesome::fa_icon_solid(self.name).size(self.size).into()
    }
}
