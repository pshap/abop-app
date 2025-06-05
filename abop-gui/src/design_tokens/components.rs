//! Component-specific token groups for consistent styling

/// Component-specific token groups for consistent styling
#[derive(Debug, Clone)]
pub struct ComponentTokens {
    /// Button-specific tokens
    pub button: ButtonTokens,
    /// Input-specific tokens
    pub input: InputTokens,
    /// Card-specific tokens
    pub card: CardTokens,
    /// Modal-specific tokens
    pub modal: ModalTokens,
}

impl Default for ComponentTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentTokens {
    /// Create new component tokens with default values
    #[must_use]
    pub fn new() -> Self {
        Self {
            button: ButtonTokens::default(),
            input: InputTokens::default(),
            card: CardTokens::default(),
            modal: ModalTokens::default(),
        }
    }
}

/// Button component specific tokens
#[derive(Debug, Clone)]
pub struct ButtonTokens {
    /// Minimum button height for accessibility
    pub min_height: f32,
    /// Horizontal padding inside buttons
    pub padding_horizontal: f32,
    /// Vertical padding inside buttons
    pub padding_vertical: f32,
    /// Border radius for buttons
    pub border_radius: f32,
    /// Minimum width for buttons
    pub min_width: f32,
}

impl Default for ButtonTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonTokens {
    /// Create new button tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            min_height: 40.0,
            padding_horizontal: 16.0,
            padding_vertical: 8.0,
            border_radius: 8.0,
            min_width: 80.0,
        }
    }
}

/// Input component specific tokens
#[derive(Debug, Clone)]
pub struct InputTokens {
    /// Standard input field height
    pub height: f32,
    /// Internal padding for inputs
    pub padding: f32,
    /// Default border width
    pub border_width: f32,
    /// Border width when focused
    pub focus_border_width: f32,
    /// Border radius for inputs
    pub border_radius: f32,
}

impl Default for InputTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl InputTokens {
    /// Create new input tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            height: 36.0,
            padding: 12.0,
            border_width: 1.0,
            focus_border_width: 2.0,
            border_radius: 4.0,
        }
    }
}

/// Card component specific tokens
#[derive(Debug, Clone)]
pub struct CardTokens {
    /// Default padding inside cards
    pub padding: f32,
    /// Border radius for cards
    pub border_radius: f32,
    /// Default elevation for cards
    pub elevation: f32,
}

impl Default for CardTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl CardTokens {
    /// Create new card tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            padding: 16.0,
            border_radius: 12.0,
            elevation: 4.0,
        }
    }
}

/// Modal component specific tokens
#[derive(Debug, Clone)]
pub struct ModalTokens {
    /// Maximum width for modals
    pub max_width: f32,
    /// Default padding inside modals
    pub padding: f32,
    /// Border radius for modals
    pub border_radius: f32,
    /// Backdrop opacity
    pub backdrop_opacity: f32,
}

impl Default for ModalTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl ModalTokens {
    /// Create new modal tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_width: 600.0,
            padding: 24.0,
            border_radius: 16.0,
            backdrop_opacity: 0.5,
        }
    }
}
