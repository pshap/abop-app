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
//!
//! // Configure button styling based on the strategy's recommendations
//! // to maintain consistent theming and accessibility standards
//! button.set_background_color(styling.background_color);
//! button.set_text_color(styling.text_color);
//! ```

pub mod button;
pub mod checkbox;
pub mod chip;
pub mod examples;
pub mod radio;
pub mod states;
pub mod styling;
pub mod switch;
pub mod traits;

// Re-export main strategy components
pub use button::{ButtonStyleVariant, ButtonStyling};
pub use checkbox::{CheckboxStyleStrategy, CheckboxStyleVariant};
pub use chip::{ChipStyleStrategy, ChipStyleVariant};
pub use radio::{RadioStyleStrategy, RadioStyleVariant};
pub use states::{ButtonState, CheckboxState, ComponentInteractionState, InteractionState};
pub use styling::ComponentStyling;
pub use switch::{SwitchStyleStrategy, SwitchStyleVariant};
pub use traits::{ComponentState, ComponentStyleStrategy};
