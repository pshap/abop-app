//! Main entry point for the ABOP GUI application

// Import from abop_gui crate
use abop_gui::app::App;
use abop_gui::assets;

use log::info;
use thiserror::Error;
use tracing_subscriber::EnvFilter;

// Import centralized types from abop-core
use abop_core::{Config, ServiceContainer};

/// Errors that can occur during application initialization
#[derive(Error, Debug)]
pub enum InitError {
    #[error("Failed to initialize logging: {0}")]
    Logging(#[from] tracing_subscriber::util::TryInitError),
    
    #[error("Failed to parse logging directive: {0}")]
    LogDirective(#[from] tracing_subscriber::filter::ParseError),
    
    #[error("Failed to load configuration: {0}")]
    Config(#[from] abop_core::AppError),
}

fn init_logging() -> Result<(), InitError> {
    let filter = EnvFilter::default()
        .add_directive("abop_gui=info".parse()?)
        .add_directive("abop_core=info".parse()?)
        .add_directive("iced=warn".parse()?);
        
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
        
    Ok(())
}

fn load_config_with_fallback() -> Result<Config, InitError> {
    Config::load().map_err(InitError::Config)
}

fn main() -> iced::Result {
    // Initialize logging - panic on failure since this is critical
    init_logging().expect("Failed to initialize logging");

    info!("Starting ABOP GUI with Iced 0.13.1 (new functional approach)...");

    // Register embedded fonts (Roboto only, Font Awesome handled by iced_font_awesome)
    assets::register_fonts();
    info!("Material Design fonts registered");

    // Load configuration with fallback to defaults
    let config = load_config_with_fallback()
        .unwrap_or_else(|e| {
            log::warn!("Failed to load configuration: {}. Using defaults.", e);
            Config::default()
        });

    // Initialize services
    let _services = ServiceContainer::new();

    // Create the application instance
    let app = App::new();

    // Run the application using the new modular approach with system font defaults
    iced::application(
        "ABOP - Audiobook Organizer & Processor",
        App::update,
        App::view,
    )
    .subscription(App::subscription)
    .theme(|app: &App| app.state.theme_mode.theme())
    .window_size(iced::Size::new(
        config.window.min_width as f32,
        config.window.min_height as f32,
    ))
    .run_with(move || (app, iced::Task::none()))
}
