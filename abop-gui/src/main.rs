//! Main entry point for the ABOP GUI application

// Import from abop_gui crate
use abop_gui::app::App;
use abop_gui::assets;

use log::info;
use tracing_subscriber::EnvFilter;

// Import centralized types from abop-core
use abop_core::{Config, ServiceContainer};

fn main() -> iced::Result {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::default()
                .add_directive("abop_gui=info".parse().unwrap())
                .add_directive("abop_core=info".parse().unwrap())
                .add_directive("iced=warn".parse().unwrap()),
        )
        .init();

    info!("Starting ABOP GUI with Iced 0.13.1 (new functional approach)...");

    // Register embedded fonts (Roboto only, Font Awesome handled by iced_font_awesome)
    assets::register_fonts();
    info!("Material Design fonts registered");

    // Load configuration
    let config = Config::load().unwrap_or_default();

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
