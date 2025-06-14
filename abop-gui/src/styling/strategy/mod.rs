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
//! use crate::styling::strategy::{ComponentStyleStrategy, ButtonStyleVariant};
//! use crate::styling::material::MaterialTokens;
//!
//! let strategy = ButtonStyleVariant::Filled.get_strategy();
//! let styling = strategy.get_styling(
//!     ButtonState::Default,
//!     &tokens
//! );
//! ```

pub mod traits;
pub mod button;
pub mod checkbox;
pub mod states;
pub mod styling;
pub mod examples;

// Re-export main strategy components
pub use traits::{ComponentStyleStrategy, ComponentState};
pub use button::{ButtonStyleVariant, ButtonStyling};
pub use checkbox::{CheckboxStyleStrategy, CheckboxStyleVariant};
pub use states::{ButtonState, CheckboxState, InteractionState, ComponentInteractionState};
pub use styling::ComponentStyling;
