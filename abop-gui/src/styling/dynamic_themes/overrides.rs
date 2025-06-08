//! Type-safe component override system

use serde::{Deserialize, Serialize};

/// Component type enumeration for type-safe overrides
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComponentType {
    /// Button components (all variants)
    Button,
    /// Text input components  
    Input,
    /// Container/card components
    Container,
    /// Modal/dialog components
    Modal,
    /// Menu components
    Menu,
    /// Navigation components
    Navigation,
    /// Progress/feedback components
    Progress,
    /// Selection components (chips, switches, etc.)
    Selection,
}

/// Type-safe component override definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentOverrides {
    /// Button component overrides
    Button(ButtonOverride),
    /// Input component overrides
    Input(InputOverride),
    /// Container component overrides
    Container(ContainerOverride),
    /// Modal component overrides
    Modal(ModalOverride),
    /// Menu component overrides
    Menu(MenuOverride),
    /// Navigation component overrides
    Navigation(NavigationOverride),
    /// Progress component overrides
    Progress(ProgressOverride),
    /// Selection component overrides
    Selection(SelectionOverride),
}

/// Button component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonOverride {
    /// Override minimum height
    pub min_height: Option<f32>,
    /// Override horizontal padding
    pub padding_horizontal: Option<f32>,
    /// Override vertical padding
    pub padding_vertical: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override minimum width
    pub min_width: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override text color
    pub text_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override border width
    pub border_width: Option<f32>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
}

/// Input component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputOverride {
    /// Override input field height
    pub height: Option<f32>,
    /// Override internal padding
    pub padding: Option<f32>,
    /// Override border width
    pub border_width: Option<f32>,
    /// Override border width when focused
    pub focus_border_width: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override text color
    pub text_color: Option<String>,
    /// Override placeholder color
    pub placeholder_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override focus border color
    pub focus_border_color: Option<String>,
}

/// Container component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerOverride {
    /// Override padding inside containers
    pub padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override border width
    pub border_width: Option<f32>,
    /// Override margin
    pub margin: Option<f32>,
}

/// Modal component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalOverride {
    /// Override maximum width
    pub max_width: Option<f32>,
    /// Override padding inside modals
    pub padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override backdrop opacity
    pub backdrop_opacity: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
}

/// Menu component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuOverride {
    /// Override menu item height
    pub item_height: Option<f32>,
    /// Override menu padding
    pub padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override maximum height
    pub max_height: Option<f32>,
    /// Override minimum width
    pub min_width: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override item hover color
    pub item_hover_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
}

/// Navigation component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationOverride {
    /// Override navigation bar height
    pub bar_height: Option<f32>,
    /// Override item padding
    pub item_padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override active item color
    pub active_item_color: Option<String>,
    /// Override inactive item color
    pub inactive_item_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override elevation/shadow
    pub elevation: Option<f32>,
}

/// Progress component style overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressOverride {
    /// Override progress bar height
    pub bar_height: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override progress color
    pub progress_color: Option<String>,
    /// Override track color
    pub track_color: Option<String>,
    /// Override animation duration
    pub animation_duration: Option<f32>,
}

/// Selection component style overrides (chips, switches, checkboxes, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionOverride {
    /// Override item height
    pub height: Option<f32>,
    /// Override padding
    pub padding: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
    /// Override minimum width
    pub min_width: Option<f32>,
    /// Override background color
    pub background_color: Option<String>,
    /// Override selected background color
    pub selected_background_color: Option<String>,
    /// Override text color
    pub text_color: Option<String>,
    /// Override selected text color
    pub selected_text_color: Option<String>,
    /// Override border color
    pub border_color: Option<String>,
    /// Override selected border color
    pub selected_border_color: Option<String>,
    /// Override border width
    pub border_width: Option<f32>,
}

/// Type-safe component style override system
///
/// This replaces the previous HashMap<String, serde_json::Value> approach with
/// strongly-typed overrides that correspond to actual component properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentOverride {
    /// The type of component being overridden
    pub component_type: ComponentType,
    /// Optional variant identifier (e.g., "primary", "secondary")
    pub variant: Option<String>,
    /// The actual override values
    pub overrides: ComponentOverrides,
}

impl ComponentOverride {
    /// Create a new button override
    pub fn button() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Button)
    }

    /// Create a new input override
    pub fn input() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Input)
    }

    /// Create a new container override
    pub fn container() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Container)
    }

    /// Create a new modal override
    pub fn modal() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Modal)
    }

    /// Create a new menu override
    pub fn menu() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Menu)
    }

    /// Create a new navigation override
    pub fn navigation() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Navigation)
    }

    /// Create a new progress override
    pub fn progress() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Progress)
    }

    /// Create a new selection override
    pub fn selection() -> ComponentOverrideBuilder {
        ComponentOverrideBuilder::new(ComponentType::Selection)
    }

    /// Validate the component override configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate that the component type matches the override type
        match (&self.component_type, &self.overrides) {
            (ComponentType::Button, ComponentOverrides::Button(_)) => Ok(()),
            (ComponentType::Input, ComponentOverrides::Input(_)) => Ok(()),
            (ComponentType::Container, ComponentOverrides::Container(_)) => Ok(()),
            (ComponentType::Modal, ComponentOverrides::Modal(_)) => Ok(()),
            (ComponentType::Menu, ComponentOverrides::Menu(_)) => Ok(()),
            (ComponentType::Navigation, ComponentOverrides::Navigation(_)) => Ok(()),
            (ComponentType::Progress, ComponentOverrides::Progress(_)) => Ok(()),
            (ComponentType::Selection, ComponentOverrides::Selection(_)) => Ok(()),
            _ => Err(format!(
                "Component type {:?} does not match override type",
                self.component_type
            )),
        }
    }
}

/// Builder for creating component overrides with fluent API
pub struct ComponentOverrideBuilder {
    component_type: ComponentType,
    variant: Option<String>,
}

impl ComponentOverrideBuilder {
    /// Create a new builder for the specified component type
    pub fn new(component_type: ComponentType) -> Self {
        Self {
            component_type,
            variant: None,
        }
    }

    /// Set the component variant
    pub fn variant<S: Into<String>>(mut self, variant: S) -> Self {
        self.variant = Some(variant.into());
        self
    }

    /// Build a button override
    pub fn button_override(self, override_def: ButtonOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Button(override_def),
        }
    }

    /// Build an input override
    pub fn input_override(self, override_def: InputOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Input(override_def),
        }
    }

    /// Build a container override
    pub fn container_override(self, override_def: ContainerOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Container(override_def),
        }
    }

    /// Build a modal override
    pub fn modal_override(self, override_def: ModalOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Modal(override_def),
        }
    }

    /// Build a menu override
    pub fn menu_override(self, override_def: MenuOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Menu(override_def),
        }
    }

    /// Build a navigation override
    pub fn navigation_override(self, override_def: NavigationOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Navigation(override_def),
        }
    }

    /// Build a progress override
    pub fn progress_override(self, override_def: ProgressOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Progress(override_def),
        }
    }

    /// Build a selection override
    pub fn selection_override(self, override_def: SelectionOverride) -> ComponentOverride {
        ComponentOverride {
            component_type: self.component_type,
            variant: self.variant,
            overrides: ComponentOverrides::Selection(override_def),
        }
    }
}
