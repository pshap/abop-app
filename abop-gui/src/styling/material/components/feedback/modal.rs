//! Material Design 3 Modal Components
//!
//! This module provides Material Design 3 modal overlay components for
//! creating modal dialogs and other overlay content.
//!
//! Note: Modal functionality has been moved to the main view module.
//! For modal overlays, use the modal functions in views/mod.rs.

/// Modal state for component tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalState {
    /// Modal is visible
    Visible,
    /// Modal is hidden
    Hidden,
    /// Modal is being dismissed
    Dismissed,
}
