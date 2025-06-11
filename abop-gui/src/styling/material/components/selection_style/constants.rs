//! Material Design 3 constants for selection components
//!
//! This module provides all the constants needed for consistent Material Design 3
//! selection component styling across checkbox, radio, chip, and switch variants.

/// Material Design 3 constants for selection components
pub struct SelectionConstants {
    /// Opacity values for different states (disabled, pressed, hover, focus)
    pub opacity: OpacityConstants,
    /// Border radius values for different component variants
    pub border_radius: BorderRadiusConstants,
    /// Size-related constants for selection components
    pub size: SizeConstants,
    /// Color-related constants and effects
    pub color: ColorConstants,
}

impl Default for SelectionConstants {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectionConstants {
    /// Creates a new instance of SelectionConstants with Material Design 3 values
    pub const fn new() -> Self {
        Self {
            opacity: OpacityConstants::new(),
            border_radius: BorderRadiusConstants::new(),
            size: SizeConstants::new(),
            color: ColorConstants::new(),
        }
    }
}

/// Opacity values following Material Design 3 specifications
pub struct OpacityConstants {
    /// Disabled state opacity (Material Design 3 specification)
    pub disabled: f32,
    /// Pressed state opacity for state layers
    pub pressed: f32,
    /// Hover state opacity for state layers  
    pub hover: f32,
    /// Focus state opacity for state layers
    pub focus: f32,
    /// Surface overlay opacity for disabled backgrounds
    pub disabled_surface: f32,
}

impl Default for OpacityConstants {
    fn default() -> Self {
        Self::new()
    }
}

impl OpacityConstants {
    /// Creates a new instance of OpacityConstants with Material Design 3 values
    pub const fn new() -> Self {
        Self {
            disabled: 0.38,
            pressed: 0.12,
            hover: 0.08,
            focus: 0.12,
            disabled_surface: 0.12,
        }
    }
}

/// Border radius values for different component variants
pub struct BorderRadiusConstants {
    /// Checkbox border radius
    pub checkbox: f32,
    /// Radio button border radius (circular)
    pub radio: f32,
    /// Chip border radius
    pub chip: f32,
    /// Switch border radius
    pub switch: f32,
}

impl Default for BorderRadiusConstants {
    fn default() -> Self {
        Self::new()
    }
}

impl BorderRadiusConstants {
    /// Creates a new instance of BorderRadiusConstants with Material Design 3 values
    pub const fn new() -> Self {
        Self {
            checkbox: 2.0,
            radio: 12.0,
            chip: 8.0,
            switch: 16.0,
        }
    }
}

/// Size constants for components
pub struct SizeConstants {
    /// Component sizes in pixels for small components
    pub small_px: f32,
    /// Component sizes in pixels for medium components  
    pub medium_px: f32,
    /// Component sizes in pixels for large components
    pub large_px: f32,

    /// Touch target size for small components
    pub small_touch: f32,
    /// Touch target size for medium components
    pub medium_touch: f32,
    /// Touch target size for large components
    pub large_touch: f32,

    /// Border width for small components
    pub small_border: f32,
    /// Border width for medium components
    pub medium_border: f32,
    /// Border width for large components
    pub large_border: f32,

    /// Text size for small components
    pub small_text: f32,
    /// Text size for medium components
    pub medium_text: f32,
    /// Text size for large components
    pub large_text: f32,

