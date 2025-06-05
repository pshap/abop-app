//! UI-specific design tokens for visual treatments

/// Visual treatment constants for UI effects
#[derive(Debug, Clone)]
pub struct VisualTokens {
    /// 0.04 - Subtle hover opacity adjustment
    pub hover_opacity_adjustment: f32,
    /// 0.08 - Pressed state opacity adjustment
    pub pressed_opacity_adjustment: f32,
    /// 0.38 - Disabled state opacity
    pub disabled_opacity: f32,
    /// 0.12 - Standard opacity for surface overlays
    pub surface_overlay_opacity: f32,
    /// 1.0 - Standard border width
    pub border_width_standard: f32,
    /// 2.0 - Thick border width
    pub border_width_thick: f32,
    /// 0.2 - Animation duration in seconds (fast)
    pub animation_duration_fast: f32,
    /// 0.3 - Animation duration in seconds (standard)
    pub animation_duration_standard: f32,
    /// 0.4 - Animation duration in seconds (slow)
    pub animation_duration_slow: f32,
}

impl Default for VisualTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl VisualTokens {
    /// Create a new set of visual tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hover_opacity_adjustment: 0.04,
            pressed_opacity_adjustment: 0.08,
            disabled_opacity: 0.38,
            surface_overlay_opacity: 0.12,
            border_width_standard: 1.0,
            border_width_thick: 2.0,
            animation_duration_fast: 0.2,
            animation_duration_standard: 0.3,
            animation_duration_slow: 0.4,
        }
    }
}
