//! Repair handlers module

pub mod repair_handler;
pub mod library;
pub mod audiobook;
pub mod progress;
pub mod preferences;
pub mod file;
pub mod integrity;

// Re-export handler trait and implementations
pub use repair_handler::RepairHandler;
pub use library::LibraryRepairHandler;
pub use audiobook::AudiobookRepairHandler;
pub use progress::ProgressRepairHandler;
pub use preferences::PreferencesRepairHandler;
pub use file::FileRepairHandler;
pub use integrity::IntegrityRepairHandler;

/// Get all available repair handlers
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
