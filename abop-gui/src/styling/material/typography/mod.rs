//! Material Design 3 Typography System
//!
//! Implements the complete Material Design 3 typography system including:
//! - Type scale with all 15 typography roles
//! - Font families and weights
//! - Integration with Iced text styling

pub mod builder;
pub mod constants;
pub mod font_mapping;
pub mod roles;
pub mod scale;
pub mod tests;
pub mod utils;

// Re-export main types for compatibility
pub use builder::{TypographyBuilder, TypographyConfig};
pub use constants::*;
pub use font_mapping::{MaterialFont, MaterialWeight};
pub use roles::TypographyRole;
pub use scale::{MaterialTypography, TypeStyle};
pub use utils::{ContentType, calculate_line_height, get_recommended_role, px_to_rem, rem_to_px};
