//! Material Design token system for ABOP GUI
//!
//! This module provides a comprehensive implementation of Material Design 3 tokens,
//! including the latest M3 Expressive style tokens. All tokens are based on the
//! official Material Web design kit and follow Material Design 3 specifications.
//!
//! ## Phase 2 Architecture
//!
//! The token system is now organized with trait-based separation of concerns:
//! - **Tokens**: Core token structures and semantic mappings
//! - **Helpers**: Specialized traits for elevation, animation, and component creation
//! - **Builders**: Builder patterns for customizable token creation (Phase 3 foundation)
//! - **Themes**: Theme management and dynamic switching (Phase 3 foundation)
//! - **Factories**: Component factory patterns (Phase 3 foundation)

// Core Material Design modules
pub mod colors; // DEPRECATED - Use unified_colors instead
pub mod components;
pub mod elevation;
pub mod motion;
pub mod seed;
pub mod shapes;
pub mod sizing;
pub mod spacing;
pub mod typography;
pub mod visual;
// pub mod md3_color;       // DEPRECATED - Use unified_colors instead

// Phase 2: New organizational modules
pub mod builders;
pub mod factories;
pub mod helpers;
pub mod themes;
pub mod tokens;

// Phase 3: Unified Material Design 3 System
pub mod color_utilities; // Advanced color utilities
pub mod enhanced_tokens; // Complete token system
pub mod migration_guide;
pub mod unified_colors; // THE unified color system // Migration helpers

// Re-export the UNIFIED Material Design types (Phase 3)
pub use color_utilities::{AccessibilityReport, ColorUtilities, ThemeUtilities};
pub use components::*;
pub use elevation::{ElevationLevel, MaterialElevation};
pub use enhanced_tokens::EnhancedMaterialTokens;
pub use motion::{Animation, AnimationPattern, AnimationState, EasingCurve, MotionTokens};
pub use seed::generate_palette_from_seed;
pub use shapes::{MaterialShapes, ShapeStyle};
pub use sizing::SizingTokens;
pub use spacing::SpacingTokens;
pub use typography::{MaterialTypography, TypographyRole};
pub use unified_colors::{ColorRole, MaterialColors, MaterialPalette, TonalPalette};
pub use visual::VisualTokens;

// Re-export Phase 2 structures
pub use tokens::{core::MaterialTokens, semantic::SemanticColors, states::*};

// Re-export helper traits for enhanced functionality
pub use helpers::{AnimationHelpers, ComponentHelpers, ElevationHelpers};

// Re-export Phase 3 preparation infrastructure
pub use builders::{MaterialTokensBuilder, ThemeBuilder};
pub use factories::MaterialComponentFactory;
pub use themes::{DynamicTheme, ThemeMode};

// Re-export Material Design 3 selection components
pub use components::{
    Checkbox, CheckboxBuilder, CheckboxState, ComponentSize, Switch, SwitchBuilder, SwitchState,
};
