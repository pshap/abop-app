//! Main entry point for the ABOP GUI application

// Import from abop_gui crate
use abop_gui::app::App;
use abop_gui::assets;

use log::info;
use thiserror::Error;
use tracing_subscriber::EnvFilter;
const DEFAULT_LOG_DIRECTIVES: &str = "abop_gui=info, abop_core=info, iced=warn";

// Import configuration from abop-core
// NOTE: Direct dependency on concrete Config type is acceptable for the main entry point.
// Future refactoring could introduce a ConfigProvider trait if needed for testing or modularity.
use abop_core::Config;

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
    // Respect RUST_LOG while providing sane defaults; preserve parse error context in fallback
    let filter = match EnvFilter::try_from_default_env() {
        Ok(f) => f,
        Err(e) => {
            // Logging may not be initialized yet; use stderr to preserve context
            eprintln!(
                "RUST_LOG parse error: {e}. Falling back to defaults: {DEFAULT_LOG_DIRECTIVES}"
            );
            let mut f = EnvFilter::default();
            // Defaults if RUST_LOG is not provided
            for directive in DEFAULT_LOG_DIRECTIVES.split(',') {
                let directive = directive.trim();
                f = f.add_directive(directive.parse().expect("static filter"));
            }
            f
        }
    };

    // Initialize tracing subscriber (handles log crate compatibility automatically)
    // Use try_init() in case of test double-initialization
    if tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .try_init()
        .is_err()
    {
        eprintln!(
            "Warning: Tracing subscriber already initialized (test-mode double-initialization)"
        );
    }
    Ok(())
}

fn load_config_with_fallback() -> Result<Config, InitError> {
    Config::load().map_err(InitError::Config)
}

fn main() -> iced::Result {
    // Initialize logging - panic on failure since this is critical
    init_logging().expect("Failed to initialize logging");

    info!("Starting ABOP GUI with Iced 0.13.1 (trait-based approach)...");

    // Load configuration with fallback to defaults
    let config = load_config_with_fallback().unwrap_or_else(|e| {
        log::warn!("Failed to load configuration: {e}. Using defaults.");
        Config::default()
    });

    // Run the application using the Application trait with proper window settings
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .window_size(iced::Size::new(
            config.window.initial_width as f32,
            config.window.initial_height as f32,
        ))
        .scale_factor(config.ui.scale_factor)
        .run_with(App::initial)
        .map_err(|e| {
            log::error!("Application failed to run: {e}");
            e
        })
}
