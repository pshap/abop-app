//! Material Design 3 constants for selection components
//!
//! This module provides all the constants needed for consistent Material Design 3
//! selection component styling across checkbox, radio, chip, and switch variants.

/// Material Design 3 constants for selection components
pub struct SelectionConstants {
    pub opacity: OpacityConstants,
    pub border_radius: BorderRadiusConstants,
    pub size: SizeConstants,
    pub color: ColorConstants,
}

impl SelectionConstants {
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

impl OpacityConstants {
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

impl BorderRadiusConstants {
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
    /// Component sizes in pixels
    pub small_px: f32,
    pub medium_px: f32,
    pub large_px: f32,
    
    /// Touch target sizes
    pub small_touch: f32,
    pub medium_touch: f32,
    pub large_touch: f32,

    /// Border widths
    pub small_border: f32,
    pub medium_border: f32,
    pub large_border: f32,

    /// Text sizes
    pub small_text: f32,
    pub medium_text: f32,
    pub large_text: f32,

    /// Padding values
    pub small_padding: f32,
    pub medium_padding: f32,
    pub large_padding: f32,
}

impl SizeConstants {
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

impl ColorConstants {
    pub const fn new() -> Self {
        Self {
            chip_pressed_darken: 0.1,
        }
    }
}

/// Global constants instance
pub const SELECTION_CONSTANTS: SelectionConstants = SelectionConstants::new();

// Legacy module constants for backward compatibility
pub mod opacity {
    use super::SELECTION_CONSTANTS;
    pub const DISABLED: f32 = SELECTION_CONSTANTS.opacity.disabled;
    pub const PRESSED: f32 = SELECTION_CONSTANTS.opacity.pressed;
    pub const HOVER: f32 = SELECTION_CONSTANTS.opacity.hover;
    pub const FOCUS: f32 = SELECTION_CONSTANTS.opacity.focus;
    #[allow(dead_code)]
    pub const DISABLED_SURFACE: f32 = SELECTION_CONSTANTS.opacity.disabled_surface;
}

pub mod border_radius {
    use super::SELECTION_CONSTANTS;
    pub const CHECKBOX: f32 = SELECTION_CONSTANTS.border_radius.checkbox;
    pub const RADIO: f32 = SELECTION_CONSTANTS.border_radius.radio;
    pub const CHIP: f32 = SELECTION_CONSTANTS.border_radius.chip;
    pub const SWITCH: f32 = SELECTION_CONSTANTS.border_radius.switch;
}

pub mod size {
    use super::SELECTION_CONSTANTS;
    pub const SMALL_PX: f32 = SELECTION_CONSTANTS.size.small_px;
    pub const MEDIUM_PX: f32 = SELECTION_CONSTANTS.size.medium_px;
    pub const LARGE_PX: f32 = SELECTION_CONSTANTS.size.large_px;
    #[allow(dead_code)]
    pub const SMALL_TOUCH: f32 = SELECTION_CONSTANTS.size.small_touch;
    #[allow(dead_code)]
    pub const MEDIUM_TOUCH: f32 = SELECTION_CONSTANTS.size.medium_touch;
    #[allow(dead_code)]
    pub const LARGE_TOUCH: f32 = SELECTION_CONSTANTS.size.large_touch;
    pub const SMALL_BORDER: f32 = SELECTION_CONSTANTS.size.small_border;
    pub const MEDIUM_BORDER: f32 = SELECTION_CONSTANTS.size.medium_border;
    pub const LARGE_BORDER: f32 = SELECTION_CONSTANTS.size.large_border;
    pub const SMALL_TEXT: f32 = SELECTION_CONSTANTS.size.small_text;
    pub const MEDIUM_TEXT: f32 = SELECTION_CONSTANTS.size.medium_text;
    pub const LARGE_TEXT: f32 = SELECTION_CONSTANTS.size.large_text;
    pub const SMALL_PADDING: f32 = SELECTION_CONSTANTS.size.small_padding;
    pub const MEDIUM_PADDING: f32 = SELECTION_CONSTANTS.size.medium_padding;
    pub const LARGE_PADDING: f32 = SELECTION_CONSTANTS.size.large_padding;
}

pub mod color {
    use super::SELECTION_CONSTANTS;
    pub const CHIP_PRESSED_DARKEN: f32 = SELECTION_CONSTANTS.color.chip_pressed_darken;
}
