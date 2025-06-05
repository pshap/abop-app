//! Material Design 3 Context Menu Component
//!
//! This module provides the context menu component for right-click actions.

use crate::styling::material::MaterialTokens;
use crate::styling::material::components::menus::base_menu::{MenuComponent, Openable};
use crate::styling::material::components::menus::dropdown_menu::MaterialMenu;
use crate::styling::material::components::menus::menu_item::MenuItem;

use iced::Element;

/// Material Design 3 Context Menu component
///
/// Context menus provide additional actions for specific content.
/// They typically appear on right-click or long press.
#[derive(Debug, Clone)]
pub struct MaterialContextMenu {
    /// The underlying menu
    pub menu: MaterialMenu,
    /// The position where the menu should appear
    pub position: Option<(f32, f32)>,
}

impl Default for MaterialContextMenu {
    fn default() -> Self {
        Self {
            menu: MaterialMenu::new(),
            position: None,
        }
    }
}

impl MaterialContextMenu {
    /// Create a new context menu
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a menu item
    #[must_use]
    pub fn item(mut self, item: MenuItem) -> Self {
        self.menu = self.menu.item(item);
        self
    }

    /// Set the menu position
    #[must_use]
    pub const fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Some((x, y));
        self
    }

    /// Create the context menu element
    pub fn view<'a, Message>(
        &'a self,
        on_select: impl Fn(usize) -> Message + 'a + Clone,
        on_close: Message,
        tokens: &'a MaterialTokens,
    ) -> Element<'a, Message>
    where
        Message: 'a + Clone,
    {
        // Context menu uses the same rendering as dropdown menu
        // In a more complex implementation, we would position the menu at self.position
        self.menu.view(on_select, on_close, tokens)
    }
}

impl Openable for MaterialContextMenu {
    fn open(mut self, open: bool) -> Self {
        self.menu = self.menu.open(open);
        self
    }

    fn is_open(&self) -> bool {
        self.menu.is_open()
    }
}

impl MenuComponent for MaterialContextMenu {
    fn enabled(self, _enabled: bool) -> Self {
        // Context menus don't have an enabled state directly
        // This is a placeholder for the MenuComponent trait
        self
    }
}
