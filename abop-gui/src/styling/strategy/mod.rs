//! Color Strategy System for Material Design 3 Components
//!
//! This module implements the strategy pattern for component styling, ensuring
//! consistent color application across all UI components while maintaining
//! accessibility and theme support.
//!
//! # Overview
//!
//! The strategy system provides a unified approach to component styling that:
//! - Eliminates direct color access in components
//! - Ensures accessibility compliance
//! - Supports all theme modes
//! - Provides variant-specific styling
//!
//! # Usage
//!
//! ```rust
//! use abop_gui::styling::strategy::{ComponentStyleStrategy, ButtonStyleVariant};
//! use abop_gui::styling::material::MaterialTokens;
//! use abop_gui::styling::strategy::traits::ComponentState;
//!
//! let tokens = MaterialTokens::default();
//! let strategy = ButtonStyleVariant::Filled.get_strategy();
//! let styling = strategy.get_styling(
//!     ComponentState::Default,
//!     &tokens
//! );
//! ```

pub mod traits;
pub mod button;
pub mod checkbox;
pub mod radio;
pub mod switch;
pub mod chip;
pub mod states;
pub mod styling;
pub mod examples;

// Re-export main strategy components
pub use traits::{ComponentStyleStrategy, ComponentState};
pub use button::{ButtonStyleVariant, ButtonStyling};
pub use checkbox::{CheckboxStyleStrategy, CheckboxStyleVariant};
pub use radio::{RadioStyleStrategy, RadioStyleVariant};
pub use switch::{SwitchStyleStrategy, SwitchStyleVariant};
pub use chip::{ChipStyleStrategy, ChipStyleVariant};
pub use states::{ButtonState, CheckboxState, InteractionState, ComponentInteractionState};
pub use styling::ComponentStyling;
