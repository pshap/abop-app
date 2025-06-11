//! Selection component variant strategy implementations
//!
//! This module contains the concrete implementations of `SelectionStyleStrategy`
//! for each Material Design 3 selection component variant.

pub mod checkbox;
pub mod radio;
pub mod chip;
pub mod switch;

pub use checkbox::CheckboxStrategy;
pub use radio::RadioStrategy;
pub use chip::ChipStrategy;
pub use switch::SwitchStrategy;
