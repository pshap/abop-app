//! Repair handlers module

pub mod audiobook;
pub mod file;
pub mod integrity;
pub mod library;
pub mod preferences;
pub mod progress;
pub mod repair_handler;

// Re-export handler trait and implementations
pub use audiobook::AudiobookRepairHandler;
pub use file::FileRepairHandler;
pub use integrity::IntegrityRepairHandler;
pub use library::LibraryRepairHandler;
pub use preferences::PreferencesRepairHandler;
pub use progress::ProgressRepairHandler;
pub use repair_handler::RepairHandler;

/// Get all available repair handlers
#[must_use]
pub fn get_all_handlers() -> Vec<Box<dyn RepairHandler>> {
    vec![
        Box::new(LibraryRepairHandler),
        Box::new(AudiobookRepairHandler),
        Box::new(ProgressRepairHandler),
        Box::new(PreferencesRepairHandler),
        Box::new(FileRepairHandler),
        Box::new(IntegrityRepairHandler),
    ]
}
