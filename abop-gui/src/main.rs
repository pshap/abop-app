//! Main entry point for the ABOP GUI application

// Import from abop_gui crate
use abop_gui::app::App;

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
            // Only complain when RUST_LOG was actually set to something invalid
            if std::env::var_os("RUST_LOG").is_some() {
                eprintln!(
                    "Invalid RUST_LOG ({e}). Falling back to defaults: {DEFAULT_LOG_DIRECTIVES}"
                );
            }
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

/// Window size settings from the configuration, with sane fallbacks
fn window_settings(window: &abop_core::config::WindowConfig) -> iced::window::Settings {
    let dimension = |value: u32, fallback: f32| {
        u16::try_from(value)
            .ok()
            .filter(|v| *v > 0)
            .map_or(fallback, f32::from)
    };
    let min_size = iced::Size::new(
        dimension(window.min_width, 800.0),
        dimension(window.min_height, 600.0),
    );
    let size = iced::Size::new(
        dimension(window.initial_width, 1200.0).max(min_size.width),
        dimension(window.initial_height, 800.0).max(min_size.height),
    );
    iced::window::Settings {
        size,
        min_size: Some(min_size),
        ..iced::window::Settings::default()
    }
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

    let scale_factor = f64::from(config.ui.scale_factor);

    // Optional first argument: a library folder to open and scan on startup
    let initial_library = std::env::args_os()
        .nth(1)
        .map(std::path::PathBuf::from)
        .filter(|path| {
            let is_dir = path.is_dir();
            if !is_dir {
                log::warn!(
                    "Ignoring startup path that is not a directory: {}",
                    path.display()
                );
            }
            is_dir
        });

    // Run the application using the Application trait with proper window settings
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .window(window_settings(&config.window))
        .scale_factor(move |_| scale_factor)
        .run_with(move || App::with_library(initial_library))
        .map_err(|e| {
            log::error!("Application failed to run: {e}");
            e
        })
}