    /// Padding value for small components
    pub small_padding: f32,
    /// Padding value for medium components
    pub medium_padding: f32,
    /// Padding value for large components
    pub large_padding: f32,
}

impl Default for SizeConstants {
    fn default() -> Self {
        Self::new()
    }
}

impl SizeConstants {
    /// Creates a new instance of SizeConstants with Material Design 3 values
    pub const fn new() -> Self {
        Self {
            small_px: 16.0,
            medium_px: 20.0,
            large_px: 24.0,
            small_touch: 32.0,
            medium_touch: 40.0,
            large_touch: 48.0,
            small_border: 1.5,
            medium_border: 2.0,
            large_border: 2.5,
            small_text: 12.0,
            medium_text: 14.0,
            large_text: 16.0,
            small_padding: 4.0,
            medium_padding: 8.0,
            large_padding: 12.0,
        }
    }
}

/// Color modifier constants
pub struct ColorConstants {
    /// Darken amount for pressed chip states
    pub chip_pressed_darken: f32,
}

impl Default for ColorConstants {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorConstants {
    /// Creates a new instance of ColorConstants with Material Design 3 values
    pub const fn new() -> Self {
        Self {
            chip_pressed_darken: 0.1,
        }
    }
}

/// Global constants instance
pub const SELECTION_CONSTANTS: SelectionConstants = SelectionConstants::new();

// Legacy module constants for backward compatibility
/// Opacity constants for different interaction states
pub mod opacity {
    use super::SELECTION_CONSTANTS;
    /// Opacity value for disabled state elements
    pub const DISABLED: f32 = SELECTION_CONSTANTS.opacity.disabled;
    /// Opacity value for pressed state overlay
    pub const PRESSED: f32 = SELECTION_CONSTANTS.opacity.pressed;
    /// Opacity value for hover state overlay
    pub const HOVER: f32 = SELECTION_CONSTANTS.opacity.hover;
    /// Opacity value for focus state overlay
    pub const FOCUS: f32 = SELECTION_CONSTANTS.opacity.focus;
    /// Opacity value for disabled surface elements
    #[allow(dead_code)]
    pub const DISABLED_SURFACE: f32 = SELECTION_CONSTANTS.opacity.disabled_surface;
}

/// Border radius constants for different component variants
pub mod border_radius {
    use super::SELECTION_CONSTANTS;
    /// Border radius for checkbox components
    pub const CHECKBOX: f32 = SELECTION_CONSTANTS.border_radius.checkbox;
    /// Border radius for radio button components
    pub const RADIO: f32 = SELECTION_CONSTANTS.border_radius.radio;
    /// Border radius for chip components
    pub const CHIP: f32 = SELECTION_CONSTANTS.border_radius.chip;
    /// Border radius for switch components
    pub const SWITCH: f32 = SELECTION_CONSTANTS.border_radius.switch;
}

/// Size-related constants for all selection components
pub mod size {
    use super::SELECTION_CONSTANTS;
    /// Small component size in pixels
    pub const SMALL_PX: f32 = SELECTION_CONSTANTS.size.small_px;
    /// Medium component size in pixels
    pub const MEDIUM_PX: f32 = SELECTION_CONSTANTS.size.medium_px;
    /// Large component size in pixels
    pub const LARGE_PX: f32 = SELECTION_CONSTANTS.size.large_px;
    /// Small component touch target size
    #[allow(dead_code)]
    pub const SMALL_TOUCH: f32 = SELECTION_CONSTANTS.size.small_touch;
    /// Medium component touch target size
    #[allow(dead_code)]
    pub const MEDIUM_TOUCH: f32 = SELECTION_CONSTANTS.size.medium_touch;
    /// Large component touch target size
    #[allow(dead_code)]
    pub const LARGE_TOUCH: f32 = SELECTION_CONSTANTS.size.large_touch;
    /// Small component border width
    pub const SMALL_BORDER: f32 = SELECTION_CONSTANTS.size.small_border;
    /// Medium component border width
    pub const MEDIUM_BORDER: f32 = SELECTION_CONSTANTS.size.medium_border;
    /// Large component border width
    pub const LARGE_BORDER: f32 = SELECTION_CONSTANTS.size.large_border;
    /// Small component text size
    pub const SMALL_TEXT: f32 = SELECTION_CONSTANTS.size.small_text;
    /// Medium component text size
    pub const MEDIUM_TEXT: f32 = SELECTION_CONSTANTS.size.medium_text;
    /// Large component text size
    pub const LARGE_TEXT: f32 = SELECTION_CONSTANTS.size.large_text;
    /// Small component padding
    pub const SMALL_PADDING: f32 = SELECTION_CONSTANTS.size.small_padding;
    /// Medium component padding
    pub const MEDIUM_PADDING: f32 = SELECTION_CONSTANTS.size.medium_padding;
    /// Large component padding
    pub const LARGE_PADDING: f32 = SELECTION_CONSTANTS.size.large_padding;
}

/// Color effect constants for special styling needs
pub mod color {
    use super::SELECTION_CONSTANTS;
    /// Darken amount for pressed chip states to enhance visual feedback
    pub const CHIP_PRESSED_DARKEN: f32 = SELECTION_CONSTANTS.color.chip_pressed_darken;
}
