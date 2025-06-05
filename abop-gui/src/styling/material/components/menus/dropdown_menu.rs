//! Material Design 3 Dropdown Menu Component
//!
//! This module provides the standard dropdown menu component.

use iced::{Element, widget::Column};

use crate::styling::material::MaterialTokens;
use crate::styling::material::components::menu_constants::dimensions;
use crate::styling::material::components::menu_container_style::MenuContainerVariant;
use crate::styling::material::components::menus::base_menu::{
    HasElevation, HasMaxHeight, HasMinWidth, Openable, create_empty_element, create_menu_container,
    get_shadow_properties,
};
use crate::styling::material::components::menus::menu_item::MenuItem;

/// Material Design 3 Menu component
///
/// Menus display a list of choices on temporary surfaces. They appear when
/// users interact with a button, action, or other control. This implementation
/// follows Material Design 3 specifications for menu behavior and styling.
#[derive(Debug, Clone)]
pub struct MaterialMenu {
    /// The menu items
    pub items: Vec<MenuItem>,
    /// Whether the menu is currently open
    pub is_open: bool,
    /// The elevation level of the menu
    pub elevation: u8,
    /// Optional minimum width
    pub min_width: Option<f32>,
    /// Optional maximum height before scrolling
    pub max_height: Option<f32>,
}

impl Default for MaterialMenu {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            is_open: false,
            elevation: 2,
            min_width: Some(dimensions::DEFAULT_MENU_MIN_WIDTH),
            max_height: Some(dimensions::DEFAULT_MENU_MAX_HEIGHT),
        }
    }
}

impl MaterialMenu {
    /// Create a new menu
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a menu item
    #[must_use]
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Create the menu element
    pub fn view<'a, Message>(
        &'a self,
        on_select: impl Fn(usize) -> Message + 'a + Clone,
        _on_close: Message,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message>
    where
        Message: 'a + Clone,
    {
        if !self.is_open {
            return create_empty_element();
        }

        // Get elevation properties
        let elevation_level = self.get_elevation_level();
        let (shadow_offset_y, blur_radius) = get_shadow_properties(elevation_level, tokens);

        // Build menu items
        let mut column = Column::new();
        for (index, item) in self.items.iter().enumerate() {
            let on_select_clone = on_select.clone();
            column = column.push(item.view(index, on_select_clone, tokens));
        }

        // Create the menu container
        create_menu_container(
            column,
            MenuContainerVariant::DropdownMenu,
            self.min_width,
            self.max_height,
            shadow_offset_y,
            blur_radius,
            tokens,
        )
    }
}

impl Openable for MaterialMenu {
    fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    fn is_open(&self) -> bool {
        self.is_open
    }
}

impl HasMinWidth for MaterialMenu {
    fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    fn get_min_width(&self) -> Option<f32> {
        self.min_width
    }
}

impl HasMaxHeight for MaterialMenu {
    fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    fn get_max_height(&self) -> Option<f32> {
        self.max_height
    }
}

impl HasElevation for MaterialMenu {
    fn elevation(mut self, elevation: u8) -> Self {
        self.elevation = elevation.min(5);
        self
    }

    fn get_elevation(&self) -> u8 {
        self.elevation
    }
}
