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
//! use abop_gui::styling::strategy::{ButtonStyleVariant, ButtonStyling};
//! use abop_gui::styling::material::MaterialTokens;
//! use abop_gui::styling::strategy::states::ComponentInteractionState;
//!
//! let tokens = MaterialTokens::default();
//! let strategy = ButtonStyleVariant::Filled.get_strategy();
//! let styling = strategy.get_styling(
//!     ComponentInteractionState::Default,
//!     &tokens
//! );
//!
//! // Apply the styling colors to your button component
//! // styling.background contains the appropriate background color
//! // styling.text_color contains the appropriate text color
//! // styling.border contains the appropriate border styling
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
