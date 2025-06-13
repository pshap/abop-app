//! Icon-related functionality for buttons

use iced::{Element, widget::{Row, Text}};
use super::variants::IconPosition;
use super::ButtonError;

/// Configuration for an icon in a button
#[derive(Debug, Clone)]
pub(crate) struct IconConfig<'a> {
    /// The name of the icon
    pub name: &'a str,
    
    /// The position of the icon relative to the label
    pub position: IconPosition,
    
    /// The size of the icon
    pub size: f32,
}

impl<'a> IconConfig<'a> {
    /// Create a new icon configuration
    pub fn new(name: &'a str, position: IconPosition, size: f32) -> Self {
        Self { name, position, size }
    }
    
    /// Render the icon as an element
    pub fn render(&self) -> Element<'a, ()> {
        iced_font_awesome::fa_icon_solid(self.name).size(self.size).into()
    }
    
    /// Convert the icon configuration to an element with the given message type
    pub fn to_element<M: 'a>(&self) -> Element<'a, M> {
        iced_font_awesome::fa_icon_solid(self.name).size(self.size).into()
    }
}

/// Create a button content with icon and label
pub(crate) fn button_content<'a, M: Clone + 'a>(
    label: Option<&'a str>,
    icon: Option<IconConfig<'a>>,
) -> Result<Element<'a, M>, ButtonError> {
    match (label, icon) {
        (Some(label_text), Some(icon_cfg)) => {
            // Button with both icon and label
            let icon_element = icon_cfg.to_element::<M>();
            
            let content: Row<'_, M> = match icon_cfg.position {
                IconPosition::Leading => Row::new()
                    .push(icon_element)
                    .push(Text::new(" ")) // Add some spacing
                    .push(Text::new(label_text)),
                    
                IconPosition::Trailing => Row::new()
                    .push(Text::new(label_text))
                    .push(Text::new(" ")) // Add some spacing
                    .push(icon_element),
                    
                IconPosition::Only => {
                    // If only icon is requested but label is provided, include it for accessibility
                    let mut row = Row::new().push(icon_element);
                    if !label_text.is_empty() {
                        row = row.push(Text::new(" ")).push(Text::new(label_text));
                    }
                    row
                }
            };
            
            Ok(content.into())
        }
        
        (Some(label_text), None) => {
            // Button with label only
            Ok(Text::new(label_text).into())
        }
        
        (None, Some(icon_cfg)) => {
            // Button with icon only
            Ok(icon_cfg.to_element::<M>())
        }
        
        (None, None) => {
            // No content provided - this is an error
            Err(ButtonError::MissingField(
                "Button must have either a label, an icon, or both"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Note: Actual icon rendering would require a proper iced renderer
    // These tests just verify the structure of the returned elements
    
    #[test]
    fn test_icon_config() {
        let icon = IconConfig::new("save", IconPosition::Leading, 24.0);
        assert_eq!(icon.name, "save");
        assert_eq!(icon.position, IconPosition::Leading);
        assert_eq!(icon.size, 24.0);
    }
    
    #[test]
    fn test_button_content_label_only() {
        let result = button_content::<()>(Some("Save"), None);
        assert!(result.is_ok());
        
        // Can't easily test the rendered content without a full iced renderer
    }
    
    #[test]
    fn test_button_content_icon_only() {
        let icon = IconConfig::new("save", IconPosition::Only, 24.0);
        let result = button_content::<()>(None, Some(icon));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_button_content_both() {
        let icon = IconConfig::new("save", IconPosition::Leading, 24.0);
        let result = button_content::<()>(Some("Save"), Some(icon));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_button_content_none() {
        let result = button_content::<()>(None, None);
        assert!(matches!(result, Err(ButtonError::MissingField(_))));
    }
}
